use crate::command::assert_cmd_snapshot;

fn start() -> crate::command::Command {
    crate::biff(["time", "start-of"])
}

fn end() -> crate::command::Command {
    crate::biff(["time", "end-of"])
}

#[test]
fn various_start() {
    assert_cmd_snapshot!(
        start().args(["year", "now"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-01-01T00:00:00-05:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        start().args(["month", "now"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-01T00:00:00-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        start().args(["week-sunday", "now"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-14T00:00:00-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        start().args(["week-monday", "now"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-15T00:00:00-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        start().args(["day", "now"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-20T00:00:00-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        start().args(["hour", "now"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-20T16:00:00-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        start().args(["minute", "now"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-20T16:30:00-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        start().args(["second", "2024-07-20T16:30:55.123456789"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-20T16:30:55-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        start().args(["milli", "2024-07-20T16:30:55.123456789"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-20T16:30:55.123-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        start().args(["micro", "2024-07-20T16:30:55.123456789"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-20T16:30:55.123456-04:00[America/New_York]

    ----- stderr -----
    ",
    );
}

#[test]
fn various_end() {
    assert_cmd_snapshot!(
        end().args(["year", "now"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-12-31T23:59:59.999999999-05:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        end().args(["month", "now"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-31T23:59:59.999999999-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        end().args(["week-sunday", "now"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-20T23:59:59.999999999-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        end().args(["week-monday", "now"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-21T23:59:59.999999999-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        end().args(["day", "now"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-20T23:59:59.999999999-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        end().args(["hour", "now"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-20T16:59:59.999999999-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        end().args(["minute", "now"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-20T16:30:59.999999999-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        end().args(["second", "2024-07-20T16:30:55.123456789"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-20T16:30:55.999999999-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        end().args(["milli", "2024-07-20T16:30:55.123456789"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-20T16:30:55.123999999-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        end().args(["micro", "2024-07-20T16:30:55.123456789"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-20T16:30:55.123456999-04:00[America/New_York]

    ----- stderr -----
    ",
    );
}
