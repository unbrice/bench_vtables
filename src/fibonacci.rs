use crate::{
    inline_sieve_table::InlineSieveTable, multi_ptrs::MultiVTable,
    packed_sieve_table::PackedSieveTable, v_table::VTable, FuncTablePtr,
};

mod funcptrs {
    use crate::FuncTablePtr;

    /// Returns fib(n).
    pub fn fibonacci_00<I>(index: I, n: u64) -> u64
    where
        I: FuncTablePtr<Arg = u64, Ret = u64>,
    {
        match n {
            0 => {
                let fib_02 = unsafe { index.get_function(2) };
                fib_02(index, n)
            }
            1 => {
                let fib_04 = unsafe { index.get_function(4) };
                fib_04(index, n)
            }
            n => {
                let fib_06 = unsafe { index.get_function(6) };
                let fib_07 = unsafe { index.get_function(7) };
                 fib_06(index, n) + fib_07(index, n)
            }
        }
    }

    declare_filler!(fibonacci_01, u64, u64);

    /// Returns fib(0) == 0.
    pub fn fibonacci_02<I>(_index: I, _n: u64) -> u64
    where
        I: FuncTablePtr<Arg = u64, Ret = u64>,
    {
        0
    }

    declare_filler!(fibonacci_03, u64, u64);

    /// Returns fib(1) == 1.
    pub fn fibonacci_04<I>(_index: I, _n: u64) -> u64
    where
        I: FuncTablePtr<Arg = u64, Ret = u64>,
    {
        1
    }

    declare_filler!(fibonacci_05, u64, u64);

    /// Returns fib(n-1).
    pub fn fibonacci_06<I>(index: I, n: u64) -> u64
    where
        I: FuncTablePtr<Arg = u64, Ret = u64>,
    {
        let a0_00 = unsafe { index.get_function(0) };
        a0_00(index, n - 1)
    }

    /// Returns fib(n-2).
    pub fn fibonacci_07<I>(index: I, n: u64) -> u64
    where
        I: FuncTablePtr<Arg = u64, Ret = u64>,
    {
        let a0_00 = unsafe { index.get_function(0) };
        a0_00(index, n - 2)
    }
}

pub fn make_fibonacci_vtable<I>() -> VTable<I, 8>
where
    I: FuncTablePtr<Arg = u64, Ret = u64>,
{
    use funcptrs::*;
    VTable::new([
        fibonacci_00,
        fibonacci_01,
        fibonacci_02,
        fibonacci_03,
        fibonacci_04,
        fibonacci_05,
        fibonacci_06,
        fibonacci_07,
    ])
}

pub fn fibonacci<I>(index: I, n: u64) -> u64
where
    I: FuncTablePtr<Arg = u64, Ret = u64>,
{
    unsafe { index.get_function(0)(index, n) }
}

pub fn fibonacci_vtable(n: u64) -> u64 {
    use crate::v_table::*;
    let table = make_fibonacci_vtable();
    let ptr = VPtr::new(&table);
    fibonacci(ptr, n)
}

#[test]
fn fibonacci_vptr_test() {
    for n in 0..10 {
        assert!(fibonacci_vtable(n) == fibonacci_fast(n));
    }
}

pub fn make_fibonacci_multiptr<I, const TRAIT_N: usize, const FUNC_N: usize>(
) -> MultiVTable<I, TRAIT_N, FUNC_N>
where
    I: FuncTablePtr<Arg = u64, Ret = u64>,
{
    use crate::multi_ptrs::*;
    use funcptrs::*;
    MultiVTable::<I, TRAIT_N, FUNC_N>::new(&[
        fibonacci_00,
        fibonacci_01,
        fibonacci_02,
        fibonacci_03,
        fibonacci_04,
        fibonacci_05,
        fibonacci_06,
        fibonacci_07,
    ])
}

#[test]
fn fibonacci_multiptr_test() {
    fn fibonacci_multiptr<const TRAIT_N: usize, const FUNC_N: usize>(n: u64) -> u64 {
        use crate::multi_ptrs::MultiVPtr;

        let table = make_fibonacci_multiptr::<_, TRAIT_N, FUNC_N>();
        let ptr = MultiVPtr::new(&table);
        fibonacci(ptr, n)
    }

    for n in 0..10 {
        assert!(fibonacci_multiptr::<1, 10>(n) == fibonacci_fast(n));
        assert!(fibonacci_multiptr::<5, 2>(n) == fibonacci_fast(n));
    }
}

pub fn make_fibonacci_inline_sieve<I>() -> InlineSieveTable<I, 12>
where
    I: FuncTablePtr<Arg = u64, Ret = u64>,
{
    use funcptrs::*;
    InlineSieveTable::new([
        Some(fibonacci_00),
        None,
        Some(fibonacci_01),
        Some(fibonacci_02),
        Some(fibonacci_03),
        None,
        Some(fibonacci_04),
        None,
        Some(fibonacci_05),
        None,
        Some(fibonacci_06),
        Some(fibonacci_07),
    ])
}

#[test]
fn fibonacci_inline_sieve_test() {
    pub fn fibonacci_inline_sieve(n: u64) -> u64 {
        use crate::inline_sieve_table::InlineSievePtr;
        let table = make_fibonacci_inline_sieve();
        let ptr = InlineSievePtr::new(&table);
        fibonacci(ptr, n)
    }

    for n in 0..10 {
        assert!(fibonacci_inline_sieve(n) == fibonacci_fast(n));
    }
}

pub fn make_fibonacci_packed_sieve<I, const TRAIT_N: usize, const FUNC_N: usize>(
) -> PackedSieveTable<I, TRAIT_N, FUNC_N>
where
    I: FuncTablePtr<Arg = u64, Ret = u64>,
{
    use funcptrs::*;
    PackedSieveTable::<I, TRAIT_N, FUNC_N>::new(&[
        fibonacci_00,
        fibonacci_01,
        fibonacci_02,
        fibonacci_03,
        fibonacci_04,
        fibonacci_05,
        fibonacci_06,
        fibonacci_07,
    ])
}

#[test]
fn fibonacci_sieve_test() {
    fn fibonacci_packed_sieve<const TRAIT_N: usize, const FUNC_N: usize>(n: u64) -> u64 {
        use crate::packed_sieve_table::PackedSievePtr;

        let table = make_fibonacci_packed_sieve::<_, TRAIT_N, FUNC_N>();
        let ptr = PackedSievePtr::new(&table);
        fibonacci(ptr, n)
    }
    for n in 0..10 {
        assert!(fibonacci_packed_sieve::<1, 10>(n) == fibonacci_fast(n));
        assert!(fibonacci_packed_sieve::<4, 3>(n) == fibonacci_fast(n));
        assert!(fibonacci_packed_sieve::<5, 2>(n) == fibonacci_fast(n));
    }
}

#[cfg(test)]
fn fibonacci_fast(n: u64) -> u64 {
    if n == 1 {
        return 1;
    }

    let mut sum = 0;
    let mut last = 0;
    let mut curr = 1;
    for _ in 1..n {
        sum = last + curr;
        last = curr;
        curr = sum;
    }
    sum
}
