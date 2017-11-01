mod post;

#[macro_use]
pub mod macros;


pub use self::post::post;
use hyper::server::Request;

pub fn get_request_params(req: &Request) -> Result<HashMap<String, String>> {
    // need base or will parse err
    let s = format!("http://127.0.0.1{}", req.uri());

    debug!("url: {:?}", s);

    let url = Url::parse(&s)?;
    let pairs = url.query_pairs().into_owned();
    Ok(pairs.collect())
}
