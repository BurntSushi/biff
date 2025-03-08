#[derive(Clone, Debug)]
pub struct Locale(());

impl Locale {
    pub fn unknown() -> Locale {
        Locale(())
    }

    pub fn to_formatter(&self) -> anyhow::Result<StrtimeLocaleFormatter> {
        Ok(StrtimeLocaleFormatter(()))
    }
}

impl std::str::FromStr for Locale {
    type Err = anyhow::Error;

    fn from_str(_: &str) -> anyhow::Result<Locale> {
        anyhow::bail!(
            "Biff must be compiled with the `locale` feature to \
             format datetimes in a particular locale",
        )
    }
}

impl std::fmt::Display for Locale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "und")
    }
}

#[derive(Debug)]
pub struct StrtimeLocaleFormatter(());

// This will just use all of Jiff's defaults.
impl jiff::fmt::strtime::Custom for StrtimeLocaleFormatter {}
