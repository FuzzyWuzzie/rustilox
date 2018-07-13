use super::compile;
use chunk::Chunk;
use std::fs::File;
use std::io::prelude::*;

#[test]
fn test_arithmetic() {
    let src = "-5 + 5 * 2 / (1.05 + 1)";
    let mut chunk = Chunk::init();
    compile(&src.to_owned(), &mut chunk).unwrap();
}

#[test]
fn scan_benchmark_binary_trees() {
    let mut f = File::open("test/benchmark/binary_trees.lox").unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    let mut chunk = Chunk::init();
    compile(&contents, &mut chunk).unwrap();
}
