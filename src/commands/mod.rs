crate::prelude!(current, env, ls, lsr, r#use, drop, clean, check, exec);

use anyhow::Result;

/// Definition of an executable CLI sub command.
pub trait Command {
    fn run(&self) -> Result<()>;
}
