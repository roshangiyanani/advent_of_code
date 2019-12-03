#![feature(const_generics)]
#![feature(test)]

extern crate test;

mod lib;
use lib::run;

mod solutions;
use solutions::*;

use std::path::Path;

fn main() {
    let resources = Path::new("./resources");
    run::<Day01, 1>(&resources);
    run::<Day02, 2>(&resources);
}
