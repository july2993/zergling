use hyper;
use std::io::Read;
use futures::future;
use futures::future::Future;
use futures::sync::oneshot;
use hyper::header::ContentLength;
use hyper::header;
use std::collections::HashMap;
use hyper::header::{Headers, LastModified, IfModifiedSince, HttpDate};
use hyper::{StatusCode, Method};
use hyper::server::{Request, Response, Service};
use std::time::{SystemTime, Duration};
use std::thread;
use grpcio::*;
use futures::*;
use pb;
use std::ops::Add;
use url::Url;
use super::{Store, NeedleMapType};
use std;
use std::time;
use std::error::Error;
use storage::{Result, VolumeInfo, Needle};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Mutex;
use std::boxed::FnBox;
use std::sync::Arc;
use std::path::Path;
use libflate::gzip::{Encoder, Decoder};
use serde_json;
use util;

const PHRASE: &'static str = "Hello, World!";
pub type APICallback = Box<FnBox(Result<Response>) + Send>;

pub enum Msg {
    API { req: Request, cb: APICallback },
}

fn make_callback() -> (Box<FnBox(Result<Response>) + Send>, oneshot::Receiver<Result<Response>>) {
    let (tx, rx) = oneshot::channel();
    let callback = move |resp| { tx.send(resp).unwrap(); };
    (Box::new(callback), rx)
}

#[derive(Clone)]
pub struct Context {
    pub sender: Sender<Msg>,
    pub store: Arc<Mutex<Store>>,
    pub needle_map_kind: NeedleMapType,
    pub read_redirect: bool,
    pub pulse_seconds: u64,
    pub master_node: String,
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
            Msg::API { req, cb } => {
                debug!("hanle msg: [{}] {}", req.method(), req.path());
                match (req.method(), req.path()) {
                    (&Method::Get, "/stats") => {
                        let handle = status_handler(self, &req);
                        cb(handle);
                    }
                    (&Method::Get, "/admin/assign_volume") => {
                        let handle = assign_volume_handler(self, &req);
                        cb(handle);
                    }
                    (&Method::Get, "/favicon.ico") => {
                        let handle = test_handler(&req);
                        cb(handle);
                    }
                    (&Method::Get, _) => {
                        let handle = get_or_head_handler(self, &req);
                        cb(handle);
                    }
                    (&Method::Head, _) => {
                        let handle = get_or_head_handler(self, &req);
                        cb(handle);
                    }
                    (_, _) => {
                        let handle = test_handler(&req);
                        cb(handle);
                    }
                }
            }
        }

    }

    pub fn heartbeat(&self) {
        loop {
            warn!("start heartbeat....");
            self.start_heartbeat();
            warn!("heartbeat end....");
            thread::sleep(Duration::from_secs(self.pulse_seconds as u64));
        }
    }

    pub fn start_heartbeat(&self) {
        let env = Arc::new(Environment::new(2));
        let channel = ChannelBuilder::new(env).connect(&self.master_node);
        let client = pb::zergling_grpc::SeaweedClient::new(channel);

        let (mut sink, mut receiver) = client.send_heartbeat();

        let h = thread::spawn(move || loop {
            match receiver.into_future().wait() {
                Ok((Some(beat), r)) => {
                    debug!("recv: {:?}", beat);
                    receiver = r;
                }
                Ok((None, _)) => break,
                Err((e, _)) => {
                    error!("RPC failed: {:?}", e);
                    break;
                }
            }
        });

        loop {
            let beat = self.store.lock().unwrap().collect_heartbeat();
            match sink.send((beat, WriteFlags::default())).wait() {
                Ok(ret) => sink = ret,
                Err(err) => {
                    error!("send err: {}", err);
                    break;
                }
            }

            thread::sleep(Duration::from_secs(self.pulse_seconds as u64));
        }

        // sink.close();
        if let Err(err) = h.join() {
            error!("join err: {:?}", err);
        }
    }
}


impl Service for Context {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;

    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let (cb, future) = make_callback();
        self.sender.send(Msg::API { req, cb }).unwrap();

        let future = future
            .map_err(|_err| {
                // TODO more specify error?
                hyper::Error::Timeout
            })
            .map(|v| match v {
                Ok(resp) => resp,
                Err(err) => {
                    debug!("err: {:?}", err);
                    let s = format!("{{\"Error\": {}}}", err.description());
                    Response::new()
                        .with_header(ContentLength(s.len() as u64))
                        .with_body(s)
                }
            });
        Box::new(future)
    }
}

pub fn test_handler(_req: &Request) -> Result<Response> {
    Ok(
        Response::new()
            .with_header(ContentLength(PHRASE.len() as u64))
            .with_body(PHRASE),
    )
}

fn status_handler(ctx: &Context, _req: &Request) -> Result<Response> {
    let store = ctx.store.lock().unwrap();

    let mut infos: Vec<VolumeInfo> = vec![];
    for location in store.locations.iter() {
        for (_, v) in location.volumes.iter() {
            let vinfo = v.get_volume_info();
            infos.push(vinfo);
        }
    }

    let volumes = serde_json::to_string(&infos)?;
    let stat = json!({
        "Version": "0.1",
        "Volumes": volumes,
    });

    let ret = stat.to_string();

    let resp = Response::new()
        .with_header(ContentLength(ret.len() as u64))
        .with_body(ret);
    Ok(resp)
}

pub fn assign_volume_handler(ctx: &Context, req: &Request) -> Result<Response> {
    let params = util::get_request_params(req);

    let pre_allocate = params
        .get("preallocate")
        .unwrap_or(&String::from("0"))
        .parse::<i64>()
        .unwrap_or_default();

    let mut store = ctx.store.lock().unwrap();
    store.add_volume(
        params.get("volume").unwrap_or(&String::from("")),
        params.get("collection").unwrap_or(&String::from("")),
        ctx.needle_map_kind,
        params.get("replication").unwrap_or(&String::from("")),
        params.get("ttl").unwrap_or(&String::from("")),
        pre_allocate,
    )?;


    let mut resp = Response::new();

    resp.set_status(StatusCode::Accepted);

    Ok(resp)
}

// pub fn post_handler(ctx: &Context, req: &Request) -> Result<Response> {
//     let (svid, _, _, _, _) = parse_url_path(req.path());
//     let vid = svid.parse::<u32>()?;


// }

pub fn get_or_head_handler(ctx: &Context, req: &Request) -> Result<Response> {
    let params = util::get_request_params(req);

    let (svid, fid, mut filename, mut ext, _) = parse_url_path(req.path());

    let vid = svid.parse::<u32>()?;

    let mut n = Needle::default();
    n.parse_path(&fid)?;
    let cookie = n.cookie;

    let mut store = ctx.store.lock().unwrap();
    let mut resp = Response::new();

    if !store.has_volume(vid) {
        if !ctx.read_redirect {
            info!("volume is not local: {}", req.path());
            resp.set_status(StatusCode::NotFound);
            return Ok(resp);
        } else {
            //TODO support read_redirect
            panic!("TODO");
        }
    }

    let count = store.read_volume_needle(vid, &mut n)?;
    debug!("read {} byte for {}", count, fid);
    if n.cookie != cookie {
        info!("cookie not match from {:?} recv: {} file is {}", req.remote_addr(), cookie, n.cookie);
        resp.set_status(StatusCode::NotFound);
        return Ok(resp);
    }

    if n.last_modified != 0 {
        let modified = time::UNIX_EPOCH.add(Duration::new(n.last_modified, 0));
        resp.headers_mut().set(LastModified(modified.into()));

        if let Some(since) = req.headers().get::<IfModifiedSince>() {
            if since.0.le(&HttpDate::from(modified)) {
                resp.set_status(StatusCode::NotModified);
                return Ok(resp);
            }
        }
    }

    let etag = n.etag();

    if let Some(not_match) = req.headers().get_raw("If-None-Match") {
        if not_match == etag.as_str() {
            resp.set_status(StatusCode::NotModified);
            return Ok(resp);
        }
    }

    if n.has_pairs() {
        // TODO support pairs
        // https://hyper.rs/hyper/0.8.0/hyper/header/index.html
        // let j: serde_json::Value = serde_json::from_slice(&n.pairs)?;
        // for (k, v) in j.as_object().unwrap() {
        //     debug!("{} {}", k, v);
        //     resp.headers_mut().set_raw(k, v.as_str().unwrap());
        // }
    }

    // chunk file
    // TODO

    if n.name.len() > 0 && filename.len() == 0 {
        filename = String::from_utf8(n.name.clone()).unwrap();
        if ext.len() == 0 {
            ext = Path::new(&filename)
                .extension()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
        }
    }

    let _ = ext;

    let mut mtype = String::new();
    if n.mime.len() > 0 {
        if !n.mime.starts_with(b"application/octet-stream") {
            mtype = String::from_utf8(n.mime.clone()).unwrap();
        }
    }

    if n.is_gzipped() {
        if let Some(ae) = req.headers().get::<header::AcceptEncoding>() {
            let mut gzip = false;
            for qitem in ae.0.iter() {
                if qitem.item == header::Encoding::Gzip {
                    gzip = true;
                    break;
                }
            }
            if gzip {
                resp.headers_mut().set_raw("Content-Encoding", "gzip");
            } else {
                let mut decoded_data = Vec::new();
                {
                    let mut decoder = Decoder::new(&n.data[..])?;
                    decoder.read_to_end(&mut decoded_data)?;
                }
                n.data = decoded_data;
            }
        }
    }

    // TODO support  image resize


    resp = write_response_content(params, &filename, &mtype, resp, &n.data);

    resp.set_status(StatusCode::Accepted);

    Ok(resp)
}

fn write_response_content(
    _params: HashMap<String, String>,
    _filename: &str,
    _mtype: &str,
    _resp: Response,
    data: &Vec<u8>,
) -> Response {

    //TODO handle range contenttype and...
    let len = data.len() as u64;
    let resp = _resp.with_header(ContentLength(len)).with_body(
        String::from_utf8(
            data.clone(),
        ).unwrap(),
    );


    resp
}




// support following format
// http://localhost:8080/3/01637037d6/my_preferred_name.jpg
// http://localhost:8080/3/01637037d6.jpg
// http://localhost:8080/3,01637037d6.jpg
// http://localhost:8080/3/01637037d6
// http://localhost:8080/3,01637037d6
// @return vid, fid, filename, ext, is_volume_id_only
fn parse_url_path(path: &str) -> (String, String, String, String, bool) {
    let vid: String;
    let mut fid: String;
    let filename: String;
    let mut ext: String = String::default();
    let mut is_volume_id_only = false;

    let parts: Vec<&str> = path.split("/").collect();
    match parts.len() {
        3 => {
            vid = parts[1].to_string();
            fid = parts[2].to_string();
            filename = parts[3].to_string();

            // must be valid utf8
            ext = Path::new(&filename)
                .extension()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
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
