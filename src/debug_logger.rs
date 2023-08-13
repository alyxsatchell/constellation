use std::fs::OpenOptions;
use std::io::Write;

pub fn debug_log(message: &str){
    let mut f = OpenOptions::new().create(true).append(true).open("data/debug/log.txt").expect("Failed To Open File");
    write!(f, "{}\n", message).expect("Failed To Write To Debug Log");
}