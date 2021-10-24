use std::{marker::PhantomData, ptr::null};

use crate::{v_table::VTable, FuncTable, FuncTablePtr, Function};

#[repr(align(256))]
pub struct PackedSieveTable<Ptr: FuncTablePtr, const TRAIT_N: usize, const FUNC_N: usize> {
    traits: Vec<VTable<Ptr, FUNC_N>>,
    traits_ptr: [*const VTable<Ptr, FUNC_N>; TRAIT_N],
}

impl<Ptr: FuncTablePtr, const TRAIT_N: usize, const FUNC_N: usize>
    PackedSieveTable<Ptr, TRAIT_N, FUNC_N>
{
    pub fn new(funcs: &[Function<Ptr>]) -> Self {
        let traits: Vec<VTable<Ptr, FUNC_N>> = funcs
            .chunks(FUNC_N)
            .map(|fs| VTable::new_from_slice(fs))
            .collect();
        let mut traits_ptr = [null(); TRAIT_N];
        for (i, vt) in traits.iter().enumerate() {
            traits_ptr[i] = vt;
        }
        Self { traits, traits_ptr }
    }
}

pub struct PackedSievePtr<'vt, ArgT, RetT, const TRAIT_N: usize, const FUNC_N: usize> {
    multi_v_mangled: usize, //  *const SieveTable<Self, TRAIT_N, FUNC_N> | sieve
    multi_v_phantom: PhantomData<&'vt PackedSieveTable<Self, TRAIT_N, FUNC_N>>,
}

impl<'vt, ArgT, RetT, const TRAIT_N: usize, const FUNC_N: usize>
    PackedSievePtr<'vt, ArgT, RetT, TRAIT_N, FUNC_N>
{
    // A mask over the largest sieve we can store. Since the table is aligned 2^8,
    // we can have 8 traits max.
    const SIEVE_MASK: usize = (!0) << 8;

    pub fn new(sieve_table: &'vt PackedSieveTable<Self, TRAIT_N, FUNC_N>) -> Self {
        let sieve = usize::MAX >> (usize::BITS - TRAIT_N as u32);
        assert!(Self::SIEVE_MASK > sieve, "More than 8 traits");
        let ptr: *const _ = sieve_table;
        Self {
            multi_v_mangled: (ptr as usize) | sieve,
            multi_v_phantom: PhantomData,
        }
    }

    #[inline(always)]
    unsafe fn multi_v(&self) -> &'vt PackedSieveTable<Self, TRAIT_N, FUNC_N> {
        let ptr = self.multi_v_mangled & Self::SIEVE_MASK;
        &*(ptr as *const PackedSieveTable<Self, TRAIT_N, FUNC_N>)
    }
}

impl<'vt, ArgT, RetT, const TRAIT_N: usize, const FUNC_N: usize> Copy
    for PackedSievePtr<'vt, ArgT, RetT, TRAIT_N, FUNC_N>
{
}
impl<'vt, ArgT, RetT, const TRAIT_N: usize, const FUNC_N: usize> Clone
    for PackedSievePtr<'vt, ArgT, RetT, TRAIT_N, FUNC_N>
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<'vt, ArgT, RetT, const TRAIT_N: usize, const FUNC_N: usize> FuncTablePtr
    for PackedSievePtr<'vt, ArgT, RetT, TRAIT_N, FUNC_N>
{
    type Arg = ArgT;
    type Ret = RetT;
    #[inline(always)]
    unsafe fn get_function(&self, n: u32) -> Function<Self> {
        let trait_n = n / (FUNC_N as u32);
        let trait_func_n = n % (FUNC_N as u32);
        let trait_offset = crate::select_bit::find_nth_set_bit(self.multi_v_mangled, trait_n);
        let vt_ptr = &self
            .multi_v()
            .traits_ptr
            .get_unchecked(trait_offset as usize)
            .read();
        vt_ptr.get_function(trait_func_n)
    }
}
