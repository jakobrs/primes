#![allow(clippy::missing_safety_doc)]

// These bindings are based on version 7.7 of primesieve

use std::os::raw::{c_char, c_int, c_void};

// std::os::raw::c_size_t is unstable
#[allow(non_camel_case_types)]
type c_size_t = usize;

pub const PRIMESIEVE_ERROR: u64 = !0u64;

/// Corresponds to the unnamed enum in primesieve.h
#[allow(non_camel_case_types)]
#[repr(C)]
pub enum Type {
    SHORT_PRIMES,
    USHORT_PRIMES,
    INT_PRIMES,
    UINT_PRIMES,
    LONG_PRIMES,
    ULONG_PRIMES,
    LONGLONG_PRIMES,
    ULONGLONG_PRIMES,
    INT16_PRIMES,
    UINT16_PRIMES,
    INT32_PRIMES,
    UINT32_PRIMES,
    INT64_PRIMES,
    UINT64_PRIMES,
}

#[repr(C)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct primesieve_iterator {
    i: c_size_t,
    size: c_size_t,
    start: u64,
    stop_hint: u64,
    primes: *mut u64,
    memory: *mut c_void,
    is_error: c_int,
}

#[link(name = "primesieve")]
extern "C" {
    // From primesieve.h
    pub fn primesieve_generate_primes(
        start: u64,
        stop: u64,
        size: *mut c_size_t,
        type_: c_int,
    ) -> *mut c_void;
    pub fn primesieve_generate_n_primes(n: u64, start: u64, type_: c_int) -> *mut c_void;
    pub fn primesieve_nth_prime(n: i64, start: u64) -> u64;
    pub fn primesieve_count_primes(start: u64, stop: u64) -> u64;
    pub fn primesieve_count_twins(start: u64, stop: u64) -> u64;
    pub fn primesieve_count_triplets(start: u64, stop: u64) -> u64;
    pub fn primesieve_count_quadruplets(start: u64, stop: u64) -> u64;
    pub fn primesieve_count_quintuplets(start: u64, stop: u64) -> u64;
    pub fn primesieve_count_sextuplets(start: u64, stop: u64) -> u64;
    pub fn primesieve_print_primes(start: u64, stop: u64) -> u64;
    pub fn primesieve_print_twins(start: u64, stop: u64);
    pub fn primesieve_print_triplets(start: u64, stop: u64);
    pub fn primesieve_print_triples(start: u64, stop: u64);
    pub fn primesieve_print_quadruplets(start: u64, stop: u64);
    pub fn primesieve_print_quintuples(start: u64, stop: u64);
    pub fn primesieve_print_sextuples(start: u64, stop: u64);
    pub fn primesieve_get_max_stop() -> u64;
    pub fn primesieve_get_sieve_size() -> c_int;
    pub fn primesieve_get_num_threads() -> c_int;
    pub fn primesieve_set_sieve_size(sieve_size: c_int);
    pub fn primesieve_set_num_threads(num_threads: c_int);
    pub fn primesieve_free(primes: *mut c_void);
    pub fn primesieve_version() -> *const c_char;

    // From interator.h
    pub fn primesieve_init(it: *mut primesieve_iterator);
    pub fn primesieve_free_iterator(it: *mut primesieve_iterator);
    pub fn primesieve_skipto(it: *mut primesieve_iterator, start: u64, stop_hint: u64);

    fn primesieve_generate_next_primes(it: *mut primesieve_iterator);
    fn primesieve_generate_prev_primes(it: *mut primesieve_iterator);
}

#[inline]
pub unsafe fn primesieve_next_prime(it: *mut primesieve_iterator) -> u64 {
    (*it).i += 1;
    if (*it).i >= (*it).size {
        primesieve_generate_next_primes(it);
    }
    *(*it).primes.add((*it).i)
}

#[inline]
pub unsafe fn primesieve_prev_prime(it: *mut primesieve_iterator) -> u64 {
    if (*it).i == 0 {
        primesieve_generate_prev_primes(it);
    }
    (*it).i -= 1;
    *(*it).primes.add((*it).i)
}

#[cfg(test)]
mod tests {
    use crate::*;

    // Test non-iterator API
    #[test]
    fn generate_nth_prime_works() {
        let result = unsafe { primesieve_nth_prime(10, 100) };

        assert_eq!(result, 149);
    }

    // Test iterator API
    #[test]
    fn iteration_works() {
        unsafe {
            let mut it = std::mem::zeroed();
            primesieve_init(&mut it);

            assert_eq!(primesieve_next_prime(&mut it), 2);
            assert_eq!(primesieve_next_prime(&mut it), 3);
            assert_eq!(primesieve_next_prime(&mut it), 5);

            primesieve_free_iterator(&mut it);
        }
    }
}
