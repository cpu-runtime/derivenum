//! This library provides useful derive macros intended for enums.
//!
//! The main point of that is to reduce boilerplate code and add useful shortcuts.

extern crate convert_case;
extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

mod enum_match;
mod enum_take;

use crate::{
    enum_match::EnumMatch, enum_take::EnumTake, proc_macro::TokenStream, syn::parse_macro_input,
};

/// This derive macro adds matching functions for each of your enum's variants.
///
/// These functions have the signature `pub fn am_{enum_variant}(&self) -> bool` where `{enum_variant}` is to be replaced by each variant's name in `snake_case`.
/// The conversion to `snake_case` is done by the [`convert_case`] crate.
///
/// # Example
///
/// ```rust
/// use derivenum::EnumMatch;
///
/// #[derive(EnumMatch)]
/// enum MyEnum {
///     FirstVariant(Vec<i32>),
///     SecondVariant {
///         field: String,
///     },
///     ThirdVariant
/// }
///
/// let enum_variant = MyEnum::FirstVariant(vec![1, 2, 3]);
/// assert!(enum_variant.am_first_variant());
/// assert!(!enum_variant.am_second_variant());
/// assert!(!enum_variant.am_third_variant());
///
/// let enum_variant = MyEnum::SecondVariant {
///     field: String::from("123"),
/// };
/// assert!(!enum_variant.am_first_variant());
/// assert!(enum_variant.am_second_variant());
/// assert!(!enum_variant.am_third_variant());
///
/// let enum_variant = MyEnum::ThirdVariant;
/// assert!(!enum_variant.am_first_variant());
/// assert!(!enum_variant.am_second_variant());
/// assert!(enum_variant.am_third_variant());
/// ```
///
/// Add the arrtibute `#[enum_match(ignore)]` to any variant to skip adding a `am_...()` function:
///
/// ```rust,compile_fail
/// use derivenum::EnumMatch;
///
/// #[derive(EnumMatch)]
/// enum MyEnum {
///     #[enum_match(ignore)]
///     SkippedVariant(i32),
/// }
///
/// let variant = MyEnum::SkippedVariant(1);
/// // this should not compile, because the function does not exist:
/// assert!(variant.am_skipped_variant());
/// ```
#[proc_macro_derive(EnumMatch, attributes(enum_match))]
pub fn enum_match(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as EnumMatch).0
}

/// This derive macro adds take functions to your enum's variants to take the inner values out.
///
/// These functions have the signature `pub fn take_{enum_variant}(self) -> {return_type}` where `{enum_variant}` is to be replaced by the each unnamed enum variant in `snake_case`.
/// The conversion to `snake_case` is done by the [`convert_case`] crate.
/// In case the enum variant is a single unnamed one, `{return_type}` is to be replaced by the type inside the variant.
/// If the enum variant is unnamed, but has multiple fields, `{return_type}` is to be replaced by a tuple holding all types of the variant.
///
/// Named and Unit enum variants are skipped.
///
/// # Example
///
/// ```rust
/// use derivenum::EnumTake;
///
/// #[derive(EnumTake)]
/// enum MyEnum {
///     FirstVariant(Vec<i32>),
///     SecondVariant(u32, Vec<u32>, String),
///     // named variant is skipped
///     ThirdVariant {
///         field: String,
///     },
///     // unit variant is skipped
///     FourthVariant
/// }
///
/// let enum_variant = MyEnum::FirstVariant(vec![1, 2, 3]);
/// assert_eq!(enum_variant.take_first_variant(), vec![1, 2, 3]);
/// let enum_variant = MyEnum::SecondVariant(1, vec![1, 2, 3], String::from("alice"));
/// // note the tuple that encloses all types
/// assert_eq!(enum_variant.take_second_variant(), (1, vec![1, 2, 3], String::from("alice")));
/// ```
///
/// Add the arrtibute `#[enum_take(ignore)]` to any variant to skip adding a `take_...()` function:
///
/// ```rust,compile_fail
/// use derivenum::EnumTake;
///
/// #[derive(EnumTake)]
/// enum MyEnum {
///     #[enum_take(ignore)]
///     SkippedVariant(i32),
/// }
///
/// let variant = MyEnum::SkippedVariant(1);
/// // this should not compile, because the function does not exist:
/// let value = variant.take_skipped_variant();
/// ```
#[proc_macro_derive(EnumTake, attributes(enum_take))]
pub fn take_enum(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as EnumTake).0
}
