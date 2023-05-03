use std::path::PathBuf;

use super::get_current_bin_dir;
use crate::env::get_current_install_dir;
use anyhow::Result;

pub fn print_vars() -> Result<()> {
    let vars = vec![
        (
            "PATH",
            format!("{}:$PATH", path_to_string(get_current_bin_dir()?)),
        ),
        ("GOROOT", path_to_string(get_current_install_dir()?)),
    ];

    let lines: Vec<_> = vars
        .iter()
        .map(|(k, v)| format!("export {k}=\"{v}\""))
        .collect();

    println!("{}", lines.join("\n"));

    Ok(())
}

fn path_to_string(p: PathBuf) -> String {
    p.to_string_lossy().to_string()
}
