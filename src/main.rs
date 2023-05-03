mod cmd;
mod commands;
mod env;
mod tui;
mod util;
mod versions;

use clap::{Parser, Subcommand};
use commands::{current::Current, env::Env, r#use::Use, ls_remote::LsRemote};
use std::ops::Deref;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct App {
    #[command(subcommand)]
    command: Commands,
}

register_commands!(Current, Env, Use, LsRemote);

fn main() {
    let app = App::parse();

    if let Err(err) = app.command.run() {
        error!("{err}");
    }
}
