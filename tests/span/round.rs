use crate::{biff, command::assert_cmd_snapshot};

fn round() -> crate::command::Command {
    crate::biff(["span", "round"])
}

#[test]
fn year_noleap() {
    assert_cmd_snapshot!(
        round().args(["31536000s", "-syears", "-mfloor"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    1y

    ----- stderr -----
    ",
    );
}

#[test]
fn year_leap() {
    assert_cmd_snapshot!(
        round().args(["31536000s", "-syears", "-mfloor", "-r2024-01-01"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    0s

    ----- stderr -----
    ",
    );
}

#[test]
fn since_then_round() {
    assert_cmd_snapshot!(
        biff(["span", "since", "2024-07-19T00:29:31.123456789"]).pipe(
            round().args(["-sns"]),
        ),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    40h 1m 23s 876ms 543µs 211ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        biff(["span", "since", "2024-07-19T00:29:31.123456789"]).pipe(
            round().args(["-sus"]),
        ),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    40h 1m 23s 876ms 543µs

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        biff(["span", "since", "2024-07-19T00:29:31.123456789"]).pipe(
            round().args(["-sms"]),
        ),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    40h 1m 23s 877ms

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        biff(["span", "since", "2024-07-19T00:29:31.123456789"]).pipe(
            round().args(["-ssecs"]),
        ),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    40h 1m 24s

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        biff(["span", "since", "2024-07-19T00:29:31.123456789"]).pipe(
            round().args(["-smin"]),
        ),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    40h 1m

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        biff(["span", "since", "2024-07-19T00:29:31.123456789"]).pipe(
            round().args(["-shour"]),
        ),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    40h

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        biff(["span", "since", "2024-07-19T00:29:31.123456789"]).pipe(
            round().args(["-sday"]),
        ),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2d

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        biff(["span", "since", "2024-07-19T00:29:31.123456789"]).pipe(
            round().args(["-sday", "-mtrunc"]),
        ),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    1d

    ----- stderr -----
    ",
    );
}
