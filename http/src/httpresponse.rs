use std::{collections::HashMap, hash::Hash, io::Write};

// HTTP/1.1 200 OK
//headers
//body
//=======================
//Default trait
//new()
//send_response
//getter
//From trait

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1".into(),
            status_code: "200".into(),
            status_text: "OK".into(),
            headers: None,
            body: None,
        }
    }
}

impl<'a> HttpResponse<'a> {
    //new
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> HttpResponse<'a> {
        let mut response = HttpResponse::default();

        if status_code != "200" {
            response.status_code = status_code;
        }
        response.headers = match &headers {
            Some(data) => headers,
            None => {
                let mut m = HashMap::new();
                m.insert("Content-Type", "text/html");
                Some(m)
            }
        };

        response.status_text = match response.status_code {
            "200" => "OK",
            "400" => "Bad Request",
            "404" => "Not Found",
            "500" => "Internal Server Error",
            _ => "Not Found",
        };

        response.body = body;
        response
    }

    //send_response
    pub fn send_response(&self, write_stream: &mut impl Write) {
        let res = self.clone();
        let response_str = String::from(res);
        write!(write_stream, "{}", response_str);

        // Ok(())
    }

    //getter
    pub fn version(&self) -> &str {
        self.version
    }
    pub fn status_code(&self) -> &str {
        self.status_code
    }
    pub fn status_text(&self) -> &str {
        self.status_text
    }

    pub fn headers(&self) -> String {
        let map = self.headers.clone().unwrap();
        let mut headers_str = "".into();
        for (k, v) in map.iter() {
            headers_str = format!("{}{}:{}\r\n", headers_str, k, v);
        }
        headers_str
    }

    pub fn body(&self) -> &str {
        match &self.body {
            Some(data) => data.as_str(),
            None => "",
        }
    }
}

//From trait
impl<'a> From<HttpResponse<'a>> for String {
    fn from(res: HttpResponse) -> String {
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            res.version(),
            res.status_code(),
            res.status_text(),
            res.headers(),
            res.body().len(),
            res.body(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_response_from() {
        let expected_res_str_200 = String::from(
            "HTTP/1.1 200 OK\r\nContetn-Type: text/html\r\nContent-Length: 8\r\n\r\nzhangsan",
        );
        let expected_res_str_404 = String::from(
            "HTTP/1.1 404 Not Found\r\nContetn-Type: text/html\r\nContent-Length: 8\r\n\r\nzhangsan",
        );
        let mut headers = HashMap::new();
        headers.insert("Contetn-Type", " text/html");
        let res = HttpResponse::new("404", Some(headers), Some("zhangsan".to_string()));
        // let res_str = String::from(res);
        let res_str2: String = res.into();
        // assert_eq!(expected_res_str_200, res_str);
        assert_eq!(expected_res_str_404, res_str2);
    }

    #[test]
    fn test_http_response_creation_200() {
        let res1 = HttpResponse::new("200", None, Some("zhangsan".to_string()));

        let expected_res = HttpResponse {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: {
                let mut map = HashMap::new();
                map.insert("Content-Type", "text/html");
                Some(map)
            },
            body: Some("zhangsan".to_string()),
        };

        assert_eq!(expected_res, res1);
    }

    #[test]
    fn test_http_response_creation_404() {
        let res1 = HttpResponse::new("404", None, Some("zhangsan".to_string()));

        let expected_res = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut map = HashMap::new();
                map.insert("Content-Type", "text/html");
                Some(map)
            },
            body: Some("zhangsan".to_string()),
        };

        assert_eq!(expected_res, res1);
    }
}
