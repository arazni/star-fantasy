use std::{collections::VecDeque};

pub struct Ring<T> {
	inner: VecDeque<T>
}

impl<T> Ring<T> where T: Copy {
	pub fn from_iter<IT: Iterator<Item = T>>(ts: IT) -> Option<Self> {
		let ring = VecDeque::from_iter(ts);

		if ring.is_empty() {
			return None;
		}

		Some(Ring { inner: ring })
	}

	pub fn current(&self) -> T {
		*self.inner.front().unwrap()
	}

	pub fn next(&mut self) -> T {
		self.inner.rotate_left(1);
		self.current()
	}

	// pub fn previous(&mut self) -> T {
	// 	self.inner.rotate_right(1);
	// 	self.current()
	// }
}