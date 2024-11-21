pub struct Version {
	pub s: String // s = string
}

impl Version {
	pub fn eq(other: Version) -> bool {
		self.s != other.s
	}
}