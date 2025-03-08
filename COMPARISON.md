# Comparison with similar tools

## Table of Contents

* [GNU date](#gnu-date)
* [dateutils](#dateutils)

## GNU date

The `date` command is specified by POSIX, and thus, there are multiple
implementations of it. Here, we just consider GNU date, which is part of
[GNU coreutils].

As a modern suite of tools that follow the Unix philosophy, Biff relies
heavily on composition. In contrast, GNU date has more limited functionality
and discourages many forms of composition. For example, let's say you have a
stream of datetimes, and you want to add 1 week to each. You can mostly
accomplish this with GNU date by taking advantage of its arithmetic
embedded into its datetime parsing:

```console
$ printf '2025-05-01T00-04\n2025-07-31T00-04\n' \
    | awk '{ print $1 " + 1 week"}' \
    | date -f -
Thu May  8 12:00:00 AM EDT 2025
Thu Aug  7 12:00:00 AM EDT 2025
```

With Biff, this is much more streamlined:

```console
$ printf '2025-05-01T00-04\n2025-07-31T00-04\n' | biff time add 1w
2025-05-08T00:00:00-04:00[-04:00]
2025-08-07T00:00:00-04:00[-04:00]
```

That is, you don't have to mold everything into GNU date's own custom
datetime parsing. This also of course means you can do things with Biff that
you can't accomplish with GNU date's datetime parser. Like getting the end of
the month that a particular datetime resides in:

```console
$ printf '2025-05-01T00-04\n2025-07-31T00-04\n' | biff time end-of month
2025-05-31T23:59:59.999999999-04:00[-04:00]
2025-07-31T23:59:59.999999999-04:00[-04:00]
```

Or finding the duration between two datetimes:

```console
$ printf '2025-05-01T00-04\n2025-07-31T00-04\n' \
    | biff time in system
    | biff span since -l year -r 2025-11-01
6mo
3mo 1d
```

Or sorting datetimes:

```console
$ printf '2025-05-01T00-04\n2025-07-31T00-04\n' | biff time sort -r
2025-07-31T00:00:00-04:00[-04:00]
2025-05-01T00:00:00-04:00[-04:00]
```

Separately from composition, Biff also supports a lot of things that GNU
date does not. For example, Biff lets you use a `strptime`-like API to parse
datetimes. Consider if you want to parse a `DD/MM/YY` format in the United
States:

```console
$ date -d '05/09/25'
Fri May  9 12:00:00 AM EDT 2025
```

This is wrong for our purposes, since we want September 5. And even changing
the locale doesn't help either:

```console
$ LC_ALL=en_GB date -d '05/09/25'
Fri May  9 00:00:00 EDT 2025
```

With Biff, you can just use `strptime` to parse it as needed:

```console
$ biff time parse -f '%d/%m/%y' 05/09/25 | biff time fmt -f '%c'
Fri, Sep 5, 2025, 12:00:00 AM EDT
```

Biff also provides first class support for duration strings like
`2 weeks, 5 hours ago`, including rounding durations. And Biff also supports
tagging arbitrary data with datetimes, which makes it effortless to, e.g.,
reformat UTC timestamps in log files to your local time on-the-fly:

```console
$ head -n1 /tmp/access.log
2025-04-30T05:25:14Z    INFO    http.log.access.log0    handled request

$ biff tag lines /tmp/access.log | biff time in system | biff time fmt -f '%c' | head -n1 | biff untag -s
Wed, Apr 30, 2025, 1:25:14 AM EDT       INFO    http.log.access.log0    handled request
```

## dateutils

[dateutils] is a suite of distinct programs for datetime handling. Like Biff,
it does a lot more than what GNU date can do. Moreover, like Biff, it enables
composition. For example, here's how to print 5 dates and the duration from
a reference point using dateutils:

```console
$ dateseq 2010-01-01 2010-01-05 | datediff 2009-12-01 -f '%d days'
31 days
32 days
33 days
34 days
35 days
```

And here's the Biff equivalent:

```console
$ biff time seq daily 2010-01-01 -c5 | biff span until -l year -r 2009-12-01
1mo
1mo 1d
1mo 2d
1mo 3d
1mo 4d
```

Overall, Biff and dateutils serve similar use cases. dateutils does seem to
have more of a focus on civil dates, where as Biff is more deeply connected to
instants in a time via deeper integration with time zones. The biggest features
Biff probably has over dateutils are:

* More comprehensive support for generating sequences of datetimes (thanks to
Biff's RFC 5545 implementation).
* Support for tagging arbitrary data into JSON lines with datetimes. All Biff
commands can then accept this data on stdin and transform tagged datetime
values.
* Biff has better handling for rounding time spans.

With that said, there are currently some use cases that are easier with
dateutils. While its "tagging support" isn't quite as sophisticated as Biff's,
it does have some support for transforming datetimes within arbitrary data. For
example, this rounds a stream of dates strictly to the first of the next month:

```console
$ cat todo
pay cable	2012-02-28
pay gas	2012-02-29
pay rent	2012-03-01
redeem loan	2012-03-02

$ dateround -S -n 1 < todo
pay cable       2012-03-01
pay gas 2012-03-01
pay rent        2012-04-01
redeem loan     2012-04-01
```

Biff can do this too, but it's a fair bit more annoying at present:

```console
$ biff tag lines -e '[0-9]{4}-[0-9]{2}-[0-9]{2}' todo \
    | biff time parse -f flexible \
    | biff time add 1mo \
    | biff time start-of month \
    | biff time fmt -f '%F' \
    | biff untag -s
pay cable       2012-03-01
pay gas 2012-03-01
pay rent        2012-04-01
redeem loan     2012-04-01
```

While I personally don't mind doing the "add 1 month and then get the start of
the month on the result" in two steps, there is a lot more ceremony here
involved in finding, parsing and formatting the dates. But Biff does have the
infrastructure in place to deal with datetimes within arbitrary data.

Dateutils does have its own version of duration rounding via its duration
format strings:

```console
$ datediff 2025-03-20 now -f '%m months %d days'
1 months 20 days

$ datediff 2025-03-20 now -f '%d days'
51 days
```

Biff does something similar (with a few more bells and whistles) via a
dedicated duration rounding command:

```console
$ biff span since 2025-03-20 | biff span round -l year -s day
1mo 20d

$ biff span since 2025-03-20 | biff span round -l day -s day
51d
```

Biff's duration rounding is a bit more declarative. You specify the bounds of
units you want, and Biff does the rest. In contrast, with dateutils, it's hard
to express concepts like "show durations with units up to years, but start the
duration with a non-zero unit." For example, here you get `0 years`, which is
a little odd:

```console
$ datediff 2025-03-20 now -f '%y years, %m months, %d days'
0 years, 1 months, 20 days
```

Another thing Biff does for duration rounding is that it knows that not all
days are necessarily 24 hours. That is, duration rounding is aware of daylight
saving time. For example, most days are 24 hours, and so rounding 11.75h to the
nearest day in most cases will result in a zero span:

```console
$ biff span round -s day -r '2025-03-10[America/New_York]' 11.75h
0s
```

But 2025-03-09 in New York was only 23 hours. So rounding 11.75h to the nearest
day will actually round up:

```console
$ biff span round -s day -r '2025-03-09[America/New_York]' 11.75h
1d
```

dateutils doesn't really have the command line interface to deal with this
sort of thing (as far as I can tell). The key is that rounding a time span
is always tied to a reference datetime.

A good example of something dateutils can't really do (I think) is extract
datetimes, associate them with arbitrary data, do some transformations,
filtering or sorting and then stitch everything back together again. For
example, this command will get the last commit date on each file in a git
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

[dateutils]: https://www.fresse.org/dateutils/
[GNU coreutils]: https://www.gnu.org/software/coreutils/
