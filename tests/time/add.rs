use crate::command::assert_cmd_snapshot;

fn add() -> crate::command::Command {
    crate::biff(["time", "add"])
}

/// Test that we can use a span as the first positional argument.
#[test]
fn span_first_positional() {
    assert_cmd_snapshot!(
        add().arg("-1d").arg("now"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-19T16:30:55-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        add().arg("-1d").arg("now").arg("2025-01-01"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-19T16:30:55-04:00[America/New_York]
    2024-12-31T00:00:00-05:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        add().arg("-1d").stdin(
            "2024-01-01T00:00:00-05\n2025-01-01 00:00:00-05",
        ),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2023-12-31T00:00:00-05:00[-05:00]
    2024-12-31T00:00:00-05:00[-05:00]

    ----- stderr -----
    ",
    );

    // Should just do nothing.
    assert_cmd_snapshot!(
        add().arg("-1d").stdin(""),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ",
    );
}

/// Test that we can use a datetime as the first positional argument.
#[test]
fn datetime_first_positional() {
    assert_cmd_snapshot!(
        add().arg("now").arg("-1d"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-19T16:30:55-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        add().arg("now").arg("-1d").arg("1d"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-19T16:30:55-04:00[America/New_York]
    2024-07-21T16:30:55-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        add().arg("now").stdin("-1d\n1d\n"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-19T16:30:55-04:00[America/New_York]
    2024-07-21T16:30:55-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    // Should just do nothing.
    assert_cmd_snapshot!(
        add().arg("now").stdin(""),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ",
    );
}

/// This tests the output we get when we *intend* to provide a span, but we
/// write it incorrectly.
///
/// This is interesting to test because the first positional argument can be
/// either a span or a datetime. If parsing the span fails, then its error
/// gets swallowed.
#[test]
fn invalid_span_error() {
    assert_cmd_snapshot!(
        add().arg("1h1d").arg("now"),
        @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    failed to parse as datetime or as time span: unrecognized datetime `1h1d`
    ",
    );
}

/// Reject "flexible" datetimes on stdin.
#[test]
fn rejects_flexible_datetimes() {
    assert_cmd_snapshot!(
        add().arg("-1d").stdin("2024-01-01"),
        @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    line 1 of <stdin>: invalid datetime: RFC 3339 timestamp requires an offset, but 2024-01-01 is missing an offset
    ",
    );
}

/// Adding calendar units versus phyiscal time units has different behavior
/// with respect to DST.
#[test]
fn dst_calendar_versus_time() {
    assert_cmd_snapshot!(
        add().arg("1d").arg("2025-03-08 17:00-05[America/New_York]"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2025-03-09T17:00:00-04:00[America/New_York]

    ----- stderr -----
    "
    );

    assert_cmd_snapshot!(
        add().arg("24h").arg("2025-03-08 17:00-05[America/New_York]"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2025-03-09T18:00:00-04:00[America/New_York]

    ----- stderr -----
    "
    );
}

/// When adding a span to a datetime, if we land in a gap or a fold, Biff will
/// disambiguate automatically.
#[test]
fn disambiguate_automatically() {
    // Chooses the later time
    assert_cmd_snapshot!(
        add().arg("1h").arg("2025-03-09T01:30-05[America/New_York]"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2025-03-09T03:30:00-04:00[America/New_York]

    ----- stderr -----
    "
    );

    // Chooses the earlier time
    assert_cmd_snapshot!(
        add().arg("1h").arg("2025-11-02T00:30-04[America/New_York]"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2025-11-02T01:30:00-04:00[America/New_York]

    ----- stderr -----
    "
    );
}

/// Tests some month arithmetic.
///
/// Biff isn't the one implementing these semantics. They are the domain of
/// Jiff. I wanted to write tests here though because I think it's likely
/// that `biff time add` will want to grow a way to change how month
/// arithmetic is done.
#[test]
fn month_arithmetic() {
    assert_cmd_snapshot!(
        add().arg("1mo").arg("2024-01-31"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-02-29T00:00:00-05:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        add().arg("1mo").arg("2025-01-31"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2025-02-28T00:00:00-05:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        add().arg("2mo").arg("2024-01-31"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-03-31T00:00:00-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        add().arg("2mo").arg("2025-01-31"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2025-03-31T00:00:00-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        add().arg("3mo").arg("2024-01-31"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-04-30T00:00:00-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        add().arg("3mo").arg("2025-01-31"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2025-04-30T00:00:00-04:00[America/New_York]

    ----- stderr -----
    ",
    );
}

/// Test that we can use a span as the first positional argument.
#[test]
fn bespoke_snapshotting() {
    crate::command::assert_cmd_snapshot!(
        add().arg("-1d").arg("now"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-19T16:30:55-04:00[America/New_York]

    ----- stderr -----
    ",
    );
}
