use crate::command::assert_cmd_snapshot;

static NANOS: &str = "999999999999999999ns";

fn balance() -> crate::command::Command {
    crate::biff(["span", "balance"])
}

#[test]
fn year() {
    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("years"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    31y 8mo 8d 1h 46m 39s 999ms 999µs 999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("year"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    31y 8mo 8d 1h 46m 39s 999ms 999µs 999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("yrs"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    31y 8mo 8d 1h 46m 39s 999ms 999µs 999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("yr"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    31y 8mo 8d 1h 46m 39s 999ms 999µs 999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("y"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    31y 8mo 8d 1h 46m 39s 999ms 999µs 999ns

    ----- stderr -----
    ",
    );
}

#[test]
fn month() {
    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("months"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    380mo 8d 1h 46m 39s 999ms 999µs 999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("month"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    380mo 8d 1h 46m 39s 999ms 999µs 999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("mos"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    380mo 8d 1h 46m 39s 999ms 999µs 999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("mo"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    380mo 8d 1h 46m 39s 999ms 999µs 999ns

    ----- stderr -----
    ",
    );
}

#[test]
fn week() {
    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("weeks"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    1653w 3d 1h 46m 39s 999ms 999µs 999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("week"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    1653w 3d 1h 46m 39s 999ms 999µs 999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("wks"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    1653w 3d 1h 46m 39s 999ms 999µs 999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("wk"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    1653w 3d 1h 46m 39s 999ms 999µs 999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("w"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    1653w 3d 1h 46m 39s 999ms 999µs 999ns

    ----- stderr -----
    ",
    );
}

#[test]
fn day() {
    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("days"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    11574d 1h 46m 39s 999ms 999µs 999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("day"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    11574d 1h 46m 39s 999ms 999µs 999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("d"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    11574d 1h 46m 39s 999ms 999µs 999ns

    ----- stderr -----
    ",
    );
}

#[test]
fn hour() {
    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("hours"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    277777h 46m 39s 999ms 999µs 999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("hour"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    277777h 46m 39s 999ms 999µs 999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("hrs"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    277777h 46m 39s 999ms 999µs 999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("hr"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    277777h 46m 39s 999ms 999µs 999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("h"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    277777h 46m 39s 999ms 999µs 999ns

    ----- stderr -----
    ",
    );
}

#[test]
fn minute() {
    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("minutes"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    16666666m 39s 999ms 999µs 999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("minute"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    16666666m 39s 999ms 999µs 999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("mins"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    16666666m 39s 999ms 999µs 999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("min"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    16666666m 39s 999ms 999µs 999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("m"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    16666666m 39s 999ms 999µs 999ns

    ----- stderr -----
    ",
    );
}

#[test]
fn second() {
    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("seconds"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    999999999s 999ms 999µs 999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("second"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    999999999s 999ms 999µs 999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("secs"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    999999999s 999ms 999µs 999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("sec"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    999999999s 999ms 999µs 999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("s"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    999999999s 999ms 999µs 999ns

    ----- stderr -----
    ",
    );
}

#[test]
fn millisecond() {
    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("milliseconds"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    999999999999ms 999µs 999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("millisecond"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    999999999999ms 999µs 999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("millis"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    999999999999ms 999µs 999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("milli"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    999999999999ms 999µs 999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("msecs"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    999999999999ms 999µs 999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("msec"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    999999999999ms 999µs 999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("ms"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    999999999999ms 999µs 999ns

    ----- stderr -----
    ",
    );
}

#[test]
fn microsecond() {
    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("microseconds"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    999999999999999µs 999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("microsecond"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    999999999999999µs 999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("micros"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    999999999999999µs 999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("micro"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    999999999999999µs 999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("usecs"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    999999999999999µs 999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("µsecs"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    999999999999999µs 999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("usec"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    999999999999999µs 999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("µsec"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    999999999999999µs 999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("us"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    999999999999999µs 999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("µs"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    999999999999999µs 999ns

    ----- stderr -----
    ",
    );
}

#[test]
fn nanosecond() {
    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("nanoseconds"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    999999999999999999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("nanosecond"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    999999999999999999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("nanos"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    999999999999999999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("nano"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    999999999999999999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("nsecs"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    999999999999999999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("nsec"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    999999999999999999ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg(NANOS).arg("-l").arg("ns"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    999999999999999999ns

    ----- stderr -----
    ",
    );
}

#[test]
fn balance_down() {
    assert_cmd_snapshot!(
        balance().arg("1 year").arg("-l").arg("nanos"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    31536000000000000ns

    ----- stderr -----
    ",
    );

    assert_cmd_snapshot!(
        balance().arg("1 year").arg("-l").arg("nanos").arg("-r2024-01-15"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    31622400000000000ns

    ----- stderr -----
    ",
    );
}
