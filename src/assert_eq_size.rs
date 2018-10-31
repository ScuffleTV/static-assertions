/// Asserts that types are equal in size.
///
/// When performing operations such as pointer casts or dealing with [`usize`]
/// versus [`u64`] versus [`u32`], the size of your types matter. This is where
/// this macro comes into play.
///
/// # Alternatives
///
/// There are also [`assert_eq_size_val`](macro.assert_eq_size_val.html) and
/// [`assert_eq_size_ptr`](macro.assert_eq_size_ptr.html). Instead of specifying
/// types to compare, values' sizes can be directly compared against each other.
///
/// # Examples
///
/// ```
/// # #[macro_use]
/// # extern crate static_assertions;
/// // Can be declared outside of a function if labeled
/// assert_eq_size!(bytes; (u8, u8), u16);
///
/// fn main() {
///     // Supports unlimited arguments:
///     assert_eq_size!([u8; 4], (u16, u16), u32);
/// }
/// ```
///
/// The following produces a compilation failure because `u32` has 4 times the
/// size of `u8`:
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions;
/// # fn main() {
/// assert_eq_size!(u32, u8);
/// # }
/// ```
///
/// [`usize`]: https://doc.rust-lang.org/std/primitive.usize.html
/// [`u64`]: https://doc.rust-lang.org/std/primitive.u64.html
/// [`u32`]: https://doc.rust-lang.org/std/primitive.u32.html
#[macro_export]
macro_rules! assert_eq_size {
    ($x:ty, $($xs:ty),+ $(,)*) => {
        $(let _ = $crate::_core::mem::transmute::<$x, $xs>;)+
    };
    ($label:ident; $($xs:tt)+) => {
        #[allow(dead_code, non_snake_case)]
        fn $label() { assert_eq_size!($($xs)+); }
    };
}

/// Asserts that values pointed to are equal in size.
///
/// This especially is useful for when coercing pointers between different types
/// and ensuring the underlying values are the same size.
///
/// # Examples
///
/// ```
/// # #[macro_use]
/// # extern crate static_assertions;
/// fn operation(x: &(u32, u32), y: &[u16; 4]) {
///     assert_eq_size_ptr!(x, y);
/// }
/// # fn main() {}
/// ```
///
/// Byte arrays of different lengths have different sizes:
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions;
/// # fn main() {
/// static BYTES: &[u8; 4] = &[
///     /* ... */
///     # 0; 4
/// ];
///
/// static TABLE: &[u8; 16] = &[
///     /* ... */
///     # 0; 16
/// ];
///
/// assert_eq_size_ptr!(BYTES, TABLE);
/// # }
/// ```
#[macro_export]
macro_rules! assert_eq_size_ptr {
    ($x:expr, $($xs:expr),+ $(,)*) => {
        #[allow(unknown_lints, unsafe_code, forget_copy, useless_transmute)]
        let _ = || unsafe {
            use $crate::_core::{mem, ptr};
            let mut copy = ptr::read($x);
            $(ptr::write(&mut copy, mem::transmute(ptr::read($xs)));)+
            mem::forget(copy);
        };
    }
}

/// Asserts that values are equal in size.
///
/// This macro doesn't consume its arguments and thus works for
/// non-[`Clone`]able values.
///
/// # Examples
///
/// ```
/// # #[macro_use]
/// # extern crate static_assertions;
/// # fn main() {
/// struct Byte(u8);
///
/// let x = 10u8;
/// let y = Byte(42); // Works for non-cloneable types
///
/// assert_eq_size_val!(x, y);
/// assert_eq_size_val!(x, y, 0u8);
/// # }
/// ```
///
/// Even though both values are 0, they are of types with different sizes:
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions;
/// # fn main() {
/// assert_eq_size_val!(0u8, 0u32);
/// # }
/// ```
///
/// [`Clone`]: https://doc.rust-lang.org/std/clone/trait.Clone.html
#[macro_export]
macro_rules! assert_eq_size_val {
    ($x:expr, $($xs:expr),+ $(,)*) => {
        assert_eq_size_ptr!(&$x, $(&$xs),+);
    }
}
