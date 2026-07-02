/// HTTP transport for MCP server.
/// Simple HTTP/1.1 server using only std (zero dependencies).
/// Enables Claude web interface to connect via custom connector.
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, BufRead, BufReader};
use std::thread;

use crate::inference::InferenceEngine;
use super::server::handle_message;

pub struct HttpServer {
    addr: String,
}

impl HttpServer {
    pub fn new(addr: &str) -> Self {
        HttpServer {
            addr: addr.to_string(),
        }
    }

    pub fn run(&self) -> Result<(), String> {
        let listener = TcpListener::bind(&self.addr)
            .map_err(|e| format!("Failed to bind: {}", e))?;

        eprintln!("CID MCP HTTP server listening on {}", self.addr);
        eprintln!("Add http://{}/mcp as custom connector in Claude web interface", self.addr);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let addr = self.addr.clone();
                    thread::spawn(move || {
                        handle_connection(stream, &addr);
                    });
                }
                Err(e) => {
                    eprintln!("Connection failed: {}", e);
                }
            }
        }

        Ok(())
    }
}

fn handle_connection(mut stream: TcpStream, _addr: &str) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut engine = InferenceEngine::new();

    // Read HTTP request line
    let mut request_line = String::new();
    if reader.read_line(&mut request_line).is_err() {
        return;
    }

    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() < 2 {
        send_http_response(&mut stream, 400, "Bad Request", "{}");
        return;
    }

    let method = parts[0];
    let path = parts[1];

    // Read headers
    let mut content_length = 0;
    let mut headers = Vec::new();
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line).is_err() {
            return;
        }

        let line = line.trim().to_string();
        if line.is_empty() {
            break;
        }

        headers.push(line.clone());

        if line.to_lowercase().starts_with("content-length:") {
            if let Some(len) = line.split(':').nth(1) {
                content_length = len.trim().parse().unwrap_or(0);
            }
        }
    }

    // Handle CORS preflight
    if method == "OPTIONS" {
        send_cors_response(&mut stream);
        return;
    }

    // Only accept POST /mcp
    if method != "POST" || path != "/mcp" {
        // Also support GET /health
        if method == "GET" && path == "/health" {
            send_http_response(&mut stream, 200, "OK", r#"{"status":"ok","server":"cid-mcp"}"#);
            return;
        }
        send_http_response(&mut stream, 404, "Not Found", r#"{"error":"Only POST /mcp supported"}"#);
        return;
    }

    // Read body
    let mut body = vec![0u8; content_length];
    if content_length > 0
        && reader.read_exact(&mut body).is_err() {
            send_http_response(&mut stream, 400, "Bad Request", "{}");
            return;
        }

    let body_str = match String::from_utf8(body) {
        Ok(s) => s,
        Err(_) => {
            send_http_response(&mut stream, 400, "Bad Request", r#"{"error":"Invalid UTF-8"}"#);
            return;
        }
    };

    // Handle MCP request
    let response = handle_message(&body_str, &mut engine)
        .unwrap_or_else(|| r#"{"jsonrpc":"2.0","error":{"code":-32603,"message":"Internal error"}}"#.to_string());

    send_http_response(&mut stream, 200, "OK", &response);
}

fn send_http_response(stream: &mut TcpStream, status: u16, reason: &str, body: &str) {
    let status_line = format!("HTTP/1.1 {} {}\r\n", status, reason);
    let headers = format!(
        "Content-Type: application/json\r\n\
         Content-Length: {}\r\n\
         Access-Control-Allow-Origin: *\r\n\
         Access-Control-Allow-Methods: POST, OPTIONS\r\n\
         Access-Control-Allow-Headers: Content-Type\r\n\
         \r\n",
        body.len()
    );

    let _ = stream.write_all(status_line.as_bytes());
    let _ = stream.write_all(headers.as_bytes());
    let _ = stream.write_all(body.as_bytes());
    let _ = stream.flush();
}

fn send_cors_response(stream: &mut TcpStream) {
    let response = "HTTP/1.1 204 No Content\r\n\
                    Access-Control-Allow-Origin: *\r\n\
                    Access-Control-Allow-Methods: POST, OPTIONS\r\n\
                    Access-Control-Allow-Headers: Content-Type\r\n\
                    Access-Control-Max-Age: 86400\r\n\
                    \r\n";
    let _ = stream.write_all(response.as_bytes());
    let _ = stream.flush();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_server_new() {
        let server = HttpServer::new("127.0.0.1:8080");
        assert_eq!(server.addr, "127.0.0.1:8080");
    }
}
