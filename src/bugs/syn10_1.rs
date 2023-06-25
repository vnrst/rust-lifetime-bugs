use crate::bugs::Bug;

// https://rustsec.org/advisories/RUSTSEC-2022-0078.html

use std::ptr::{self, NonNull};
use std::marker::PhantomData;
use std::mem;
use std::fmt::Debug;

// The actual crate uses the bump object to allocate blocks of memory
// for the data in the Vec.
// While the bump object is alive, the memory is reserved.
// If the bump object is dropped, the memory can be re-allocated to another Vec.
// The following is a crude approximation of that, with the bump object replaced by a String.

pub struct RawVec<'a, T> {
	ptr: NonNull<T>,
    cap: usize,
    bump: &'a String
}

impl<'bump, T: 'bump> RawVec<'bump, T> {

	pub fn new_in(bump: &'bump String) -> RawVec<'bump, T> {
		RawVec {
			ptr: NonNull::dangling(),
			cap: 0,
			bump
		}
	}

	pub fn as_mut_ptr(&mut self) -> *mut T {
		self.ptr.as_ptr()
	}

	pub fn push(&mut self, value: T) {
		self.ptr = NonNull::new(Box::into_raw(Box::new(value))).unwrap();
		self.cap = 1;
	}
}

pub struct IntoIter<T> {
    phantom: PhantomData<T>,
    ptr: *const T,
    len: usize,
    ind: usize,
    bump: *const String
}

impl<'bump, T: 'bump> IntoIterator for RawVec<'bump, T> {

    type Item = T;
    type IntoIter = IntoIter<T>;

    // Bug
	fn into_iter(mut self) -> IntoIter<T> {

        let begin = self.as_mut_ptr();
        let bump = self.bump;
		mem::forget(self);

        IntoIter {
            phantom: PhantomData,
            ptr: begin,
            len: 1,
            ind: 0,
            bump
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
	            println!("Bump: {}", &*self.bump);
	            Some(ptr::read(old))
	        }
	    }
    }
}

pub struct Syn10_1;

impl Bug for Syn10_1 {

	fn exploit() {

		let mut bump = "Bump".to_string();
		let mut vec: RawVec<&String> = RawVec::new_in(&mut bump);

		let mut mystr = "Hello".to_string();
		vec.push(&mut mystr);

		let into_iter = vec.into_iter();

		drop(bump);

		for x in into_iter {
			println!("{:?}", x);
		}
	}
}
