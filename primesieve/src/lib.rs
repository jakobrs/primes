use std::mem::MaybeUninit;

use primesieve_sys as sys;

#[repr(transparent)]
pub struct PrimesieveIterator {
    it: sys::primesieve_iterator,
}

impl PrimesieveIterator {
    pub fn new() -> Self {
        unsafe {
            let mut it = MaybeUninit::zeroed();
            sys::primesieve_init(it.as_mut_ptr());
            PrimesieveIterator {
                it: it.assume_init(),
            }
        }
    }

    pub fn new_start(start: u64) -> Self {
        unsafe {
            let mut it = MaybeUninit::zeroed();
            sys::primesieve_init(it.as_mut_ptr());
            sys::primesieve_skipto(it.as_mut_ptr(), start, sys::primesieve_get_max_stop());
            PrimesieveIterator {
                it: it.assume_init(),
            }
        }
    }

    pub fn new_start_stop(start: u64, stop_hint: u64) -> Self {
        unsafe {
            let mut it = MaybeUninit::zeroed();
            sys::primesieve_init(it.as_mut_ptr());
            sys::primesieve_skipto(it.as_mut_ptr(), start, stop_hint);
            PrimesieveIterator {
                it: it.assume_init(),
            }
        }
    }
}

impl Drop for PrimesieveIterator {
    fn drop(&mut self) {
        unsafe {
            sys::primesieve_free_iterator(&mut self.it);
        }
    }
}

impl Iterator for PrimesieveIterator {
    type Item = u64;

    /// Returns u64::MAX if the next prime > 2^64
    fn next(&mut self) -> Option<Self::Item> {
        Some(unsafe { sys::primesieve_next_prime(&mut self.it) })
    }
}

#[cfg(test)]
mod tests {
    use crate::PrimesieveIterator;

    #[test]
    fn iteration_works() {
        let it = PrimesieveIterator::new();

        assert_eq!(
            it.take(10).collect::<Vec<u64>>(),
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]
        );
    }
}
