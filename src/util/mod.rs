mod post;

#[macro_use]
pub mod macros;
pub mod errors;


pub use self::post::post;
pub use self::errors::Result;
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


///  returns the boolean value represented by the string. It accepts 1, t, T, TRUE, true, True, 0, f, F, FALSE, false, False. Any other value returns an error.
pub fn parse_bool(s: &str) -> Result<bool> {
    match s {
        "1" => Ok(true),
        "t" => Ok(true),
        "T" => Ok(true),
        "TRUE" => Ok(true),
        "true" => Ok(true),
        "True" => Ok(true),
        "0" => Ok(false),
        "f" => Ok(false),
        "F" => Ok(false), 
        "FALSE" => Ok(false), 
        "false" => Ok(false), 
        "False" => Ok(false), 
        _ => Err(box_err!("no valid boolean value: {}", s)),
    }
}
