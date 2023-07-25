use super::*;
use crate::versions::Version;
use anyhow::Result;
use std::{fs, os::windows::fs::symlink_dir};

/// Creates a symlink to the SDK installation dir of
/// the given [`Version`].
///
/// If [`Some(Version)`] is passed, the specified SDK
/// version directory is set.
/// If [`None`] is passed, the symlink is remoevd, if
/// existent.
pub fn link_current_version(v: Option<&Version>) -> Result<()> {
    let link = get_current_link_dir()?;

    match v {
        None => {
            if link.exists() {
                fs::remove_dir(&link)?;
            }
        }
        Some(v) => {
            let original = get_version_installation_dir(v)?;

            if link.exists() {
                fs::remove_dir(&link)?;
            }

            symlink_dir(original, link)?;
        }
    }

    Ok(())
}
