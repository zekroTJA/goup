use crate::env::print_vars;

use super::Command;
use clap::Args;

/// Print env variables required to use goup.
#[derive(Args)]
pub struct Env {}

impl Command for Env {
    fn run(&self) -> anyhow::Result<()> {
        print_vars()
    }
}
