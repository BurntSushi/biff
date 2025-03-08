use crate::command::assert_cmd_snapshot;

fn since() -> crate::command::Command {
    crate::biff(["span", "since"])
}

#[test]
fn basic() {
    assert_cmd_snapshot!(
        since().arg("2023-01-01"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    13599h 30m 55s

    ----- stderr -----
    ",
    );
}

#[test]
fn unit() {
    assert_cmd_snapshot!(
        since().arg("2023-01-01").arg("-l").arg("month"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    18mo 19d 16h 30m 55s

    ----- stderr -----
    ",
    );
}

#[test]
fn relative() {
    assert_cmd_snapshot!(
        since().arg("-r").arg("2023-04-30").arg("2023-05-31"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    744h ago

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        since().arg("-r").arg("2023-04-30").arg("2023-05-31").arg("-lmo"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    1mo 1d ago

    ----- stderr -----
    ",
    );
}
