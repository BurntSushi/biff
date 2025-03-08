use std::{
    fmt::Display,
    io::IsTerminal,
    sync::{Arc, LazyLock},
};

use anstyle::{AnsiColor, Style};

/// A theme that indicates how output should be styled.
///
/// Each theme comes with a set of styles for particular output components of
/// Biff. The getter methods on a theme provide these styles on a per-component
/// basis.
///
/// The styles returned may be completely unstyled, for example, when no theme
/// is set.
#[derive(Clone, Debug)]
pub struct Theme {
    inner: Option<Arc<ThemeInner>>,
}

impl Theme {
    /// Returns a theme for stdout.
    pub fn stdout() -> &'static Theme {
        static THEME: LazyLock<Theme> = LazyLock::new(|| {
            if !tty_stdout() || !can_use_colors() {
                return Theme::none();
            }
            let inner = Some(Arc::new(ThemeInner::default()));
            Theme { inner }
        });
        &*THEME
    }

    /// Returns a theme for stderr.
    pub fn stderr() -> &'static Theme {
        static THEME: LazyLock<Theme> = LazyLock::new(|| {
            if !tty_stderr() || !can_use_colors() {
                return Theme::none();
            }
            let inner = Some(Arc::new(ThemeInner::default()));
            Theme { inner }
        });
        &*THEME
    }

    /// Returns a theme that never does any styling.
    const fn none() -> Theme {
        Theme { inner: None }
    }

    pub fn highlight<T: Display>(&self, data: T) -> Styled<'_, T> {
        let style = self.inner().map(|inner| &inner.highlight);
        Styled { data, style }
    }

    /// Returns true if this is theme is known to never have any styling.
    ///
    /// This is useful for callers that would otherwise need to do potentially
    /// expensive work to support colors.
    pub fn is_none(&self) -> bool {
        self.inner.is_none()
    }

    fn inner(&self) -> Option<&ThemeInner> {
        self.inner.as_deref()
    }
}

#[derive(Debug)]
struct ThemeInner {
    highlight: Style,
}

impl Default for ThemeInner {
    fn default() -> ThemeInner {
        ThemeInner {
            highlight: Style::new()
                .bold()
                .fg_color(Some(AnsiColor::Magenta.into())),
        }
    }
}

/// A possibly unstyled piece of renderable data.
///
/// When this is unstyled, its `Display` impl does no styling and just
/// renders the underlying data.
#[derive(Clone, Debug)]
pub struct Styled<'s, T> {
    data: T,
    style: Option<&'s Style>,
}

impl<'s, T: Display> Display for Styled<'s, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let Some(style) = self.style else {
            return self.data.fmt(f);
        };
        write!(f, "{style}")?;
        write!(f, "{}", self.data)?;
        write!(f, "{style:#}")
    }
}

/// Returns true if there's a tty attached to stdout.
fn tty_stdout() -> bool {
    static YES: LazyLock<bool> =
        LazyLock::new(|| std::io::stdout().is_terminal());
    *YES
}

/// Returns true if there's a tty attached to stderr.
fn tty_stderr() -> bool {
    static YES: LazyLock<bool> =
        LazyLock::new(|| std::io::stderr().is_terminal());
    *YES
}

/// Whether colors have been globally disabled or not.
fn can_use_colors() -> bool {
    static YES: LazyLock<bool> = LazyLock::new(|| {
        if let Some(v) = std::env::var_os("NO_COLOR") {
            if !v.is_empty() {
                return false;
            }
        }
        if let Some(v) = std::env::var_os("TERM") {
            if v == std::ffi::OsStr::new("dumb") {
                return false;
            }
        }
        true
    });
    *YES
}
