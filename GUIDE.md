# User Guide

## Table of Contents

* [Command Structure](#command-structure)
* [Relative Datetimes](#relative-datetimes)
* [Datetime Formatting](#datetime-formatting)
* [Datetime Arithmetic](#datetime-arithmetic)
* [Duration Formatting](#duration-formatting)
* [Duration Rounding](#duration-rounding)
* [Composition](#composition)
* [Tagging](#tagging)
* [Datetime Sequences](#datetime-sequences)
* [Time Zones](#time-zones)
* [Localization](#localization)

## Command Structure

Biff contains _many_ commands, but most follow a pattern:

```console
$ biff <output-type> <action>
```

That is, the first sub-command indicates the thing you want in the output and
the second sub-command indicates the action you want to perform. So for
example, if you want to format a time, then you'd use:

```console
$ biff time fmt -f rfc2822 now
Sat, 3 May 2025 09:39:11 -0400
```

Or if you want the time span since a date, then you'd use:

```console
$ biff span since 1973-01-05
458672h 40m 8s 762ms 993µs 765ns

$ biff span since -l year 1973-01-05
52y 3mo 29d 9h 41m 57s 679ms 313µs 892ns
```

Similarly, if you're looking for time zones as output, then you'll want to use
`biff tz` followed by an action sub-command:

```console
$ biff tz compatible '1952-10-01T23:59:59-11:19:40'
Pacific/Niue

$ biff tz compatible '2025-05-01T17:30-01'
America/Godthab
America/Nuuk
America/Scoresbysund
Atlantic/Cape_Verde
Etc/GMT+1

$ biff tz compatible '2025-05-01T17:30+05:30'
Asia/Calcutta
Asia/Colombo
Asia/Kolkata
```

One exception to this pattern are the time zone transition commands:

```console
$ biff tz prev Asia/Calcutta now
1945-10-14T23:00:00+05:30[Asia/Calcutta]

$ biff tz next America/New_York now
2025-11-02T01:00:00-05:00[America/New_York]

$ biff tz seq -c5 America/New_York
2025-11-02T01:00:00-05:00[America/New_York]
2026-03-08T03:00:00-04:00[America/New_York]
2026-11-01T01:00:00-05:00[America/New_York]
2027-03-14T03:00:00-04:00[America/New_York]
2027-11-07T01:00:00-05:00[America/New_York]
```

These commands are niche enough and very explicitly tied to time zones that
they fall under the `biff tz` command even though the output type is a
datetime.

## Relative Datetimes

One of the most common types of inputs to Biff is a datetime. As such, Biff is
very flexible in the kinds of ways that one can write a datetime.

Firstly, the standard formats are accepted. This includes RFC 2822, RFC 3339,
RFC 9557 and ISO 8601:

```console
$ biff time fmt 'Sat, 3 May 2025 17:30:00 -0400'
2025-05-03T17:30:00-04:00[-04:00]

$ biff time fmt '2025-05-03T17:30:00-04'
2025-05-03T17:30:00-04:00[-04:00]

$ biff time fmt '2025-05-03T17:30+07[Asia/Bangkok]'
2025-05-03T17:30:00+07:00[Asia/Bangkok]

$ biff time fmt '20250503T173000-0400'
2025-05-03T17:30:00-04:00[-04:00]
```

Or even some mixtures of RFC 3339 (the `T` separator may be omitted) and ISO
8601 (the `:` separators may be omitted):

```console
$ biff time fmt '20250503 173000-0400'
2025-05-03T17:30:00-04:00[-04:00]
```

Secondly, civil dates and times are supported as well. They are automatically
interpreted with respect to your system's default time zone:

```console
$ ls -l /etc/localtime
lrwxrwxrwx 1 root root 36 May  4  2022 /etc/localtime -> /usr/share/zoneinfo/America/New_York

$ biff time fmt now
2025-05-03T11:05:18.425555496-04:00[America/New_York]

$ TZ=Asia/Shanghai biff time fmt 2025-05-03
2025-05-03T00:00:00+08:00[Asia/Shanghai]
```

When only a time is given, the current date is used automatically:

```console
$ TZ=Asia/Kolkata biff time fmt 17:30
2025-05-03T17:30:00+05:30[Asia/Kolkata]
```

Thirdly, relative datetimes can be given. A number of formats can be given.
These examples show some special keywords:

```console
$ biff time fmt now
2025-05-03T13:47:39.849207318-04:00[America/New_York]

$ biff time fmt today
2025-05-03T00:00:00-04:00[America/New_York]

$ biff time fmt yesterday
2025-05-02T00:00:00-04:00[America/New_York]

$ biff time fmt tomorrow
2025-05-04T00:00:00-04:00[America/New_York]
```

These examples show how to write spans that are interpreted relative to the
current time:

```console
$ biff time fmt now
2025-05-03T13:56:24.317006753-04:00[America/New_York]

$ biff time fmt 1d
2025-05-04T13:56:26.110334233-04:00[America/New_York]

$ biff time fmt -1d
2025-05-02T13:56:27.885632425-04:00[America/New_York]

$ biff time fmt '1 day ago'
2025-05-02T13:56:31.190976094-04:00[America/New_York]

$ biff time fmt -1y6mo2w3d18h
2023-10-16T19:57:44.218605101-04:00[America/New_York]

$ biff time fmt '1 year, 6 months, 2 weeks, 3 days, 18 hrs ago'
2023-10-16T19:57:53.736077358-04:00[America/New_York]

$ biff time fmt '-P1Y6M2W3DT18H'
2023-10-16T19:58:40.223065286-04:00[America/New_York]

$ biff time fmt '-P1y6m2w3dT18h'
2023-10-16T19:58:40.223065286-04:00[America/New_York]
```

You can also write relative datetimes based on the weekday:

```console
$ biff time fmt 'this saturday'
2025-05-03T13:59:32.480322775-04:00[America/New_York]

$ biff time fmt '21:00 this saturday'
2025-05-03T21:00:00-04:00[America/New_York]

$ biff time fmt '9pm this saturday'
2025-05-03T21:00:00-04:00[America/New_York]

$ biff time fmt 'next sat'
2025-05-10T13:59:59.465410937-04:00[America/New_York]

$ biff time fmt 'last sat'
2025-04-26T14:00:02.112850205-04:00[America/New_York]

$ biff time fmt 'second sat'
2025-05-17T14:00:14.288219217-04:00[America/New_York]

$ biff time fmt 'third sat'
2025-05-24T14:00:26.054331564-04:00[America/New_York]
```

Finally, Biff's `strptime` functionality can be used to parse other formats
not supported above. For example, an ISO 8601 week date:

```console
$ biff time parse -f '%G-W%V-%u' 2025-W18-6
2025-05-03T00:00:00-04:00[America/New_York]
```

Or a U.S.-style datetime:

```console
$ biff time parse -f '%m/%d/%y at %I:%M%P' '5/3/25 at 2:08pm'
2025-05-03T14:08:00-04:00[America/New_York]
```

## Datetime Formatting

The `biff time fmt` command can be used to format datetimes in a variety of
well known formats:

```console
$ biff time fmt -f rfc9557 now
2025-05-04T14:52:14.954155783-04:00[America/New_York]

$ biff time fmt -f rfc3339 now
2025-05-04T14:52:20.249420725-04:00

$ biff time fmt -f rfc2822 now
Sun, 4 May 2025 14:52:23 -0400

$ biff time fmt -f rfc9110 now
Sun, 04 May 2025 18:52:32 GMT
```

Just as with `biff time parse`, bespoke formats are also supported via Biff's
`strftime` functionality:

```console
$ biff time fmt -f '%G-W%V-%u' now
2025-W18-7

$ biff time fmt -f '%Y-%m-%d %H:%M:%S%.f %z' now
2025-05-04 14:56:05.008306486 -0400

$ biff time fmt -f '%Y-%m-%d %H:%M:%S%.f %z' today
2025-05-04 00:00:00 -0400
```

If Biff was built with [locale support][localization], then `%c`, `%r`, `%X`
and `%x` are all locale aware:

```console
$ BIFF_LOCALE=en-GB TZ=Europe/London biff time fmt -f '%c' now
Fri, 2 May 2025, 19:31:41 BST

$ BIFF_LOCALE=en-GB TZ=Europe/London biff time fmt -f '%r' now
7:31:46 pm

$ BIFF_LOCALE=en-GB TZ=Europe/London biff time fmt -f '%X' now
19:31:49

$ BIFF_LOCALE=en-GB TZ=Europe/London biff time fmt -f '%x' now
2 May 2025
```

Without locale support, Biff will behave as if using Unicode's "undetermined"
locale:

```console
$ biff time fmt -f '%c' now
2025 M05 4, Sun 15:02:31
```

If you want to format a datetime in a way that is the same as the "POSIX"
locale, then you can specify the formatting string to do so manually:

```console
$ biff time fmt -f '%a %b %e %H:%M:%S %Y' now
Sun May  4 15:05:04 2025

$ biff time fmt -f '%a %b %e %H:%M:%S %Z %Y' now
Sun May  4 15:05:41 EDT 2025
```

## Datetime Arithmetic

Biff makes it very easy to add a time span to a datetime:

```console
$ biff time add 1d now
2025-05-04T14:36:58.826354541-04:00[America/New_York]

$ biff time add now 1d
2025-05-04T14:37:01.565019185-04:00[America/New_York]
```

Or add one time span to multiple datetimes:

```console
$ biff time add 1month 2025-05-01 2025-07-15
2025-06-01T00:00:00-04:00[America/New_York]
2025-08-15T00:00:00-04:00[America/New_York]
```

Or add multiple time spans to one datetime:

```console
$ biff time add 2025-03-08T17:30 24h 1d
2025-03-09T18:30:00-04:00[America/New_York]
2025-03-09T17:30:00-04:00[America/New_York]
```

(Note: The above demonstrates that days are not always 24 hours. That's because
at 2am on 2025-03-09, `America/New_York` entered daylight saving time.)

Or subtract a time span from a datetime:

```console
$ biff time add -1mo 2025-05-01 2025-07-15
2025-04-01T00:00:00-04:00[America/New_York]
2025-06-15T00:00:00-04:00[America/New_York]
```

You can also get the time span since the current time:

```console
$ biff span since 1973-01-05
458696h 49m 51s 892ms 477µs 464ns
```

Or if you want to extend the duration up to the largest possible unit:

```console
$ biff span since -l year 1973-01-05
52y 3mo 30d 9h 49m 58s 332ms 31µs 494ns
```

Or get the span since a particular time relative to a time other than the
current time:

```console
$ biff span since -l year -r 2023-09-30 2010-08-01
13y 1mo 29d
```

The span defaults to using hours as the largest unit so that the operation is
guaranteed to be reversible. That is, units of hours or lower always have an
exact meaning and their actual duration never varies. Conversely, calendar
units (units of days or greater) can vary depending on the datetime they are
relative to. For example, 2025-03-09 is only 23 hours long in New York, but
24 hours in London:

```console
$ TZ=America/New_York biff span since -r 2025-03-10 2025-03-09
23h

$ TZ=Europe/London biff span since -r 2025-03-10 2025-03-09
24h
```

And 2025-11-02 is 25 hours in New York, but 24 in London:

```console
$ TZ=America/New_York biff span since -r 2025-11-03 2025-11-02
25h

$ TZ=Europe/London biff span since -r 2025-11-03 2025-11-02
24h
```

This is because there are time zone transitions (into daylight saving time
and out of daylight saving time, respectively) in New York on these days.
London also has daylight saving time, but it transitions on different days
than New York:

```console
$ biff tz seq -c2 -r 2025-01-01 America/New_York
2025-03-09T03:00:00-04:00[America/New_York]
2025-11-02T01:00:00-05:00[America/New_York]

$ biff tz seq -c2 -r 2025-01-01 Europe/London
2025-03-30T02:00:00+01:00[Europe/London]
2025-10-26T01:00:00+00:00[Europe/London]
```

Days are not the only unit that can vary. Years and months can as well:

```console
$ biff span since -l day -r 2025-03-01 2024-03-01
365d

$ biff span since -l day -r 2024-03-01 2023-03-01
366d

$ biff span since -l day -r 2025-06-01 2025-05-01
31d

$ biff span since -l day -r 2025-07-01 2025-06-01
30d
```

At present, adding `1 month` to any datetime can only increase the month part
of a date by at most 1. That is, the result is constrained. For example:

```console
$ biff time add 1mo 2025-05-31 2025-09-30
2025-06-30T00:00:00-04:00[America/New_York]
2025-10-30T00:00:00-04:00[America/New_York]
```

Subtracting one month from the result doesn't necessarily get you back where
you started. For example:

```console
$ biff time add -1mo 2025-06-30 2025-10-30
2025-05-30T00:00:00-04:00[America/New_York]
2025-09-30T00:00:00-04:00[America/New_York]
```

This is what was meant earlier by the operation not being reversible, and it's
why units of hours or smaller are used by default.

Similarly for years:

```console
$ biff time add 1y 2023-02-28 2024-02-29
2024-02-28T00:00:00-05:00[America/New_York]
2025-02-28T00:00:00-05:00[America/New_York]

$ biff time add -1y 2024-02-28 2025-02-28
2023-02-28T00:00:00-05:00[America/New_York]
2024-02-28T00:00:00-05:00[America/New_York]
```

## Duration Formatting

Biff comes with a command for formatting time spans. By default, Biff uses
a somewhat compact representation:

```console
$ biff span since -r '2025-05-04 17:30:00.123456789' 2025-04-15
473h 30m 123ms 456µs 789ns
```

Negative spans are also supported via the `ago` suffix:

```console
$ biff span until -r '2025-05-04 17:30:00.123456789' 2025-04-15
473h 30m 123ms 456µs 789ns ago
```

The `biff span fmt` command provides a lot of different options for how
a span is formatted. For example, you can make them even more compact by
removing all spacing:

```console
$ biff span fmt -s none '473h 30m 123ms 456µs 789ns ago'
-473h30m123ms456µs789ns
```

Or spread it out with lots of spacing and verbose unit designators:

```console
$ biff span fmt -s units-and-designators -d verbose --comma '473h 30m 123ms 456µs 789ns ago'
473 hours, 30 minutes, 123 milliseconds, 456 microseconds, 789 nanoseconds ago
```

Or use a bit of a hybrid format that utilizes `HH:MM:SS` for representing
units less than days:

```console
$ biff span fmt --hms '473h 30m 123ms'
473:30:00.123

$ biff span fmt --hms '19d 17h 30m 123ms'
19d 17:30:00.123
```

One also does not need to use `--hms` to get fractional seconds:

```console
$ biff span fmt -f seconds '19d 17h 30m 123ms'
19d 17h 30m 0.123s

$ biff span fmt -f hours '19d 17h 30m 123ms'
19d 17.500034166h
```

By design, every example above is a valid instance of
[Jiff's "friendly" duration format][fmt::friendly]. That means you can pipe
it into any command that accepts a duration on `stdin`. For example, instead
of this:

```console
$ biff span fmt -f hours '19d 17h 30m 123ms'
19d 17.500034166h

$ biff time add 2024-07-01 '19d 17.500034166h'
2024-07-20T17:30:00.1229976-04:00[America/New_York]
```

You can just do:

```console
$ biff span fmt -f hours '19d 17h 30m 123ms' | biff time add 2024-07-01
2024-07-20T17:30:00.1229976-04:00[America/New_York]
```

Finally, when you need it, you can convert any "friendly" duration into the
stricter ISO 8601 duration format:

```console
$ biff span iso8601 '19d 17h 30m 123ms'
P19DT17H30M0.123S
```

The ISO 8601 duration format tends to be harder to read, but it is more
widely supported.

## Duration Rounding

Biff has sophisticated support for rounding time spans. One common use case is
to reduce the precision of time spans returned by Biff. For example, if your
system's clock provides nanosecond precision, then asking how long it's been
since a date in the past is likely to produce too much information:

```console
$ biff span since -l year 2025-03-20
1mo 16d 10h 59m 53s 545ms 578µs 997ns
```

If you instead just want the duration rounded to the nearest day, you can use
`biff span round` and set the smallest unit to be days:

```console
$ biff span since 2025-03-20 | biff span round -l year -s day
1mo 16d
```

You can also set the largest unit to days, which will cause any bigger units
to get balanced down:

```console
$ biff span since 2025-03-20 | biff span round -s day -l day
47d
```

Rounding works with time too. For example, to round to the nearest hour:

```console
$ biff span since 2025-03-20 | biff span round -l year -s hour
1mo 16d 11h
```

Or even to the nearest 15 minute interval:

```console
$ biff span since 2025-03-20 | biff span round -l year -s minute -i 15
1mo 16d 11h 15m
```

Rounding is aware of daylight saving time. For example, most days are 24 hours,
and so rounding 11.75h to the nearest day in most cases will result in a zero
span:

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

## Composition

Most Biff commands that accept datetimes (or time spans) can also accept
multiple datetimes. For example:

```console
$ biff time fmt -f '%G-W%V-%w' 2025-01-01T00:00:00-05 2025-12-31T00-05
2025-W01-3
2026-W01-3
```

But when no datetimes are provided, for most commands, they can be provided on
`stdin`, one per line:

```
$ printf '2025-01-01T00:00:00-05\n2025-12-31T00-05' | biff time fmt -f '%G-W%V-%w'
2025-W01-3
2026-W01-3
```

This composition makes it easy to string together multiple Biff commands in
a single shell pipeline. For example, this command prints the end of the month
relative to 3 weeks ago and formats it into my system's locale (assuming that
[locale support][localization] in Biff is enabled):

```console
$ biff time end-of month -3w | biff time fmt -f '%c'
Wed, Apr 30, 2025, 11:59:59 PM EDT
```

When piping datetimes from `stdin`, Biff requires that they are in an
unambiguous format and correspond to a precise instant. This is why Biff
prints all datetimes as RFC 9557 timestamps. For example:

```console
$ biff time end-of month -3w
2025-04-30T23:59:59.999999999-04:00[America/New_York]
```

By using this output format, the datetime is accepted on `stdin` for any other
Biff command.

Biff has this restriction on `stdin` to avoid making implicit assumption about
how to interpret a date. For example, when one writes `2025-05-20 17:30` on
the command line, then Biff will happily interpret that as a local time in your
current time zone:

```console
$ biff time fmt '2025-05-20 17:30'
2025-05-20T17:30:00-04:00[America/New_York]
```

But if this same string is piped into `biff time fmt` on `stdin`, then you'll
get an error:

```console
$ echo '2025-05-20 17:30' | biff time fmt
line 1 of <stdin>: invalid datetime: RFC 3339 timestamp requires an offset, but 2025-05-20 17:30 is missing an offset
```

That is, Biff is complaining that you haven't provided an unambiguous instant
in time. The distinction here is that data on `stdin` might be coming from
anywhere, but data on the command line is likely being typed by an end user
in their own local time.

If you do want the full convenience of Biff's command line argument datetime
parsing, you can explicitly opt into it via Biff's `biff time parse` command:

```console
$ echo '2025-05-20 17:30' | biff time parse -f flexible
2025-05-20T17:30:00-04:00[America/New_York]
```

This will automatically assume the current time when applicable. For example,
if only the time is given:

```console
$ echo '17:30' | biff time parse -f flexible
2025-05-08T17:30:00-04:00[America/New_York]
```

To use a different reference date, use the `-r/--relative` flag:

```console
$ echo '17:30' | biff time parse -f flexible -r 2025-01-01
2025-01-01T17:30:00-05:00[America/New_York]
```

Biff can also do comparisons. For example, if you have a list of dates, this
will only shows the dates more recent than last Monday:

```console
$ printf '2025-05-04\n2025-05-05\n2025-05-06\n2025-05-07\n2025-05-08' \
    | biff time parse -f flexible \
    | biff time cmp ge 'last monday'
2025-05-06T00:00:00-04:00[America/New_York]
2025-05-07T00:00:00-04:00[America/New_York]
2025-05-08T00:00:00-04:00[America/New_York]
```

As one final mildly contrived example, let's try to predict when season 3 of
_House of the Dragon_ will air based on the duration between seasons 1 and
2. Season 1 ended on 2022-10-22. Season 2 started on 2024-06-16 and ended on
2024-08-04. This command finds the difference between the end of season 1 and
the start of season 2, and then adds that duration to the end of season 2. We
also try to predict the start time and format it into our locale:

```console
$ biff span since -r 2024-06-16 2022-10-22 \
    | biff time add 2024-08-04 \
    | biff time add 21h \
    | biff time fmt -f '%c'
Mon, Mar 30, 2026, 9:00:00 PM EDT
```

## Tagging

One possibly novel aspect of Biff compared to other datetime utilities is
its ability to tag arbitrary data with datetimes. A simple demonstration
of this concept can be done with most kinds of log files. For example,
here's an excerpt from my `journalctl` log (acquired via `journalctl -o
short-iso-precise`).

```
2025-05-07T00:00:15.862321-04:00 duff systemd[1]: Started Verify integrity of password and group files.
2025-05-07T00:00:15.886304-04:00 duff systemd[1]: shadow.service: Deactivated successfully.
2025-05-07T01:53:00.068083-04:00 duff systemd[1]: Starting Daily man-db regeneration...
2025-05-07T01:53:00.791005-04:00 duff systemd[1]: man-db.service: Deactivated successfully.
2025-05-07T01:53:00.791125-04:00 duff systemd[1]: Finished Daily man-db regeneration.
```

Now let's see what happens when we ask Biff to tag each line of this data:

```console
$ biff tag lines /tmp/output.log
{"tags":[{"value":"2025-05-07T00:00:15.862321-04:00","range":[0,32]}],"data":{"text":"2025-05-07T00:00:15.862321-04:00 duff systemd[1]: Started Verify integrity of password and group files.\n"}}
{"tags":[{"value":"2025-05-07T00:00:15.886304-04:00","range":[0,32]}],"data":{"text":"2025-05-07T00:00:15.886304-04:00 duff systemd[1]: shadow.service: Deactivated successfully.\n"}}
{"tags":[{"value":"2025-05-07T01:53:00.068083-04:00","range":[0,32]}],"data":{"text":"2025-05-07T01:53:00.068083-04:00 duff systemd[1]: Starting Daily man-db regeneration...\n"}}
{"tags":[{"value":"2025-05-07T01:53:00.791005-04:00","range":[0,32]}],"data":{"text":"2025-05-07T01:53:00.791005-04:00 duff systemd[1]: man-db.service: Deactivated successfully.\n"}}
{"tags":[{"value":"2025-05-07T01:53:00.791125-04:00","range":[0,32]}],"data":{"text":"2025-05-07T01:53:00.791125-04:00 duff systemd[1]: Finished Daily man-db regeneration.\n"}}
```

What's happening here is Biff has detected the RFC 3339 timestamp in each line
and extracted it as a "tag." It then encodes the original data, along with the
tag, in the [JSON lines] format. The power of this comes from the fact that
this tagged data can be piped into any other Biff command on `stdin`. For
example, to select only the log lines that come after 1am:

```console
$ biff tag lines /tmp/output.log | biff time cmp ge 2025-05-07T01
{"tags":[{"value":"2025-05-07T01:53:00.068083-04:00[-04:00]","range":[0,32]}],"data":{"text":"2025-05-07T01:53:00.068083-04:00 duff systemd[1]: Starting Daily man-db regeneration...\n"}}
{"tags":[{"value":"2025-05-07T01:53:00.791005-04:00[-04:00]","range":[0,32]}],"data":{"text":"2025-05-07T01:53:00.791005-04:00 duff systemd[1]: man-db.service: Deactivated successfully.\n"}}
{"tags":[{"value":"2025-05-07T01:53:00.791125-04:00[-04:00]","range":[0,32]}],"data":{"text":"2025-05-07T01:53:00.791125-04:00 duff systemd[1]: Finished Daily man-db regeneration.\n"}}
```

And then you can untag the data to get the original format back (with color):

```console
$ biff tag lines /tmp/output.log | biff time cmp ge 2025-05-07T01 | biff untag
2025-05-07T01:53:00.068083-04:00 duff systemd[1]: Starting Daily man-db regeneration...
2025-05-07T01:53:00.791005-04:00 duff systemd[1]: man-db.service: Deactivated successfully.
2025-05-07T01:53:00.791125-04:00 duff systemd[1]: Finished Daily man-db regeneration.
```

Maybe you don't want to read RFC 3339 timestamps and instead want to read the
localized datetime, _and_ in the original log format. The `biff untag` command
takes a `-s/--substitute` flag that will automatically replace the the tag in
the original data with the tag in the JSON data:

```console
$ biff tag lines /tmp/output.log | biff time fmt -f '%c' | biff untag -s
Wed, May 7, 2025, 12:00:15 AM GMT-4 duff systemd[1]: Started Verify integrity of password and group files.
Wed, May 7, 2025, 12:00:15 AM GMT-4 duff systemd[1]: shadow.service: Deactivated successfully.
Wed, May 7, 2025, 1:53:00 AM GMT-4 duff systemd[1]: Starting Daily man-db regeneration...
Wed, May 7, 2025, 1:53:00 AM GMT-4 duff systemd[1]: man-db.service: Deactivated successfully.
Wed, May 7, 2025, 1:53:00 AM GMT-4 duff systemd[1]: Finished Daily man-db regeneration.
```

Since the original timestamps weren't in a particular time zone, the localized
representation above isn't quite as good as it could be. You can put datetimes
into your system's time zone explicitly when necessary:

```console
$ biff tag lines /tmp/output.log | biff time in system | biff time fmt -f '%c' | biff untag -s
Wed, May 7, 2025, 12:00:15 AM EDT duff systemd[1]: Started Verify integrity of password and group files.
Wed, May 7, 2025, 12:00:15 AM EDT duff systemd[1]: shadow.service: Deactivated successfully.
Wed, May 7, 2025, 1:53:00 AM EDT duff systemd[1]: Starting Daily man-db regeneration...
Wed, May 7, 2025, 1:53:00 AM EDT duff systemd[1]: man-db.service: Deactivated successfully.
Wed, May 7, 2025, 1:53:00 AM EDT duff systemd[1]: Finished Daily man-db regeneration.
```

Biff supports a number of other ways out of the box to created tagged data.
For example, this command will get the last commit date on each file in a git
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

(Biff will automatically use parallelism by default. Running `git log` on each
file in a git repository can be surprisingly slow!)

You are encouraged to explore the other sub-commands of `biff tag`, which
provide a few other ways of extracting tags from arbitrary data.

## Datetime Sequences

Biff has support for [RFC 5545 recurrence rules][recurrence-rule] in the form
of a command line interface. This allows you to use incredibly flexible rules
to generate sequences of datetimes. For example, starting simple, this command
generates the next 5 days, starting at the current time:

```console
$ biff time seq day now -c5
2025-05-08T21:40:43.717484378-04:00[America/New_York]
2025-05-09T21:40:43.717484378-04:00[America/New_York]
2025-05-10T21:40:43.717484378-04:00[America/New_York]
2025-05-11T21:40:43.717484378-04:00[America/New_York]
2025-05-12T21:40:43.717484378-04:00[America/New_York]
```

Or generate the next 5 dates at a specific time:

```console
$ biff time seq day today -c5 -H 9
2025-05-08T09:00:00-04:00[America/New_York]
2025-05-09T09:00:00-04:00[America/New_York]
2025-05-10T09:00:00-04:00[America/New_York]
2025-05-11T09:00:00-04:00[America/New_York]
2025-05-12T09:00:00-04:00[America/New_York]
```

Or the next 5 weekdays:

```console
$ biff time seq day today -c5 -H 9 -w mon,tue,wed,thu,fri
2025-05-08T09:00:00-04:00[America/New_York]
2025-05-09T09:00:00-04:00[America/New_York]
2025-05-12T09:00:00-04:00[America/New_York]
2025-05-13T09:00:00-04:00[America/New_York]
2025-05-14T09:00:00-04:00[America/New_York]
```

Find the last Saturday every other month, starting with the current month, for
the next year. And format the datetime in your locale:

```console
$ biff time seq monthly today -i2 -w -1-sat --until 1y | biff time fmt -f '%c'
Sat, May 31, 2025, 12:00:00 AM EDT
Sat, Jul 26, 2025, 12:00:00 AM EDT
Sat, Sep 27, 2025, 12:00:00 AM EDT
Sat, Nov 29, 2025, 12:00:00 AM EST
Sat, Jan 31, 2026, 12:00:00 AM EST
Sat, Mar 28, 2026, 12:00:00 AM EDT
```

Print every day remaining in the current month:

```console
$ biff time seq daily --until $(biff time end-of month now) today
2025-05-08T00:00:00-04:00[America/New_York]
2025-05-09T00:00:00-04:00[America/New_York]
2025-05-10T00:00:00-04:00[America/New_York]
2025-05-11T00:00:00-04:00[America/New_York]
2025-05-12T00:00:00-04:00[America/New_York]
2025-05-13T00:00:00-04:00[America/New_York]
[.. snip ..]
```

Print every day that has already passed of the current month:

```console
$ biff time seq daily --until today $(biff time start-of month today)
2025-05-01T00:00:00-04:00[America/New_York]
2025-05-02T00:00:00-04:00[America/New_York]
2025-05-03T00:00:00-04:00[America/New_York]
2025-05-04T00:00:00-04:00[America/New_York]
2025-05-05T00:00:00-04:00[America/New_York]
2025-05-06T00:00:00-04:00[America/New_York]
2025-05-07T00:00:00-04:00[America/New_York]
2025-05-08T00:00:00-04:00[America/New_York]
```

Print all Friday the 13th occurrences for the next 3 years:

```console
$ biff time seq monthly today --until 5y -w fri -d 13
2025-06-13T00:00:00-04:00[America/New_York]
2026-02-13T00:00:00-05:00[America/New_York]
2026-03-13T00:00:00-04:00[America/New_York]
2026-11-13T00:00:00-05:00[America/New_York]
2027-08-13T00:00:00-04:00[America/New_York]
2028-10-13T00:00:00-04:00[America/New_York]
2029-04-13T00:00:00-04:00[America/New_York]
2029-07-13T00:00:00-04:00[America/New_York]
```

The `biff time seq` command has several other flags and features for generating
datetimes. Be warned that unless you specify the `-c/--count` or `--until`
flags, Biff will generate datetimes until it reaches its maximum datetime
(which isn't that big at `9999-12-31`, but is still probably not what you
want).

## Time Zones

Biff comes with reasonably sophisticated time zone support. In most cases, Biff
should automatically detect your system configured time zone. You can see what
time zone Biff thinks yours is with the following command:

```console
$ biff time in system now
2025-05-08T21:52:44.112231333-04:00[America/New_York]
```

The output shows an [RFC 9557] timestamp that includes your system's IANA
time zone identifier in square brackets. This ensures that the time zone is
encoded as part of the timestamp.

If Biff can't detect your system's time zone, you can forcefully set it via the
`TZ` environment variable:

```console
$ TZ=Australia/Tasmania biff time in system now
2025-05-09T11:54:04.82510765+10:00[Australia/Tasmania]
```

Biff makes use of the IANA time zone database on your system by default when
available. Otherwise, it uses a copy of the database compiled into the Biff
binary. Either way, this means Biff can deal with other time zones just as
well:

```console
$ biff time in America/Los_Angeles now
2025-05-08T18:55:43.546845778-07:00[America/Los_Angeles]

$ biff time in Asia/Kolkata now
2025-05-09T07:25:45.364267821+05:30[Asia/Kolkata]

$ biff time in Asia/Bangkok now
2025-05-09T08:55:46.950163414+07:00[Asia/Bangkok]
```

Biff also knows how to deal with daylight saving time. For example, if you
try to print a datetime that never actually existed on the clocks in a
particular time zone (a gap), Jiff will automatically adjust the time for you:

```console
$ biff time fmt -f '%c' '2025-03-09T02:30[America/New_York]'
Sun, Mar 9, 2025, 3:30:00 AM EDT
```

Or if you try to use a time that occurred twice (a fold), Jiff will pick one
for you:

```console
$ biff time fmt -f '%c' '2025-11-02T01:30[America/New_York]'
Sun, Nov 2, 2025, 1:30:00 AM EDT
```

In the case of a fold, you can supply an offset to explicitly disambiguate
between which instance you want. In this case, `-04` in New York reflects when
it was still in daylight saving time:

```console
$ biff time fmt -f '%c' '2025-11-02T01:30-04[America/New_York]'
Sun, Nov 2, 2025, 1:30:00 AM EDT
```

And `-05` in New York reflects when it has transitioned to standard time:

```console
$ biff time fmt -f '%c' '2025-11-02T01:30-05[America/New_York]'
Sun, Nov 2, 2025, 1:30:00 AM EST
```

Daylight saving time is account for when doing arithmetic as well. For example,
consider the diference between adding 1 day and 24 hours when that span crosses
a time zone transition. First, let's look at a gap:

```console
$ biff time add 1d '2025-03-08T17:30-05[America/New_York]'
2025-03-09T17:30:00-04:00[America/New_York]

$ biff time add 24h '2025-03-08T17:30-05[America/New_York]'
2025-03-09T18:30:00-04:00[America/New_York]
```

That is, Biff regards `2025-03-09` as 23 hours long in New York. In contrast,
in New York, `2025-11-02` is treated as 25 hours long:

```console
$ biff time add 1d '2025-11-01T17:30-04[America/New_York]'
2025-11-02T17:30:00-05:00[America/New_York]

$ biff time add 24h '2025-11-01T17:30-04[America/New_York]'
2025-11-02T16:30:00-05:00[America/New_York]
```

Biff also provides partial access to the IANA Time Zone database itself. For
example, this is how you can print the next 5 time zone transitions in your
system's time zone:

```console
$ biff tz seq system -c5 | biff time fmt -f '%Y-%m-%d %H:%M:%S %Z'
2025-11-02 01:00:00 EST
2026-03-08 03:00:00 EDT
2026-11-01 01:00:00 EST
2027-03-14 03:00:00 EDT
2027-11-07 01:00:00 EST
```

Or in another time zone:

```console
$ biff tz seq Australia/Sydney -c5 | biff time fmt -f '%Y-%m-%d %H:%M:%S %Z'
2025-10-05 03:00:00 AEDT
2026-04-05 02:00:00 AEST
2026-10-04 03:00:00 AEDT
2027-04-04 02:00:00 AEST
2027-10-03 03:00:00 AEDT
```

Biff also provides a way of providing a timestamp, with an offset, and then
printing all time zones compatible with that time:

```console
$ biff tz compatible '2025-05-01T17:30+05:30'
Asia/Calcutta
Asia/Colombo
Asia/Kolkata

$ biff tz compatible '1952-10-01T23:59:59-11:19:40'
Pacific/Niue
```

## Localization

Biff has some rudimentary support for localizing datetimes as prescribed by
Unicode. Biff specifically does not and will never support [POSIX locales].

First and foremost is checking whether your installation of Biff has locale
support enabled:

```console
$ biff --version
Biff 0.1.0 (rev 2659045dba) (locale support enabled)
```

If you don't see "locale support enabled" in the output, then that means your
Biff installation cannot localize datetimes. To fix that, either rebuild Biff
with the `locale` feature enabled, or use one of the binaries distributed on
GitHub. If you installed Biff from a package manager, then you'll need to ask
them to rebuild Biff with locale support enabled.

(Note that when Biff is compiled with the `locale` feature, all necessary
localization data is bundled into the binary. This increases the binary size
of Biff substantially.)

When locale support is enabled, you'll need to set a locale. At present, Biff
doesn't try to discover your system's locale automatically. Instead, it can
only be set with through the `BIFF_LOCALE` environment variable. Here are
some examples:

```console
$ BIFF_LOCALE=en-US biff
Fri, May 2, 2025, 2:25:34 PM EDT

$ BIFF_LOCALE=en-GB biff
Fri, 2 May 2025, 14:25:37 GMT-4

$ BIFF_LOCALE=en-GB TZ=Europe/London biff
Fri, 2 May 2025, 19:29:17 BST

$ BIFF_LOCALE=fr-FR TZ=Europe/Paris biff
ven. 2 mai 2025, 20:29:00 UTC+2

$ BIFF_LOCALE=th-TH TZ=Asia/Bangkok biff
ส. 3 พ.ค. 2568 01:28:36 GMT+7
```

(We set `TZ` above since it can influence localization since localization may
take your region into account.)

When Biff does not have locale support enabled, then it will behave as if its
locale is undetermined:

```console
$ BIFF_LOCALE=und biff
2025 M05 2, Fri 14:30:38
```

Locale support also impacts the `%c`, `%r`, `%X` and `%x` specifiers when
using Biff's `strftime` formatting command:

```console
$ BIFF_LOCALE=en-GB TZ=Europe/London biff time fmt -f '%c' now
Fri, 2 May 2025, 19:31:41 BST

$ BIFF_LOCALE=en-GB TZ=Europe/London biff time fmt -f '%r' now
7:31:46 pm

$ BIFF_LOCALE=en-GB TZ=Europe/London biff time fmt -f '%X' now
19:31:49

$ BIFF_LOCALE=en-GB TZ=Europe/London biff time fmt -f '%x' now
2 May 2025
```

When `biff` is run with no arguments (as shown above), then it is equivalent to
`biff time fmt -f '%c'`.

What are the legal values for `BIFF_LOCALE`? Predictably, Unicode's documentation
on this is completely impenetrable, and seems to go out of its way to specifically
avoid listing legal values. For example, "[Picking the Right Language Code]"
and "[Unicode Utility BCP47]" seem promising (and are the top web search
results), but neither aide in discovering what the legal values are. In particular
for discovering region codes.

Instead, I suggest "[Using Language Identifiers]", which has a very helpful
table of available language and region codes with useful information density.

For most people, this should be all you need. But up to this point, we've only
covered Unicode language identifiers. `BIFF_LOCALE`, however, supports
[Unicode Locale Identifiers], which offer even more functionality. For example,
maybe you live in London, but for whatever reason, you don't like the 24-hour
clock:

```console
$ BIFF_LOCALE=en-GB TZ=Europe/London biff
Fri, 2 May 2025, 19:40:33 BST
$ BIFF_LOCALE=en-GB-u-hc-h12 TZ=Europe/London biff
Fri, 2 May 2025, 7:41:07 pm BST
```

Or maybe you live in the United States, but you want to use the Hebrew
calendar:

```
$ BIFF_LOCALE=en-US-u-ca-hebrew biff
Fri, 4 Iyar 5785, 2:42:07 PM EDT
```

In the future, Biff may support detecting your system's locale for you
automatically. This is blocked on [ICU4X support for querying this
information][ICU4X system language]. Nevertheless, `BIFF_LOCALE` will always
work to override the system locale and will likely be necessary for accessing
the full expressivity of Unicode Locale Identifiers.


[POSIX locales]: https://github.com/mpv-player/mpv/commit/1e70e82baa9193f6f027338b0fab0f5078971fbe
[Unicode Locale Identifiers]: https://unicode.org/reports/tr35/tr35.html#Unicode_locale_identifier
[Picking the Right Language Code]: https://cldr.unicode.org/index/cldr-spec/picking-the-right-language-code
[Unicode Utility BCP47]: https://util.unicode.org/UnicodeJsps/languageid.jsp
[Using Language Identifiers]: http://www.i18nguy.com/unicode/language-identifiers.html
[ICU4X system language]: https://github.com/unicode-org/icu4x/issues/3990
[fmt::friendly]: https://docs.rs/jiff/latest/jiff/fmt/friendly/index.html
[localization]: #localization
[JSON lines]: https://jsonlines.org/
[recurrence-rule]: https://icalendar.org/iCalendar-RFC-5545/3-8-5-3-recurrence-rule.html
[RFC 9557]: https://datatracker.ietf.org/doc/rfc9557/
