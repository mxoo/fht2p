use stderr::Loger;

use super::*;
use self::statics::*;
/// `GET` and `HEAD` Methods
pub mod get;
use self::get::*;
/// Get `Path`'s `Info`
pub mod path_info;
/// `HTML` depends on `Maud`
pub mod html;
use client::content_type::ContentType;

use std::io::{self, Read, Write, BufReader, BufWriter};
use std::error::Error;
use std::fs::File;
use std::mem;

/// `HTTP/1.1 200 OK`
#[derive(Debug,Clone)]
pub struct StatusLine {
    pub protocol: String,
    pub version: String,
    pub code: StatusCode,
}

impl StatusLine {
    pub fn protocol(&self) -> &str {
        &self.protocol
    }
    pub fn version(&self) -> &str {
        &self.version
    }
    pub fn code(&self) -> &StatusCode {
        &self.code
    }
}

impl Default for StatusLine {
    fn default() -> Self {
        Self {
            protocol: HTTP_PROTOCOL.to_owned(),
            version: HTTP_VERSION.to_owned(),
            code: StatusCode::default(),
        }
    }
}
/// HTTP/1.1 200 OK\r\n`Headers`\r\n\r\n`Body`
#[derive(Debug)]
pub struct Response {
    pub line: StatusLine,
    pub header: Map<String, String>,
    /// file/dir/static_file
    pub content: Content,
}

impl Response {
    pub fn code(&self) -> u16 {
        self.line.code.code()
    }
    pub fn code_set<C: Into<u16>>(&mut self, code: C) {
        self.line.code.set(code);
    }
    pub fn line(&self) -> &StatusLine {
        &self.line
    }
    pub fn header(&self) -> &Map<String, String> {
        &self.header
    }
    pub fn header_insert<S1: Into<String>, S2: Into<String>>(&mut self, key: S1, value: S2) {
        self.header.insert(key.into(), value.into());
    }
    pub fn content_type_insert(&mut self, content_type: ContentType) {
        self.header
            .insert("Content-Type".to_owned(), content_type.to_string());
    }
    pub fn content_length_insert(&mut self) {
        self.header
            .insert("Content-Length".to_owned(),
                    format!("{}", self.content.len()));
    }
    pub fn get(&mut self, req: &Request) {
        get(self, req)
    }

    pub fn call(self, mut stream: &mut TcpStream, req: &Request) {
        fn write_response(resp: Response, req: &Request, mut stream: &mut TcpStream) -> io::Result<()> {
            write!(&mut stream,
                   "{}/{} {} {}\r\n",
                   resp.line.protocol(),
                   resp.line.version(),
                   resp.line.code().code(),
                   resp.line.code().desc())?;
            for (k, v) in resp.header() {
                write!(&mut stream, "{}: {}\r\n", k, v)?;
            }
            write!(&mut stream, "\r\n")?;
            if req.line().method() == "HEAD" {
                return Ok(());
            }
            resp.content.write(&mut stream)?;
            Ok(())
        }
        if let Err(e) = write_response(self, req, &mut stream) {
            dbstln!("{}_Warning@response_call(): {}", NAME, e.description());
        };
    }
}

impl Default for Response {
    fn default() -> Self {
        let mut header: Map<String, String> = Map::new();
        header.insert("Server".to_owned(), format!("{}/{}", NAME, VERSION));
        if keep_alive() {
            header.insert("Connection".to_owned(), "keep-alive".to_owned());
        } else {
            header.insert("Connection".to_owned(), "close".to_owned());
        }
        Response {
            line: StatusLine::default(),
            header: header,
            content: Content::default(),
        }
    }
}
/// `Response`'s `Content`
#[derive(Debug)]
pub enum Content {
    /// dir,oher status -> `String` -> `Vec<u8>`
    Str(String),
    /// file
    File(File),
    /// static file -> `&[u8]`
    Sf(&'static [u8]),
}

#[allow(unknown_lints,len_without_is_empty)]
impl Content {
    pub fn update(&mut self, mut other: Content) {
        mem::swap(self, &mut other);
    }
    pub fn len(&self) -> u64 {
        fn file_lenth(file: &File) -> u64 {
            match file.metadata() {
                Ok(ok) => ok.len(),
                Err(e) => {
                    dbstln!("{}_Warning@file_lenth(): {}@{:?}",
                            NAME,
                            e.description(),
                            file);
                    panic!();
                }
            }
        }
        match *self {
            Content::Str(ref x) => x.len() as u64,
            Content::File(ref y) => file_lenth(y),
            Content::Sf(z) => z.len() as u64,
        }
    }
    pub fn write(self, mut stream: &mut TcpStream) -> io::Result<()> {
        match self {
            Content::Str(x) => {
                let _ = stream.write(x.as_bytes())?;
            }
            Content::File(mut y) => {
                file_write_to_tcpstream(&mut y, &mut stream)?;
            }
            Content::Sf(z) => {
                let _ = stream.write(z)?;
            }
        };
        stream.flush()?;
        Ok(())
    }
}

fn file_write_to_tcpstream(file: &mut File, stream: &mut TcpStream) -> io::Result<()> {
    let mut stream = BufWriter::with_capacity(BUFFER_SIZE, stream);
    let file = BufReader::with_capacity(BUFFER_SIZE, file);
    for byte in file.bytes() {
        let byte = byte?;
        stream.write_all(&[byte])?;
    }
    stream.flush()?;
    Ok(())
}

impl Default for Content {
    fn default() -> Self {
        Content::Str(String::with_capacity(0))
    }
}