/// The V-Ptr or Sieve-Ptr or Extra-Fat-Ptr we are benching.
///
/// For simplicity all functions have the same type.
pub trait FuncTablePtr: Copy {
    type Arg;
    type Ret;
    /// Returns the `n`-th function from the pointer.
    ///
    /// # Safety
    ///
    /// Behavior is undefined if the pointer knows about fewer than `n` functions.
    unsafe fn get_function(&self, n: u32) -> fn(Self, Self::Arg) -> Self::Ret;
}

pub trait FuncTable: Copy {
    type Ptr: FuncTablePtr;
    /// Returns the `n`-th function from the table.
    ///
    /// # Safety
    ///
    /// Behavior is undefined if the table knows about fewer than `n` functions.
    unsafe fn get_function(&self, n: u32) -> Function<Self::Ptr>;
}

// A function for a given pointer type.
type Function<P> = fn(P, <P as FuncTablePtr>::Arg) -> <P as FuncTablePtr>::Ret;

/// Panics when called.
///
/// Used a placeholder when a `Function` is needed.
pub fn filler_function<I: FuncTablePtr>(_: I, _: I::Arg) -> I::Ret {
    panic!("filler function should never be called");
}

/// Declares a function that panics when called.
///
/// Used a placeholder when a `Function` is needed.
macro_rules! declare_filler {
    ($name:ident, $arg:ty, $ret:ty) => {
        /// A filler function that panics.
        pub fn $name<P>(_: P, _: P::Arg) -> P::Ret
        where
            P: $crate::FuncTablePtr<Arg = $arg, Ret = $ret>,
        {
            panic!(
                "filler function `{}` should not be called",
                stringify!($name)
            );
        }
    };
}

pub mod fibonacci;
pub mod inline_sieve_table;
pub mod multi_ptrs;
pub mod packed_sieve_table;
mod select_bit;
pub mod v_table;
