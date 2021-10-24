use crate::{FuncTable, FuncTablePtr, Function};

use crate::v_table::VTable;
pub struct MultiVTable<Ptr: FuncTablePtr, const TRAIT_N: usize, const FUNC_N: usize> {
    tables: [VTable<Ptr, FUNC_N>; TRAIT_N],
}

impl<Ptr: FuncTablePtr, const TRAIT_N: usize, const FUNC_N: usize>
    MultiVTable<Ptr, TRAIT_N, FUNC_N>
{
    pub fn new(funcs: &[Function<Ptr>]) -> Self {
        let mut tables = [VTable::new_with_filler(); TRAIT_N];
        for (i, chunk) in funcs.chunks(FUNC_N).enumerate() {
            tables[i] = VTable::new_from_slice(chunk);
        }
        Self { tables }
    }
    #[inline(always)]
    pub unsafe fn get_trait_function(&self, trait_n: u32, trait_func_n: u32) -> Function<Ptr> {
        let trait_ptr = self.tables.get_unchecked(trait_n as usize);
        trait_ptr.get_function(trait_func_n)
    }
}

pub struct MultiVPtr<'vt, ArgT, RetT, const TRAIT_N: usize, const FUNC_N: usize> {
    ptrs: [&'vt VTable<Self, FUNC_N>; TRAIT_N],
}

impl<'vt, ArgT, RetT, const TRAIT_N: usize, const FUNC_N: usize>
    MultiVPtr<'vt, ArgT, RetT, TRAIT_N, FUNC_N>
{
    pub fn new(mvt: &'vt MultiVTable<Self, TRAIT_N, FUNC_N>) -> Self {
        let mut tables_ptr = [&mvt.tables[0]; TRAIT_N];
        for i in 1..TRAIT_N {
            tables_ptr[i] = &mvt.tables[i]
        }
        Self { ptrs: tables_ptr }
    }
    #[inline(always)]
    pub unsafe fn get_trait_function(self, trait_n: u32, trait_func_n: u32) -> Function<Self> {
        let trait_ptr = self.ptrs.get_unchecked(trait_n as usize);
        trait_ptr.get_function(trait_func_n)
    }
}

impl<'vt, ArgT, RetT, const TRAIT_N: usize, const FUNC_N: usize> Copy
    for MultiVPtr<'vt, ArgT, RetT, TRAIT_N, FUNC_N>
{
}
impl<'vt, ArgT, RetT, const TRAIT_N: usize, const FUNC_N: usize> Clone
    for MultiVPtr<'vt, ArgT, RetT, TRAIT_N, FUNC_N>
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<'vt, ArgT, RetT, const TRAIT_N: usize, const FUNC_N: usize> FuncTablePtr
    for MultiVPtr<'vt, ArgT, RetT, TRAIT_N, FUNC_N>
{
    type Arg = ArgT;
    type Ret = RetT;
    #[inline(always)]
    unsafe fn get_function(&self, n: u32) -> Function<Self> {
        let trait_n = n / (FUNC_N as u32);
        let trait_func_n = n % (FUNC_N as u32);
        self.get_trait_function(trait_n, trait_func_n)
    }
}
