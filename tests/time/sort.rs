use crate::{biff, command::assert_cmd_snapshot};

fn sort() -> crate::command::Command {
    biff(["time", "sort"])
}

#[test]
fn simple() {
    assert_cmd_snapshot!(
        sort().arg("-1d").arg("now").arg("1d"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-19T16:30:55-04:00[America/New_York]
    2024-07-20T16:30:55-04:00[America/New_York]
    2024-07-21T16:30:55-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        sort().arg("-r").arg("-1d").arg("now").arg("1d"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-21T16:30:55-04:00[America/New_York]
    2024-07-20T16:30:55-04:00[America/New_York]
    2024-07-19T16:30:55-04:00[America/New_York]

    ----- stderr -----
    ",
    );
}

#[test]
fn simple_tagged() {
    let stdin = "\
2024-07-20T16:30:55-04:00[America/New_York]
2024-07-21T16:30:55-04:00[America/New_York]
2024-07-19T16:30:55-04:00[America/New_York]
";

    assert_cmd_snapshot!(
        biff(["tag", "lines"]).stdin(stdin).pipe(sort()),
        @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {"tags":[{"value":"2024-07-19T16:30:55-04:00[America/New_York]","range":[0,43]}],"data":{"text":"2024-07-19T16:30:55-04:00[America/New_York]\n"}}
    {"tags":[{"value":"2024-07-20T16:30:55-04:00[America/New_York]","range":[0,43]}],"data":{"text":"2024-07-20T16:30:55-04:00[America/New_York]\n"}}
    {"tags":[{"value":"2024-07-21T16:30:55-04:00[America/New_York]","range":[0,43]}],"data":{"text":"2024-07-21T16:30:55-04:00[America/New_York]\n"}}

    ----- stderr -----
    "#,
    );

    assert_cmd_snapshot!(
        biff(["tag", "lines"]).stdin(stdin).pipe(sort().arg("-r")),
        @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {"tags":[{"value":"2024-07-21T16:30:55-04:00[America/New_York]","range":[0,43]}],"data":{"text":"2024-07-21T16:30:55-04:00[America/New_York]\n"}}
    {"tags":[{"value":"2024-07-20T16:30:55-04:00[America/New_York]","range":[0,43]}],"data":{"text":"2024-07-20T16:30:55-04:00[America/New_York]\n"}}
    {"tags":[{"value":"2024-07-19T16:30:55-04:00[America/New_York]","range":[0,43]}],"data":{"text":"2024-07-19T16:30:55-04:00[America/New_York]\n"}}

    ----- stderr -----
    "#,
    );
}

#[test]
fn multiple_tagged_simple() {
    let stdin = "\
2024-07-20T00Z 2024-07-21T00Z
2024-07-19T00Z 2024-07-22T00Z
";

    assert_cmd_snapshot!(
        biff(["tag", "lines", "--all"]).stdin(stdin).pipe(sort()),
        @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {"tags":[{"value":"2024-07-19T00:00:00Z[Etc/Unknown]","range":[0,14]},{"value":"2024-07-22T00:00:00Z[Etc/Unknown]","range":[15,29]}],"data":{"text":"2024-07-19T00Z 2024-07-22T00Z\n"}}
    {"tags":[{"value":"2024-07-20T00:00:00Z[Etc/Unknown]","range":[0,14]},{"value":"2024-07-21T00:00:00Z[Etc/Unknown]","range":[15,29]}],"data":{"text":"2024-07-20T00Z 2024-07-21T00Z\n"}}

    ----- stderr -----
    "#,
    );
}

#[test]
fn multiple_tagged_different_length() {
    let stdin = "\
2024-07-20T00Z 2024-07-21T00Z
2024-07-19T00Z
";

    assert_cmd_snapshot!(
        biff(["tag", "lines", "--all"]).stdin(stdin).pipe(sort()),
        @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {"tags":[{"value":"2024-07-19T00:00:00Z[Etc/Unknown]","range":[0,14]}],"data":{"text":"2024-07-19T00Z\n"}}
    {"tags":[{"value":"2024-07-20T00:00:00Z[Etc/Unknown]","range":[0,14]},{"value":"2024-07-21T00:00:00Z[Etc/Unknown]","range":[15,29]}],"data":{"text":"2024-07-20T00Z 2024-07-21T00Z\n"}}

    ----- stderr -----
    "#,
    );
}

#[test]
fn multiple_tagged_same_first_tag() {
    let stdin = "\
2024-07-19T00Z 2024-07-22T00Z
2024-07-19T00Z 2024-07-21T00Z
";

    assert_cmd_snapshot!(
        biff(["tag", "lines", "--all"]).stdin(stdin).pipe(sort()),
        @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {"tags":[{"value":"2024-07-19T00:00:00Z[Etc/Unknown]","range":[0,14]},{"value":"2024-07-21T00:00:00Z[Etc/Unknown]","range":[15,29]}],"data":{"text":"2024-07-19T00Z 2024-07-21T00Z\n"}}
    {"tags":[{"value":"2024-07-19T00:00:00Z[Etc/Unknown]","range":[0,14]},{"value":"2024-07-22T00:00:00Z[Etc/Unknown]","range":[15,29]}],"data":{"text":"2024-07-19T00Z 2024-07-22T00Z\n"}}

    ----- stderr -----
    "#,
    );
}
