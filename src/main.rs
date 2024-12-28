use reqwest;
use std::io;
use std::env;

#[tokio::main]
async fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let url = args[1].clone();
    let mut i: u8 = 1;
    while i < 100 {
	let result = reqwest::get(&url).await.expect("failed to issue get request");
	println!("{result:?}");
	i += 1;
    }
    Ok(())
}
