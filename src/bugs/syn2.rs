use crate::bugs::Bug;

// https://rustsec.org/advisories/RUSTSEC-2020-0023.html

use std::marker::PhantomData;

#[derive(Debug)]
pub struct Matrix<T> {
	data: Vec<T>
}

#[derive(Debug)]
pub struct MatrixSliceMut<'a, T: 'a> {
	ptr: *mut T,
	len: usize,
	phantom: PhantomData<&'a T>
}

impl<'a, T: 'a> MatrixSliceMut<'a, T> {

	pub fn new(mat: &'a mut Matrix<T>) -> Self {
		MatrixSliceMut{ptr: mat.data.as_mut_ptr(), len: mat.data.len(), phantom: PhantomData}
	}

	// Bug
	pub fn raw_slice_mut(&mut self) -> &'a mut [T] {
		unsafe{std::slice::from_raw_parts_mut(self.ptr, self.len)}
	}
}

pub struct Syn2;

impl Bug for Syn2 {

	fn exploit() {

		let data : Vec<i32> = Vec::from([1, 2, 3]);
		let mut mat = Matrix{data: data};
		let mut mat_slice = MatrixSliceMut::new(&mut mat);

		// 'a1 starts here
		let mut slice1 = mat_slice.raw_slice_mut(); // '_1
		// 'a2 starts here
		let mut slice2 = mat_slice.raw_slice_mut(); // '_2

		println!("{:?}", slice1);
		println!("{:?}", slice2);
		// 'a1 ends here
		// 'a2 ends here
	}
}
