use crate::{filler_function, FuncTablePtr, Function};

pub struct InlineSieveTable<Ptr: FuncTablePtr, const N: usize> {
    funcs: [Function<Ptr>; N],
    /// A default sieve pointer for sieves.
    ///
    /// The real SieveTable would not contain a sieve (only pointers). But this makes the simulation
    /// a bit more realistic. We the bit will be 0 where None was passed to `SieveTable::new()`.
    default_sieve: usize,
}

impl<Ptr: FuncTablePtr, const N: usize> InlineSieveTable<Ptr, N> {
    pub fn new(opt_funcs: [Option<Function<Ptr>>; N]) -> Self {
        let funcs = opt_funcs.map(|of| of.unwrap_or(filler_function));
        let mut default_sieve: usize = 0;
        for (n, f) in opt_funcs.iter().enumerate() {
            if f.is_some() {
                default_sieve |= 1 << n;
            }
        }
        Self {
            funcs,
            default_sieve,
        }
    }
}

pub struct InlineSievePtr<'vt, ArgT, RetT, const N: usize> {
    sieve: usize,
    funcs: &'vt [Function<Self>; N],
}

impl<'vt, ArgT, RetT, const N: usize> InlineSievePtr<'vt, ArgT, RetT, N> {
    pub fn new(sieve_table: &'vt InlineSieveTable<Self, N>) -> Self {
        Self {
            sieve: sieve_table.default_sieve,
            funcs: &sieve_table.funcs,
        }
    }
}

impl<'vt, ArgT, RetT, const N: usize> Copy for InlineSievePtr<'vt, ArgT, RetT, N> {}
impl<'vt, ArgT, RetT, const N: usize> Clone for InlineSievePtr<'vt, ArgT, RetT, N> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'vt, ArgT, RetT, const N: usize> FuncTablePtr for InlineSievePtr<'vt, ArgT, RetT, N> {
    type Arg = ArgT;
    type Ret = RetT;
    #[inline(always)]
    unsafe fn get_function(&self, n: u32) -> Function<Self> {
        let offset = crate::select_bit::find_nth_set_bit(self.sieve, n);
        let ptr = self.funcs.get_unchecked(offset as usize);
        *ptr
    }
}
