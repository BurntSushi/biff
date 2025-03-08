use crate::command::assert_cmd_snapshot;

fn relative() -> crate::command::Command {
    crate::biff(["time", "relative"])
}

#[test]
fn basic() {
    assert_cmd_snapshot!(
        relative().arg("this sat").arg("now").arg("tomorrow"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-20T16:30:55-04:00[America/New_York]
    2024-07-27T00:00:00-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        relative().arg("this sat").stdin(
            "2025-04-01T17:00-04\n2025-04-05T17:00-04",
        ),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2025-04-05T17:00:00-04:00[-04:00]
    2025-04-05T17:00:00-04:00[-04:00]

    ----- stderr -----
    ",
    );
}

#[test]
fn invalid() {
    assert_cmd_snapshot!(
        relative().arg("2025-04-01T00:00:00Z").arg("now"),
        @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    unrecognized relative datetime `2025-04-01T00:00:00Z`
    ",
    );
}
