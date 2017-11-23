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
use futures::Stream;
use env_logger::LogBuilder;
use std::env;
use std::fmt::Display;

pub fn get_request_params(req: &Request) -> HashMap<String, String> {
    // need base or will parse err
    let s = format!("http://127.0.0.1{}", req.uri());
    let url = Url::parse(&s).unwrap();
    let pairs = url.query_pairs().into_owned();
    pairs.collect()
}

///  get query params and merge form params if content type is form_url_encoded
///  use get query first
pub fn get_form_params(req: Request) -> HashMap<String, String> {
    let mut res: HashMap<String, String>;
    res = get_request_params(&req);

    if let Some(ct) = req.headers().get::<header::ContentType>() {
        if *ct != header::ContentType::form_url_encoded() {
            return res;
        }
    } else {
        return res;
    }

    let q = read_req_body_full(req.body()).unwrap_or_default();
    let s = format!(
        "http://127.0.0.1/?{}",
        String::from_utf8(q).unwrap_or_default()
    );
    let url = Url::parse(&s).unwrap();
    let pairs = url.query_pairs().into_owned();
    for (k, v) in pairs {
        res.entry(k).or_insert(v);
    }
    res
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

pub fn error_json_response<E: Display>(error: E) -> Response {
    let msg = format!("{}", error);

    let to_j = json!({
        "error": &msg,
    });

    let j = serde_json::to_string(&to_j).unwrap();

    let resp = Response::new()
        .with_status(hyper::StatusCode::BadRequest)
        .with_header(header::ContentType(mime::APPLICATION_JSON))
        .with_header(header::ContentLength(j.len() as u64))
        .with_body(j);

    resp
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

pub fn read_req_body_full(body: hyper::Body) -> Result<Vec<u8>> {
    debug!("start read body full");
    let mut data: Vec<u8> = vec![];
    for item_res in body.wait() {
        debug!("itemp: {:?}", item_res);
        match item_res {
            Ok(item) => {
                // debug!("{:?}", item);
                for u in item {
                    data.push(u);
                }
            }
            Err(err) => {
                debug!("read err: {:?}", err);
            }
        }
    }
    debug!("read {} bytes", data.len());

    Ok(data)
}

pub fn test_echo(req: hyper::server::Request) -> Result<Response> {
    let data = read_req_body_full(req.body())?;

    let resp = Response::new()
        .with_header(hyper::header::ContentLength(data.len() as u64))
        .with_body(data);
    Ok(resp)
}
