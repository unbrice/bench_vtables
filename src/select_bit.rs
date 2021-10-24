//! Functions to find the n-th set bit of an integer in a fast-ish way.
//!
//! That's basically the overhead for `SieveTable`, so it's worth optimising them.
//! This is an active field of research, but existing library generally
//!
//! The main optimisations are:
//!
//! * Relying on at least one of the bits of the `sieve` to be set, which allows using `bsf` on x86
//!   without needing an extra if.
//! * Relying on the rank `n` to be a number known at compile time and within bound.
use std::{
    mem,
    num::{NonZeroU32, NonZeroU64, NonZeroUsize},
};

use bitintr::Pdep as _;

/// Returns the offset of the n-th bit, from the end set to 1.
/// This version gets unrolled for small value of n.
///
/// # Safety
///
/// Behavior is undefined if `sieve` has fewer than `n` bit sets.
#[inline(always)]
unsafe fn find_nth_set_bit_small(sieve: usize, n: u32) -> u32 {
    let mut bits = sieve;
    for _ in 0..n {
        bits &= bits.wrapping_sub(1);
    }
    NonZeroUsize::new_unchecked(bits).trailing_zeros()
}

/// Returns the offset of the n-th bit, from the end set to 1.
///
/// # Safety
///
/// Behavior is undefined if `sieve` has fewer than `n` bit sets.
#[inline(always)]
pub unsafe fn find_nth_set_bit(sieve: usize, n: u32) -> u32 {
    if n <= 3 {
        // 3 comes from building find_nth_set_bit_small with fixed n on godbolt.
        // It was 3.1 cycle, slightly less than what I expect of PDEP.
        // TODO: Is 3 the best threshold?
        find_nth_set_bit_small(sieve, n)
    } else if cfg!(all(
        any(target_arch = "x86", target_arch = "x86_64"),
        all(target_feature = "bmi1", target_feature = "bmi2"),
        not(feature = "slow_pdep"),
    )) {
        if mem::size_of::<usize>() == 4 {
            let mask = 1u32 << n;
            let bits = mask.pdep(sieve as u32);
            NonZeroU32::new_unchecked(bits).trailing_zeros()
        } else {
            let mask = 1u64 << n;
            let bits = mask.pdep(sieve as u64);
            NonZeroU64::new_unchecked(bits).trailing_zeros()
        }
    } else if n < 21 {
        // 16 comes from building select1_raw with fixed n on godbolt.
        // It was 45 instructions, 2320 cycles, 5k uOPS.
        // find_nth_set_bit_small(_, 21) is about the same number of instructions, 1533 cycles, 5k uOPS.
        // TODO: Is 21 the best threshold? Check after optimising the generic solution.
        find_nth_set_bit_small(sieve, n)
    } else {
        // This isn't properly optimised, a better variant of this algorithm (using pocount) seems available
        // here https://github.com/facebook/folly/blob/bd600cd4e88f664f285489c76b6ad835d8367cd2/folly/experimental/Select64.h
        // Also this doesn't get inlined etc.
        succinct::broadword::select1_raw(n as usize, sieve as u64) as u32
        // Other alternatives to benchmark:
        // https://github.com/simongog/sdsl-lite/blob/c32874cb2d8524119f25f3b501526fe692df29f4/include/sdsl/bits.hpp#L333
        // http://bitmagic.io/rank-select.html
        // https://docs.rs/succinct/0.5.2/src/succinct/broadword.rs.html#47-51
        // https://graphics.stanford.edu/~seander/bithacks.html#SelectPosFromMSBRank
    }
}
