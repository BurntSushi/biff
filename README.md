Biff
====
A command line tool for datetime arithmetic, parsing, formatting and more.

[![Build status](https://github.com/BurntSushi/biff/workflows/ci/badge.svg)](https://github.com/BurntSushi/biff/actions)
[![Crates.io](https://img.shields.io/crates/v/biff-datetime-cli.svg)](https://crates.io/crates/biff-datetime-cli)

Dual-licensed under MIT or the [UNLICENSE](https://unlicense.org/).

### CHANGELOG

Please see the [CHANGELOG](CHANGELOG.md) for a release history.

### Documentation

The [user guide] should be your first stop for understanding the high level
concepts that Biff deals with. Otherwise, consult `biff --help` or
`biff <sub-command> --help` for more specific details.

Alternatively, there is a [comparison] between other similar tools that might
give you a quick sense of what Biff is like.

### Brief Examples

Print the current time:

```console
$ biff
Sat, May 10, 2025, 8:02:04 AM EDT
```

> [!TIP]
> If you get output like `2025 M05 10, Mon 08:02:04` instead, that's because
> you likely don't have [locale support][locale] support configured. That
> requires setting `BIFF_LOCALE` and using a release binary or building Biff
> with the `locale` feature enabled.

Print the current time in a format of your choosing:

```console
$ biff time fmt -f rfc3339 now
2025-05-10T08:08:30.101066734-04:00

$ biff time fmt -f rfc9557 now
2025-05-10T08:08:33.420946447-04:00[America/New_York]

$ biff time fmt -f '%Y-%m-%d %H:%M:%S %Z' now
2025-05-10 08:08:48 EDT
```

Print multiple relative times in one command:

```console
$ biff time fmt -f '%c' now -1d 'next sat' 'last monday' '9pm last mon'
Sat, May 10, 2025, 10:44:39 AM EDT
Fri, May 9, 2025, 10:44:39 AM EDT
Sat, May 17, 2025, 10:44:39 AM EDT
Mon, May 5, 2025, 10:44:39 AM EDT
Mon, May 5, 2025, 9:00:00 PM EDT
```

Print the current time in another time zone, and round it the nearest 15 minute
increment:

```console
$ biff time in Asia/Bangkok now | biff time round -i 15 -s minute
2025-05-10T19:15:00+07:00[Asia/Bangkok]
```

Add a duration to the current time:

```console
$ biff time add -1w now
2025-05-03T10:34:30.819577918-04:00[America/New_York]

$ biff time add '1 week, 12 hours ago' now
2025-05-02T22:34:44.114109514-04:00[America/New_York]

$ biff time add 6mo now
2025-11-10T10:34:49.023321635-05:00[America/New_York]
```

Find the duration since a date in the past and round it to the desired
precision:

```console
$ biff span since 2025-01-20T12:00
2636h 1m 21s 324ms 691µs 216ns

$ biff span since 2025-01-20T12:00 -l year
3mo 20d 21h 1m 25s 171ms 886µs 534ns

$ biff span since 2025-01-20T12:00 | biff span round -l year -s day
3mo 18d

$ biff span since 2025-01-20T12:00 | biff span round -l day -s day
110d
```

Find timestamps in a log file and reformat them into your local time in place:

```console
$ head -n3 /tmp/access.log
2025-04-30T05:25:14Z    INFO    http.log.access.log0    handled request
2025-04-30T05:25:17Z    INFO    http.log.access.log0    handled request
2025-04-30T05:25:18Z    INFO    http.log.access.log0    handled request

$ biff tag lines /tmp/access.log | biff time in system | biff time fmt -f '%c' | head -n3 | biff untag -s
Wed, Apr 30, 2025, 1:25:14 AM EDT       INFO    http.log.access.log0    handled request
Wed, Apr 30, 2025, 1:25:17 AM EDT       INFO    http.log.access.log0    handled request
Wed, Apr 30, 2025, 1:25:18 AM EDT       INFO    http.log.access.log0    handled request
```

Generate a sequence of the next 5 days that are Monday, Wednesday or Friday
at a specific time, and then format them in your locale:

```console
$ biff time seq day today -c5 -H 9 -w mon,wed,fri | biff time fmt -f '%c'
Mon, May 12, 2025, 9:00:00 AM EDT
Wed, May 14, 2025, 9:00:00 AM EDT
Fri, May 16, 2025, 9:00:00 AM EDT
Mon, May 19, 2025, 9:00:00 AM EDT
Wed, May 21, 2025, 9:00:00 AM EDT
```

Print every day remaining in the current month:

```console
$ biff time seq daily --until $(biff time end-of month now) today
2025-05-10T00:00:00-04:00[America/New_York]
2025-05-11T00:00:00-04:00[America/New_York]
2025-05-12T00:00:00-04:00[America/New_York]
2025-05-13T00:00:00-04:00[America/New_York]
[.. snip ..]
```

Find the last weekday in each of the next 12 months and print them in a
succinct format:

```console
$ biff time seq -c12 monthly -w mon,tue,wed,thu,fri --set-position -1 | biff time fmt -f '%a, %Y-%m-%d'
Fri, 2025-05-30
Mon, 2025-06-30
Thu, 2025-07-31
Fri, 2025-08-29
Tue, 2025-09-30
Fri, 2025-10-31
Fri, 2025-11-28
Wed, 2025-12-31
Fri, 2026-01-30
Fri, 2026-02-27
Tue, 2026-03-31
Thu, 2026-04-30
```

Or print the second Tuesday of each month until the end of the year:

```console
$ biff time seq monthly -w 2-tue --until $(biff time end-of year now) | biff time fmt -f '%a, %F'
Tue, 2025-05-13
Tue, 2025-06-10
Tue, 2025-07-08
Tue, 2025-08-12
Tue, 2025-09-09
Tue, 2025-10-14
Tue, 2025-11-11
Tue, 2025-12-09
```

Finally, this command will get the last commit date on each file in a git
repository, sort them in ascending order, format the datetime to a fixed-width
format and then print the data in a tabular format:

```console
$ git ls-files \
    | biff tag exec git log -n1 --format='%aI' \
    | biff time sort \
    | biff time fmt -f '%Y-%m-%d %H:%M:%S %z' \
    | biff untag -f '{tag}\t{data}'
[.. snip ..]
2025-05-05 21:54:09 -0400       src/tz/timezone.rs
2025-05-05 21:54:09 -0400       src/tz/tzif.rs
2025-05-05 22:06:38 -0400       Cargo.toml
2025-05-05 22:06:38 -0400       crates/jiff-static/Cargo.toml
2025-05-07 18:55:23 -0400       CHANGELOG.md
2025-05-07 18:55:23 -0400       scripts/test-various-feature-combos
2025-05-07 18:55:23 -0400       src/error.rs
2025-05-08 08:38:22 -0400       src/tz/system/mod.rs
2025-05-08 16:52:55 -0400       crates/jiff-icu/Cargo.toml
2025-05-08 16:52:55 -0400       crates/jiff-icu/src/lib.rs
```

To see more examples, check out the [user guide] or the [comparison] between
Biff and other datetime command line tools.

### Installation

The binary name for Biff is `biff`. It is also on
[crates.io under the name `biff-datetime-cli`](https://crates.io/crates/biff-datetime-cli).

**[Archives of precompiled binaries for Biff are available for Windows,
macOS and Linux.](https://github.com/BurntSushi/biff/releases)** Linux and
Windows binaries are static executables.

Alternatively, if you're a **Rust programmer**, Biff can be installed with
`cargo`. Note that the binary may be bigger than expected because it contains
debug symbols. This is intentional. To remove debug symbols and therefore
reduce the file size, run `strip` on the binary.

```console
cargo install biff-datetime-cli
```

Or, if you want [locale support][locale] (which is enabled in the
binaries distributed on GitHub), then install with the `locale` feature
enabled:

```console
cargo install biff-datetime-cli --features locale
```

### Biff as a library

There is relatively little datetime logic inside of Biff proper.
(Except for its RFC 5545 implementation, which may eventually move
out to a library.) Most of the datetime logic is instead provided by
[Jiff](https://github.com/BurntSushi/jiff). Additionally, localization is
provided by [ICU4X](https://docs.rs/icu) and integrated with Jiff via
[jiff-icu](https://docs.rs/jiff-icu).

### WARNING

I may ship arbitrary and capricious breaking changes at this point. You have
been warned.

Also, no compatibility with `date` is intended. This is not a drop-in
replacement. It is not intended to be. It never will be. And it doesn't give
a hoot about POSIX (other than the `TZ` environment variable). If you need a
`date` compatible program, then go use an implementation of POSIX `date`.
With that said, Biff's `biff time fmt` command generally supports a `strftime`
syntax that has a large amount of compatibility with GNU `date`.

If you have use cases serviced by `date` that aren't possible with Biff, I'd
like to hear about them.

### Motivation

I built this tool primarily as a way to expose some of the library
functionality offered by [Jiff](https://github.com/BurntSushi/jiff) on the
command line. I was after a succinct way to format datetimes or do arithmetic.
So I built this tool.

`date` is one of those commands that I use infrequently enough, and its flags
and behavior is weird enough, that I constantly have to re-read its manual in
order to use it effectively. So perhaps there is room for improvement there.

As I progressed in constructing this tool, I quickly found it somewhat limited
by the fact that the *only* data it could process was datetimes. To make Biff
much more versatile, I added a `biff tag` command that looks for datetimes in
arbitrary data and wraps them up into a JSON lines format. It's unclear to me
how broadly useful folks will find this functionality, but other datetime
utilities don't seem to have it.

I also wanted to use Jiff in "anger," and in particular, as part of confidently
getting it to a 1.0 state. Is its performance acceptable? Are there APIs
missing that are needed for real world programs? And so on. For example,
because of my development on Biff, I added a way to hook ICU4X localization
into Jiff's `jiff::fmt::strtime` APIs.

### Building

Biff is written in Rust, so you'll need to grab a
[Rust installation](https://www.rust-lang.org/) in order to compile it.

To build Biff:

```console
git clone https://github.com/BurntSushi/biff
cd biff
cargo build --release
./target/release/biff --version
```

Additionally, optional locale support can be built with Biff by enabling the
`locale` feature:

```console
cargo build --release --features locale
```

Biff can be built with the musl target on Linux by first installing the musl
library on your system (consult your friendly neighborhood package manager).
Then you just need to add musl support to your Rust toolchain and rebuild
Biff, which yields a fully static executable:

```console
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
```

Applying the `--features locale` flag from above should also work.

### Running tests

To run both unit tests and integration tests, use:

```console
cargo test
```

from the repository root. If you're hacking on Biff and need to change or
add tests, Biff makes heavy use of [cargo insta] for snapshot testing. For
example, to run tests with Insta, use:

```console
cargo insta test
```

And if there are any snapshots to review, you can review them via:

```console
cargo insta review
```

[cargo insta]: https://insta.rs/docs/cli/
[locale]: ./GUIDE.md#localization
[user guide]: ./GUIDE.md
[comparison]: ./COMPARISON.md
