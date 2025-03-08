use jiff::fmt::strtime::Config;

pub use self::inner::*;

pub type StrtimeConfig = Config<StrtimeLocaleFormatter>;

pub fn jiff_strtime_config() -> anyhow::Result<StrtimeConfig> {
    Ok(Config::new().custom(crate::LOCALE.to_formatter()?))
}

#[cfg(feature = "locale")]
#[path = "enabled.rs"]
mod inner;

#[cfg(not(feature = "locale"))]
#[path = "disabled.rs"]
mod inner;
