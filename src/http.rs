use std::str;
use std::io::{self, Write, Result};
use futures::{Future, Stream, future};
use hyper;
use hyper::{Client, Request, Response, Method};
use hyper::header::{ContentLength, ContentType};
use tokio_core::reactor::Core;

pub fn exec(ip: String, body: String) -> Result<()> {
    let mut core = Core::new()?;
    let client = Client::new(&core.handle());
    let uri = format!("http://{}/YamahaRemoteControl/ctrl", ip).parse().unwrap();
    let mut req: Request = Request::new(Method::Post, uri);
    req.headers_mut().set(ContentType::xml());
    req.headers_mut().set(ContentLength(body.len() as u64));
    req.set_body(body);
    let work = client.request(req).and_then(|res: Response| {
        println!("Response: {}", res.status());
        res.body().fold(Vec::new(), |mut v, chunk| {
            v.extend(&chunk[..]);
            future::ok::<_, hyper::Error>(v)
        }).and_then(move |chunks| {
            let body = String::from_utf8(chunks).unwrap();
            future::ok(body)
        })
    });
    let c = core.run(work).unwrap();
    Ok(())
}
