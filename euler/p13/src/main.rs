#![feature(iter_arith)]
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut f = File::open("numbers.txt").unwrap(); 
    let mut data = String::new();
    f.read_to_string(&mut data).unwrap();
    let sum = data.trim()
        .lines()
        .map(|line| &line[0 .. 12])
        .filter_map(|s| s.parse::<u64>().ok())
        .sum::<u64>()
        .to_string()[0 .. 10]
        .to_string();
    println!("{}",sum);

}
