use std::env;
use std::process;

async fn fetch(uri: &String) {
    let client = reqwest::Client::new();
    let response = client.get(uri)
        .send().await;
    if response.is_ok() {
        let t = response.unwrap().text().await;
        if t.is_ok() {
            let text = t.unwrap();
            println!("text {}", text);
        } else {
            println!("text error");
        }
    } else {
        println!("response error");
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];

    let api_key = match env::var("API_KEY") {
        Ok(val) => val,
        Err(err) => {
            println!("{}: API_KEY", err);
            process::exit(1);
        },
    };
    let uri = format!("https://www.googleapis.com/youtube/v3/search?part=snippet&q={}&maxResults=1&key={}", query, api_key);
    let result = fetch(&uri);
    result.await;
}
