use crate::command::assert_cmd_snapshot;

fn seq() -> crate::command::Command {
    crate::biff(["time", "seq"])
}

// N.B. We don't really try to test the RFC 5545 functionality here too much,
// since that is extensively tested via unit tests within Biff. Instead, we
// try to focus a bit more on the CLI interaction points.

#[test]
fn by_month() {
    assert_cmd_snapshot!(
        seq().args(["-c5", "yearly", "-m2"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2025-02-20T16:30:55-05:00[America/New_York]
    2026-02-20T16:30:55-05:00[America/New_York]
    2027-02-20T16:30:55-05:00[America/New_York]
    2028-02-20T16:30:55-05:00[America/New_York]
    2029-02-20T16:30:55-05:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        seq().args(["-c15", "yearly", "-m5..7"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-20T16:30:55-04:00[America/New_York]
    2025-05-20T16:30:55-04:00[America/New_York]
    2025-06-20T16:30:55-04:00[America/New_York]
    2025-07-20T16:30:55-04:00[America/New_York]
    2026-05-20T16:30:55-04:00[America/New_York]
    2026-06-20T16:30:55-04:00[America/New_York]
    2026-07-20T16:30:55-04:00[America/New_York]
    2027-05-20T16:30:55-04:00[America/New_York]
    2027-06-20T16:30:55-04:00[America/New_York]
    2027-07-20T16:30:55-04:00[America/New_York]
    2028-05-20T16:30:55-04:00[America/New_York]
    2028-06-20T16:30:55-04:00[America/New_York]
    2028-07-20T16:30:55-04:00[America/New_York]
    2029-05-20T16:30:55-04:00[America/New_York]
    2029-06-20T16:30:55-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        seq().args(["-c15", "yearly", "-m1,3,5,7,9,11"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-20T16:30:55-04:00[America/New_York]
    2024-09-20T16:30:55-04:00[America/New_York]
    2024-11-20T16:30:55-05:00[America/New_York]
    2025-01-20T16:30:55-05:00[America/New_York]
    2025-03-20T16:30:55-04:00[America/New_York]
    2025-05-20T16:30:55-04:00[America/New_York]
    2025-07-20T16:30:55-04:00[America/New_York]
    2025-09-20T16:30:55-04:00[America/New_York]
    2025-11-20T16:30:55-05:00[America/New_York]
    2026-01-20T16:30:55-05:00[America/New_York]
    2026-03-20T16:30:55-04:00[America/New_York]
    2026-05-20T16:30:55-04:00[America/New_York]
    2026-07-20T16:30:55-04:00[America/New_York]
    2026-09-20T16:30:55-04:00[America/New_York]
    2026-11-20T16:30:55-05:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        seq().args(["-c15", "yearly", "-m1..3,10..12"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-10-20T16:30:55-04:00[America/New_York]
    2024-11-20T16:30:55-05:00[America/New_York]
    2024-12-20T16:30:55-05:00[America/New_York]
    2025-01-20T16:30:55-05:00[America/New_York]
    2025-02-20T16:30:55-05:00[America/New_York]
    2025-03-20T16:30:55-04:00[America/New_York]
    2025-10-20T16:30:55-04:00[America/New_York]
    2025-11-20T16:30:55-05:00[America/New_York]
    2025-12-20T16:30:55-05:00[America/New_York]
    2026-01-20T16:30:55-05:00[America/New_York]
    2026-02-20T16:30:55-05:00[America/New_York]
    2026-03-20T16:30:55-04:00[America/New_York]
    2026-10-20T16:30:55-04:00[America/New_York]
    2026-11-20T16:30:55-05:00[America/New_York]
    2026-12-20T16:30:55-05:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        seq().args(["-c15", "yearly", "-m1..3", "-m10..12"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-10-20T16:30:55-04:00[America/New_York]
    2024-11-20T16:30:55-05:00[America/New_York]
    2024-12-20T16:30:55-05:00[America/New_York]
    2025-01-20T16:30:55-05:00[America/New_York]
    2025-02-20T16:30:55-05:00[America/New_York]
    2025-03-20T16:30:55-04:00[America/New_York]
    2025-10-20T16:30:55-04:00[America/New_York]
    2025-11-20T16:30:55-05:00[America/New_York]
    2025-12-20T16:30:55-05:00[America/New_York]
    2026-01-20T16:30:55-05:00[America/New_York]
    2026-02-20T16:30:55-05:00[America/New_York]
    2026-03-20T16:30:55-04:00[America/New_York]
    2026-10-20T16:30:55-04:00[America/New_York]
    2026-11-20T16:30:55-05:00[America/New_York]
    2026-12-20T16:30:55-05:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        seq().args(["-c15", "yearly", "-mjan..mar,oct..DECEMBER"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-10-20T16:30:55-04:00[America/New_York]
    2024-11-20T16:30:55-05:00[America/New_York]
    2024-12-20T16:30:55-05:00[America/New_York]
    2025-01-20T16:30:55-05:00[America/New_York]
    2025-02-20T16:30:55-05:00[America/New_York]
    2025-03-20T16:30:55-04:00[America/New_York]
    2025-10-20T16:30:55-04:00[America/New_York]
    2025-11-20T16:30:55-05:00[America/New_York]
    2025-12-20T16:30:55-05:00[America/New_York]
    2026-01-20T16:30:55-05:00[America/New_York]
    2026-02-20T16:30:55-05:00[America/New_York]
    2026-03-20T16:30:55-04:00[America/New_York]
    2026-10-20T16:30:55-04:00[America/New_York]
    2026-11-20T16:30:55-05:00[America/New_York]
    2026-12-20T16:30:55-05:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        seq().args(["-c15", "yearly", "-mjanu"]),
        @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    -m/--month: failed to parse `janu` within sequence `janu`: failed to parse `janu` as a single signed integer
    ",
    );
}

#[test]
fn by_week() {
    assert_cmd_snapshot!(
        seq().args(["-c10", "yearly", "--week=52"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-12-23T16:30:55-05:00[America/New_York]
    2024-12-24T16:30:55-05:00[America/New_York]
    2024-12-25T16:30:55-05:00[America/New_York]
    2024-12-26T16:30:55-05:00[America/New_York]
    2024-12-27T16:30:55-05:00[America/New_York]
    2024-12-28T16:30:55-05:00[America/New_York]
    2024-12-29T16:30:55-05:00[America/New_York]
    2025-12-22T16:30:55-05:00[America/New_York]
    2025-12-23T16:30:55-05:00[America/New_York]
    2025-12-24T16:30:55-05:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        seq().args(["-c10", "yearly", "--week=53", "2020-01-01"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2020-12-28T00:00:00-05:00[America/New_York]
    2020-12-29T00:00:00-05:00[America/New_York]
    2020-12-30T00:00:00-05:00[America/New_York]
    2020-12-31T00:00:00-05:00[America/New_York]
    2021-01-01T00:00:00-05:00[America/New_York]
    2021-01-02T00:00:00-05:00[America/New_York]
    2021-01-03T00:00:00-05:00[America/New_York]
    2026-12-28T00:00:00-05:00[America/New_York]
    2026-12-29T00:00:00-05:00[America/New_York]
    2026-12-30T00:00:00-05:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        seq().args(["-c10", "yearly", "--week=-1", "2020-01-01"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2020-12-28T00:00:00-05:00[America/New_York]
    2020-12-29T00:00:00-05:00[America/New_York]
    2020-12-30T00:00:00-05:00[America/New_York]
    2020-12-31T00:00:00-05:00[America/New_York]
    2021-01-01T00:00:00-05:00[America/New_York]
    2021-01-02T00:00:00-05:00[America/New_York]
    2021-01-03T00:00:00-05:00[America/New_York]
    2022-12-26T00:00:00-05:00[America/New_York]
    2022-12-27T00:00:00-05:00[America/New_York]
    2022-12-28T00:00:00-05:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        seq().args(["-c10", "yearly", "--week=-2..-1", "2020-01-01"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2020-12-21T00:00:00-05:00[America/New_York]
    2020-12-22T00:00:00-05:00[America/New_York]
    2020-12-23T00:00:00-05:00[America/New_York]
    2020-12-24T00:00:00-05:00[America/New_York]
    2020-12-25T00:00:00-05:00[America/New_York]
    2020-12-26T00:00:00-05:00[America/New_York]
    2020-12-27T00:00:00-05:00[America/New_York]
    2020-12-28T00:00:00-05:00[America/New_York]
    2020-12-29T00:00:00-05:00[America/New_York]
    2020-12-30T00:00:00-05:00[America/New_York]

    ----- stderr -----
    ",
    );

    // Difference based on --week-start. This is copied from the ical
    // unit tests to ensure we're permitting setting the start of the
    // week correctly.
    assert_cmd_snapshot!(
        seq().args([
            "weekly",
            "-c4",
            "-i2",
            "--week-start=mon",
            "-wTue,Sun",
            "19970805T090000[America/New_York]",
        ]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    1997-08-05T09:00:00-04:00[America/New_York]
    1997-08-10T09:00:00-04:00[America/New_York]
    1997-08-19T09:00:00-04:00[America/New_York]
    1997-08-24T09:00:00-04:00[America/New_York]

    ----- stderr -----
    ",
    );
    assert_cmd_snapshot!(
        seq().args([
            "weekly",
            "-c4",
            "-i2",
            "--week-start=sun",
            "-wTue,Sun",
            "19970805T090000[America/New_York]",
        ]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    1997-08-05T09:00:00-04:00[America/New_York]
    1997-08-17T09:00:00-04:00[America/New_York]
    1997-08-19T09:00:00-04:00[America/New_York]
    1997-08-31T09:00:00-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        seq().args(["-c10", "yearly", "--week=-1..-2", "2020-01-01"]),
        @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    --week: failed to parse `-1..-2` within sequence `-1..-2`: parsed ranges must have start <= end, but `-1..-2` has start > end
    ",
    );
    assert_cmd_snapshot!(
        seq().args(["-c10", "yearly", "--week=1,,2", "2020-01-01"]),
        @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    --week: failed to parse `` within sequence `1,,2`: failed to parse `` as a single signed integer
    ",
    );
    assert_cmd_snapshot!(
        seq().args(["-c10", "monthly", "--week=1"]),
        @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    'by week' cannot be used with any frequency except yearly
    ",
    );
}

#[test]
fn by_year_day() {
    assert_cmd_snapshot!(
        seq().args(["-c5", "yearly", "--doy=2"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2025-01-02T16:30:55-05:00[America/New_York]
    2026-01-02T16:30:55-05:00[America/New_York]
    2027-01-02T16:30:55-05:00[America/New_York]
    2028-01-02T16:30:55-05:00[America/New_York]
    2029-01-02T16:30:55-05:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        seq().args(["-c5", "yearly", "--doy=366"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-12-31T16:30:55-05:00[America/New_York]
    2028-12-31T16:30:55-05:00[America/New_York]
    2032-12-31T16:30:55-05:00[America/New_York]
    2036-12-31T16:30:55-05:00[America/New_York]
    2040-12-31T16:30:55-05:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        seq().args(["-c5", "yearly", "--doy=-1,1"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-12-31T16:30:55-05:00[America/New_York]
    2025-01-01T16:30:55-05:00[America/New_York]
    2025-12-31T16:30:55-05:00[America/New_York]
    2026-01-01T16:30:55-05:00[America/New_York]
    2026-12-31T16:30:55-05:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        seq().args(["-c15", "yearly", "--doy=-3..-1,1..3"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-12-29T16:30:55-05:00[America/New_York]
    2024-12-30T16:30:55-05:00[America/New_York]
    2024-12-31T16:30:55-05:00[America/New_York]
    2025-01-01T16:30:55-05:00[America/New_York]
    2025-01-02T16:30:55-05:00[America/New_York]
    2025-01-03T16:30:55-05:00[America/New_York]
    2025-12-29T16:30:55-05:00[America/New_York]
    2025-12-30T16:30:55-05:00[America/New_York]
    2025-12-31T16:30:55-05:00[America/New_York]
    2026-01-01T16:30:55-05:00[America/New_York]
    2026-01-02T16:30:55-05:00[America/New_York]
    2026-01-03T16:30:55-05:00[America/New_York]
    2026-12-29T16:30:55-05:00[America/New_York]
    2026-12-30T16:30:55-05:00[America/New_York]
    2026-12-31T16:30:55-05:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        seq().args(["-c15", "hourly", "--doy=201..203"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-20T16:30:55-04:00[America/New_York]
    2024-07-20T17:30:55-04:00[America/New_York]
    2024-07-20T18:30:55-04:00[America/New_York]
    2024-07-20T19:30:55-04:00[America/New_York]
    2024-07-20T20:30:55-04:00[America/New_York]
    2024-07-20T21:30:55-04:00[America/New_York]
    2024-07-20T22:30:55-04:00[America/New_York]
    2024-07-20T23:30:55-04:00[America/New_York]
    2024-07-21T00:30:55-04:00[America/New_York]
    2024-07-21T01:30:55-04:00[America/New_York]
    2024-07-21T02:30:55-04:00[America/New_York]
    2024-07-21T03:30:55-04:00[America/New_York]
    2024-07-21T04:30:55-04:00[America/New_York]
    2024-07-21T05:30:55-04:00[America/New_York]
    2024-07-21T06:30:55-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        seq().args(["-c15", "monthly", "--doy=201..203"]),
        @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    'by day of the year' cannot be used with monthly, weekly or daily frequency
    ",
    );
    assert_cmd_snapshot!(
        seq().args(["-c15", "week", "--doy=201..203"]),
        @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    'by day of the year' cannot be used with monthly, weekly or daily frequency
    ",
    );
    assert_cmd_snapshot!(
        seq().args(["-c15", "d", "--doy=201..203"]),
        @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    'by day of the year' cannot be used with monthly, weekly or daily frequency
    ",
    );
}

#[test]
fn by_month_day() {
    assert_cmd_snapshot!(
        seq().args(["-c5", "yearly", "-d10"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-08-10T16:30:55-04:00[America/New_York]
    2024-09-10T16:30:55-04:00[America/New_York]
    2024-10-10T16:30:55-04:00[America/New_York]
    2024-11-10T16:30:55-05:00[America/New_York]
    2024-12-10T16:30:55-05:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        seq().args(["-c5", "yearly", "-d10,12,30"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-30T16:30:55-04:00[America/New_York]
    2024-08-10T16:30:55-04:00[America/New_York]
    2024-08-12T16:30:55-04:00[America/New_York]
    2024-08-30T16:30:55-04:00[America/New_York]
    2024-09-10T16:30:55-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        seq().args(["-c10", "yearly", "-d1,-1,14..16"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-31T16:30:55-04:00[America/New_York]
    2024-08-01T16:30:55-04:00[America/New_York]
    2024-08-14T16:30:55-04:00[America/New_York]
    2024-08-15T16:30:55-04:00[America/New_York]
    2024-08-16T16:30:55-04:00[America/New_York]
    2024-08-31T16:30:55-04:00[America/New_York]
    2024-09-01T16:30:55-04:00[America/New_York]
    2024-09-14T16:30:55-04:00[America/New_York]
    2024-09-15T16:30:55-04:00[America/New_York]
    2024-09-16T16:30:55-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        seq().args(["-c5", "monthly", "-d10"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-08-10T16:30:55-04:00[America/New_York]
    2024-09-10T16:30:55-04:00[America/New_York]
    2024-10-10T16:30:55-04:00[America/New_York]
    2024-11-10T16:30:55-05:00[America/New_York]
    2024-12-10T16:30:55-05:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        seq().args(["-c5", "monthly", "-d29", "2024-02-01"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-02-29T00:00:00-05:00[America/New_York]
    2024-03-29T00:00:00-04:00[America/New_York]
    2024-04-29T00:00:00-04:00[America/New_York]
    2024-05-29T00:00:00-04:00[America/New_York]
    2024-06-29T00:00:00-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        seq().args(["-c5", "weekly", "-d10"]),
        @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    'by day of the month' cannot be used with weekly frequency
    ",
    );
}

#[test]
fn by_week_day() {
    assert_cmd_snapshot!(
        seq().args(["-c5", "yearly", "-wFri"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-26T16:30:55-04:00[America/New_York]
    2024-08-02T16:30:55-04:00[America/New_York]
    2024-08-09T16:30:55-04:00[America/New_York]
    2024-08-16T16:30:55-04:00[America/New_York]
    2024-08-23T16:30:55-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        seq().args(["-c10", "weekly", "-wMon..Wed"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-22T16:30:55-04:00[America/New_York]
    2024-07-23T16:30:55-04:00[America/New_York]
    2024-07-24T16:30:55-04:00[America/New_York]
    2024-07-29T16:30:55-04:00[America/New_York]
    2024-07-30T16:30:55-04:00[America/New_York]
    2024-07-31T16:30:55-04:00[America/New_York]
    2024-08-05T16:30:55-04:00[America/New_York]
    2024-08-06T16:30:55-04:00[America/New_York]
    2024-08-07T16:30:55-04:00[America/New_York]
    2024-08-12T16:30:55-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        seq().args(["-c10", "yearly", "-w1-Fri,-1-Fri"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-12-27T16:30:55-05:00[America/New_York]
    2025-01-03T16:30:55-05:00[America/New_York]
    2025-12-26T16:30:55-05:00[America/New_York]
    2026-01-02T16:30:55-05:00[America/New_York]
    2026-12-25T16:30:55-05:00[America/New_York]
    2027-01-01T16:30:55-05:00[America/New_York]
    2027-12-31T16:30:55-05:00[America/New_York]
    2028-01-07T16:30:55-05:00[America/New_York]
    2028-12-29T16:30:55-05:00[America/New_York]
    2029-01-05T16:30:55-05:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        seq().args(["-c10", "yearly", "-w1-Mon..2-Mon"]),
        @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    -w/--week-day: failed to parse `1-Mon..2-Mon` within sequence `1-Mon..2-Mon`: numbered weekday `1-Mon` is not allowed in a range
    ",
    );
    assert_cmd_snapshot!(
        seq().args(["-c10", "yearly", "-wTHUR"]),
        @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    -w/--week-day: failed to parse `THUR` within sequence `THUR`: failed to parse `THUR` as a single weekday or numbered weekday
    ",
    );
}

#[test]
fn by_hour() {
    assert_cmd_snapshot!(
        seq().args(["-c5", "yearly", "-H17"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-20T17:30:55-04:00[America/New_York]
    2025-07-20T17:30:55-04:00[America/New_York]
    2026-07-20T17:30:55-04:00[America/New_York]
    2027-07-20T17:30:55-04:00[America/New_York]
    2028-07-20T17:30:55-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        seq().args(["-c5", "yearly", "-H11..14"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2025-07-20T11:30:55-04:00[America/New_York]
    2025-07-20T12:30:55-04:00[America/New_York]
    2025-07-20T13:30:55-04:00[America/New_York]
    2025-07-20T14:30:55-04:00[America/New_York]
    2026-07-20T11:30:55-04:00[America/New_York]

    ----- stderr -----
    ",
    );
}

#[test]
fn by_minute() {
    assert_cmd_snapshot!(
        seq().args(["-c5", "yearly", "-M17"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2025-07-20T16:17:55-04:00[America/New_York]
    2026-07-20T16:17:55-04:00[America/New_York]
    2027-07-20T16:17:55-04:00[America/New_York]
    2028-07-20T16:17:55-04:00[America/New_York]
    2029-07-20T16:17:55-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        seq().args(["-c5", "yearly", "-M11..14"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2025-07-20T16:11:55-04:00[America/New_York]
    2025-07-20T16:12:55-04:00[America/New_York]
    2025-07-20T16:13:55-04:00[America/New_York]
    2025-07-20T16:14:55-04:00[America/New_York]
    2026-07-20T16:11:55-04:00[America/New_York]

    ----- stderr -----
    ",
    );
}

#[test]
fn by_second() {
    assert_cmd_snapshot!(
        seq().args(["-c5", "yearly", "-S17"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2025-07-20T16:30:17-04:00[America/New_York]
    2026-07-20T16:30:17-04:00[America/New_York]
    2027-07-20T16:30:17-04:00[America/New_York]
    2028-07-20T16:30:17-04:00[America/New_York]
    2029-07-20T16:30:17-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        seq().args(["-c5", "yearly", "-S11..14"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2025-07-20T16:30:11-04:00[America/New_York]
    2025-07-20T16:30:12-04:00[America/New_York]
    2025-07-20T16:30:13-04:00[America/New_York]
    2025-07-20T16:30:14-04:00[America/New_York]
    2026-07-20T16:30:11-04:00[America/New_York]

    ----- stderr -----
    ",
    );
}
