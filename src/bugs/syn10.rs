use crate::bugs::Bug;

// https://rustsec.org/advisories/RUSTSEC-2022-0078.html

use std::ptr::{self, NonNull};
use std::marker::PhantomData;
use std::mem;
use std::fmt::Debug;
use std::alloc::{alloc, dealloc, Layout};

pub struct Bump<T> {
	ptr: *mut T,
	cap: usize
}

impl<T> Bump<T> {

	pub fn new(cap: usize) -> Bump<T> {

		unsafe{
			let layout = Layout::array::<T>(cap).unwrap();
			let ptr: *mut T = alloc(layout) as *mut T;
			Bump {
				ptr,
				cap
			}
		}
	}

	pub fn drop(&mut self) {
		unsafe{
			let layout = Layout::array::<T>(self.cap).unwrap();
			dealloc(self.ptr as *mut u8, layout);
		}
	}
}

pub struct RawVec<'a, T> {
	buf: NonNull<T>,
	len: usize,
    cap: usize,
    bump: &'a Bump<T>
}

impl<'bump, T: 'bump> RawVec<'bump, T> {

	pub fn new_in(bump: &'bump mut Bump<T>) -> RawVec<'bump, T> {
		RawVec {
			buf: NonNull::dangling(),
			len: 0,
			cap: bump.cap,
			bump
		}
	}

	pub fn as_mut_ptr(&mut self) -> *mut T {
		self.buf.as_ptr()
	}

	pub fn push(&mut self, value: T) {

		unsafe{
			if self.len == 0 {
				self.buf = NonNull::new_unchecked(self.bump.ptr);
				ptr::write(self.buf.as_ptr(), value);
				self.len = 1;
			}
			else if self.len < self.cap {
				let end = self.buf.as_ptr().add(self.len);
				ptr::write(end, value);
				self.len += 1;
			}
		}
	}
}

pub struct IntoIter<T> {
    phantom: PhantomData<T>,
    ptr: *const T,
    len: usize,
    ind: usize,
}

impl<'bump, T: 'bump> IntoIterator for RawVec<'bump, T> {

    type Item = T;
    type IntoIter = IntoIter<T>;

    // Bug
	fn into_iter(mut self) -> IntoIter<T> {

        let begin = self.as_mut_ptr();
        let bump = self.bump;
        let len = self.len;
		mem::forget(self);

        IntoIter {
            phantom: PhantomData,
            ptr: begin,
            len: len,
            ind: 0
        }
	}
}

impl<'bump, T: 'bump> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {

    	if self.ind >= self.len {
    		None
    	}
    	else {
	        unsafe {
	            let old = self.ptr;
	            self.ptr = self.ptr.offset(1);
	            self.ind += 1;
	            Some(ptr::read(old))
	        }
	    }
    }
}

pub struct Syn10;

impl Bug for Syn10 {

	fn exploit() {

		let mut bump = Bump::<&String>::new(1);
		let mut vec: RawVec<&String> = RawVec::new_in(&mut bump);

		let mystr = "Hello".to_string();
		vec.push(&mystr);

		let into_iter = vec.into_iter();

		let mut newvec: RawVec<&String> = RawVec::new_in(&mut bump);
		let mystr = "World".to_string();
		newvec.push(&mystr);

		for x in into_iter {
			println!("{:?}", x); // Prints "World", showing that the value has been overwritten
			assert_eq!(x, "Hello");
		}
	}
}
