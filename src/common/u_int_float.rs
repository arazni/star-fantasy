#[derive(Clone)]
pub struct UIntFloat {
	i: i32,
	f: f32,
	u: u32
}

impl UIntFloat {
	pub fn new(int: u32) -> Self {
		Self {i: int as i32, f: int as f32, u: int}
	}

	pub fn i(&self) -> i32 {
		self.i
	}

	pub fn f(&self) -> f32 {
		self.f
	}

	pub fn u(&self) -> u32 {
		self.u
	}
}