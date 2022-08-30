use std::env;

fn main() {
    let api_token = env::var("API_TOKEN").expect("Expected to be an api token.");

    // let api_token = env::var("API_TOKEN");

    dbg!(&api_token);

    let mut arg_iterator = env::args();

    arg_iterator.next();

    let args: String = arg_iterator.collect();

    dbg!(&args);

    let client = reqwest::blocking::Client::new();

    let response = client
        .get("https://api.waqi.info/search/")
        .query(&[("token", &api_token), ("keyword", &args)])
        .send()
        .expect("a successful request")
        .json::<serde_json::Value>()
        .expect("expected the body to be json");

    dbg!(&response);
}
