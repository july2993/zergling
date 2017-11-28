use url::Url;
use std::error::Error;


use std::io::Write;
use futures::{Future, Stream};
use hyper::{self, header, Client, Method, Request};
use tokio_core::reactor::Core;


pub fn delete(url: &str, params: &Vec<(&str, &str)>) -> Result<Vec<u8>, String> {
    request(url, params, Method::Delete, None)
}

pub fn get(url: &str, params: &Vec<(&str, &str)>) -> Result<Vec<u8>, String> {
    request(url, params, Method::Get, None)
}

pub fn post(url: &str, params: &Vec<(&str, &str)>, pbody: &[u8]) -> Result<Vec<u8>, String> {
    request(url, params, Method::Post, Some(pbody))
}

fn request(
    url: &str,
    params: &Vec<(&str, &str)>,
    method: Method,
    op_pbody: Option<&[u8]>,
) -> Result<Vec<u8>, String> {
    let url = Url::parse_with_params(url, params).map_err(|e| String::from(e.description()))?;

    let mut core = Core::new().map_err(|e| String::from(e.description()))?;
    let client = Client::new(&core.handle());

    let mut body: Vec<u8> = vec![];

    let uri = url.into_string()
        .parse::<hyper::Uri>()
        .map_err(|e| String::from(e.description()))?;
    debug!("request [{}]: {}", method, uri);

    {
        let mut req: Request<hyper::Body> = Request::new(method, uri);
        if let Some(pbody) = op_pbody {
            req.headers_mut()
                .set(header::ContentLength(pbody.len() as u64));
            req.set_body(pbody.to_owned());
        }
        let work = client.request(req).and_then(|res| {
            // println!("Response: {}", res.status());
            res.body().for_each(|chunk| {
                body.write_all(&chunk).map_err(From::from)
                // body.extend(chunk);
            })
        });

        core.run(work).map_err(|e| String::from(e.description()))?;
    }
    debug!("resp: {}", String::from_utf8(body.clone()).unwrap());
    Ok(body)
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get() {
        let url = "http://www.baidu.com";
        let r = get(url, &vec![]);
        assert!(r.is_ok());
    }

    #[test]
    fn test_post() {
        let url = "http://www.baidu.com";
        let r = post(url, &vec![], &vec![]);
        assert!(r.is_ok());
    }

}
