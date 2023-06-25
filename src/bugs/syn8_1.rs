use crate::bugs::Bug;

// https://rustsec.org/advisories/RUSTSEC-2022-0040.html

use std::ptr;
use std::marker::PhantomData;

use std::fmt::Debug;
use std::ops::Deref;

pub struct OwningRef<O, T: ?Sized> {
    owner: O,
    reference: *const T,
}


impl<'a, O, T: ?Sized +'a> OwningRef<O, T> {

    pub fn new(o: O) -> Self
        where O: Deref<Target = T>,
    {
        OwningRef {
            reference: &*o,
            owner: o,
        }
    }
    // Bug
    pub fn map<F, U: ?Sized + 'a>(self, f: F) -> OwningRef<O, U>
        where F: FnOnce(&T) -> &U
    {
        OwningRef {
            reference: f(&self),
            owner: self.owner,
        }
    }
}

impl<O, T: ?Sized> Deref for OwningRef<O, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe {
            &*self.reference
        }
    }
}

pub struct Syn8_1;

impl Bug for Syn8_1 {

    fn exploit() {

        // Exploit taken from
        // https://github.com/Kimundi/owning-ref-rs/issues/71

        let x = OwningRef::new(Box::new(()));
        let z: OwningRef<Box<()>, str>;

        let s = "Hello World!".to_string();
        let s_ref: &str = &s;
        let y: OwningRef<Box<()>, &str> = x.map(|_| &s_ref);
        z = y.map(|s: &&str| *s);
        // s deallocated here
        drop(s);

        println!("{}", &*z); // printing garbage, accessing `s` after it’s freed
    }
}
