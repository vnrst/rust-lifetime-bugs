use crate::bugs::Bug;

// https://rustsec.org/advisories/RUSTSEC-2021-0130.html

use std::ptr;
use std::marker::PhantomData;
use std::fmt::Debug;
use std::ops::Drop;

pub struct LruEntry<T> {
	data: T
}

pub struct LruCache<T> {
	head: *mut LruEntry<T>
}

pub struct Iter<'a, T: 'a> {
	ptr: *const LruEntry<T>,
	phantom: PhantomData<&'a T>
}

impl<'a, T: Debug + 'a> LruCache<T> {

	pub fn new(entry: &'a mut LruEntry<T>) -> LruCache<T> {
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

		let data = unsafe{&(*(self.ptr)).data};
		data
	}

}

pub struct Syn5_1;

impl Bug for Syn5_1 {

	fn exploit() {

		let data = "Hello".to_string();
		let mut entry = LruEntry{data};

		let mut cache = LruCache::new(&mut entry);

		let mut iter = cache.iter();
		let rec_data = iter.get();
		drop(entry);
		println!("{:?}", rec_data);
	}
}
