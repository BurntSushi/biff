use crate::{biff, command::assert_cmd_snapshot};

fn seq() -> crate::command::Command {
    biff(["tz", "seq"])
}

fn next() -> crate::command::Command {
    biff(["tz", "next"])
}

fn prev() -> crate::command::Command {
    biff(["tz", "prev"])
}

#[test]
fn seq_future() {
    assert_cmd_snapshot!(
        seq().args(["America/New_York", "-c10"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-11-03T01:00:00-05:00[America/New_York]
    2025-03-09T03:00:00-04:00[America/New_York]
    2025-11-02T01:00:00-05:00[America/New_York]
    2026-03-08T03:00:00-04:00[America/New_York]
    2026-11-01T01:00:00-05:00[America/New_York]
    2027-03-14T03:00:00-04:00[America/New_York]
    2027-11-07T01:00:00-05:00[America/New_York]
    2028-03-12T03:00:00-04:00[America/New_York]
    2028-11-05T01:00:00-05:00[America/New_York]
    2029-03-11T03:00:00-04:00[America/New_York]

    ----- stderr -----
    ",
    );
}

#[test]
fn seq_past() {
    assert_cmd_snapshot!(
        seq().args(["America/New_York", "-c10", "--past"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-03-10T03:00:00-04:00[America/New_York]
    2023-11-05T01:00:00-05:00[America/New_York]
    2023-03-12T03:00:00-04:00[America/New_York]
    2022-11-06T01:00:00-05:00[America/New_York]
    2022-03-13T03:00:00-04:00[America/New_York]
    2021-11-07T01:00:00-05:00[America/New_York]
    2021-03-14T03:00:00-04:00[America/New_York]
    2020-11-01T01:00:00-05:00[America/New_York]
    2020-03-08T03:00:00-04:00[America/New_York]
    2019-11-03T01:00:00-05:00[America/New_York]

    ----- stderr -----
    ",
    );
}

#[test]
fn seq_distant_past() {
    assert_cmd_snapshot!(
        seq().args(["America/New_York", "-c10", "--past", "-r1920-01-01"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    1919-10-26T01:00:00-05:00[America/New_York]
    1919-03-30T03:00:00-04:00[America/New_York]
    1918-10-27T01:00:00-05:00[America/New_York]
    1918-03-31T03:00:00-04:00[America/New_York]
    1883-11-18T12:00:00-05:00[America/New_York]

    ----- stderr -----
    ",
    );
}

#[test]
fn next_basic() {
    assert_cmd_snapshot!(
        next().args(["America/New_York", "now"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-11-03T01:00:00-05:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        next().args(["America/New_York", "now", "-c2"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2025-03-09T03:00:00-04:00[America/New_York]

    ----- stderr -----
    ",
    );
}

#[test]
fn next_tags() {
    let stdin = "\
foo 2025-01-01T00:00:00-05 bar
quux 2025-07-01T00:00:00-04 baz
";

    assert_cmd_snapshot!(
        biff(["tag", "lines"]).stdin(stdin).pipe(
            next().args(["America/New_York"]),
        ),
        @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {"tags":[{"value":"2025-03-09T03:00:00-04:00[America/New_York]","range":[4,26]}],"data":{"text":"foo 2025-01-01T00:00:00-05 bar\n"}}
    {"tags":[{"value":"2025-11-02T01:00:00-05:00[America/New_York]","range":[5,27]}],"data":{"text":"quux 2025-07-01T00:00:00-04 baz\n"}}

    ----- stderr -----
    "#,
    );

    assert_cmd_snapshot!(
        biff(["tag", "lines"]).stdin(stdin).pipe(
            next().args(["America/New_York", "-c2"]),
        ),
        @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {"tags":[{"value":"2025-11-02T01:00:00-05:00[America/New_York]","range":[4,26]}],"data":{"text":"foo 2025-01-01T00:00:00-05 bar\n"}}
    {"tags":[{"value":"2026-03-08T03:00:00-04:00[America/New_York]","range":[5,27]}],"data":{"text":"quux 2025-07-01T00:00:00-04 baz\n"}}

    ----- stderr -----
    "#,
    );
}

#[test]
fn next_weird() {
    assert_cmd_snapshot!(
        next().args(["America/New_York", "1800-01-01"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    1883-11-18T12:00:00-05:00[America/New_York]

    ----- stderr -----
    ",
    );
}

#[test]
fn next_inclusive() {
    assert_cmd_snapshot!(
        next().args([
            "America/New_York",
            "2025-03-09T03:00-04[America/New_York]",
        ]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2025-11-02T01:00:00-05:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        next().args([
            "America/New_York",
            "-i",
            "2025-03-09T03:00-04[America/New_York]",
        ]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2025-03-09T03:00:00-04:00[America/New_York]

    ----- stderr -----
    ",
    );
}

#[test]
fn next_no_dst() {
    assert_cmd_snapshot!(
        next().args(["Asia/Kolkata", "now"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ",
    );
}

#[test]
fn next_tags_no_dst() {
    let stdin = "\
foo 2025-01-01T00:00:00-05 bar
quux 2025-07-01T00:00:00-04 baz
";

    assert_cmd_snapshot!(
        biff(["tag", "lines"]).stdin(stdin).pipe(
            next().args(["Asia/Kolkata"]),
        ),
        @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {"data":{"text":"foo 2025-01-01T00:00:00-05 bar\n"}}
    {"data":{"text":"quux 2025-07-01T00:00:00-04 baz\n"}}

    ----- stderr -----
    "#,
    );
}

#[test]
fn next_multi_tags_no_dst() {
    let stdin = "\
foo 2025-01-01T00:00:00-05 bar 9999-11-31T00Z
9999-11-31T00Z quux 2025-07-01T00:00:00-04 baz
";

    assert_cmd_snapshot!(
        biff(["tag", "lines", "--all"]).stdin(stdin).pipe(
            next().args(["America/New_York"]),
        ),
        @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {"tags":[{"value":"2025-03-09T03:00:00-04:00[America/New_York]","range":[4,26]}],"data":{"text":"foo 2025-01-01T00:00:00-05 bar 9999-11-31T00Z\n"}}
    {"tags":[{"value":"2025-11-02T01:00:00-05:00[America/New_York]","range":[20,42]}],"data":{"text":"9999-11-31T00Z quux 2025-07-01T00:00:00-04 baz\n"}}

    ----- stderr -----
    "#,
    );
}

#[test]
fn next_fixed() {
    assert_cmd_snapshot!(
        next().args(["+05:00", "now"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ",
    );
}

#[test]
fn next_error_n0() {
    assert_cmd_snapshot!(
        next().args(["America/New_York", "now", "-c0"]),
        @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    -c/--count must be greater than zero
    ",
    );
}

#[test]
fn prev_basic() {
    assert_cmd_snapshot!(
        prev().args(["America/New_York", "now"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-03-10T03:00:00-04:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        prev().args(["America/New_York", "now", "-c2"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2023-11-05T01:00:00-05:00[America/New_York]

    ----- stderr -----
    ",
    );
}

#[test]
fn prev_tags() {
    let stdin = "\
quux 2025-07-01T00:00:00-04 baz
foo 2025-01-01T00:00:00-05 bar
";

    assert_cmd_snapshot!(
        biff(["tag", "lines"]).stdin(stdin).pipe(
            prev().args(["America/New_York"]),
        ),
        @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {"tags":[{"value":"2025-03-09T03:00:00-04:00[America/New_York]","range":[5,27]}],"data":{"text":"quux 2025-07-01T00:00:00-04 baz\n"}}
    {"tags":[{"value":"2024-11-03T01:00:00-05:00[America/New_York]","range":[4,26]}],"data":{"text":"foo 2025-01-01T00:00:00-05 bar\n"}}

    ----- stderr -----
    "#,
    );

    assert_cmd_snapshot!(
        biff(["tag", "lines"]).stdin(stdin).pipe(
            prev().args(["America/New_York", "-c2"]),
        ),
        @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {"tags":[{"value":"2024-11-03T01:00:00-05:00[America/New_York]","range":[5,27]}],"data":{"text":"quux 2025-07-01T00:00:00-04 baz\n"}}
    {"tags":[{"value":"2024-03-10T03:00:00-04:00[America/New_York]","range":[4,26]}],"data":{"text":"foo 2025-01-01T00:00:00-05 bar\n"}}

    ----- stderr -----
    "#,
    );
}

#[test]
fn prev_weird() {
    assert_cmd_snapshot!(
        prev().args(["America/New_York", "1900-01-01"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    1883-11-18T12:00:00-05:00[America/New_York]

    ----- stderr -----
    ",
    );
}

#[test]
fn prev_inclusive() {
    assert_cmd_snapshot!(
        prev().args([
            "America/New_York",
            "2025-03-09T03:00-04[America/New_York]",
        ]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024-11-03T01:00:00-05:00[America/New_York]

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        prev().args([
            "America/New_York",
            "-i",
            "2025-03-09T03:00-04[America/New_York]",
        ]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2025-03-09T03:00:00-04:00[America/New_York]

    ----- stderr -----
    ",
    );
}

#[test]
fn prev_no_dst() {
    assert_cmd_snapshot!(
        prev().args(["Asia/Kolkata", "now"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    1945-10-14T23:00:00+05:30[Asia/Kolkata]

    ----- stderr -----
    ",
    );
}

#[test]
fn prev_tags_no_dst() {
    let stdin = "\
quux 2025-07-01T00:00:00-04 baz
foo 2025-01-01T00:00:00-05 bar
";

    assert_cmd_snapshot!(
        biff(["tag", "lines"]).stdin(stdin).pipe(
            prev().args(["Asia/Kolkata"]),
        ),
        @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {"tags":[{"value":"1945-10-14T23:00:00+05:30[Asia/Kolkata]","range":[5,27]}],"data":{"text":"quux 2025-07-01T00:00:00-04 baz\n"}}
    {"tags":[{"value":"1945-10-14T23:00:00+05:30[Asia/Kolkata]","range":[4,26]}],"data":{"text":"foo 2025-01-01T00:00:00-05 bar\n"}}

    ----- stderr -----
    "#,
    );
}

#[test]
fn prev_multi_tags_no_dst() {
    let stdin = "\
1800-01-01T00Z quux 2025-07-01T00:00:00-04 baz
foo 2025-01-01T00:00:00-05 bar 1800-01-01T00Z
";

    assert_cmd_snapshot!(
        biff(["tag", "lines", "--all"]).stdin(stdin).pipe(
            prev().args(["America/New_York"]),
        ),
        @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {"tags":[{"value":"2025-03-09T03:00:00-04:00[America/New_York]","range":[20,42]}],"data":{"text":"1800-01-01T00Z quux 2025-07-01T00:00:00-04 baz\n"}}
    {"tags":[{"value":"2024-11-03T01:00:00-05:00[America/New_York]","range":[4,26]}],"data":{"text":"foo 2025-01-01T00:00:00-05 bar 1800-01-01T00Z\n"}}

    ----- stderr -----
    "#,
    );
}

#[test]
fn prev_fixed() {
    assert_cmd_snapshot!(
        prev().args(["+05:00", "now"]),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ",
    );
}

#[test]
fn prev_error_n0() {
    assert_cmd_snapshot!(
        prev().args(["America/New_York", "now", "-c0"]),
        @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    -c/--count must be greater than zero
    ",
    );
}
