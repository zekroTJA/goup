use super::Version;
use crate::{
    cmd::{self, exec},
    warning,
};
use anyhow::Result;
use reqwest::{blocking::Client, header};
use serde::Deserialize;
use std::str::FromStr;

const GOLANG_REPO: &str = "https://github.com/golang/go.git";
const GOLANG_TAGS_ENDPOINT: &str =
    "https://api.github.com/repos/golang/go/git/matching-refs/tags/go";

/// Queries all tags from the upstream
/// [Go repository on GitHub](https://github.com/golang/go),
/// filters for all tags prefixed with `go`, parses the
/// [`Version`]s and returns the sorted list of versions
/// (oldest to latest).
///
/// The tags are first tried to be fetched via the GitHub API.
/// If this fails, a warning message is printed and
/// `git ls-remote --tags` is used as fallback.
pub fn get_upstream_versions() -> Result<Vec<Version>> {
    let mut tags = get_upstream_versions_api().or_else(|err| {
        warning!(
            "Listing remote versions via GitHub API failed, falling back to using git ls-remote.\n\
            Error was: {err}"
        );
        get_upstream_versions_git()
    })?;

    tags.sort();

    Ok(tags)
}

/// Fetches a list of versions from the Go remote repository on
/// GitHub using `git ls-remote --tags`.
fn get_upstream_versions_git() -> Result<Vec<Version>> {
    let res = exec(&["git", "ls-remote", "--tags", GOLANG_REPO]);

    let res = match res {
        Ok(res) => res,
        Err(err) if matches!(err.kind(), cmd::errors::ErrorKind::NotFound) => {
            anyhow::bail!(
                "Seems you don't have git installed on your system. Listing versions failed."
            )
        }
        Err(err) => return Err(err.into()),
    };

    res.split('\n')
        .filter_map(|line| line.split_once("refs/tags/"))
        .map(|(_, tag)| tag)
        .filter_map(|tag| tag.strip_prefix("go"))
        .map(FromStr::from_str)
        .collect()
}

#[derive(Deserialize)]
struct Ref {
    r#ref: String,
}

/// Fetches a list of versions from the Go remote repository on
/// GitHub using the GitHub REST API.
fn get_upstream_versions_api() -> Result<Vec<Version>> {
    let refs: Vec<Ref> = Client::builder()
        .build()?
        .get(GOLANG_TAGS_ENDPOINT)
        .header(header::USER_AGENT, "goup")
        .send()?
        .error_for_status()?
        .json()?;

    refs.iter()
        .filter_map(|r| r.r#ref.strip_prefix("refs/tags/go"))
        .map(FromStr::from_str)
        .collect()
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
    get_upstream_versions()?
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
pub fn find_upstream_version(s: &Version) -> Result<Version> {
    get_upstream_versions()?
        .iter()
        .rev()
        .filter(|v| v.is_stable() || !s.is_stable())
        .find(|v| s.covers(v))
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("no matching stable version found"))
}
