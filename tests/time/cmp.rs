use crate::{biff, command::assert_cmd_snapshot};

fn cmp() -> crate::command::Command {
    biff(["time", "cmp"])
}

#[test]
fn eq() {
    assert_cmd_snapshot!(
        cmp().args(["eq", "now", "2000-01-01", "now", "2030-01-01"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-20T16:30:55-04:00[America/New_York]

    ----- stderr -----
    ",
    );
}

#[test]
fn ne() {
    assert_cmd_snapshot!(
        cmp().args(["ne", "now", "2000-01-01", "now", "2030-01-01"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2000-01-01T00:00:00-05:00[America/New_York]
    2030-01-01T00:00:00-05:00[America/New_York]

    ----- stderr -----
    ",
    );
}

#[test]
fn lt() {
    assert_cmd_snapshot!(
        cmp().args(["lt", "now", "2000-01-01", "now", "2030-01-01"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2000-01-01T00:00:00-05:00[America/New_York]

    ----- stderr -----
    ",
    );
}

#[test]
fn gt() {
    assert_cmd_snapshot!(
        cmp().args(["gt", "now", "2000-01-01", "now", "2030-01-01"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2030-01-01T00:00:00-05:00[America/New_York]

    ----- stderr -----
    ",
    );
}

#[test]
fn le() {
    assert_cmd_snapshot!(
        cmp().args(["le", "now", "2000-01-01", "now", "2030-01-01"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2000-01-01T00:00:00-05:00[America/New_York]
    2024-07-20T16:30:55-04:00[America/New_York]

    ----- stderr -----
    ",
    );
}

#[test]
fn ge() {
    assert_cmd_snapshot!(
        cmp().args(["ge", "now", "2000-01-01", "now", "2030-01-01"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-20T16:30:55-04:00[America/New_York]
    2030-01-01T00:00:00-05:00[America/New_York]

    ----- stderr -----
    ",
    );
}

#[test]
fn single_tag() {
    let stdin = "\
2000-01-01T00Z
2024-07-20T16:30:55-04:00[America/New_York]
2030-01-01T00Z
";
    assert_cmd_snapshot!(
        biff(["tag", "lines"]).stdin(stdin).pipe(
            cmp().args(["lt", "now"]),
        ),
        @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {"tags":[{"value":"2000-01-01T00:00:00Z[Etc/Unknown]","range":[0,14]}],"data":{"text":"2000-01-01T00Z\n"}}

    ----- stderr -----
    "#,
    );
}

#[test]
fn multi_tag() {
    let stdin = "\
2000-01-01T00Z 2001-01-01T00Z
2024-07-20T16:30:55-04:00[America/New_York]
2030-01-01T00Z 2002-01-01T00Z
2030-01-01T00Z 2031-01-01T00Z
";

    assert_cmd_snapshot!(
        biff(["tag", "lines", "--all"]).stdin(stdin).pipe(
            cmp().args(["lt", "now"]),
        ),
        @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {"tags":[{"value":"2000-01-01T00:00:00Z[Etc/Unknown]","range":[0,14]},{"value":"2001-01-01T00:00:00Z[Etc/Unknown]","range":[15,29]}],"data":{"text":"2000-01-01T00Z 2001-01-01T00Z\n"}}
    {"tags":[{"value":"2002-01-01T00:00:00Z[Etc/Unknown]","range":[15,29]}],"data":{"text":"2030-01-01T00Z 2002-01-01T00Z\n"}}

    ----- stderr -----
    "#,
    );

    assert_cmd_snapshot!(
        biff(["tag", "lines", "--all"]).stdin(stdin).pipe(
            cmp().args(["eq", "now"]),
        ),
        @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {"tags":[{"value":"2024-07-20T16:30:55-04:00[America/New_York]","range":[0,43]}],"data":{"text":"2024-07-20T16:30:55-04:00[America/New_York]\n"}}

    ----- stderr -----
    "#,
    );
}
