use crate::bugs::Bug;
use std::fmt::Debug;

pub struct Waker {
	data: *const ()
}

// Bug
pub fn waker<W>(wake: W) -> Waker {
	let data: *mut W = Box::into_raw(Box::new(wake));
	let raw = data as *mut ();
	Waker{data: raw}
}

impl Waker {

	pub fn get_data<T>(&mut self) -> &T {
		unsafe{&*(self.data as *const T)}
	}
}

pub struct Syn3_1;

impl Bug for Syn3_1 {

	fn exploit() {

		let mut w: Waker;

		let mut data : Vec<i32> = Vec::from([1, 2, 3]);
		w = waker(&data);

		drop(data);

		let rec_data : &&Vec<i32> = w.get_data::<&Vec<i32>>();

		println!("{:?}", rec_data);
	}
}
