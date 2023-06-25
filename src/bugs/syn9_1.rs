use crate::bugs::Bug;

// https://rustsec.org/advisories/RUSTSEC-2022-0070.html

use std::ptr;
use std::marker::PhantomData;

use std::fmt::Debug;

pub trait Context {}
pub struct MyContext {}
impl Context for MyContext {}

pub struct Secp256k1<C: Context> {
    ctx: *const [u8],
    phantom: PhantomData<C>,
}

impl<'buf, C: Context + 'buf> Secp256k1<C> {

	//Bug
	pub fn preallocated_gen_new(buf: &'buf [u8]) -> Secp256k1<C> {

		let data: *const [u8] = buf as *const [u8];

		Secp256k1 {
			ctx: data,
			phantom: PhantomData
		}
	}

	fn get_data(&self) -> &[u8] {
		unsafe{&*(self.ctx)}
	}
}

pub struct Syn9_1;

impl Bug for Syn9_1 {

	fn exploit() {

		let mut buf = Vec::from([0u8, 1, 2, 3]);

		let obj: Secp256k1<MyContext> = Secp256k1::preallocated_gen_new(&buf[..]);

		drop(buf);
		let rec = obj.get_data();

		println!("{:?}", rec);
	}
}
