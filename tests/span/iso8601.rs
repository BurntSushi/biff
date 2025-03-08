use crate::command::assert_cmd_snapshot;

fn iso8601() -> crate::command::Command {
    crate::biff(["span", "iso8601"])
}

#[test]
fn basic() {
    assert_cmd_snapshot!(
        iso8601().arg("75y5mo22d5h30m12s"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    P75Y5M22DT5H30M12S

    ----- stderr -----
    ",
    );
}

#[test]
fn lowercase() {
    assert_cmd_snapshot!(
        iso8601().arg("-l").arg("75y5mo22d5h30m12s"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    P75y5m22dT5h30m12s

    ----- stderr -----
    ",
    );
}

#[test]
fn fractional() {
    assert_cmd_snapshot!(
        iso8601().arg("999ns"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    PT0.000000999S

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        iso8601().arg("2000ms"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    PT2S

    ----- stderr -----
    ",
    );
}
