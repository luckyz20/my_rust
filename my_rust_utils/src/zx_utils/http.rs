use std::collections::HashMap;

use reqwest::header::HeaderMap;
use serde_json::Value;

/*
    @Desc: send http get request
        eg:
            let url = "https://httpbin.org/ip";
    @Return: json data
*/
pub fn http_get(url: &String) -> Result<HashMap<String, String>, reqwest::Error> {
    let resp = reqwest::blocking::get(url)?.json::<HashMap<String, String>>()?;
    return Ok(resp);
}

/*
    @Desc: send http post request
        eg:
            let url = "https://httpbin.org/post";
    @Return: json data
*/
pub fn http_post(url: &String) -> Result<HashMap<String, Value>, reqwest::Error> {
    //创建clien
    let client = reqwest::blocking::Client::new();

    //组装header
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    //组装提交的数据
    let mut data = HashMap::new();
    data.insert("delay", 1);

    //发起请求并返回
    let resp = client
        .post(url)
        .headers(headers)
        // .json(&data)
        .send()?
        .json::<HashMap<String, Value>>()?;

    Ok(resp)
}
