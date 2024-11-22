use std::cmp::Ordering;
use std::cmp::PartialEq;
use rust_apt::util;

#[derive(Debug)]
pub struct Version {
    pub s: String // s = string
}

impl PartialEq for Version {
    fn eq(&self, other: &Version) -> bool {
        util::cmp_versions(self.s.as_str(), other.s.as_str()) == Ordering::Equal
    }
    fn ne(&self, other: &Version) -> bool {
        !self.eq(other)
    }
}

#[allow(dead_code)]
impl Version {
    pub fn new(s: String) -> Version {
        Version { s: s.trim().to_string() }
    }

    pub fn gt(&self, other: &Version) -> bool {
        util::cmp_versions(self.s.as_str(), other.s.as_str()) == Ordering::Greater
    }

    pub fn ge(&self, other: &Version) -> bool {
        util::cmp_versions(self.s.as_str(), other.s.as_str()) != Ordering::Less
    }

    pub fn lt(&self, other: &Version) -> bool {
        util::cmp_versions(self.s.as_str(), other.s.as_str()) == Ordering::Less
    }

    pub fn le(&self, other: &Version) -> bool {
        util::cmp_versions(self.s.as_str(), other.s.as_str()) != Ordering::Greater
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_eq() {
        let v = Version::new("1.2.3".to_string());
        assert_eq!(v, Version::new("1.2.3".to_string()));
    }

    #[test]
    fn test_version_ne() {
        let v = Version::new("5.4.3".to_string());
        assert_ne!(v, Version::new("5.4.4".to_string()));
    }

    #[test]
    fn test_version_lt() {
        let v = Version::new("5.4.3".to_string());
        assert!(v.lt(&Version::new("5.4.4".to_string())));
        assert!(!v.lt(&Version::new("5.4.3".to_string())));
    }

    #[test]
    fn test_version_gt() {
        let v = Version::new("5.4.4".to_string());
        assert!(v.gt(&Version::new("5.4.3".to_string())));
        assert!(!v.gt(&Version::new("5.4.4".to_string())));
		assert!(!v.gt(&Version::new("5.4.5".to_string())));
    }

    #[test]
    fn test_version_le() {
        let v = Version::new("5.4.3".to_string());
        assert!(v.le(&Version::new("5.4.4".to_string())));
        assert!(v.le(&Version::new("5.4.3".to_string())));
    }

    #[test]
    fn test_version_ge() {
        let v = Version::new("5.4.4".to_string());
        assert!(v.ge(&Version::new("5.4.3".to_string())));
        assert!(v.ge(&Version::new("5.4.4".to_string())));
    }
}