use reqwest;

pub async fn directories(wordlist: &String, url: &String) {
    let http = String::from("http://");
    for line in wordlist.lines() {
	let result = reqwest::get(http.clone() + &url.clone() + line).await.expect("failed to issue get request");
	let status_code = result.status();
	if status_code == 200 {
	    println!("http://{url}{line}: Status_Code:{status_code}");
	}
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
