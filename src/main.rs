use std::io;
use std::env;
use std::fs::*;
use std::io::Read;

mod enumeration;
//mod subdomains;

#[tokio::main]
async fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 4 {
	eprintln!("ERROR: <executable> <url> <path-to-wordlist> <options> <extension/crawl>");
	return Ok(())
    }

    let url = args[1].clone();
    let wordlist = args[2].clone();
    let mut extension = String::from("");
    let option = args[3].clone(); // dir sub/vhost
    if args.len() == 5 {
	extension = args[4].clone();
    }
    let mut file = File::open(wordlist).expect("file not found");
    let mut wordlist_contents = String::new();
    file.read_to_string(&mut wordlist_contents).expect("Faield to read file into buffer");

    if option == "dir" {
	enumeration::directories(&wordlist_contents, &url, extension).await;
    }
    if option == "sub" {
	enumeration::sub_domains(&wordlist_contents, &url).await;
    }
    Ok(())
}
