use crate::bugs::Bug;

use std::sync::Arc;

pub struct Waker {
	data: *const ()
}

// Bug
pub fn waker<W>(wake: Arc<W>) -> Waker {
	let data = Arc::into_raw(wake) as *const ();
	Waker{data}
}

impl Waker {

	pub fn get_data<T>(&mut self) -> Arc<T> {
		unsafe{Arc::from_raw(self.data as *const T)}
	}
}

pub struct Syn3;

impl Bug for Syn3 {

	fn exploit() {

		let mut w: Waker;

		let mut data : Vec<i32> = Vec::from([1, 2, 3]);
		let arc = Arc::new(&data);
		w = waker(arc);

		drop(data);

		let rec_data : Arc<&Vec<i32>> = w.get_data::<&Vec<i32>>();

		println!("{:?}", rec_data);
	}
}
