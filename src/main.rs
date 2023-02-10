extern crate regex;

use regex::Regex;
use std::env;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    let aaa_reg = Regex::new(r"http(s)://aaa.org/(.*)$").unwrap();
    let bbb_reg = Regex::new(r"http(s)://bbb.org/users/(\d*)$").unwrap();
    let ccc_reg = Regex::new(r"http(s)://ccc.org/(.*)$").unwrap();

    for url in args.iter(){
        match url.as_str() {
            n if aaa_reg.is_match(n) => aaa(n),
            n if bbb_reg.is_match(n) => bbb(n),
            n if ccc_reg.is_match(n) => ccc(n),
            n => println!("ERROR:\t{}", n),
        }
    }
}

fn aaa(url: &str) {
    println!("aaa:\t{}", url);
    add_uri(url)
}

fn bbb(url: &str) {
    println!("bbb:\t{}", url);
    add_uri(url)
}

fn ccc(url: &str) {
    println!("ccc:\t{}", url);
    add_uri(url)
}

fn add_uri(url: &str) {
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("./test.txt")
        .expect("error");

    let mut buf_writer = BufWriter::new(file);
    writeln!(buf_writer, "{}", url).expect("error")
}
