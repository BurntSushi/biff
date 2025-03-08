use crate::command::assert_cmd_snapshot;

fn fmt() -> crate::command::Command {
    crate::biff(["time", "fmt"])
}

/// Test that passing some special strings on the CLI works.
#[test]
fn special() {
    assert_cmd_snapshot!(
        fmt().args(["now", "today", "yesterday", "tomorrow"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-20T16:30:55-04:00[America/New_York]
    2024-07-20T00:00:00-04:00[America/New_York]
    2024-07-19T00:00:00-04:00[America/New_York]
    2024-07-21T00:00:00-04:00[America/New_York]

    ----- stderr -----
    ",
    );
}

/// Tests that we read durations as relative datetimes on the CLI.
#[test]
fn span_as_relative_datetime() {
    // Test Jiff's "friendly" duration support.
    assert_cmd_snapshot!(
        fmt().args([
            "1s ago",
            "1s",
            "-6mo",
            "6mo",
            "-1y",
            "1y",
            "1y 1s ago",
            "1 year, 1 second ago",
        ]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-20T16:30:54-04:00[America/New_York]
    2024-07-20T16:30:56-04:00[America/New_York]
    2024-01-20T16:30:55-05:00[America/New_York]
    2025-01-20T16:30:55-05:00[America/New_York]
    2023-07-20T16:30:55-04:00[America/New_York]
    2025-07-20T16:30:55-04:00[America/New_York]
    2023-07-20T16:30:54-04:00[America/New_York]
    2023-07-20T16:30:54-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    // Test ISO 8601 duration support.
    assert_cmd_snapshot!(
        fmt().args([
            "-PT1S",
            "PT1s",
            "-P6M",
            "P6m",
            "-P1Y",
            "P1y",
            "-P1YT1S",
        ]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-20T16:30:54-04:00[America/New_York]
    2024-07-20T16:30:56-04:00[America/New_York]
    2024-01-20T16:30:55-05:00[America/New_York]
    2025-01-20T16:30:55-05:00[America/New_York]
    2023-07-20T16:30:55-04:00[America/New_York]
    2025-07-20T16:30:55-04:00[America/New_York]
    2023-07-20T16:30:54-04:00[America/New_York]

    ----- stderr -----
    ",
    );
}

/// Test Biff's "Intuitive" flexible relative datetime format.
#[test]
fn intuitive_relative_datetime() {
    // Different ways to spell Saturday.
    assert_cmd_snapshot!(
        fmt().args([
            "sat",
            "saturday",
            "Saturday",
            "SATURDAY",
            "SaTuRdAy",
            "this sat",
            "0 sat",
        ]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-20T16:30:55-04:00[America/New_York]
    2024-07-20T16:30:55-04:00[America/New_York]
    2024-07-20T16:30:55-04:00[America/New_York]
    2024-07-20T16:30:55-04:00[America/New_York]
    2024-07-20T16:30:55-04:00[America/New_York]
    2024-07-20T16:30:55-04:00[America/New_York]
    2024-07-20T16:30:55-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        fmt().args([
            "next sat",
            "1 sat",
            "+1 sat",
            "last sat",
            "-1 sat",
            "2 sat",
            "-2 sat",
        ]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-27T16:30:55-04:00[America/New_York]
    2024-07-27T16:30:55-04:00[America/New_York]
    2024-07-27T16:30:55-04:00[America/New_York]
    2024-07-13T16:30:55-04:00[America/New_York]
    2024-07-13T16:30:55-04:00[America/New_York]
    2024-08-03T16:30:55-04:00[America/New_York]
    2024-07-06T16:30:55-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        fmt().args([
            "5pm sat",
            "5pm this sat",
            "5pm next sat",
            "5pm last sat",
            "5pm 0 sat",
            "5pm 1 sat",
            "5pm -1 sat",
            "5pm this sunday",
        ]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-20T17:00:00-04:00[America/New_York]
    2024-07-20T17:00:00-04:00[America/New_York]
    2024-07-27T17:00:00-04:00[America/New_York]
    2024-07-13T17:00:00-04:00[America/New_York]
    2024-07-20T17:00:00-04:00[America/New_York]
    2024-07-27T17:00:00-04:00[America/New_York]
    2024-07-13T17:00:00-04:00[America/New_York]
    2024-07-21T17:00:00-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        fmt().args([
            "5pm",
            "5pm 1 day ago",
            "5pm 1 day",
            "5pm 1 week ago",
            "5pm 1 week",
            "5pm -1d",
            "5pm 1d",
            "5pm -1w",
            "5pm 1w",
        ]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-20T17:00:00-04:00[America/New_York]
    2024-07-19T17:00:00-04:00[America/New_York]
    2024-07-21T17:00:00-04:00[America/New_York]
    2024-07-13T17:00:00-04:00[America/New_York]
    2024-07-27T17:00:00-04:00[America/New_York]
    2024-07-19T17:00:00-04:00[America/New_York]
    2024-07-21T17:00:00-04:00[America/New_York]
    2024-07-13T17:00:00-04:00[America/New_York]
    2024-07-27T17:00:00-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        fmt().args([
            "5pm today",
            "5pm yesterday",
            "5pm tomorrow",
        ]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-20T17:00:00-04:00[America/New_York]
    2024-07-19T17:00:00-04:00[America/New_York]
    2024-07-21T17:00:00-04:00[America/New_York]

    ----- stderr -----
    ",
    );
}

/// Unlike when given on the CLI, this tests that durations are rejected when
/// given in stdin.
#[test]
fn relative_rejected_on_stdin() {
    assert_cmd_snapshot!(
        fmt().stdin("-1s"),
        @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    line 1 of <stdin>: invalid datetime: unrecognized datetime `-1s`
    ",
    );

    assert_cmd_snapshot!(
        fmt().stdin("now"),
        @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    line 1 of <stdin>: invalid datetime: unrecognized datetime `now`
    ",
    );
}

/// This tests that we can use things like `-1h` as positional arguments.
///
/// Normally, the `-1` would be interpreted as a short flag by `lexopt`. But
/// we have some special handling for `-[0-9]` that lets this pass through
/// as a positional argument.
///
/// N.B. We also permit `-P1D` as well, giving up the ability to use `-P` as
/// a short flag too.
#[test]
fn negative_duration() {
    assert_cmd_snapshot!(
        fmt().args(["-1s", "-1 second", "-PT1S", "-0s"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-20T16:30:54-04:00[America/New_York]
    2024-07-20T16:30:54-04:00[America/New_York]
    2024-07-20T16:30:54-04:00[America/New_York]
    2024-07-20T16:30:55-04:00[America/New_York]

    ----- stderr -----
    ",
    );
}

/// Test that we can pass datetimes positionally or on stdin.
#[test]
fn positional_or_stdin() {
    assert_cmd_snapshot!(
        fmt().arg("-frfc2822").arg("2025-03-15T12Z").arg("2026-03-15T12-04"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Sat, 15 Mar 2025 12:00:00 +0000
    Sun, 15 Mar 2026 12:00:00 -0400

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        fmt().arg("-frfc2822").stdin("2025-03-15T12Z\n2026-03-15T12-04"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Sat, 15 Mar 2025 12:00:00 +0000
    Sun, 15 Mar 2026 12:00:00 -0400

    ----- stderr -----
    ",
    );
}

/// Test that RFC 9557 works.
#[test]
fn rfc9557() {
    assert_cmd_snapshot!(
        fmt().arg("-f").arg("rfc9557").arg("now"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-20T16:30:55-04:00[America/New_York]

    ----- stderr -----
    ",
    );
}

/// Test that RFC 3339 works.
#[test]
fn rfc3339() {
    assert_cmd_snapshot!(
        fmt().arg("-f").arg("rfc3339").arg("now"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-20T16:30:55-04:00

    ----- stderr -----
    ",
    );
}

/// Test that RFC 2822 works.
#[test]
fn rfc2822() {
    assert_cmd_snapshot!(
        fmt().arg("-f").arg("rfc2822").arg("now"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Sat, 20 Jul 2024 16:30:55 -0400

    ----- stderr -----
    ",
    );
}

/// Test that RFC 9110 works.
#[test]
fn rfc9110() {
    assert_cmd_snapshot!(
        fmt().arg("-f").arg("rfc9110").arg("now"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Sat, 20 Jul 2024 20:30:55 GMT

    ----- stderr -----
    ",
    );
}

/// Test that strftime works.
#[test]
fn strftime() {
    assert_cmd_snapshot!(
        fmt().arg("-f").arg("%A %Y-%m-%d %H:%M:%S %:z %Z %Q").arg("now"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Saturday 2024-07-20 16:30:55 -04:00 EDT America/New_York

    ----- stderr -----
    ",
    );
}
