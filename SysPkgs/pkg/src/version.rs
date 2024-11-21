use std::cmp::Ordering;
use std::cmp::PartialEq;
use rust_apt::util;

#[derive(Debug)]
pub struct Version<'a> {
	pub s: &'a str // s = string
}

impl PartialEq for Version<'_> {
	fn eq(&self, other: &Version) -> bool {
		util::cmp_versions(self.s, other.s) == Ordering::Equal
	}
	fn ne(&self, other: &Version) -> bool {
		!self.eq(other)
	}
}

#[allow(dead_code)]
impl Version<'_> {
	pub fn new(s: &str) -> Version {
		Version { s }
	}

	pub fn gt(&self, other: &Version) -> bool {
		util::cmp_versions(self.s, other.s) == Ordering::Less
	}

	pub fn ge(&self, other: &Version) -> bool {
		util::cmp_versions(self.s, other.s) != Ordering::Greater
	}

	pub fn lt(&self, other: &Version) -> bool {
		util::cmp_versions(self.s, other.s) == Ordering::Greater
	}

	pub fn le(&self, other: &Version) -> bool {
		util::cmp_versions(self.s, other.s) != Ordering::Less
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_version_eq() {
		let v = Version::new("1.2.3");
		assert_eq!(v, Version { s: "1.2.3" });
	}

	#[test]
	fn test_version_ne() {
		let v = Version { s: "5.4.3" };
		assert_ne!(v, Version { s: "5.4.4" });
	}

	#[test]
	fn test_version_lt() {
		let v = Version { s: "5.4.3" };
		assert!(v.lt(&Version { s: "5.4.4" }));
		assert!(!v.lt(&Version { s: "5.4.3" }));
	}

	#[test]
	fn test_version_gt() {
		let v = Version { s: "5.4.4" };
		assert!(v.gt(&Version { s: "5.4.3" }));
		assert!(!v.gt(&Version { s: "5.4.4" }));
	}

	#[test]
	fn test_version_le() {
		let v = Version { s: "5.4.3" };
		assert!(v.le(&Version { s: "5.4.4" }));
		assert!(v.le(&Version { s: "5.4.3" }));
	}

	#[test]
	fn test_version_ge() {
		let v = Version { s: "5.4.4" };
		assert!(v.ge(&Version { s: "5.4.3" }));
		assert!(v.ge(&Version { s: "5.4.4" }));
	}
}