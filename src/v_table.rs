use crate::{filler_function, FuncTable, FuncTablePtr, Function};

pub struct VTable<Ptr: FuncTablePtr, const N: usize> {
    funcs: [Function<Ptr>; N],
}

impl<Ptr: FuncTablePtr, const N: usize> VTable<Ptr, N> {
    pub fn new(funcs: [Function<Ptr>; N]) -> Self {
        Self { funcs }
    }
    pub fn new_with_filler() -> Self {
        Self::new([filler_function; N])
    }
    pub fn new_from_slice(funcs: &[Function<Ptr>]) -> Self {
        assert!(
            funcs.len() <= N,
            "slice has size {}, maximum is {}",
            funcs.len(),
            N
        );
        let mut array: [Function<Ptr>; N] = [filler_function; N];
        array[..funcs.len()].copy_from_slice(funcs);
        Self::new(array)
    }
}

impl<Ptr: FuncTablePtr, const N: usize> Copy for VTable<Ptr, N> {}
impl<Ptr: FuncTablePtr, const N: usize> Clone for VTable<Ptr, N> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<PtrT: FuncTablePtr, const N: usize> FuncTable for VTable<PtrT, N> {
    type Ptr = PtrT;
    #[inline(always)]
    unsafe fn get_function(&self, n: u32) -> Function<Self::Ptr> {
        let func_ptr = self.funcs.get_unchecked(n as usize);
        *func_ptr
    }
}
pub struct VPtr<'vt, ArgT, RetT, const N: usize> {
    table: &'vt VTable<Self, N>,
}

impl<'vt, ArgT, RetT, const N: usize> VPtr<'vt, ArgT, RetT, N> {
    pub fn new(table: &'vt VTable<Self, N>) -> Self {
        Self { table }
    }
}

impl<'vt, ArgT, RetT, const N: usize> Copy for VPtr<'vt, ArgT, RetT, N> {}
impl<'vt, ArgT, RetT, const N: usize> Clone for VPtr<'vt, ArgT, RetT, N> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'vt, ArgT, RetT, const N: usize> FuncTablePtr for VPtr<'vt, ArgT, RetT, N> {
    type Arg = ArgT;
    type Ret = RetT;
    #[inline(always)]
    unsafe fn get_function(&self, n: u32) -> Function<Self> {
        self.table.get_function(n)
    }
}
