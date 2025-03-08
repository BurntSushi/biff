use std::io::Write;

use {
    anyhow::Context,
    bstr::ByteSlice,
    lexopt::{Arg, Parser},
};

use crate::{
    args::{self, Usage, flags},
    extract::ExtractorBuilder,
    parse::BufReadExt,
    tag::{Tag, Tagged},
};

const USAGE: &'static str = r#"
Tag line oriented data.

This command iterates over lines in a single file provided as an argument, or
on data provided to stdin. By default, tags matching well specified datetime
formats will be automatically extracted. Currently, this includes RFC 9557,
RFC 3339, RFC 2822 and RFC 9110 timestamps.

To extract arbitrary tags, use the `-e/--regex` flag to write your own regex.
Then you can use `biff time parse` to parse it into an actual point in time
via strftime-like syntax.

USAGE:
    biff tag lines <path>
    biff tag lines < line delimited data

TIP:
    use -h for short docs and --help for long docs

EXAMPLES:
    Extract datetimes from each line in a Caddy log file and reformat them
    into local time in a format of your choosing:

        biff tag lines < access.log \
            | biff time fmt -f '%B %-d, %Y at %H:%M:%S' \
            | biff untag --substitute

POSITIONAL ARGUMENTS:
%args%
OPTIONS:
%flags%
"#;

pub fn run(p: &mut Parser) -> anyhow::Result<()> {
    let mut extractor = ExtractorBuilder::default();
    let mut config = Config::default();
    args::configure(p, USAGE, &mut [&mut extractor, &mut config])?;

    let extractor = extractor.build()?;
    let mut wtr = std::io::stdout().lock();
    let result = config.input.reader()?.for_byte_line(|line| {
        let haystack = line.content();
        let mut tagged = Tagged::new(line.full());
        for range in extractor.find_iter(haystack) {
            let s = haystack[range.clone()].to_str()?;
            tagged = tagged.tag(Tag::new(s).with_range(range));
        }
        tagged.write(&mut wtr)?;
        writeln!(wtr)?;
        Ok(true)
    });
    result.with_context(|| format!("{}", config.input.display()))?;
    Ok(())
}

#[derive(Debug, Default)]
struct Config {
    input: flags::FileOrStdin,
}

impl args::Configurable for Config {
    fn configure(
        &mut self,
        _: &mut Parser,
        arg: &mut Arg,
    ) -> anyhow::Result<bool> {
        match *arg {
            Arg::Value(ref mut v) => {
                self.input.set(std::mem::take(v))?;
            }
            _ => return Ok(false),
        }
        Ok(true)
    }

    fn usage(&self) -> &[Usage] {
        const PATH: Usage = Usage::arg(
            "<path>",
            "A file path to read lines from.",
            r#"
A file path to read lines from.

In lieu of a specific file path, users may also pass line delimited data into
stdin.
"#,
        );
        &[PATH]
    }
}
