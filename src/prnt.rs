use rand::Rng;
use regex::Regex;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
pub fn random_url() -> String {
    let base = String::from("https://prnt.sc");
    let num = rand::thread_rng().gen_range(100000..999999);
    let new_url = format!("{}/{}", base, num);
    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static(
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:109.0) Gecko/20100101 Firefox/109.0",
        ),
    );
    let html = reqwest::blocking::Client::new()
        .get(new_url)
        .headers(headers)
        .send()
        .unwrap()
        .text()
        .unwrap();
    let re = Regex::new(r"https://i\.imgur\.com/(.*)").unwrap();
    let capt = re.captures(html.as_str()).unwrap();
    let mut y = capt[0].split(" ").next().unwrap().trim().to_string();
    y.drain(y.len() - 3..);
    y
}
