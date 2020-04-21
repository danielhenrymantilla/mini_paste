#![no_std]
#![cfg_attr(feature = "nightly",
    feature(external_doc),
    doc(include = "../README.md"),
)]

extern crate proc_macro;

/** Not part of the public API **/ #[doc(hidden)]
pub use ::proc_macro::{
    item as __item__,
    __expr_hack__,
};

#[macro_export]
macro_rules! expr {(
    $($input:tt)*
) => ({
    #[derive($crate::__expr_hack__)]
    enum __mini_paste__Hack__ {
        __mini_paste__Hack__ = (stringify!($($input)*), 42).1
    }
    __mini_paste__Hack__!()
})}

#[macro_export]
macro_rules! item {(
    $($input:tt)*
) => (
    $crate::__as_item__! {
        $crate::__item__! {
            $($input)*
        }
    }
)}

/** Not part of the public API **/ #[doc(hidden)] #[macro_export]
macro_rules! __as_item__ {($item:item) => ($item)}
