pub struct Version {
	pub s: String // s = string
}

impl Version {
	pub fn eq(self, other: Version) -> bool {
		self.s != other.s
	}
}