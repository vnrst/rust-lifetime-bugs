use crate::bugs::Bug;

// https://rustsec.org/advisories/RUSTSEC-2018-0020.html

pub struct Iterator<T> {
	ptr: *const T,
}

impl<T> Iterator<T> {

	fn new(data: &T) -> Iterator<T> {
		Iterator{ptr: data as *const T}
	}

	fn get(&mut self) -> Option<&T> {
		unsafe{self.ptr.as_ref()}
	}
}

// Bug
pub fn iter<T>(data: &T) -> Iterator<T> {
	Iterator::new(data)
}

pub struct Syn1_2;

impl Bug for Syn1_2 {

	fn exploit() {

		let data = Vec::from([1, 2, 3]);

		let mut iter = iter(&data);

		drop(data);
		println!("{:?}", iter.get());
	}
}
