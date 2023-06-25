use crate::bugs::Bug;

// https://rustsec.org/advisories/RUSTSEC-2022-0070.html

use std::ptr;
use std::marker::PhantomData;

use std::fmt::Debug;

pub struct MyContext {}

pub struct Secp256k1<C> {
    ctx: *const (),
    phantom: PhantomData<C>,
}

impl<'buf, C : 'buf> Secp256k1<C> {

	//Bug
	pub fn preallocated_gen_new<T: 'buf>(buf: T) -> Secp256k1<C> {

		let boxed: Box<T> = Box::new(buf);
		let data: *const Box<T> = Box::into_raw(Box::new(boxed));

		Secp256k1 {
			ctx: data as *const (),
			phantom: PhantomData
		}
	}

	fn get_data<T>(&self) -> &T {
		unsafe{&*(*(self.ctx as *const Box<T>))}
	}
}

pub struct Syn9_2;

impl Bug for Syn9_2 {

	fn exploit() {

		let mut buf = Vec::from([0u8, 1, 2, 3]);

		let obj: Secp256k1<MyContext> = Secp256k1::preallocated_gen_new(&buf[..]);

		drop(buf);
		let rec = obj.get_data::<&[u8]>();

		println!("{:?}", rec);
	}
}
