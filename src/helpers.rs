use std::error::{Error};
use std::io::{Read, Write};
use std::net::TcpStream;

fn make_request(method: &str, path: &str, body: Option<&str>, target: &str, extra_headers: Option<Vec<String>>) -> Result<String, Box<dyn Error>> {
    let mut headers = vec![
        format!("{} {} HTTP/1.1", method, path),
        format!("Host: {}", target.to_string()),
        "Connection: close".to_string(),
    ];

    if let Some(head) = extra_headers {
        for item in head {
            headers.push(item);
        }
    }

    let body_str = match body {
        Some(body) => {
            headers.push("Content-Type: application/json".to_string()); // Changed to JSON content type
            headers.push(format!("Content-Length: {}", body.len()));
            body
        },
        None => {
            ""
        }
    };

    let request_data = format!(
        "{}\r\n\r\n{}",
        headers.join("\r\n"),
        body_str
    );

    let response = {
        let mut stream = TcpStream::connect(target)?;
        stream.write_all(request_data.as_bytes())?;
        let mut response = String::new();
        stream.read_to_string(&mut response)?;
        response
    };
    Ok(response)
}

pub fn make_get_request(url: &str, extra_headers: Option<Vec<String>>) -> Result<String, Box<dyn Error>> {
    make_request("GET", "/get", None, url, extra_headers)
}

pub fn make_post_request(url: &str, data: &str, extra_headers: Option<Vec<String>>) -> Result<String, Box<dyn Error>> {
    make_request("POST", "/post", Some(data), url, extra_headers)
}

