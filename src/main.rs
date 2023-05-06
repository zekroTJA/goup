mod cmd;
mod commands;
mod env;
mod tui;
mod util;
mod versions;

use clap::{Parser, Subcommand};
use commands::*;
use std::ops::Deref;

#[cfg(target_family = "unix")]
const LONG_ABOUT: &str = "\
goup helps to install, update and switch between Go SDK versions in an as easy as possible way.

Simply use `goup env -p && eval \"$(goup env)\"` to add the required environment variables. \
After that, download the latest version of Go using `goup use`.";

#[derive(Parser)]
#[command(author, version, about, long_about = LONG_ABOUT)]
struct App {
    #[command(subcommand)]
    command: Commands,
}

register_commands!(Check, Clean, Current, Drop, Env, Ls, Lsr, Use);

fn main() {
    let app = App::parse();

    if let Err(err) = app.command.run() {
        error!("{err}");
    }
}
