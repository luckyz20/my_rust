mod zx_utils;

use zx_utils::http;

fn main() {
    let resp_get = http::http_get(&"https://httpbin.org/ip".to_string()).expect("failed");
    let resp_post = http::http_post(&"https://httpbin.org/post".to_string()).expect("failed");
    println!("Get: {:#?}", resp_get);
    println!("Post: {:#?}", resp_post);
}
