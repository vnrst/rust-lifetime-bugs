// This currently does not work

use crate::bugs::Bug;

// https://rustsec.org/advisories/RUSTSEC-2022-0002.html

use std::ptr;
use std::marker::PhantomData;

use std::fmt::Debug;

pub struct Syn6;

// Yes it's true, the crate literally has this method
pub unsafe fn change_lifetime_const<'a, 'b, T>(x: &'a T) -> &'b T {
    &*(x as *const T)
}

pub unsafe fn change_lifetime_const_mut<'a, 'b, T>(x: &'a mut T) -> &'b mut T {
    &mut *(x as *mut T)
}

pub struct Ref<'a, V> {
    v: &'a V,
}

pub struct RefMut<'a, V> {
    v: &'a mut V,
}

impl<'a, V> Ref<'a, V> {

	pub fn value(&self) -> &'a V {
        self.v
    }
}

impl<'a, V> RefMut<'a, V> {

	fn value_mut(&self) -> &V {
		self.v
	}
}

pub struct Dashmap<V> {
	val: *mut V
}

impl<'a, V> Dashmap<V> {

	pub fn new(value: V) -> Dashmap<V> {
		Dashmap{val: Box::into_raw(Box::new(value))}
	}

	pub fn get_mut(&'a mut self) -> Option<RefMut<'a, V>> {
		if let Some(v) = unsafe{self.val.as_mut()} {
			unsafe{
				let v = change_lifetime_const_mut(v);
				Some(RefMut{v})
			}
		}
		else {
			None
		}
	}

	pub fn get(&'a self) -> Option<Ref<'a, V>> {
		if let Some(v) = unsafe{self.val.as_ref()} {
			unsafe{
				let v = change_lifetime_const(v);
				Some(Ref{v})
			}
		}
		else {
			None
		}
	}
}

impl Bug for Syn6 {

	fn exploit() {

		let key: u32 = 1;
		let mut value: String = "Hello".to_string();

		let mut x: Dashmap<String> = Dashmap::new(value);

		let borrow = x.get().unwrap();
		let val = borrow.value();
		drop(borrow);

		let borrow_mut = x.get_mut().unwrap();
		let val_mut = borrow_mut.value_mut();

		println!("{:?}", val);
	}
}
