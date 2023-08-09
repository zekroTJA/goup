use super::Command;
use crate::{
    env::{self, get_env_vars},
    shell::{self, ShellEnv},
    success, warning,
};
use anyhow::Result;
use clap::Args;
use console::style;
use whattheshell::Shell;

const PROFILE_MARKER: &str = "# goup:envvars";

fn get_long_about() -> String {
    let shell = shell::get_shell();

    format!(
        "This command prints all necessary environment variables and values required \
        to use goup. \
        \n\n\
        Using `goup env -p` appends the variables to your profile file ({}). \
        After that, you can apply the changes to your current terminal session using \
        `{}`.",
        shell
            .get_profile_dir()
            .and_then(|p| shell.path_to_string(p))
            .expect("failed getting profile directory"),
        shell
            .get_apply_env_command()
            .expect("failed getting env apply command")
    )
}

/// Print env variables required to use goup.
#[derive(Args)]
#[command(long_about = get_long_about())]
pub struct Env {
    /// Apply the environment variables to your profile
    #[arg(short, short_alias = 'p', long, alias = "profile")]
    apply: bool,
}

impl Command for Env {
    fn run(&self) -> anyhow::Result<()> {
        let shell = shell::get_shell();
        if self.apply {
            return apply_profile(&shell);
        }

        let vars = get_env_vars(&shell)?;
        println!("{}", vars);

        Ok(())
    }
}

fn apply_profile(shell: &Shell) -> Result<()> {
    let profile_content = env::read_profile(shell)?;
    if profile_content.contains(PROFILE_MARKER) {
        warning!(
            "You already have applied goup's env variables to your profile.\n\
                    If you want to update them, please remove the entries below the \"{}\" header \
                    as well as the header itself manually.",
            PROFILE_MARKER
        );
        return Ok(());
    }

    let apply_env_command = shell.get_apply_env_command()?;

    env::append_to_profile(
        shell,
        &format!("\n{}\n{}\n\n", PROFILE_MARKER, apply_env_command),
    )?;

    success!(
        "Env vars have been appended to your profile. To apply them to the current \
                terminal session, use the following command:\n{}",
        style(apply_env_command).green().bright().italic(),
    );

    if matches!(shell, Shell::PowerShell | Shell::Cmd) {
        success!(
            "\n{}\n{}",
            style(
                "You might need to enable script execution in PowerShell to load the \
                profile automatically. Please go to this page for more information:"
            )
            .green()
            .italic(),
            style("https:/go.microsoft.com/fwlink/?LinkID=135170")
                .green()
                .italic()
                .underlined()
        );
    }

    Ok(())
}
