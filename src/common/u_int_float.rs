#[derive(Clone)]
pub struct UIntFloat {
	pub i: i32,
	pub f: f32,
	pub u: u32
}

impl UIntFloat {
	pub fn new(int: i32) -> Self {
		Self {i: int, f: int as f32, u: int as u32}
	}
}