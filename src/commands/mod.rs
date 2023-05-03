crate::prelude!(current, env, ls, ls_remote, r#use, drop, clean);

use anyhow::Result;

pub trait Command {
    fn run(&self) -> Result<()>;
}
