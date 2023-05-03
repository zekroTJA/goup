pub mod current;
pub mod env;
pub mod r#use;
pub mod ls_remote;

use anyhow::Result;

pub trait Command {
    fn run(&self) -> Result<()>;
}
