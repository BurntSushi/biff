use bstr::ByteSlice;

use crate::biff;

#[test]
fn basic() {
    let snap = biff(["tz", "list"]).snapshot();
    let stdout = snap.stdout();
    // We don't want to be too pedantic about what we get here. But we
    // can assert some basic stuff that should generally always be true,
    // or else something very odd has happened.
    assert!(stdout.lines().count() >= 10);
    assert!(stdout.contains_str("America/New_York"));
    // We should be printing POSIX or leap second time zone bullshit.
    assert!(!stdout.contains_str("posix/America/New_York"));
    assert!(!stdout.contains_str("right/America/New_York"));
}
