use crate::bugs::Bug;

// https://rustsec.org/advisories/RUSTSEC-2022-0070.html

use std::ptr;
use std::marker::PhantomData;

use std::fmt::Debug;

pub trait Context {}
pub struct MyContext {}
impl Context for MyContext {}

pub trait PreallocatedContext<'a> {}
pub struct MyBoundedContext<'a> {
	phantom: PhantomData<&'a i32>
}
impl<'a> PreallocatedContext<'a> for MyBoundedContext<'a> {}
impl<'a> Context for MyBoundedContext<'a> {}

pub struct Secp256k1<C: Context> {
    ctx: *const (), // Using *const () instad of *const c_void
    phantom: PhantomData<C>,
}

// Try compiling with both variants of this impl header
impl<'buf, C: Context + 'buf> Secp256k1<C> {
// impl<'buf, C: Context + PreallocatedContext<'buf>> Secp256k1<C> {

	//Bug
	pub fn preallocated_gen_new(buf: &'buf [u8]) -> Secp256k1<C> {

		let temp: & &[u8] = &buf;
		let data: *const &[u8] = temp as *const &[u8];

		Secp256k1 {
			ctx: data as *const (),
			phantom: PhantomData
		}
	}

	fn get_data(&self) -> &[u8] {
		unsafe{*(self.ctx as *const &[u8])}
	}
}

pub struct Syn9;

impl Bug for Syn9 {

	fn exploit() {

		let mut buf = Vec::from([0u8, 1, 2, 3]);

		let obj: Secp256k1<MyBoundedContext> = Secp256k1::preallocated_gen_new(&buf[..]);

		drop(buf);
		let rec = obj.get_data();

		println!("{:?}", rec);
	}
}
