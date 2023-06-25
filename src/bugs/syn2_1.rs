use crate::bugs::Bug;

// https://rustsec.org/advisories/RUSTSEC-2020-0023.html

use std::marker::PhantomData;

#[derive(Debug)]
pub struct MatrixSliceMut<'a> {
	ptr: *mut [i32],
	phantom: PhantomData<&'a [i32]>
}

impl<'a> MatrixSliceMut<'a> {

	pub fn new<T: AsMut<[i32]> + 'a>(mut mat: T) -> Self {
		MatrixSliceMut{ptr: mat.as_mut(), phantom: PhantomData}
	}

	// Bug
	pub fn raw_slice_mut(&mut self) -> &'a mut [i32] {
		unsafe{&mut *(self.ptr)}
	}
}

pub struct Syn2_1;

impl Bug for Syn2_1 {

	fn exploit() {

		let mut data : Vec<i32> = Vec::from([1, 2, 3]);
		let mut mat_slice = MatrixSliceMut::new(&mut data);

		let mut slice1 = mat_slice.raw_slice_mut();
		let mut slice2 = mat_slice.raw_slice_mut();

		println!("{:?}", slice1);
		slice1[0] = 5;
		println!("{:?}", slice2);
	}
}
