pub mod errors;
pub use self::errors::{Error, Result};

use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::Client as HClient;
use hyper::{self, Method, Request};
use hyper::header::{ContentLength, ContentType};
use tokio_core::reactor::Core;
use mime;
use serde_json;

pub struct Client {
    core: Core,
    master: String,
    // hclient: HClient,
}

impl Client {
    pub fn new(master: &str) -> Result<Client> {
        let core = Core::new()?;

        let cli = Client {
            core: core,
            master: master.to_string(),
            // hclient: HClient::new(&core.handle()),
        };

        Ok(cli)
    }

    fn get_content(&mut self, url: &str) -> Result<Vec<u8>> {
        let uri = url.parse()?;
        let cli = HClient::new(&self.core.handle());
        let work = cli.get(uri).map_err(Error::from).and_then(|res| {
            debug!("Response: {}", res.status());

            res.body()
                .concat2()
                .map(move |chunk| chunk.to_vec())
                .map_err(From::from)
        });
        self.core.run(work)
    }

    fn get_json(&mut self, url: &str) -> Result<serde_json::Value> {
        self.get_content(url).and_then(|c| {
            serde_json::from_slice::<serde_json::Value>(&c)
                .map_err(Error::from)
                .and_then(|v| {
                    if let Some(err) = v["errors"].as_str() {
                        if err.len() > 0 {
                            return Err(box_err!("{}", err));
                        }
                    }
                    Ok(v)
                })
        })
    }


    fn post_file(
        &mut self,
        url: &str,
        filename: &str,
        content_type: &str,
        body: &[u8],
    ) -> Result<serde_json::Value> {
        let boundary = "------------------------8f0722fc2b0ab361";
        let data = make_file_multipart(boundary, filename, content_type, body);
        let mime = format!("multipart/form-data; boundary={}", boundary)
            .parse::<mime::Mime>()
            .unwrap();

        let ct = ContentType(mime);

        self.post(url, ct, &data)
    }

    fn post(
        &mut self,
        url: &str,
        content_type: ContentType,
        body: &[u8],
    ) -> Result<serde_json::Value> {
        let uri = url.parse()?;

        let mut req: Request<hyper::Body> = Request::new(Method::Post, uri);
        req.headers_mut().set(content_type);
        req.headers_mut().set(ContentLength(body.len() as u64));
        // TODO can avoid copy ?
        req.set_body(body.to_owned());

        let cli = HClient::new(&self.core.handle());

        let work = cli.request(req).map_err(Error::from).and_then(|res| {
            debug!("Response: {}", res.status());

            res.body()
                .concat2()
                .map(move |chunk| chunk.to_vec())
                .map_err(From::from)
                .and_then(move |c| {
                    serde_json::from_slice::<serde_json::Value>(&c)
                        .map_err(Error::from)
                        .and_then(|v| {
                            if let Some(err) = v["errors"].as_str() {
                                if err.len() > 0 {
                                    return Err(box_err!("{}", err));
                                }
                            }
                            Ok(v)
                        })
                })
        });
        self.core.run(work)
    }

    // {"fid":"2,17379c608","url":"127.0.0.1:8080","publicUrl":"127.0.0.1:8080","count":1,"error":""}
    pub fn assign(&mut self) -> Result<serde_json::Value> {
        let url = &format!("http://{}/dir/assign", self.master);
        self.get_json(url)
    }

    pub fn upload_file_content(
        &mut self,
        addr: &str,
        fid: &str,
        content: &[u8],
    ) -> Result<serde_json::Value> {
        let url = format!("http://{}/{}", addr, fid);
        self.post_file(&url, "test.txt", "text/plain", content)
    }

    pub fn get_file_content_by_fid(&mut self, addr: &str, fid: &str) -> Result<Vec<u8>> {
        let url = &format!("http://{}/{}", addr, fid);
        self.get_content(url)
    }
}


// --------------------------4ceab01e2296e20f
// Content-Disposition: form-data; name="file"; filename="a.txt"
// Content-Type: text/plain

// filecontent

// ------------------------4ceab01e2296e20f--
//
fn make_file_multipart(
    boundary: &str,
    filename: &str,
    content_type: &str,
    content: &[u8],
) -> Vec<u8> {
    let mut data: Vec<u8> = vec![];
    data.extend("--".as_bytes());
    data.extend(boundary.as_bytes());
    data.extend("\r\n".as_bytes());
    data.extend(
        format!(
            "Content-Disposition: form-data; name=\"file\"; filename=\"{}\"\r\n",
            filename
        ).as_bytes(),
    );
    data.extend(format!("Content-Type: {}\r\n\r\n", content_type).as_bytes());
    data.extend(content);
    data.extend("\r\n".as_bytes());
    data.extend("--".as_bytes());
    data.extend(boundary.as_bytes());
    data.extend("--".as_bytes());
    data.extend("\r\n".as_bytes());

    data
}
