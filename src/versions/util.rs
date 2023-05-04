use super::Version;
use crate::cmd::{self, exec};
use anyhow::Result;
use std::str::FromStr;

const GOLANG_REPO: &str = "https://github.com/golang/go.git";

/// Queries all tags from the remote repository of Go on GitHub
/// and parses all tags prefixed with `go` into a [`Version`].
///
/// The resulting list of [`Version`]'s is then returned.
pub fn get_versions() -> Result<Vec<Version>> {
    let res = exec(&["git", "ls-remote", "--tags", GOLANG_REPO]);

    let res = match res {
        Ok(res) => res,
        Err(err) if matches!(err.kind(), cmd::errors::ErrorKind::NotFound) => {
            anyhow::bail!("Seems you don't have git installed on your system. Please install git to use the ls-remote sub command.")
        }
        Err(err) => return Err(err.into()),
    };

    let tags: Result<Vec<_>, _> = res
        .split('\n')
        .filter_map(|line| line.split_once("refs/tags/"))
        .map(|(_, tag)| tag)
        .filter_map(|tag| tag.strip_prefix("go"))
        .map(Version::from_str)
        .collect();

    let mut tags = tags?;
    tags.sort();

    Ok(tags)
}

/// Fetches upstream versions *(see [`get_versions`])* and returns
/// the latest stable [`Version`].
///
/// If `include_unstable` is passed as `true`, the latest version
/// is returned including unstable versions.
///
/// # Errors
/// If no version has been found, an error of type [`anyhow::Error`]
/// is returned with a message containing more details.
pub fn get_latest_upstream_version(include_unstable: bool) -> Result<Version> {
    get_versions()?
        .iter()
        .rev()
        .find(|v| include_unstable || v.is_stable())
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("no stable version found"))
}

/// Fetches upstream versions *(see [`get_versions`])* and returns
/// the latest version that covers the given Version `s`. See
/// [`Version`] implementation for more details.
///
/// If `s` includes a `pre` part *(`"1.20rc1"`, for example)*, the
/// specified pre-release version is returned. Otherwise, only
/// matching stable versions are returned.
///
/// # Errors
/// If no version has been found, an error of type [`anyhow::Error`]
/// is returned with a message containing more details.
pub fn find_version(s: &Version) -> Result<Version> {
    get_versions()?
        .iter()
        .rev()
        .filter(|v| v.is_stable() || !s.is_stable())
        .find(|v| s.covers(v))
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("no matching stable version found"))
}
