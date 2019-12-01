#![feature(const_generics)]

use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::{Path, PathBuf};
use std::result::Result;

pub trait Solution<const ID: u8> {
    // const ID: u8 = { N };
    fn solve(input: &Path) -> Result<(String, String), String>;
}

pub fn get_input_file<S: Solution<{ ID }>, const ID: u8>(resources_folder: &Path) -> PathBuf {
    let file = format!("day{:02}.input.txt", ID);
    resources_folder.join(file)
}

pub fn run<S: Solution<{ ID }>, const ID: u8>(resources: &Path) {
    println!("Day {}", ID);
    let input = get_input_file::<S, ID>(resources);
    match S::solve(&input) {
        Ok((a, b)) => {
            println!("Part A: {}", a);
            println!("Part B: {}", b);
        }
        Err(err) => {
            println!("ERROR: {}", err);
        }
    }
}

pub fn read_lines(path: &Path) -> Result<Lines<BufReader<File>>, std::io::Error> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);
    Ok(buf.lines())
}
