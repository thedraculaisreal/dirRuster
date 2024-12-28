use reqwest;

pub async fn directories(wordlist: &String, url: &String, extension: String) {
    let http = String::from("http://");
    let mut crawl_urls: Vec<String> = Vec::new();
    let mut ext = String::new();
    match extension.as_str() {
	".php" => ext = ".php".to_string(),
	".html" => ext = ".html".to_string(),
	".txt" => ext = ".txt".to_string(),
	_ => ext = "".to_string()
    }
    for line in wordlist.lines() {
	let result = reqwest::get(http.clone() + &url.clone() + line + &ext.clone()).await.expect("failed to issue get request");
	let status_code = result.status();
	if status_code == 200 {
	    println!("http://{url}{line}{ext}: Status_Code:{status_code}");
	    if ext.len() > 2 {
		continue;
	    }
	    crawl_urls.push(http.clone() + &url.clone() + line);
	}
    }
    for url in crawl_urls {
	println!("{url}");
    }
}

pub async fn sub_domains(wordlist: &String, url: &String) {
    let http = String::from("http://");
    let period = String::from(".");
    for line in wordlist.lines() {
	let result = reqwest::get(http.clone() + line + &period + &url.clone()).await.expect("failed to issue get request");
	let status_code = result.status();
	if status_code == 200 {
	    println!("http://{line}.{url}: Status_Code:{status_code}");
	}
    }
}
