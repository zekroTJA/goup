use super::Command;
use crate::{
    env::{self, get_env_vars},
    success, warning,
};
use clap::Args;
use console::style;

#[cfg(target_family = "unix")]
const PROFILE_MARKER: &str = "# goup:envvars";

#[cfg(target_family = "unix")]
const LONG_ABOUT: &str = "\
This command prints all necessary environment variables and values required \
to use goup.

Using `goup env -p` appends the variables to your `.profile` (or `.zshenv`, \
depending on your current shell) file in your $HOME directory. After that, \
you can apply the changes to your current terminal session using \
`eval \"$(goup env)\"`.";

/// Print env variables required to use goup.
#[derive(Args)]
#[command(long_about = LONG_ABOUT)]
pub struct Env {
    /// Apply the environment variables to your .profile
    #[cfg(target_family = "unix")]
    #[arg(short, long, default_value_t = false)]
    profile: bool,
}

impl Command for Env {
    fn run(&self) -> anyhow::Result<()> {
        #[cfg(target_family = "unix")]
        if self.profile {
            let profile_content = env::read_profile()?;
            if profile_content.contains(PROFILE_MARKER) {
                warning!(
                    "You already have applied goup's env variables to your profile.\n\
                    If you want to update them, please remove the entries below the \"{}\" header \
                    as well as the header itself manually.",
                    PROFILE_MARKER
                );
                return Ok(());
            }

            env::append_to_profile(&format!(
                "\n{}\n{}\n\n",
                PROFILE_MARKER, r#"eval "$(goup env)""#
            ))?;
            success!(
                "Env vars have been appended to your profile. To apply them to the current \
                terminal session, use the following command:\n{}",
                style("$ eval \"$(goup env)\"").green().bright().italic()
            );
            return Ok(());
        }

        let vars = get_env_vars()?;
        println!("{}", vars);

        Ok(())
    }
}
