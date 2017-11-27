use url::Url;
use std::error::Error;


use std::io::Write;
use futures::{Future, Stream};
use hyper::Client;
use hyper;
use tokio_core::reactor::Core;



pub fn post(url: &str, params: &Vec<(&str, &str)>) -> Result<Vec<u8>, String> {
    let url = Url::parse_with_params(url, params).map_err(|e| String::from(e.description()))?;

    let mut core = Core::new().map_err(|e| String::from(e.description()))?;
    let client = Client::new(&core.handle());

    let mut body: Vec<u8> = vec![];

    let uri = url.into_string()
        .parse::<hyper::Uri>()
        .map_err(|e| String::from(e.description()))?;

    debug!("post: {}", uri);

    {
        let work = client.get(uri).and_then(|res| {
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
