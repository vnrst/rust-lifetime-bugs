use crate::bugs::Bug;

// https://rustsec.org/advisories/RUSTSEC-2021-0130.html

use std::ptr;
use std::marker::PhantomData;
use std::fmt::Debug;
use std::ops::Drop;

pub struct LruCache<T> {
	head: *mut T
}

pub struct Iter<'a, T: 'a> {
	ptr: *const T,
	phantom: PhantomData<&'a T>
}

impl<'a, T: Debug + 'a> LruCache<T> {

	// Also a bug
	pub fn new(entry: &'a mut T) -> LruCache<T> {
		LruCache {
			head: entry
		}
	}
	// Bug
	pub fn iter(&'_ self) -> Iter<'a, T> {
		Iter{
			ptr: self.head,
			phantom: PhantomData
		}
	}
}

impl<'a, T> Iter<'a, T> {

	fn get(&mut self) -> &'a T {

		let data = unsafe{&(*(self.ptr))};
		data
	}

}

pub struct Syn5_2;

impl Bug for Syn5_2 {

	fn exploit() {

		let mut data = "Hello".to_string();

		let mut cache = LruCache::new(&mut data);

		let mut iter = cache.iter();
		let rec_data = iter.get();
		drop(data);
		println!("{:?}", rec_data);
	}
}
