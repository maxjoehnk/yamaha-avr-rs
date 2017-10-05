extern crate xml;
extern crate futures;
extern crate hyper;
extern crate tokio_core;

pub mod yamaha;

pub fn connect(ip: String) -> yamaha::YamahaAvr {
    yamaha::YamahaAvr::new(ip)
}

pub fn discover() -> Vec<yamaha::YamahaAvr> {
    Vec::new()
}