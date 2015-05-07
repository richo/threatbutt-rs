#![feature(collections)]
use std::io;
use std::io::{Read,Write};
use url::Url;
use hyper::{Get,Post};
use hyper::error::HttpError;
use hyper::header::ContentLength;
use hyper::net::Fresh;
use hyper::client::Request;

extern crate url;
extern crate hyper;

pub struct ThreatButt;

const URL: &'static str = "http://api.threatbutt.io";

struct Sample;
struct Attribution;

impl Sample {
    fn url_for(md5: &str) -> Url {
        let mut url = Url::parse(URL).unwrap();
        {
            let mut path = url.path_mut().unwrap();
            path.append(&mut vec!["api".to_string(), "md5".to_string(), md5.to_string()]);
        }
        url
    }
}

impl Attribution {
    fn url_for() -> Url {
        let mut url = Url::parse(URL).unwrap();
        {
            let mut path = url.path_mut().unwrap();
            path.append(&mut vec!["api".to_string()]);
        }
        url
    }
}

fn execute_request(req: Request<Fresh>, data: &[u8]) -> Result<Vec<u8>, HttpError> {
    let mut conn = try!(req.start());
    conn.write(data);
    let mut resp = try!(conn.send());

    let mut body = Vec::new();
    let _ = try!(resp.read_to_end(&mut body));

    Ok(body)
}

pub fn identify_sample(md5: &str) -> Result<String, HttpError> {
    let url = Sample::url_for(md5);
    let mut req = Request::new(Post, url).unwrap();
    req.headers_mut().set(ContentLength(0));
    let body = try!(execute_request(req, &[]));

    return Ok(String::from_utf8(body).unwrap());
}

pub fn attribute_ip(ip_addr: &str) -> Result<String, HttpError> {
    let url = Attribution::url_for();
    let payload_bytes = ip_addr.as_bytes();
    let mut req = Request::new(Post, url).unwrap();
    req.headers_mut().set(ContentLength(payload_bytes.len() as u64));
    let body = try!(execute_request(req, payload_bytes));

    return Ok(String::from_utf8(body).unwrap());
}
