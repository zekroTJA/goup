use super::parser::parse_version;
use std::{
    cmp::Ordering,
    fmt::{Display, Write},
    str::FromStr,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum VersionState {
    Alpha(usize),
    Beta(usize),
    ReleaseCandidate(usize),
    Release,
}

impl Default for VersionState {
    fn default() -> Self {
        Self::Release
    }
}

impl ToString for VersionState {
    fn to_string(&self) -> String {
        match self {
            Self::Alpha(v) => format!("alpha{v}"),
            Self::Beta(v) => format!("beta{v}"),
            Self::ReleaseCandidate(v) => format!("rc{v}"),
            Self::Release => String::new(),
        }
    }
}

/// Version representation and parsing for
/// Go's version schema.
///
/// # Explanation
/// Go uses a version scheme non-compliant to semver
/// for it's SDK releases of the form
/// `major(.minor(.patch(rc|beta|alphaPreVersion)))`.
///
/// Here are some examples:
/// - `1`
/// - `1.2`
/// - `1.2.3`
/// - `1.3beta1`
/// - `1.4rc2`
///
/// # Examples
/// Basic usage:
/// ```
/// let v: Version = "1.2.3rc4".parse().unwrap();
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Version {
    pub major: usize,
    pub minor: Option<usize>,
    pub patch: Option<usize>,
    pub pre: Option<VersionState>,
}

impl Version {
    /// Returns true if the version defines a stable
    /// release (not suffixed with an `alpha`, `beta` or `rc`
    /// version part).
    ///
    /// # Example
    /// ```
    /// let version: Version = "1.2".parse().unwrap();
    /// assert!(version.is_stable());
    ///
    /// let version: Version = "1.3rc1".parse().unwrap();
    /// assert!(!version.is_stable());
    /// ```
    pub fn is_stable(&self) -> bool {
        match &self.pre {
            None => true,
            Some(x) => matches!(x, VersionState::Release),
        }
    }

    /// Returns true if the current version covers
    /// the given `other` version.
    ///
    /// # Example
    /// ```
    /// let a: Version = "1.2".parse().unwrap();
    /// let b: Version = "1.2".parse().unwrap();
    /// assert!(a.covers(&b));
    ///
    /// let a: Version = "1".parse().unwrap();
    /// let b: Version = "1.2.1".parse().unwrap();
    /// assert!(a.covers(&b));
    ///
    /// let a: Version = "1.3".parse().unwrap();
    /// let b: Version = "1.3.7".parse().unwrap();
    /// assert!(a.covers(&b));
    ///
    /// let a: Version = "1.2.3".parse().unwrap();
    /// let b: Version = "1.2.3rc1".parse().unwrap();
    /// assert!(a.covers(&b));
    ///
    /// let a: Version = "1.3".parse().unwrap();
    /// let b: Version = "1.2.1".parse().unwrap();
    /// assert!(!a.covers(&b));
    /// ```
    pub fn covers(&self, other: &Version) -> bool {
        if self.major != other.major {
            return false;
        }

        match self.minor {
            Some(x) if x != other.minor.unwrap_or_default() => return false,
            _ => {}
        };

        match self.patch {
            Some(x) if x != other.patch.unwrap_or_default() => return false,
            _ => {}
        };

        match &self.pre {
            Some(x) if x != &other.pre.clone().unwrap_or_default() => return false,
            _ => {}
        };

        true
    }
}

impl FromStr for Version {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_version(s)
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            return Ordering::Equal;
        }

        match self.major.cmp(&other.major) {
            Ordering::Equal => {}
            ord => return ord,
        }

        match self
            .minor
            .unwrap_or_default()
            .cmp(&other.minor.unwrap_or_default())
        {
            Ordering::Equal => {}
            ord => return ord,
        }

        match self
            .patch
            .unwrap_or_default()
            .cmp(&other.patch.unwrap_or_default())
        {
            Ordering::Equal => {}
            ord => return ord,
        }

        self.pre
            .clone()
            .unwrap_or_default()
            .cmp(&other.pre.clone().unwrap_or_default())
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.major.to_string())?;

        if let Some(minor) = &self.minor {
            f.write_char('.')?;
            f.write_str(&minor.to_string())?;
        }

        if let Some(patch) = &self.patch {
            f.write_char('.')?;
            f.write_str(&patch.to_string())?;
        }

        if let Some(pre) = &self.pre {
            f.write_str(&pre.to_string())?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_stable() {
        assert_eq!(
            Version::from_str("1").unwrap(),
            Version {
                major: 1,
                ..Default::default()
            }
        );

        assert_eq!(
            Version::from_str("1.2").unwrap(),
            Version {
                major: 1,
                minor: Some(2),
                ..Default::default()
            }
        );

        assert_eq!(
            Version::from_str("1.2.345").unwrap(),
            Version {
                major: 1,
                minor: Some(2),
                patch: Some(345),
                ..Default::default()
            }
        );
    }

    #[test]
    fn parse_unstable() {
        assert_eq!(
            Version::from_str("1rc1").unwrap(),
            Version {
                major: 1,
                pre: Some(VersionState::ReleaseCandidate(1)),
                ..Default::default()
            }
        );

        assert_eq!(
            Version::from_str("1.2beta34").unwrap(),
            Version {
                major: 1,
                minor: Some(2),
                pre: Some(VersionState::Beta(34)),
                ..Default::default()
            }
        );

        assert_eq!(
            Version::from_str("1.2.345alpha678").unwrap(),
            Version {
                major: 1,
                minor: Some(2),
                patch: Some(345),
                pre: Some(VersionState::Alpha(678))
            }
        );
    }

    #[test]
    fn ord() {
        assert!(Version::from_str("2").unwrap() > Version::from_str("1").unwrap());
        assert!(Version::from_str("2.1").unwrap() > Version::from_str("1.3").unwrap());
        assert!(Version::from_str("1.4").unwrap() > Version::from_str("1.3").unwrap());
        assert!(Version::from_str("1.2.3").unwrap() > Version::from_str("1.2").unwrap());
        assert!(Version::from_str("1.2.3").unwrap() > Version::from_str("1.2.2").unwrap());
        assert!(Version::from_str("1").unwrap() > Version::from_str("1rc1").unwrap());
        assert!(Version::from_str("1.2rc1").unwrap() > Version::from_str("1.2beta1").unwrap());
        assert!(Version::from_str("1.2beta2").unwrap() > Version::from_str("1.2beta1").unwrap());
        assert!(Version::from_str("2").unwrap() > Version::from_str("1rc2").unwrap());
    }

    #[test]
    fn is_stable() {
        assert!(Version::from_str("1").unwrap().is_stable());
        assert!(Version::from_str("1.1").unwrap().is_stable());
        assert!(Version::from_str("1.1.3").unwrap().is_stable());

        assert!(!Version::from_str("1alpha1").unwrap().is_stable());
        assert!(!Version::from_str("1.2beta2").unwrap().is_stable());
        assert!(!Version::from_str("1.2.3rc3").unwrap().is_stable());
    }

    #[test]
    fn covers() {
        let a: Version = "1.2".parse().unwrap();
        let b: Version = "1.2".parse().unwrap();
        assert!(a.covers(&b));

        let a: Version = "1".parse().unwrap();
        let b: Version = "1.2.1".parse().unwrap();
        assert!(a.covers(&b));

        let a: Version = "1.3".parse().unwrap();
        let b: Version = "1.3.7".parse().unwrap();
        assert!(a.covers(&b));

        let a: Version = "1.2.3".parse().unwrap();
        let b: Version = "1.2.3rc1".parse().unwrap();
        assert!(a.covers(&b));

        let a: Version = "1.3".parse().unwrap();
        let b: Version = "1.2.1".parse().unwrap();
        assert!(!a.covers(&b));
    }
}
