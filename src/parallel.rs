use std::{
    cmp::Ordering,
    collections::BinaryHeap,
    num::NonZero,
    sync::Arc,
    thread::{self, JoinHandle},
};

use flume::{Receiver, Sender};

/// An abstraction for running things in parallel via a fixed pool of workers.
///
/// This isn't the best abstraction in the world, and is probably only suitable
/// for cases where the unit of work is pretty big. For example, like spawning
/// a process.
///
/// It does have a few benefits.
///
/// Firstly, when created with `threads == 1`, it won't actually spin up
/// any threads. Instead, `Parallel::send` runs the work inline. This way,
/// you don't to pay the overhead of synchronization when running in single
/// threaded mode.
///
/// Secondly, it guarantees that the `done` function provided is run on the
/// outputs of `f` in precisely the same order as the inputs are given to
/// `Parallel::send`. In other words, you get the benefits of parallelism
/// without sacrificing ordering.
///
/// Thirdly, its API works in a "streaming" context. That is, you don't need
/// to load the full set of inputs into memory first, and you'll start getting
/// output before all of the work is done if your `done` function does the
/// printing. If you do already need your full data in memory, then likely some
/// other approach to parallelism is better.
///
/// Unfortunately, it does have some downsides.
///
/// Firstly, all of the closures are `'static`, which means that if your code
/// would normally used borrowed data in single threaded mode, it nows needs to
/// use owned data. This potentially involves extra copying, even when
/// `threads == 1`. (Thus, this is not a zero cost abstraction.)
///
/// Secondly, there's no work stealing and a min-heap is used to enforce
/// ordering. These likely have a cost, especially if the unit of work is very
/// small.
///
/// # Future work
///
/// One thing that I think might be nice to add to this is a buffering
/// mechanism. That is, `Parallel::send` might batch up inputs before sending
/// them to a thread. This would help, I think, cases where the unit of work
/// is very small and mitigate some of the downsides listed above.
pub struct Parallel<I, O> {
    kind: ParallelKind<I, O>,
}

impl<I: Send + 'static, O: Send + 'static> Parallel<I, O> {
    /// Create a new manager that runs `f` simultaneously on a number of
    /// `threads`. After each `f` is run, its output is sent back to this
    /// thread and `done` is called on it. If `done` reports an error, then the
    /// manager stops everything and returns that error.
    ///
    /// It is guaranteed that `done` is called on the outputs of `f` in
    /// precisely the same order as the corresponding inputs were given to
    /// the manager via `Parallel::send`.
    pub fn new(
        threads: NonZero<usize>,
        f: impl Fn(I) -> O + Send + Sync + 'static,
        done: impl FnMut(O) -> anyhow::Result<bool> + Send + Sync + 'static,
    ) -> Parallel<I, O> {
        let done = Box::new(done);
        if threads.get() == 1 {
            let f = Box::new(f);
            let kind = ParallelKind::Single { f, done };
            return Parallel { kind };
        }

        let (inputs_send, inputs_recv) = flume::bounded(threads.get());
        let (outputs_send, outputs_recv) = flume::bounded(threads.get());
        let f = Arc::new(f);
        let mut workers = vec![];
        for _ in 0..threads.get() {
            workers.push(Worker::run(
                inputs_recv.clone(),
                outputs_send.clone(),
                f.clone(),
            ));
        }
        let done = Done::run(outputs_recv, done);
        let kind = ParallelKind::Multi {
            sequence: 0,
            inputs: inputs_send,
            workers,
            done,
        };
        Parallel { kind }
    }

    /// Queue a unit of work to be run in parallel.
    ///
    /// In single threaded mode, the unit of work is run synchronously.
    pub fn send(&mut self, value: I) -> anyhow::Result<bool> {
        match self.kind {
            ParallelKind::Single { ref f, ref mut done } => done(f(value)),
            ParallelKind::Multi { ref mut sequence, ref inputs, .. } => {
                let input = Input { sequence: *sequence, value };
                *sequence += 1;
                Ok(inputs.send(input).is_ok())
            }
        }
    }

    /// Wait until all outstanding units of work have finished and return an
    /// error if any of them failed.
    ///
    /// Callers should only use then when all inputs have been sent to the
    /// manager.
    pub fn wait(self) -> anyhow::Result<()> {
        match self.kind {
            ParallelKind::Single { .. } => Ok(()),
            ParallelKind::Multi { inputs, workers, done, .. } => {
                // We're all done sending inputs, so
                // drop the channel and let the workers
                // drain.
                drop(inputs);
                for worker in workers {
                    // propagate panics from the worker
                    worker.handle.join().unwrap();
                }
                // And now that the workers have
                // finished, all of the output sending
                // channels have been dropped. So now
                // let's wait for the `done` thread to
                // drain and finish.
                done.handle.join().unwrap()
            }
        }
    }
}

enum ParallelKind<I, O> {
    Single {
        f: Box<dyn Fn(I) -> O + Send + Sync + 'static>,
        done:
            Box<dyn FnMut(O) -> anyhow::Result<bool> + Send + Sync + 'static>,
    },
    Multi {
        sequence: u64,
        inputs: Sender<Input<I>>,
        workers: Vec<Worker>,
        done: Done,
    },
}

struct Worker {
    handle: JoinHandle<()>,
}

impl Worker {
    fn run<I, O>(
        inputs: Receiver<Input<I>>,
        outputs: Sender<Output<O>>,
        f: Arc<dyn Fn(I) -> O + Send + Sync + 'static>,
    ) -> Worker
    where
        I: Send + 'static,
        O: Send + 'static,
    {
        let handle = thread::spawn(move || {
            for input in inputs {
                let sequence = input.sequence;
                let value = f(input.value);
                // If all receivers have been dropped, this is
                // because the `done` thread has finished prematurely
                // as a result of an error. When that happens, just
                // give up.
                if outputs.send(Output { sequence, value }).is_err() {
                    return;
                }
            }
        });
        Worker { handle }
    }
}

struct Done {
    handle: JoinHandle<anyhow::Result<()>>,
}

impl Done {
    fn run<O>(
        outputs: Receiver<Output<O>>,
        mut done: Box<
            dyn FnMut(O) -> anyhow::Result<bool> + Send + Sync + 'static,
        >,
    ) -> Done
    where
        O: Send + 'static,
    {
        let handle = thread::spawn(move || {
            let mut sequence = 0;
            let mut queue: BinaryHeap<Output<O>> = BinaryHeap::new();
            for output in outputs {
                queue.push(output);
                while queue.peek().map_or(false, |o| o.sequence == sequence) {
                    let o = queue.pop().unwrap();
                    done(o.value)?;
                    sequence += 1;
                }
            }
            Ok(())
        });
        Done { handle }
    }
}

/// A wrapper around the input values of `Parallel`.
///
/// Each input gets assigned a monotonically increasing sequence number. This
/// is used to ensure ordering of outputs.
struct Input<V> {
    sequence: u64,
    value: V,
}

/// A wrapper around the output values of `Parallel`.
///
/// The sequence number on this corresponds to the sequence number of the input
/// that generated it.
///
/// The `Ord` implementation reverses the comparison on `sequence` so that this
/// makes `BinaryHeap` behave like a min-heap (it is a max-heap by default).
struct Output<V> {
    sequence: u64,
    value: V,
}

impl<V> Eq for Output<V> {}

impl<V> PartialEq for Output<V> {
    fn eq(&self, rhs: &Output<V>) -> bool {
        self.sequence == rhs.sequence
    }
}

impl<V> Ord for Output<V> {
    fn cmp(&self, rhs: &Output<V>) -> Ordering {
        self.sequence.cmp(&rhs.sequence).reverse()
    }
}

impl<V> PartialOrd for Output<V> {
    fn partial_cmp(&self, rhs: &Output<V>) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}
