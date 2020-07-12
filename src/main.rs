extern crate raster;
use std::env;
mod interface;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    interface::interface(arguments);
}
