use crate::bugs::Bug;

// https://rustsec.org/advisories/RUSTSEC-2018-0020.html

pub struct PropList {
	ptr: Box<Vec<i32>>
}

pub struct Iterator {
	ptr: *const Vec<i32>,
	ind: usize
}

impl Iterator {

	pub fn new(pl: *const Vec<i32>) -> Self {
		Iterator{ptr: pl, ind: 0}
	}
}

impl std::iter::Iterator for Iterator {
	type Item = i32;

	fn next(&mut self) -> Option<Self::Item> {

		if self.ind >= (unsafe{(*(self.ptr)).len()} - 1) {
			None
		}
		else {
			let ret = unsafe{Some((*(self.ptr))[self.ind])};
			self.ind = self.ind + 1;
			ret
		}
	}
}

impl PropList {

	pub fn new() -> Self {
		let data : Vec<i32> = Vec::new();
		Self{ptr: Box::new(data)}
	}

	pub fn insert(&mut self, data: i32) {
		(*(self.ptr)).push(data);
	}

	// Bug
	pub fn iter(&self) -> Iterator {
		let raw_ptr = &(*self.ptr) as *const Vec<i32>;
		Iterator::new(raw_ptr)
	}
}

pub struct Syn1;

impl Bug for Syn1 {

	fn exploit() {

		let mut pl = PropList::new();
		pl.insert(1);
		pl.insert(2);

		let mut iter = pl.iter();
		drop(pl);
		println!("{:?}", iter.next());
	}
}
