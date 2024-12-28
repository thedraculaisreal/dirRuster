use reqwest;
use std::io;
use std::env;
use std::fs::*;
use std::io::Read;
use std::ops::Add;

#[tokio::main]
async fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let url = args[1].clone();
    let wordlist = args[2].clone();
    let mut file = File::open(wordlist).expect("file not found");
    let mut wordlist_contents = String::new();
    file.read_to_string(&mut wordlist_contents).expect("Faield to read file into buffer");
    for line in wordlist_contents.lines() {
	let result = reqwest::get(&url.clone().add(&line)).await.expect("failed to issue get request");
	let mut status_code = result.status();
	if status_code == 200 {
	    println!("{url}{line}: Status_Code:{status_code}");
	}
    }
    Ok(())
}
