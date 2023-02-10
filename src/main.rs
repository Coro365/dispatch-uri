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
            n if aaa_reg.is_match(n) => add_uri(n),
            n if bbb_reg.is_match(n) => add_uri(n),
            n if ccc_reg.is_match(n) => add_uri(n),
            n => println!("ERROR:\t{}", n),
        }
    }
}

fn add_uri(url: &str) {
    println!("{}", url);
    let filename = to_filename(url);

    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)
        .expect("error");

    let mut buf_writer = BufWriter::new(file);
    writeln!(buf_writer, "{}", url).expect("error")
}

fn to_filename(url: &str) -> String {
    let filename: Vec<&str> =  url.split("/").collect();
    let filename = filename[2].replace(".", "_");
    return [filename, "txt".to_string()].join(".");
}

