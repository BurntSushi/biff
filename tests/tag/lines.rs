use crate::command::assert_cmd_snapshot;

fn lines() -> crate::command::Command {
    crate::biff(["tag", "lines"])
}

#[test]
fn line_terminator() {
    assert_cmd_snapshot!(
        lines().stdin("2025-03-15T00-04: yadda yadda yadda"),
        @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {"tags":[{"value":"2025-03-15T00-04","range":[0,16]}],"data":{"text":"2025-03-15T00-04: yadda yadda yadda"}}

    ----- stderr -----
    "#,
    );

    assert_cmd_snapshot!(
        lines().stdin("2025-03-15T00-04: yadda yadda yadda\n"),
        @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {"tags":[{"value":"2025-03-15T00-04","range":[0,16]}],"data":{"text":"2025-03-15T00-04: yadda yadda yadda\n"}}

    ----- stderr -----
    "#,
    );
}

#[test]
fn multiple_tags_same_line() {
    // Without --all, we only get a single tag.
    assert_cmd_snapshot!(
        lines().stdin("2025-03-15T00-04 yadda 2025-10-08T17:54Z"),
        @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {"tags":[{"value":"2025-03-15T00-04","range":[0,16]}],"data":{"text":"2025-03-15T00-04 yadda 2025-10-08T17:54Z"}}

    ----- stderr -----
    "#,
    );

    assert_cmd_snapshot!(
        lines().arg("--all").stdin("2025-03-15T00-04 yadda 2025-10-08T17:54Z"),
        @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {"tags":[{"value":"2025-03-15T00-04","range":[0,16]},{"value":"2025-10-08T17:54Z","range":[23,40]}],"data":{"text":"2025-03-15T00-04 yadda 2025-10-08T17:54Z"}}

    ----- stderr -----
    "#,
    );
}

#[test]
fn custom_regex() {
    assert_cmd_snapshot!(
        lines()
            .arg("-e")
            .arg(r"(?<tag>\S+ [0-9]{1,2}, [0-9]{4}):")
            .stdin("July 2, 1995: I did something."),
        @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {"tags":[{"value":"July 2, 1995","range":[0,12]}],"data":{"text":"July 2, 1995: I did something."}}

    ----- stderr -----
    "#,
    );
}
