use crate::command::assert_cmd_snapshot;

fn round() -> crate::command::Command {
    crate::biff(["time", "round"])
}

#[test]
fn nearest_day_default() {
    assert_cmd_snapshot!(
        round().args(["-sday", "2025-03-05T12:01"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2025-03-06T00:00:00-05:00[America/New_York]

    ----- stderr -----
    ",
    );
}

#[test]
fn nearest_day_trunc() {
    assert_cmd_snapshot!(
        round().args(["-sday", "-mtrunc", "2025-03-05T12:01"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2025-03-05T00:00:00-05:00[America/New_York]

    ----- stderr -----
    ",
    );
}

#[test]
fn nearest_half_hour() {
    assert_cmd_snapshot!(
        round().args(["-sminute", "-i30", "2025-03-05T12:01"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2025-03-05T12:00:00-05:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        round().args(["-sminute", "-i30", "2025-03-05T12:15"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2025-03-05T12:30:00-05:00[America/New_York]

    ----- stderr -----
    ",
    );
}

#[test]
fn dst_day() {
    assert_cmd_snapshot!(
        round().args(["-sday", "2025-03-09T12:15[America/New_York]"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2025-03-09T00:00:00-05:00[America/New_York]

    ----- stderr -----
    ",
    );
    assert_cmd_snapshot!(
        round().args(["-sday", "2025-03-09T12:29[America/New_York]"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2025-03-09T00:00:00-05:00[America/New_York]

    ----- stderr -----
    ",
    );
    assert_cmd_snapshot!(
        round().args(["-sday", "2025-03-09T12:30[America/New_York]"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2025-03-10T00:00:00-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        round().args(["-sday", "2025-11-02T11:29[America/New_York]"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2025-11-02T00:00:00-04:00[America/New_York]

    ----- stderr -----
    ",
    );
    assert_cmd_snapshot!(
        round().args(["-sday", "2025-11-02T11:30[America/New_York]"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2025-11-03T00:00:00-05:00[America/New_York]

    ----- stderr -----
    ",
    );
}

#[test]
fn dst_time() {
    assert_cmd_snapshot!(
        round().args(["-shour", "2025-03-09T01:59[America/New_York]"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2025-03-09T03:00:00-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        round().args([
            "-sminute",
            "-i30",
            "2025-11-02T01:29-04[America/New_York]",
        ]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2025-11-02T01:30:00-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        round().args([
            "-sminute",
            "-i30",
            "2025-11-02T01:29-05[America/New_York]",
        ]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2025-11-02T01:30:00-05:00[America/New_York]

    ----- stderr -----
    ",
    );
}

#[test]
fn errors() {
    assert_cmd_snapshot!(
        round().args(["-swat", "now"]),
        @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    -s/--smallest: unrecognized span unit: `wat`
    ",
    );

    assert_cmd_snapshot!(
        round().args(["-syear", "now"]),
        @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    failed rounding datetime: rounding to 'years' is not supported
    ",
    );
    assert_cmd_snapshot!(
        round().args(["-smonth", "now"]),
        @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    failed rounding datetime: rounding to 'months' is not supported
    ",
    );
    assert_cmd_snapshot!(
        round().args(["-sweek", "now"]),
        @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    failed rounding datetime: rounding to 'weeks' is not supported
    ",
    );

    assert_cmd_snapshot!(
        round().args(["-sday", "-i2", "now"]),
        @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    failed rounding datetime: increment for rounding to 'days' must be equal to `1`
    ",
    );

    assert_cmd_snapshot!(
        round().args(["-sminute", "-i16", "now"]),
        @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    failed rounding datetime: increment for rounding to 'minutes' must divide into `60` evenly
    ",
    );
}
