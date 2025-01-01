use reqwest;
use std::fs;
use std::fs::*;

pub async fn directories(wordlist: &String, url: &mut String, extension: String) {
    let mut crawl_urls: Vec<String> = Vec::new();
    let mut directories = String::new();
    if !url.contains("http://") && !url.contains("https://") {
	eprintln!("<ERROR> Invalid url entered make sure it contains http:// or https://");
    }
    if extension != "crawl" {
	// extensions are split by , (i.e) .py,.js,.php,.phtml
	let extension: Vec<&str> = extension.split(",").collect();
	for ext in extension {
	    for line in wordlist.lines() {
		let result = reqwest::get(url.clone() + line + ext).await.expect("failed to issue get request");
		let status_code = result.status();
		if status_code == 200 {
		    println!("{url}{line}{ext}: Status_Code:{status_code}");
		    directories.push_str(&(url.clone() + line + ext));
		}
	    }
	}
    } else {
	for line in wordlist.lines() {
	    let result = reqwest::get(url.clone() + line).await.expect("failed to issue get request");
	    let status_code = result.status();
	    if status_code == 200 {
		println!("{url}{line}: Status_Code:{status_code}");
		crawl_urls.push(url.clone() + line + "/");
		directories.push_str(&(url.clone() + line + "/"));
	    }
	}
	// url crawling, will make better later, just first try.
	for crawl_url in crawl_urls {
	    for line in wordlist.lines() {
		let result = reqwest::get(crawl_url.clone() + line).await.expect("failed to issue get request");
		let status_code = result.status();
		if status_code == 200 {
		    println!("{crawl_url}{line}: Status_Code:{status_code}");
		    directories.push_str(&(crawl_url.clone() + line + "/"));
		}
	    }
	}
    }
    File::create("directories.txt").expect("Failed to create file");
    fs::write("directories.txt", directories).expect("Failed to write content.");
}

pub async fn sub_domains(wordlist: &String, url: &mut String) {
    let mut http = String::new();
    // removing the http protocol prefix from url.
    // wish i could use string remove_match, but its only in the nightlet compiler XD
    if url.contains("http://") {
	let mut i: usize = 0;
	while i < 7 {
	    let _black = &url.remove(i);
	    i += 1;
	}
	http = String::from("http://");
    } else if url.contains("https://") {
	let mut i: usize = 0;
	while i <= 7 {
	    let _black = &url.remove(i);
	    i += 1;
	}
	http = String::from("http://");
    } else {
	eprintln!("<ERROR> Invalid url entered make sure it contains http:// or https://");
    }
    let mut subdomains = String::new();
    let period = String::from(".");
    for line in wordlist.lines() {
	let result = reqwest::get(http.clone() + line + &period + &url.clone()).await.expect("failed to issue get request");
	let status_code = result.status();
	if status_code == 200 {
	    println!("http://{line}.{url}: Status_Code:{status_code}");
	    subdomains.push_str(&(line.to_owned() + &period + &url.clone()));
	}
    }
    File::create("subs.txt").expect("Failed to create file");
    fs::write("subs.txt", subdomains).expect("Failed to write content.");

}
