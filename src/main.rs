#![feature(const_generics)]

mod lib;
use lib::run;

mod solutions;
use solutions::*;

use std::path::Path;

fn main() {
    let resources = Path::new("./resources");
    run::<Day01, 1>(&resources);
}
