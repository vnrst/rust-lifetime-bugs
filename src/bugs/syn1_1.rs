use crate::bugs::Bug;

// https://rustsec.org/advisories/RUSTSEC-2018-0020.html

pub struct PropList<T: Default> {
	ptr: Box<T>
}

pub struct Iterator<T> {
	ptr: *const T,
}

impl<T> Iterator<T> {

	pub fn new(pl: *const T) -> Self {
		Iterator{ptr: pl}
	}

	fn get(&mut self) -> Option<&T> {
			unsafe{self.ptr.as_ref()}
	}
}

impl<T: Default> PropList<T> {

	pub fn new(data: T) -> Self {
		Self{ptr: Box::new(data)}
	}

	// Bug
	pub fn iter(&self) -> Iterator<T> {
		let raw_ptr = &(*self.ptr) as *const T;
		Iterator::new(raw_ptr)
	}
}

pub struct Syn1_1;

impl Bug for Syn1_1 {

	fn exploit() {

		let data = Vec::from([1, 2, 3]);
		let mut pl: PropList<Vec<i32>> = PropList::new(data);

		let mut iter = pl.iter();

		drop(pl);
		println!("{:?}", iter.get());
	}
}
