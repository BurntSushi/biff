use crate::command::assert_cmd_snapshot;

fn parse() -> crate::command::Command {
    crate::biff(["time", "parse"])
}

/// Test that we can parse datetimes positionally or on stdin.
#[test]
fn positional_or_stdin() {
    assert_cmd_snapshot!(
        parse().arg("-f%F").arg("2025-03-15").arg("2026-03-15"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2025-03-15T00:00:00-04:00[America/New_York]
    2026-03-15T00:00:00-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        parse().arg("-f%F").stdin("2025-03-15\n2026-03-15"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2025-03-15T00:00:00-04:00[America/New_York]
    2026-03-15T00:00:00-04:00[America/New_York]

    ----- stderr -----
    ",
    );
}

/// Test that RFC 9557 works.
#[test]
fn rfc9557() {
    assert_cmd_snapshot!(
        parse().arg("-f").arg("rfc9557").arg(
            "2025-03-15 17:50-10[Pacific/Honolulu]",
        ),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2025-03-15T17:50:00-10:00[Pacific/Honolulu]

    ----- stderr -----
    ",
    );
}

/// Test that RFC 3339 works.
#[test]
fn rfc3339() {
    assert_cmd_snapshot!(
        parse().arg("-f").arg("rfc3339").arg("2025-03-15 17:50:00-10:00"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2025-03-15T17:50:00-10:00[-10:00]

    ----- stderr -----
    ",
    );
}

/// Test that RFC 2822 works.
#[test]
fn rfc2822() {
    assert_cmd_snapshot!(
        parse().arg("-f").arg("rfc2822").arg("Sat, 15 Mar 2025 17:50:00 -1000"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2025-03-15T17:50:00-10:00[-10:00]

    ----- stderr -----
    ",
    );
}

/// Test that RFC 9110 works.
#[test]
fn rfc9110() {
    assert_cmd_snapshot!(
        parse().arg("-f").arg("rfc9110").arg("Sat, 15 Mar 2025 17:50:00 GMT"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2025-03-15T17:50:00+00:00[UTC]

    ----- stderr -----
    ",
    );
}

/// Test that strptime works.
#[test]
fn strptime() {
    assert_cmd_snapshot!(
        parse().arg("-f").arg("%A %Y-%m-%d %H:%M:%S %:z %Q").arg(
            "Saturday 2024-07-20 16:30:55 -04:00 America/New_York",
        ),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-20T16:30:55-04:00[America/New_York]

    ----- stderr -----
    ",
    );
}

/// Test that flexible datetime parsing works.
#[test]
fn flexible() {
    assert_cmd_snapshot!(
        parse().arg("-f").arg("flexible").stdin("1 hour ago"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-20T15:30:55-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        parse().args([
            "-f",
            "flexible",
            "-r",
            "2025-05-01T00:00:00-04",
        ]).stdin("1 hour ago"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2025-04-30T23:00:00-04:00[-04:00]

    ----- stderr -----
    ",
    );
}
