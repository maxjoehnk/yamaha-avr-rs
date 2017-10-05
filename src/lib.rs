extern crate xml;
extern crate futures;
extern crate hyper;
extern crate tokio_core;

pub mod yamaha;
mod http;

use yamaha::YamahaAvr;

pub fn connect(ip: String) -> YamahaAvr {
    YamahaAvr::new(ip)
}

pub fn discover() -> Vec<YamahaAvr> {
    Vec::new()
}