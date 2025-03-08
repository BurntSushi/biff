use crate::{biff, command::assert_cmd_snapshot};

fn fmt() -> crate::command::Command {
    biff(["span", "fmt"])
}

#[test]
fn designator() {
    assert_cmd_snapshot!(
        fmt().arg("1us"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    1µs

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        fmt().arg("-d").arg("compact").arg("1us"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    1µs

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        fmt().arg("-d").arg("short").arg("1us"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    1µsec

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        fmt().arg("-d").arg("verbose").arg("1us"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    1microsecond

    ----- stderr -----
    ",
    );
}

#[test]
fn spacing() {
    assert_cmd_snapshot!(
        fmt().arg("-d").arg("verbose").arg("1y 1us"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    1year 1microsecond

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        fmt().arg("-d").arg("verbose").arg("-s").arg("none").arg("1y 1us"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    1year1microsecond

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        fmt().arg("-d").arg("verbose").arg("-s").arg("units").arg("1y 1us"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    1year 1microsecond

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        fmt()
            .arg("-d").arg("verbose")
            .arg("-s").arg("units-and-designators")
            .arg("1y 1us"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    1 year 1 microsecond

    ----- stderr -----
    ",
    );
}

#[test]
fn sign() {
    assert_cmd_snapshot!(
        fmt().arg("-1h2m"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    1h 2m ago

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        fmt().arg("-snone").arg("-1h2m"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    -1h2m

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        fmt().arg("-1y1h2m"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    1y 1h 2m ago

    ----- stderr -----
    ",
    );

    // With `--hms`, the sign is added as a prefix by default.
    assert_cmd_snapshot!(
        fmt().arg("-1h2m").arg("--hms"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    -01:02:00

    ----- stderr -----
    ",
    );

    // Unless, there are calendar units, in which case, it's added as a suffix.
    assert_cmd_snapshot!(
        fmt().arg("-1y1h2m").arg("--hms"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    1y 01:02:00 ago

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        fmt().arg("--sign=prefix").arg("-1h2m"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    -1h 2m

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        fmt().arg("--sign=force-prefix").arg("1h2m"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    +1h 2m

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        fmt().arg("-snone").arg("--sign=suffix").arg("-1h2m"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    1h2m ago

    ----- stderr -----
    ",
    );
}

#[test]
fn fractional() {
    assert_cmd_snapshot!(
        fmt().arg("-fhour").arg("1h30m").arg("1h1800s"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    1.5h
    1.5h

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        fmt().arg("-fmicros").arg("500ns"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    0.5µs

    ----- stderr -----
    ",
    );

    // Demonstrates precision loss.
    assert_cmd_snapshot!(
        fmt().arg("-fhour").arg("1s 456ns"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    0.000277777h

    ----- stderr -----
    ",
    );
    // If we try to go back, we notice that
    // our duration isn't quite the same due
    // to precision loss.
    assert_cmd_snapshot!(
        biff(["span", "balance", "-l", "hour", "0.000277777h"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    999ms 997µs 200ns

    ----- stderr -----
    ",
    );
}

#[test]
fn comma() {
    assert_cmd_snapshot!(
        fmt().arg("--comma").arg("1h2m30s"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    1h, 2m, 30s

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        fmt()
            .arg("--comma")
            .arg("-dverbose")
            .arg("-sunits-and-designators")
            .arg("1h2m30s"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    1 hour, 2 minutes, 30 seconds

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        fmt()
            .arg("--comma")
            .arg("-dverbose")
            .arg("-sunits-and-designators")
            .arg("--hms")
            .arg("1h2m30s"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    01:02:30

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        fmt()
            .arg("--comma")
            .arg("-dverbose")
            .arg("-sunits-and-designators")
            .arg("--hms")
            .arg("5d1h2m30s"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    5 days, 01:02:30

    ----- stderr -----
    ",
    );
}

#[test]
fn hms() {
    assert_cmd_snapshot!(
        fmt().arg("--hms").arg("1h2m30s"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    01:02:30

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        fmt().arg("--hms").arg("999h999m999s"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    999:999:999

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        fmt().arg("--hms").arg("1y2mo3w4d1h2m30s"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    1y 2mo 3w 4d 01:02:30

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        fmt().arg("--hms").arg("1h2m30s123ms456µs789ns"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    01:02:30.123456789

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        fmt().arg("--hms").arg("123ms"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    00:00:00.123

    ----- stderr -----
    ",
    );
}

#[test]
fn padding() {
    assert_cmd_snapshot!(
        fmt().arg("--pad=2").arg("1h2m30s"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    01h 02m 30s

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        fmt().arg("--pad=3").arg("1h2m30s"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    001h 002m 030s

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        fmt().arg("--pad=0").arg("--hms").arg("1h2m30s"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    1:2:30

    ----- stderr -----
    ",
    );
}

#[test]
fn precision() {
    assert_cmd_snapshot!(
        fmt().arg("--precision=2").arg("-fsecs").arg("1s 123ms 456us 789ns"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    1.12s

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        fmt().arg("--precision=0").arg("-fsecs").arg("1s 123ms 456us 789ns"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    1s

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        fmt().arg("--precision=9").arg("-fsecs").arg("1s 123ms 456us 789ns"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    1.123456789s

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        fmt().arg("--precision=auto").arg("-fsecs").arg("1s 123ms 456us 789ns"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    1.123456789s

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        fmt().arg("--precision=10").arg("-fsecs").arg("1s 123ms 456us 789ns"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    1.123456789s

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        fmt().arg("--precision=9").arg("-fsecs").arg("1s 123ms"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    1.123000000s

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        fmt().arg("--precision=wat").arg("-fsecs").arg("1s"),
        @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    --precision: failed to parse precision amount from `wat`
    ",
    );
}

#[test]
fn zero_unit() {
    assert_cmd_snapshot!(
        fmt().arg("0ns"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    0s

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        fmt().arg("--zero-unit=day").arg("0ns"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    0d

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        fmt().arg("--zero-unit=year").arg("0ns"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    0y

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        fmt().arg("-fhour").arg("--zero-unit=year").arg("0ns"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    0h

    ----- stderr -----
    ",
    );
}
