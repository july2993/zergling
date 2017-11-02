mod post;

#[macro_use]
pub mod macros;


pub use self::post::post;
use hyper::server::Request;
use url::Url;
use std::collections::HashMap;

pub fn get_request_params(req: &Request) -> HashMap<String, String> {
    // need base or will parse err
    let s = format!("http://127.0.0.1{}", req.uri());

    debug!("url: {:?}", s);

    let url = Url::parse(&s).unwrap();
    let pairs = url.query_pairs().into_owned();
    pairs.collect()
}
