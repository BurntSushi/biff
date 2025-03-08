use crate::command::assert_cmd_snapshot;

use crate::{TempDir, biff};

#[test]
fn basic() {
    let stdin = "\
2024-07-19T00Z 2024-07-21T00Z
2024-07-19T00Z 2024-07-22T00Z
";
    assert_cmd_snapshot!(
        biff(["tag", "lines"]).stdin(stdin).pipe(biff(["untag"])),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-19T00Z 2024-07-21T00Z
    2024-07-19T00Z 2024-07-22T00Z

    ----- stderr -----
    ",
    );
}

#[test]
fn substitute() {
    let stdin = "\
2024-07-19T00Z 2024-07-21T00Z
2024-07-19T00Z 2024-07-22T00Z
";
    assert_cmd_snapshot!(
        biff(["tag", "lines", "--all"]).stdin(stdin)
            .pipe(biff(["time", "parse", "--format=rfc3339"]))
            .pipe(biff(["untag", "--substitute"])),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-07-19T00:00:00Z[Etc/Unknown] 2024-07-21T00:00:00Z[Etc/Unknown]
    2024-07-19T00:00:00Z[Etc/Unknown] 2024-07-22T00:00:00Z[Etc/Unknown]

    ----- stderr -----
    ",
    );
}

#[test]
fn format_single_tag() {
    let tmp = TempDir::new();
    tmp.create("foo", "2025-03-15T00-04 Springsteen");
    tmp.create("bar", "2025-03-15T00+11 Zevon");
    tmp.create("quux", "2025-03-15T00Z Helm");

    assert_cmd_snapshot!(
        tmp.biff(["tag", "files", "foo", "bar", "quux"])
            .pipe(biff(["untag", "-f", "{tag}:{data}"])),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2025-03-15T00-04:foo
    2025-03-15T00+11:bar
    2025-03-15T00Z:quux

    ----- stderr -----
    ",
    );
}

#[test]
fn format_substitute_single_tag() {
    let tmp = TempDir::new();
    tmp.create("foo", "2025-03-15T00-04 Springsteen");
    tmp.create("bar", "2025-03-15T00+11 Zevon");
    tmp.create("quux", "2025-03-15T00Z Helm");

    assert_cmd_snapshot!(
        tmp.biff(["tag", "files", "foo", "bar", "quux"])
            .pipe(biff(["time", "fmt", "-f", "%Y-%m-%d %H:%M:%S %:z"]))
            .pipe(biff(["untag", "-f", "{tag}:{data}"])),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2025-03-15 00:00:00 -04:00:foo
    2025-03-15 00:00:00 +11:00:bar
    2025-03-15 00:00:00 +00:00:quux

    ----- stderr -----
    ",
    );
}

#[test]
fn format_multi_tag() {
    let tmp = TempDir::new();
    tmp.create("foo", "2025-03-15T00-04 Springsteen 2024-10-01T00-04");
    tmp.create("bar", "2025-03-15T00+11 Zevon 2024-10-01T00+11");
    tmp.create("quux", "2025-03-15T00Z Helm 2024-10-01T00Z");

    assert_cmd_snapshot!(
        tmp.biff(["tag", "files", "--all", "foo", "bar", "quux"])
            .pipe(biff(["untag", "-f", "{tag}:{data}"])),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2025-03-15T00-04:foo
    2024-10-01T00-04:foo
    2025-03-15T00+11:bar
    2024-10-01T00+11:bar
    2025-03-15T00Z:quux
    2024-10-01T00Z:quux

    ----- stderr -----
    ",
    );
}

#[test]
fn format_substitute_multi_tag() {
    let tmp = TempDir::new();
    tmp.create("foo", "2025-03-15T00-04 Springsteen 2024-10-01T00-04");
    tmp.create("bar", "2025-03-15T00+11 Zevon 2024-10-01T00+11");
    tmp.create("quux", "2025-03-15T00Z Helm 2024-10-01T00Z");

    assert_cmd_snapshot!(
        tmp.biff(["tag", "files", "--all", "foo", "bar", "quux"])
            .pipe(biff(["time", "fmt", "-f", "%Y-%m-%d %H:%M:%S %:z"]))
            .pipe(biff(["untag", "-f", "{tag}:{data}"])),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2025-03-15 00:00:00 -04:00:foo
    2024-10-01 00:00:00 -04:00:foo
    2025-03-15 00:00:00 +11:00:bar
    2024-10-01 00:00:00 +11:00:bar
    2025-03-15 00:00:00 +00:00:quux
    2024-10-01 00:00:00 +00:00:quux

    ----- stderr -----
    ",
    );
}

#[test]
fn format_escaping() {
    let tmp = TempDir::new();
    tmp.create("foo", "2025-03-15T00-04 Springsteen");
    tmp.create("bar", "2025-03-15T00+11 Zevon");
    tmp.create("quux", "2025-03-15T00Z Helm");

    assert_cmd_snapshot!(
        tmp.biff(["tag", "files", "foo", "bar", "quux"])
            .pipe(biff(["untag", "-f", r"\{tag\}:\{data\}"])),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    {tag}:{data}
    {tag}:{data}
    {tag}:{data}

    ----- stderr -----
    ",
    );
}

#[test]
fn format_invalid_directive() {
    let tmp = TempDir::new();
    tmp.create("foo", "2025-03-15T00-04 Springsteen");
    tmp.create("bar", "2025-03-15T00+11 Zevon");
    tmp.create("quux", "2025-03-15T00Z Helm");

    assert_cmd_snapshot!(
        tmp.biff(["tag", "files", "foo", "bar", "quux"])
            .pipe(biff(["untag", "-f", "{tagg}:{data}"])),
        @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    -f/--format: unrecognized format directive `{tagg}`, allowed directives are `{tag}` and `{data}`
    ",
    );

    assert_cmd_snapshot!(
        tmp.biff(["tag", "files", "foo", "bar", "quux"])
            .pipe(biff(["untag", "-f", "{tag:{data}"])),
        @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    -f/--format: unrecognized format directive `{tag:{data}`, allowed directives are `{tag}` and `{data}`
    ",
    );

    assert_cmd_snapshot!(
        tmp.biff(["tag", "files", "foo", "bar", "quux"])
            .pipe(biff(["untag", "-f", "{tag"])),
        @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    -f/--format: found unclosed brace, which might be an invalid format directive (to write a brace literally, escape it with a backslash)
    ",
    );

    assert_cmd_snapshot!(
        tmp.biff(["tag", "files", "foo", "bar", "quux"])
            .pipe(biff(["untag", "-f", r"abc\"])),
        @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    -f/--format: found dangling backslash (to write a backslash literally, escape it with a backslash)
    ",
    );

    assert_cmd_snapshot!(
        tmp.biff(["tag", "files", "foo", "bar", "quux"])
            .pipe(biff(["untag", "-f", r"{abc\"])),
        @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    -f/--format: found dangling backslash (to write a backslash literally, escape it with a backslash)
    ",
    );
}
