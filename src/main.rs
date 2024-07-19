mod cmd;
mod commands;
mod env;
mod progress;
mod shell;
mod tui;
mod util;
mod versions;

use crate::shell::ShellEnv;
use clap::{Parser, Subcommand};
use commands::*;
use std::ops::Deref;

fn get_long_about() -> String {
    let shell = shell::get_shell();

    format!(
        "goup helps to install, update and switch between Go SDK versions in an as easy as possible way.\
        \n\n\
        Simply use `goup env -a` to add the required environment variables and execute \
        `{}` after, to apply the variables to your \
        current terminal session. After that, download the latest version of Go using `goup use`.",
        shell.get_apply_env_command().expect("failed getting env apply command")
    )
}

#[derive(Parser)]
#[command(author, version, about, long_about = get_long_about())]
struct App {
    #[command(subcommand)]
    command: Commands,
}

register_commands! {
    Check
    Clean
    Current
    Drop
    Env
    Ls
    Lsr
    Use
}

fn main() {
    let app = App::parse();

    if let Err(err) = app.command.run() {
        error!("{err}");
    }
}
