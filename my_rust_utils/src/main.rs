mod zx_utils;

use zx_utils::http;

fn main() {
  let resp = http::http_get(&"https://httpbin.org/ip".to_string()).expect("failed");
  println!("{:#?}", resp);
  
}



