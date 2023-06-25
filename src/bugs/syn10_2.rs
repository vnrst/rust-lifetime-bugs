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
    bump: &'a String
}

impl<'bump, T: 'bump> RawVec<'bump, T> {

	pub fn new_in(bump: &'bump String) -> RawVec<'bump, T> {
		RawVec {
			ptr: NonNull::dangling(),
			bump
		}
	}

	pub fn as_mut_ptr(&mut self) -> *mut T {
		self.ptr.as_ptr()
	}

	pub fn push(&mut self, value: T) {
		self.ptr = NonNull::new(Box::into_raw(Box::new(value))).unwrap();
	}
}

pub struct IntoIter<T> {
    ptr: *const T,
    bump: *const String
}

impl<'bump, T: 'bump> RawVec<'bump, T> {

    // Bug
	fn into_iter(mut self) -> IntoIter<T> {

        let begin = self.as_mut_ptr();
        let bump = self.bump;
		mem::forget(self);

        IntoIter {
            ptr: begin,
            bump
        }
	}
}

impl<'bump, T: 'bump> IntoIter<T> {

    fn get(&mut self) -> T {
	    unsafe {
	        println!("Bump: {}", &*self.bump);
	        ptr::read(self.ptr)
	    }
    }
}

pub struct Syn10_2;

impl Bug for Syn10_2 {

	fn exploit() {

		let mut bump = "Bump".to_string();
		let mut vec: RawVec<&String> = RawVec::new_in(&mut bump);

		let mut mystr = "Hello".to_string();
		vec.push(&mut mystr);

		let mut into_iter = vec.into_iter();

		drop(bump);

		let x = into_iter.get();

		println!("{}", x);
	}
}
