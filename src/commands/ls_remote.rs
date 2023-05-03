use super::Command;
use crate::versions::get_versions;
use clap::{Args, ValueEnum};

#[derive(ValueEnum, Clone)]
pub enum FilterOptions {
    Stable,
    Unstable,
    All,
}

/// List all upstream versions.
#[derive(Args)]
pub struct LsRemote {
    /// Filter versions by release type.
    #[arg(value_enum, short, long, default_value_t = FilterOptions::All)]
    pub filter: FilterOptions,
}

impl Command for LsRemote {
    fn run(&self) -> anyhow::Result<()> {
        let tags = get_versions()?;
        let mut tags: Box<dyn Iterator<Item = _>> = Box::new(tags.iter());

        match self.filter {
            FilterOptions::All => {}
            FilterOptions::Stable => tags = Box::new(tags.filter(|v| v.is_stable())),
            FilterOptions::Unstable => tags = Box::new(tags.filter(|v| !v.is_stable())),
        }

        let tags: Vec<_> = tags.map(|v| v.to_string()).collect();
        println!("{}", tags.join("\n"));

        Ok(())
    }
}
