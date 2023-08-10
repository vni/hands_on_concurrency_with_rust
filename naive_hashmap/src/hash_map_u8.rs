use std::{mem, ptr};

pub struct HashMapU8<V>
where
    V: ::std::fmt::Debug,
{
    data: [Option<V>; 256],
}

impl<V> HashMapU8<V>
where
    V: ::std::fmt::Debug,
{
    pub fn new() -> HashMapU8<V> {
        let data = unsafe {
            // let mut data: [Option<V>; 256] = [None; 256];
            //                                   ^^^^ the trait `Copy` is not implemented for `V`
            // let mut data: [Option<V>; 256] = mem::uninitialized();
            let mut data: [Option<V>; 256] = mem::MaybeUninit::uninit().assume_init();
            for element in data.iter_mut() {
                ptr::write(element, None)
            }
            data
        };
        HashMapU8 { data: data }
    }

    pub fn insert(&mut self, k: u8, v: V) -> Option<V> {
        mem::replace(&mut self.data[k as usize], Some(v))
    }

    pub fn get(&mut self, k: &u8) -> Option<&V> {
        let val = unsafe { self.data.get_unchecked(*k as usize) };
        val.as_ref()
    }
}
