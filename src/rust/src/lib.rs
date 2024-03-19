use extendr_api::prelude::*;

/// Return 42 to R
/// @param y A list parameter
/// @export
#[extendr]
fn add(y: Robj) -> i32 {
    let z = y.as_list().unwrap().len();
    rprintln!("{z}");
    rprintln!("hello");
    42
}

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod dapifer;
    fn hello_world;
    fn add;
}
