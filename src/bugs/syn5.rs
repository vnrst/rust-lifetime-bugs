use crate::bugs::Bug;

// https://rustsec.org/advisories/RUSTSEC-2021-0130.html

use std::ptr;
use std::marker::PhantomData;


pub struct LruEntry<T> {
	data: T,
	next: *mut LruEntry<T>
}

pub struct LruCache<T> {
	head: *mut LruEntry<T>,
	tail: *mut LruEntry<T>,
	len: usize
}

pub struct Iter<'a, T: 'a> {
	ptr: *const LruEntry<T>,
	len: usize,
	phantom: PhantomData<&'a T>
}

impl<T> LruCache<T> {

	pub fn new() -> LruCache<T> {
		LruCache {
			head: ptr::null_mut(),
			tail: ptr::null_mut(),
			len: 0
		}
	}

	pub fn insert(&mut self, data: T) {

		let entry = Box::into_raw(Box::new(LruEntry{data, next: ptr::null_mut()}));

		if self.len == 0 {
			self.head = entry;
			self.tail = entry;
			self.len = 1;
		}
		else {
			unsafe{
				(*(self.tail)).next = entry;
				self.tail = entry;
			}
			self.len += 1;
		}
	}

	pub fn clear(&mut self) {

		let mut head = self.head;

		while self.len > 0 {

			let mut temp = unsafe{(*(head)).next};
			unsafe{ptr::drop_in_place(head);}
			head = temp;
			self.len -= 1;
		}
	}
	// Bug
	pub fn iter<'a>(&'_ self) -> Iter<'a, T> {
		Iter{
			ptr: self.head,
			len: self.len,
			phantom: PhantomData
		}
	}
}

impl<'a, T> Iterator for Iter<'a, T> {

	type Item = &'a T;

	fn next(&mut self) -> Option<&'a T> {

		if self.len == 0 {
			return None;
		}
		let data = unsafe{&(*(self.ptr)).data};
		self.len -= 1;
		self.ptr =  unsafe{(*(self.ptr)).next};
		Some(data)
	}

}

pub struct Syn5;

impl Bug for Syn5 {

	fn exploit() {

		let mut cache: LruCache<&String> = LruCache::new();
		let data = "Hello".to_string();
		cache.insert(&data);

		for entry in cache.iter() {
			cache.clear();
			println!("{:?}", entry);

		}
	}
}
