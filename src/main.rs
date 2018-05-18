extern crate direct2d;
extern crate winapi;
#[macro_use]
extern crate lazy_static;

mod helpers;
mod gui;
mod examples;

fn main() {
    helpers::start_loop().unwrap();
}

