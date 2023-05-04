mod cmd;
mod commands;
mod env;
mod tui;
mod util;
mod versions;

use clap::{Parser, Subcommand};
use commands::*;
use std::ops::Deref;

const LONG_HELP: &str = r#"
goup helps to install, update and switch between Go SDK versions in an as easy as possible way.

Simply use `goup env -p && source ~/profile` to add the required environment variables. 
After that, download the latest version of Go using `goup use`."#;

#[derive(Parser)]
#[command(author, version, about, long_about = LONG_HELP)]
struct App {
    #[command(subcommand)]
    command: Commands,
}

register_commands!(Current, Env, Use, Lsr, Ls, Drop, Clean);

fn main() {
    let app = App::parse();

    if let Err(err) = app.command.run() {
        error!("{err}");
    }
}
