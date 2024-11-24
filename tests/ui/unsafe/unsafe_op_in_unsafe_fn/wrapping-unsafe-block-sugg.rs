//@ run-rustfix
//@ aux-build:external_unsafe_macro.rs

#![deny(unsafe_op_in_unsafe_fn)] //~ NOTE
#![crate_name = "wrapping_unsafe_block_sugg"]

extern crate external_unsafe_macro;

unsafe fn unsf() {}

pub unsafe fn foo() {
    //~^ NOTE an unsafe function restricts its caller, but its body is safe by default
    unsf(); //~ ERROR call to unsafe function `unsf` is unsafe
    //~^ NOTE call to unsafe function
    //~| NOTE for more information, see issue #71668
    //~| NOTE consult the function's documentation
    unsf(); //~ ERROR call to unsafe function `unsf` is unsafe
    //~^ NOTE call to unsafe function
    //~| NOTE for more information, see issue #71668
    //~| NOTE consult the function's documentation
}

pub unsafe fn bar(x: *const i32) -> i32 {
    //~^ NOTE an unsafe function restricts its caller, but its body is safe by default
    let y = *x; //~ ERROR dereference of raw pointer is unsafe and requires unsafe block
    //~^ NOTE dereference of raw pointer
    //~| NOTE for more information, see issue #71668
    //~| NOTE raw pointers may be null
    y + *x //~ ERROR dereference of raw pointer is unsafe and requires unsafe block
    //~^ NOTE dereference of raw pointer
    //~| NOTE for more information, see issue #71668
    //~| NOTE raw pointers may be null
}

static mut BAZ: i32 = 0;
pub unsafe fn baz() -> i32 {
    //~^ NOTE an unsafe function restricts its caller, but its body is safe by default
    let y = BAZ; //~ ERROR use of mutable static is unsafe and requires unsafe block
    //~^ NOTE use of mutable static
    //~| NOTE for more information, see issue #71668
    //~| NOTE mutable statics can be mutated by multiple threads
    let z = y + BAZ; //~ ERROR use of mutable static is unsafe and requires unsafe block
    //~^ NOTE use of mutable static
    //~| NOTE for more information, see issue #71668
    //~| NOTE mutable statics can be mutated by multiple threads
    z + bar(BAZ as *const _) //~ ERROR call to unsafe function `bar` is unsafe
    //~^ NOTE call to unsafe function
    //~| NOTE for more information, see issue #71668
    //~| NOTE consult the function's documentation
    //~| ERROR use of mutable static is unsafe and requires unsafe block
    //~| NOTE use of mutable static
    //~| NOTE for more information, see issue #71668
    //~| NOTE mutable statics can be mutated by multiple threads
}

macro_rules! unsafe_macro_unsf { () => (unsf()) }
//~^ ERROR call to unsafe function `unsf` is unsafe
//~| NOTE call to unsafe function
//~| NOTE for more information, see issue #71668
//~| NOTE consult the function's documentation
//~| ERROR call to unsafe function `unsf` is unsafe
//~| NOTE call to unsafe function
//~| NOTE for more information, see issue #71668
//~| NOTE consult the function's documentation

// macro_rules! unsafe_macro_multi {
//     ($x:expr) => {{
//         unsf();
//         unsafe_macro_unsf!();
//         $x + bar(BAZ as *const _)
//     }}
// }

pub unsafe fn unsafe_in_macro() {
    //~^ NOTE an unsafe function restricts its caller, but its body is safe by default
    unsafe_macro_unsf!();
    //~^ NOTE in this expansion
    //~| NOTE in this expansion
    //~| NOTE in this expansion
    unsafe_macro_unsf!();
    //~^ NOTE in this expansion
    //~| NOTE in this expansion
    //~| NOTE in this expansion

    // let _ = unsafe_macro_multi!(unsafe_macro_multi!(BAZ));
}

pub unsafe fn unsafe_in_external_macro() {
    // FIXME: https://github.com/rust-lang/rust/issues/112504
    // FIXME: ~^ NOTE an unsafe function restricts its caller, but its body is safe by default
    external_unsafe_macro::unsafe_macro!();
    external_unsafe_macro::unsafe_macro!();
}

pub unsafe fn incomplete_unsafe_op_in_unsafe_fn() {
    //~^ NOTE an unsafe function restricts its caller, but its body is safe by default
    unsafe { unsf(); }
    unsf();
    //~^ ERROR call to unsafe function `unsf` is unsafe
    //~| NOTE call to unsafe function
    //~| NOTE for more information, see issue #71668
    //~| NOTE consult the function's documentation
    unsafe { let _ = unsf(); }
    let _ = unsf();
    //~^ ERROR call to unsafe function `unsf` is unsafe
    //~| NOTE call to unsafe function
    //~| NOTE for more information, see issue #71668
    //~| NOTE consult the function's documentation
}

fn main() {}
