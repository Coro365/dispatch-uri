extern crate regex;

use regex::Regex;
use std::env;

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
}

fn bbb(url: &str) {
    println!("bbb:\t{}", url);
}

fn ccc(url: &str) {
    println!("ccc:\t{}", url);
}
