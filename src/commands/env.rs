use super::Command;
use crate::{
    env::{self, get_env_vars},
    success, warning,
};
use anyhow::Result;
use clap::Args;
use console::style;

const PROFILE_MARKER: &str = "# goup:envvars";

#[cfg(target_family = "unix")]
const LONG_ABOUT: &str = "\
This command prints all necessary environment variables and values required \
to use goup.

Using `goup env -p` appends the variables to your `.profile` (or `.zshenv`, \
depending on your current shell) file in your $HOME directory. After that, \
you can apply the changes to your current terminal session using \
`eval \"$(goup env)\"`.";

#[cfg(target_family = "windows")]
const LONG_ABOUT: &str = "\
This command prints all necessary environment variables and values required \
to use goup.

Using `goup env -a` appends the variables to your PowerShell profile script \
in your %HOME%\\Documents\\WindowsPowerShell directory. After that, \
you can apply the changes to your current terminal session using \
`goup env | Out-String | Invoke-Expression`.";

/// Print env variables required to use goup.
#[derive(Args)]
#[command(long_about = LONG_ABOUT)]
pub struct Env {
    /// Apply the environment variables to your profile
    #[arg(
        short,
        short_alias = 'p',
        long,
        alias = "profile",
        default_value_t = false
    )]
    apply: bool,
}

impl Command for Env {
    fn run(&self) -> anyhow::Result<()> {
        if self.apply {
            return apply_profile();
        }

        let vars = get_env_vars()?;
        println!("{}", vars);

        Ok(())
    }
}

#[cfg(target_family = "unix")]
fn apply_profile() -> Result<()> {
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

    Ok(())
}

#[cfg(target_family = "windows")]
fn apply_profile() -> Result<()> {
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
        PROFILE_MARKER, r#"goup env | Out-String | Invoke-Expression"#
    ))?;

    success!(
        "Env vars have been appended to your profile. To apply them to the current \
                terminal session, use the following command:\n{}\n\n{}",
        style("> goup env | Out-String | Invoke-Expression")
            .green()
            .bright()
            .italic(),
        style(
            "You might need to enable script execution in PowerShell to load the \
                profile automatically. Please go to this page for more information:\n\
                https:/go.microsoft.com/fwlink/?LinkID=135170"
        )
        .green()
        .italic()
    );

    Ok(())
}
