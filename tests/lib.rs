use std::{ffi::OsStr, path::Path, sync::LazyLock};

use jiff::{Zoned, civil};

mod command;
mod span;
mod tag;
mod time;
mod tz;
mod untag;

static NOW: LazyLock<Zoned> = LazyLock::new(|| {
    civil::date(2024, 7, 20)
        .at(16, 30, 55, 0)
        .in_tz("America/New_York")
        .unwrap()
});

/// A lightweight abstraction for managing temporary directories and the files
/// within it.
#[derive(Debug)]
struct TempDir(tempfile::TempDir);

impl TempDir {
    /// Create a new temporary directory.
    fn new() -> TempDir {
        TempDir(tempfile::tempdir().unwrap())
    }

    /// Create a new `biff` command whose CWD is this directory.
    fn biff_bare(&self) -> crate::command::Command {
        biff_bare().current_dir(self.0.path())
    }

    /// Create a new `biff` command whose CWD is this directory and the
    /// given arguments appended to it.
    fn biff<T: AsRef<OsStr>>(
        &self,
        args: impl IntoIterator<Item = T>,
    ) -> crate::command::Command {
        self.biff_bare().args(args)
    }

    /// Create a new file in this temporary directory with the given relative
    /// path and contents.
    fn create(
        &self,
        relative_path: impl AsRef<Path>,
        contents: impl AsRef<[u8]>,
    ) {
        let path = self.0.path().join(relative_path.as_ref());
        std::fs::write(&path, contents).unwrap();
    }
}

/// Return a command for the `biff` binary and no argument.
fn biff_bare() -> crate::command::Command {
    crate::command::bin("biff")
        .env("TZ", "America/New_York")
        .env("BIFF_NOW", NOW.to_string())
        // So that when tests are run with `--features locale`,
        // we still get consistent behavior as if Biff were
        // compiled without locale support.
        .env("BIFF_LOCALE", "und")
}

/// Return a command for the `biff` binary with the given arguments appended
/// to it.
fn biff<T: AsRef<OsStr>>(
    args: impl IntoIterator<Item = T>,
) -> crate::command::Command {
    biff_bare().args(args)
}

/// Test that calling `biff` with no arguments prints the current time.
#[test]
fn no_args() {
    crate::command::assert_cmd_snapshot!(
        biff_bare(),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    2024 M07 20, Sat 16:30:55

    ----- stderr -----
    ",
    );
}

/// Test that calling `biff` when compiled with `locale` and when `BIFF_LOCALE`
/// is set does something sensible.
#[cfg(feature = "locale")]
#[test]
fn no_args_locale() {
    crate::command::assert_cmd_snapshot!(
        biff_bare().env("BIFF_LOCALE", "en-US"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Sat, Jul 20, 2024, 4:30:55 PM EDT

    ----- stderr -----
    ",
    );

    crate::command::assert_cmd_snapshot!(
        biff_bare().env("BIFF_LOCALE", "en-GB"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Sat, 20 Jul 2024, 16:30:55 GMT-4

    ----- stderr -----
    ",
    );

    crate::command::assert_cmd_snapshot!(
        biff_bare().env("TZ", "Europe/London").env("BIFF_LOCALE", "en-GB"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Sat, 20 Jul 2024, 21:30:55 BST

    ----- stderr -----
    ",
    );

    crate::command::assert_cmd_snapshot!(
        biff_bare().env("TZ", "Europe/Paris").env("BIFF_LOCALE", "fr-LA"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    sam. 20 juil. 2024, 22:30:55 UTC+2

    ----- stderr -----
    ",
    );

    crate::command::assert_cmd_snapshot!(
        biff_bare()
            .env("TZ", "US/Eastern")
            .env("BIFF_LOCALE", "en-US-u-ca-buddhist"),
        @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Sat, Jul 20, 2567 BE, 4:30:55 PM EDT

    ----- stderr -----
    ",
    );
}
