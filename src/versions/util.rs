use super::Version;
use crate::cmd::{self, exec};
use anyhow::Result;
use std::str::FromStr;

const GOLANG_REPO: &str = "https://github.com/golang/go.git";

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

pub fn get_latest_stable() -> Result<Version> {
    get_versions()?
        .iter()
        .rev()
        .find(|v| v.is_stable())
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("no stable version found"))
}

pub fn get_latest_unstable() -> Result<Version> {
    get_versions()?
        .iter()
        .rev()
        .find(|v| !v.is_stable())
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("no unstable version found"))
}

pub fn find_version(s: &Version) -> Result<Version> {
    get_versions()?
        .iter()
        .rev()
        .filter(|v| v.is_stable() || !s.is_stable())
        .find(|v| {
            if s.major != v.major {
                return false;
            }

            match s.minor {
                Some(x) if x != v.minor.unwrap_or_default() => return false,
                _ => {}
            };

            match s.patch {
                Some(x) if x != v.patch.unwrap_or_default() => return false,
                _ => {}
            };

            match &s.pre {
                Some(x) if x != &v.pre.clone().unwrap_or_default() => return false,
                _ => {}
            };

            true
        })
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("no matching stable version found"))
}
