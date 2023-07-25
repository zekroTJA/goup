mod cmd;
mod commands;
mod env;
mod shell;
mod tui;
mod util;
mod versions;

use clap::{Parser, Subcommand};
use commands::*;
use shell::set_shell;
use std::ops::Deref;
use whattheshell::Shell;

#[cfg(unix)]
const LONG_ABOUT: &str = "\
goup helps to install, update and switch between Go SDK versions in an as easy as possible way.

Simply use `goup env -a && eval \"$(goup env)\"` to add the required environment variables. \
After that, download the latest version of Go using `goup use`.";

#[cfg(windows)]
const LONG_ABOUT: &str = "\
goup helps to install, update and switch between Go SDK versions in an as easy as possible way.

Simply use `goup env -a` to add the required environment variables and execute \
`goup env | Out-String | Invoke-Expression` after, to apply the variables to your \
current terminal session. After that, download the latest version of Go using `goup use`.";

#[derive(Parser)]
#[command(author, version, about, long_about = LONG_ABOUT)]
struct App {
    #[command(subcommand)]
    command: Commands,
}

register_commands!(Check, Clean, Current, Drop, Env, Ls, Lsr, Use);

fn main() {
    let app = App::parse();

    let shell = match Shell::infer() {
        Err(err) => {
            error!("failed inferring shell: {err}");
            return;
        }
        Ok(shell) => shell,
    };

    dbg!(&shell);
    set_shell(shell);

    if let Err(err) = app.command.run() {
        error!("{err}");
    }
}
