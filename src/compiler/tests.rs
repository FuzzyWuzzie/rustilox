use super::compile;
use std::fs::File;
use std::io::prelude::*;

#[test]
fn test_arithmetic() {
    let src = "-5 + 5 * 2 / (1.05 + 1)";
    compile(&src.to_owned()).unwrap();
}

#[test]
fn scan_benchmark_binary_trees() {
    let mut f = File::open("test/benchmark/binary_trees.lox").unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    compile(&contents).unwrap();
}
