pub mod config;

#[cfg(feature = "with-pr47")] pub mod pr47;
#[cfg(feature = "with-rhai")] pub mod rhai;

fn main() {
    println!("Hello, world!");
}
