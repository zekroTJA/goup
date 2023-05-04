use super::Command;
use crate::{
    env::{self, get_env_vars},
    success, warning,
};
use clap::Args;
use console::style;

#[cfg(target_family = "unix")]
const PROFILE_MARKER: &str = "# goup:envvars";

/// Print env variables required to use goup.
#[derive(Args)]
pub struct Env {
    /// Apply the environment variables to your .profile
    #[cfg(target_family = "unix")]
    #[arg(short, long, default_value_t = false)]
    profile: bool,
}

impl Command for Env {
    fn run(&self) -> anyhow::Result<()> {
        let vars = get_env_vars()?;

        #[cfg(target_family = "unix")]
        if self.profile {
            let profile_content = env::read_profile()?;
            if profile_content.contains(PROFILE_MARKER) {
                warning!(
                    "You already have applied goup's env variables to your .profile.\n\
                    If you want to update them, please remove the entries below the \"{}\" header \
                    as well as the header itself manually.",
                    PROFILE_MARKER
                );
                return Ok(());
            }

            env::append_to_profile(&format!("\n{}\n{}\n\n", PROFILE_MARKER, vars))?;
            success!(
                "Env vars have been appended to your .profile file. You can now do\n{}",
                style("$ source ~/.profile").green().bright().italic()
            );
            return Ok(());
        }

        println!("{}", vars);

        Ok(())
    }
}
