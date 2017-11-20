mod post;

#[macro_use]
pub mod macros;
pub mod errors;


pub use self::post::post;
pub use self::errors::{Result, Error};
use hyper::server::{Request, Response};
use hyper;
use hyper::header;
use url::Url;
use serde;
use serde_json;
use std::collections::HashMap;
use mime;
use chrono::Local;
use env_logger::LogBuilder;
use std::env;

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


pub fn json_response<J: serde::ser::Serialize>(
    status: hyper::StatusCode,
    to_j: &J,
) -> Result<Response> {

    let j = serde_json::to_string(to_j)?;

    let resp = Response::new()
        .with_status(status)
        .with_header(header::ContentType(mime::APPLICATION_JSON))
        .with_header(header::ContentLength(j.len() as u64))
        .with_body(j);


    Ok(resp)
}

pub fn init_log() {
    LogBuilder::new()
        .format(|record| {
            format!(
                "{} [{}:{}] - {} {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.location().file().rsplit('/').nth(0).unwrap(),
                record.location().line(),
                record.level(),
                record.args()
            )
        })
        .parse(&env::var("ZERGLING_LOG").unwrap_or_default())
        .init()
        .unwrap();
}
