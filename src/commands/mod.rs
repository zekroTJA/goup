crate::prelude!(current, env, ls, ls_remote, r#use, drop, clean);

use anyhow::Result;

/// Definition of an executable CLI sub command.
pub trait Command {
    fn run(&self) -> Result<()>;
}
