mod syn1;
mod syn1_1;
mod syn1_2;

mod syn2;
mod syn2_1;
mod syn2_2;

mod syn3;
mod syn3_1;
mod syn3_2;

mod syn4;
mod syn4_1;
mod syn4_2;

mod syn5;
mod syn5_1;
mod syn5_2;

mod syn7;
mod syn7_1;
mod syn7_2;

mod syn8;
mod syn8_1;
mod syn8_2;

mod syn9;
mod syn9_1;
mod syn9_2;

mod syn10;
mod syn10_1;
mod syn10_2;

pub use syn1::Syn1;
pub use syn1_1::Syn1_1;
pub use syn1_2::Syn1_2;

pub use syn2::Syn2;
pub use syn2_1::Syn2_1;
pub use syn2_2::Syn2_2;

pub use syn3::Syn3;
pub use syn3_1::Syn3_1;
pub use syn3_2::Syn3_2;

pub use syn4::Syn4;
pub use syn4_1::Syn4_1;
pub use syn4_2::Syn4_2;

pub use syn5::Syn5;
pub use syn5_1::Syn5_1;
pub use syn5_2::Syn5_2;

pub use syn7::Syn7;
pub use syn7_1::Syn7_1;
pub use syn7_2::Syn7_2;

pub use syn8::Syn8;
pub use syn8_1::Syn8_1;
pub use syn8_2::Syn8_2;

pub use syn9::Syn9;
pub use syn9_1::Syn9_1;
pub use syn9_2::Syn9_2;

pub use syn10::Syn10;
pub use syn10_1::Syn10_1;
pub use syn10_2::Syn10_2;

pub trait Bug {
	fn exploit();
}
