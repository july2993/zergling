use hyper;
use futures::future;
use futures::future::Future;
use futures::sync::oneshot;
use hyper::header::ContentLength;
use hyper::server::{Request, Response, Service};
use url::Url;
use std;
use std::error::Error;
use storage::Result;
use std::sync::mpsc::{Receiver, Sender};
use std::boxed::FnBox;
use std::sync::Arc;
use std::path::Path;
use util;

const PHRASE: &'static str = "Hello, World!";
pub type APICallback = Box<FnBox(Result<Response>) + Send>;

pub enum Msg {
    API {req: Request, cb: APICallback}
}

fn make_callback() -> (Box<FnBox(Result<Response>) + Send>, oneshot::Receiver<Result<Response>>) {
    let (tx, rx) = oneshot::channel();
    let callback = move |resp| {tx.send(resp).unwrap(); };
    (Box::new(callback), rx)
}

#[derive(Clone)]
pub struct Context {
    pub sender: Arc<Sender<Msg>>,
}

impl Context {
    pub fn run(&mut self, receiver: Receiver<Msg>) -> Result<()> {
        for msg in receiver.iter() {
            self.handle_msg(msg);
        }

        panic!("receiver hung up");
    }

    fn handle_msg(&mut self, msg: Msg) {
        match msg {
            Msg::API {req, cb} => {
                match (req.method(), req.path()) {
                    (method, path) => {
                        let handle = test_handler(&req);
                        cb(handle);
                    }
                }
            }
        }

    }

}


impl Service for Context {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;

    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let (cb, future) = make_callback();
        self.sender.send(Msg::API{req, cb}).unwrap();

        let future = future.map_err( |err| {
            // TODO more specify error?
            hyper::Error::Timeout
        })
        .map(|v| {
            match v {
                Ok(resp) => resp,
                Err(err) => {
                    debug!("err: {:?}", err);
                    let s = format!("{{\"Error\": {}}}", err.description());    
                    Response::new()
                            .with_header(ContentLength(s.len() as u64))
                            .with_body(s)
                }
            }
        });
        Box::new(future)
    }
}

pub fn test_handler(_req: &Request) -> Result<Response> {
    Ok(Response::new()
        .with_header(ContentLength(PHRASE.len() as u64))
        .with_body(PHRASE))
}

pub fn assign_volume_handler(req: &Request) -> Result<Response> {
    let params = util::get_request_params(req);
}


// pub fn post_handler() -> Result<Response> {

// }


 // support following format
 // http://localhost:8080/3/01637037d6/my_preferred_name.jpg
 // http://localhost:8080/3/01637037d6.jpg
 // http://localhost:8080/3,01637037d6.jpg
 // http://localhost:8080/3/01637037d6
 // http://localhost:8080/3,01637037d6
 // @return vid, fid, filename, ext, is_volume_id_only
fn parse_url_path(path: &str) -> (String,  String,  String, String, bool) {
    let mut vid: String;
    let mut fid: String;
    let mut filename: String;
    let mut ext: String = String::default();
    let mut is_volume_id_only = false;

    let parts: Vec<&str> = path.split("/").collect();
    match parts.len() {
        3 => {
            vid = parts[1].to_string();
            fid = parts[2].to_string();
            filename = parts[3].to_string();
            
            // must be valid utf8
            ext = Path::new(&filename).extension().unwrap_or_default().to_string_lossy().to_string();
        }
        2 => {
            filename = String::default();

            vid = parts[1].to_string();
            fid = parts[2].to_string();
            if let Some(idx) = parts[2].rfind(".") {
                let (fid_str, ext_str) = parts[2].split_at(idx);
                fid = fid_str.to_string();
                ext = ext_str.to_string();
            }
        }
        _ => {
            filename = String::default();
            let dot = path.rfind(".");
            let sep = path.rfind(",");

            let mut end = path.len();
            if dot.is_some() {
                let start = dot.unwrap() + 1;
                ext = path[start..].to_string();
                end = start - 1;
            }
            
            if sep.is_some() {
                let start = sep.unwrap() + 1;
                fid = path[start..end].to_string();
                end = start - 1;
            } else {
                fid = String::default();
                is_volume_id_only = true;
            }

            vid = path[1..end].to_string();
        }
        
    };

    (vid, fid, filename, ext, is_volume_id_only)
}
