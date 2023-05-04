mod cmd;
mod commands;
mod env;
mod tui;
mod util;
mod versions;

use clap::{Parser, Subcommand};
use commands::*;
use std::ops::Deref;

// TODO: add long help
const LONG_HELP: &str = r#"
TODO™"#;

#[derive(Parser)]
#[command(author, version, about, long_about = LONG_HELP)]
struct App {
    #[command(subcommand)]
    command: Commands,
}

register_commands!(Current, Env, Use, LsRemote, Ls, Drop, Clean);

fn main() {
    let app = App::parse();

    if let Err(err) = app.command.run() {
        error!("{err}");
    }
}
