use crate::command::assert_cmd_snapshot;

fn inn() -> crate::command::Command {
    crate::biff(["time", "in"])
}

#[test]
fn time_zone_first_positional() {
    assert_cmd_snapshot!(
        inn().arg("Australia/Sydney").arg("now"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-21T06:30:55+10:00[Australia/Sydney]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        inn().arg("Australia/Sydney").arg("-1d").arg("1d"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-20T06:30:55+10:00[Australia/Sydney]
    2024-07-22T06:30:55+10:00[Australia/Sydney]

    ----- stderr -----
    ",
    );
}

#[test]
fn datetime_first_positional() {
    assert_cmd_snapshot!(
        inn().arg("now").arg("Australia/Sydney"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-21T06:30:55+10:00[Australia/Sydney]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        inn().arg("now").arg("Australia/Sydney").arg("Europe/Moscow"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-21T06:30:55+10:00[Australia/Sydney]
    2024-07-20T23:30:55+03:00[Europe/Moscow]

    ----- stderr -----
    ",
    );
}

#[test]
fn posix_time_zone() {
    assert_cmd_snapshot!(
        inn().arg("EST5EDT,M3.2.0,M11.1.0").arg("now"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-20T16:30:55-04:00[-04:00]

    ----- stderr -----
    ",
    );
}

/// This tests the error message one gets when an invalid IANA time zone
/// identifier is given.
///
/// This is somewhat tricky to do well on, because the first argument can be
/// an IANA time zone identifier *or* a datetime. So the error messages here
/// are somewhat based on guessing the user's intent.
#[test]
fn invalid_time_zone() {
    assert_cmd_snapshot!(
        inn().arg("Australia/Syydney").arg("now"),
        @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    parsed apparent IANA time zone identifier, but the tzdb lookup failed: failed to find time zone `Australia/Syydney` in time zone database
    ",
    );

    assert_cmd_snapshot!(
        inn().arg("Isreal").arg("now"),
        @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    parsed apparent IANA time zone identifier, but the tzdb lookup failed: failed to find time zone `Isreal` in time zone database
    ",
    );

    // This one isn't so good, because it gets through our heuristics.
    assert_cmd_snapshot!(
        inn().arg("isreal").arg("now"),
        @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    unrecognized datetime `isreal`
    ",
    );

    assert_cmd_snapshot!(
        inn().arg("+27:00").arg("now"),
        @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    failed to parse hours in UTC numeric offset: failed to parse hours (requires a two digit integer): parameter 'time zone offset hours' is not in the required range of -25..=25
    ",
    );
}

/// This tests the error message one gets when an invalid IANA time zone
/// identifier is given.
///
/// This is somewhat tricky to do well on, because the first argument can be
/// an IANA time zone identifier *or* a datetime. So the error messages here
/// are somewhat based on guessing the user's intent.
#[test]
fn invalid_datetime() {
    assert_cmd_snapshot!(
        inn().arg("2025-02-29T00Z").arg("Australia/Sydney"),
        @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    unrecognized datetime `2025-02-29T00Z`
    ",
    );
}
