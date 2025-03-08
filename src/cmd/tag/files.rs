use std::io::Write;

use bstr::ByteSlice;

use crate::{
    args::{self, Usage, flags, positional},
    extract::ExtractorBuilder,
    parallel::Parallel,
    tag::{Tag, Tagged},
};

const USAGE: &'static str = r#"
Tag file paths by searching their contents via regex.

By default, this will extract tags in well specified datetime formats.
Currently, this includes RFC 9557, RFC 3339, RFC 2822 and RFC 9110 timestamps.

To extract arbitrary tags, use the `-e/--regex` flag to write your own regex.
Then you can use `biff time parse` to parse it into an actual point in time
via strftime-like syntax.

This command is useful when you want to associate one or more tags with an
entire file, rather than with a specific part of a file (as one might do with
`biff tag lines`).

USAGE:
    biff tag files <path> ...
    biff tag files < line delimited <path>

TIP:
    use -h for short docs and --help for long docs

EXAMPLES:
    Extract datetimes from PDF files in standard formats:

        biff tag files *.pdf

    %snip-start%

    Extract publish dates from the HTML files of the burntsushi.net blog. This
    demonstrates how to specify a capturing group that contains the tag instead
    of the full regex match:

        find ./public/ -type f \
            | biff tag files \
                -e '<span class="post-meta">(?<tag>\S+ [0-9]{1,2}, [0-9]{4})'

    %snip-end%
REQUIRED ARGUMENTS:
%args%
OPTIONS:
%flags%
"#;

pub fn run(p: &mut lexopt::Parser) -> anyhow::Result<()> {
    let mut mmapper = flags::MemoryMapper::default();
    let mut extractor = ExtractorBuilder::default();
    let mut config = Config::default();
    let mut args = positional::Arguments::default();
    args::configure(
        p,
        USAGE,
        &mut [&mut mmapper, &mut extractor, &mut config, &mut args],
    )?;

    let extractor = extractor.build()?;
    let mut wtr = std::io::stdout();
    let mut parallel = Parallel::new(
        config.threads.get(),
        move |arg: positional::Argument<'static>| {
            // SAFETY: We generally assume the file we're searching is
            // not going to change. Users generally assume responsibility
            // for this and can use --no-mmap if this isn't appropriate.
            let data = unsafe { mmapper.open(arg.to_path()?)? };
            let haystack = data.as_bytes();
            let mut tagged =
                Tagged::new(arg.original_with_line_terminator().into_owned());
            for range in extractor.find_iter(haystack) {
                let s = haystack[range.clone()].to_str()?.to_string();
                // N.B. We explicitly do not attach the range here, because
                // the range is only meant to be a range into the data in
                // `Tagged`. But the data here is a file path. This is somewhat
                // unfortunate, because it seems like the range could be
                // useful. But alas.
                tagged = tagged.tag(Tag::new(s));
            }
            Ok(tagged)
        },
        move |result: anyhow::Result<Tagged<'static, String>>| {
            let tagged = match result {
                Ok(tagged) => tagged,
                Err(err) => {
                    eprintln!("{err:#}");
                    return Ok(true);
                }
            };
            tagged.write(&mut wtr)?;
            writeln!(wtr)?;
            Ok(true)
        },
    );

    let result1 = args.try_map(|arg| parallel.send(arg.into_owned()));
    let result2 = parallel.wait();
    result1?;
    result2
}

#[derive(Debug, Default)]
struct Config {
    threads: flags::Threads,
}

impl args::Configurable for Config {
    fn configure(
        &mut self,
        p: &mut lexopt::Parser,
        arg: &mut lexopt::Arg,
    ) -> anyhow::Result<bool> {
        match *arg {
            lexopt::Arg::Short('j') | lexopt::Arg::Long("threads") => {
                self.threads = args::parse(p, "-j/--threads")?;
            }
            _ => return Ok(false),
        }
        Ok(true)
    }

    fn usage(&self) -> &[Usage] {
        const PATH: Usage = Usage::arg(
            "<path>",
            "A file path to tag.",
            r#"
A file path to tag.

The contents of the file are searched using either the regexes provided via
the `-e/--regex` flag, or by built-in regexes via the `--auto` flag (which
defaults to `--auto=datetime` when no `-e/--regex` flags are provided).

The search executes in "multi-line" mode. Which is to say, it runs as if the
entire file is contiguously stored on the heap.
"#,
        );
        &[flags::Threads::USAGE, PATH]
    }
}
