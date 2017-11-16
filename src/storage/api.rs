use hyper;
use std::io::Read;
use futures::future;
use futures::future::Future;
use futures::sync::oneshot;
use hyper::header::ContentLength;
use std::fmt::{self, Debug, Display, Formatter};
use hyper::header;
use crc::crc32;
use operation;
use hyper::mime;
use std::collections::HashMap;
use hyper::header::{Headers, LastModified, IfModifiedSince, HttpDate, ContentType};
use hyper::{StatusCode, Method};
use hyper::server::{Request, Response, Service};
use std::time::{SystemTime, Duration};
use std::thread;
use grpcio::*;
use futures::*;
use pb;
use std::ops::Add;
use storage;
use url::Url;
use super::{Store, NeedleMapType};
use std;
use std::time;
use std::error::Error;
use storage::{Result, VolumeInfo, Needle, TTL, VolumeId};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Mutex;
use std::boxed::FnBox;
use std::sync::Arc;
use std::path::Path;
use libflate::gzip::{Encoder, Decoder};
use multipart;
use multipart::server::MultipartData;
use serde_json;
use util;
use mime_guess;
use operation::Looker;

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
    pub looker: Arc<Mutex<Looker>>,
}

impl Context {
    pub fn run(&mut self, receiver: Receiver<Msg>) -> Result<()> {
        for msg in receiver.iter() {
            self.handle_msg(msg);
        }

        panic!("receiver hung up");
    }

    pub fn get_master_node(&self) -> String {
        return self.master_node.clone();
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
                    (&Method::Post, "/test_multipart") => {
                        let handle = test_multipart_handler(req);
                        cb(handle);
                    }
                    (&Method::Post, "/test_echo") => {
                        let handle = test_echo(req);
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
                    (&Method::Post, _) => {
                        let handle = post_handler(self, req);
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
                    let s = format!("{{\"error\": {}}}", err.description());
                    Response::new()
                        .with_status(StatusCode::NotAcceptable)
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

    // let volumes = serde_json::to_string(&infos)?;
    let stat = json!({
        "Version": "0.1",
        "Volumes": &infos,
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


    let resp = util::json_response(StatusCode::Accepted, &json!({"error":""}))?;


    Ok(resp)
}

fn read_req_body_full(body: hyper::Body) -> Result<Vec<u8>> {
    let mut data: Vec<u8> = vec![];
    for item_res in body.wait() {
        match item_res {
            Ok(item) => {
                // debug!("{:?}", item);
                for u in item {
                    data.push(u);
                }
            }
            Err(err) => {
                debug!("{:?}", err);
            }
        }
    }

    Ok(data)
}

pub fn test_echo(req: hyper::server::Request) -> Result<Response> {
    let data = read_req_body_full(req.body())?;

    let resp = Response::new()
        .with_header(ContentLength(data.len() as u64))
        .with_body(data);
    Ok(resp)
}

pub fn get_boundary(req: &Request) -> Result<String> {
    if *req.method() != Method::Post {
        return Err(box_err!("parse multipart err: no post reqest"));
    }

    let ct = match req.headers().get::<ContentType>() {
        Some(_ct) => _ct,
        None => return Err(box_err!("no ContentType header")),
    };

    match ct.get_param("boundary") {
        Some(bd) => return Ok(bd.to_string()),
        None => return Err(box_err!("no boundary")),
    };
}

pub fn test_multipart_handler(req: hyper::server::Request) -> Result<Response> {
    let boundary = get_boundary(&req)?;
    let data = read_req_body_full(req.body())?;
    let mut mpart = multipart::server::Multipart::with_body(&data[..], boundary);

    while let Ok(Some(field)) = mpart.read_entry() {
        debug!("field name: {}", field.name);
    }

    Ok(Response::new())
}

pub struct ParseUploadResp {
    pub file_name: String,
    pub data: Vec<u8>,
    pub mime_type: String,
    pub pair_map: HashMap<String, String>,
    pub modified_time: u64,
    pub ttl: TTL,
    pub is_chunked_file: bool,
}

impl Display for ParseUploadResp {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "file_name: {}, data_len: {}, mime_type: {}, ttl minutes: {}, is_chunked_file: {}", self.file_name, self.data.len(), self.mime_type, self.ttl.minutes(), self.is_chunked_file)
    }
}

pub fn parse_upload(req: hyper::server::Request) -> Result<ParseUploadResp> {
    let params = util::get_request_params(&req);

    let mut file_name = String::new();
    let mut data: Vec<u8> = vec![];
    let mut mime_type = String::new();
    let pair_map: HashMap<String, String> = HashMap::new();

    let modified_time: u64;
    let ttl: TTL;
    let is_chunked_file: bool;
    //
    //
    // TODO parse custom pairs header

    let boundary = get_boundary(&req)?;
    let body_data = read_req_body_full(req.body())?;
    let mut mpart = multipart::server::Multipart::with_body(&body_data[..], boundary);

    // get first file with file_name
    let mut post_mtype = String::new();
    while let Ok(Some(field)) = mpart.read_entry() {
        debug!("field name: {}", field.name);
        match field.data {
            MultipartData::File(mut file) => {
                if file.filename.is_some() {
                    file_name = file.filename.clone().unwrap();
                }
                #[allow(deprecated)] post_mtype.push_str(file.content_type().0.as_str());
                post_mtype.push_str("/");
                #[allow(deprecated)] post_mtype.push_str(file.content_type().1.as_str());
                // file.content_type().TopLevel.as_str()
                data.clear();
                file.read_to_end(&mut data)?;
            }
            MultipartData::Text(_text) => {}
        }

        if file_name.len() > 0 {
            break;
        }
    }

    is_chunked_file = util::parse_bool(params.get("cm").unwrap_or(&"false".to_string()))
        .unwrap_or(false);

    let mut guess_mtype = String::new();
    if !is_chunked_file {
        if let Some(idx) = file_name.find(".") {
            let ext = &file_name[idx..];
            let m = mime_guess::get_mime_type(ext);
            if m.0.as_str() != "application" || m.1.as_str() != "octet-stream" {
                guess_mtype.push_str(m.0.as_str());
                guess_mtype.push_str("/");
                guess_mtype.push_str(m.1.as_str());
            }
        }

        if post_mtype != "" && guess_mtype != post_mtype {
            mime_type = post_mtype.clone(); // only return if not deductable, so my can save it only when can't deductable from file name
            // guess_mtype = post_mtype.clone();
        }


        // don't auto gzip and change filename like seaweed
    }

    modified_time = params
        .get("ts")
        .unwrap_or(&"0".to_string())
        .parse()
        .unwrap_or(0);


    ttl = match params.get("ttl") {
        Some(s) => TTL::new(s).unwrap_or(TTL::default()),
        None => TTL::default(),
    };

    let resp = ParseUploadResp {
        file_name: file_name,
        data: data,
        mime_type: mime_type,
        pair_map: pair_map,
        modified_time: modified_time,
        ttl: ttl,
        is_chunked_file: is_chunked_file,
    };

    Ok(resp)
}

pub fn post_handler(ctx: &mut Context, req: Request) -> Result<Response> {
    let params = util::get_request_params(&req);
    let (svid, _, _, _, _) = parse_url_path(req.path());
    let vid = svid.parse::<u32>()?;

    debug!("post vid: {}", vid);

    let mut n = new_needle_from_request(req)?;

    debug!("post needle: {}", n);

    let size = replicate_write(ctx, &params, vid, &mut n)?;

    let mut result = operation::UploadResult::default();

    if n.has_name() {
        result.name = String::from_utf8(n.name.clone()).unwrap();
    }

    result.size = size;

    let s = serde_json::to_string(&result)?;

    debug!("post resp: {}", s);

    let response = Response::new()
        .with_header(ContentLength(s.len() as u64))
        .with_header(header::ETag(header::EntityTag::new(true, n.etag())))
        .with_body(s);

    Ok(response)
}

fn new_needle_from_request(req: Request) -> Result<Needle> {
    let path: String;
    path = req.path().to_string();

    let mut resp = parse_upload(req)?;
    debug!("parse_upload: {}", resp);

    let mut n = Needle::default();
    n.data = resp.data;

    if resp.file_name.len() > 0 {
        n.name = resp.file_name.as_bytes().to_vec();
        n.set_name();
    }

    if resp.mime_type.len() < 256 {
        n.mime = resp.mime_type.as_bytes().to_vec();
        n.set_has_mime();
    }

    // if resp.is_gzipped {
    //     n.set_gzipped();
    // }

    if resp.modified_time == 0 {
        resp.modified_time = SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    n.last_modified = resp.modified_time;
    n.set_has_last_modified_date();

    if resp.ttl.minutes() != 0 {
        n.ttl = resp.ttl;
        n.set_has_ttl();
    }

    if resp.is_chunked_file {
        n.set_is_chunk_manifest();
    }

    n.checksum = crc32::checksum_castagnoli(&n.data);

    let start = path.find(",").map(|idx| idx + 1).unwrap_or(0);
    let end = path.rfind(".").unwrap_or(path.len());

    n.parse_path(&path[start..end])?;

    Ok(n)
}

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
        data.clone(),
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
    debug!("parse url path: {}", path);
    match parts.len() {
        4 => {
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
        3 => {
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


fn replicate_write(
    ctx: &mut Context,
    params: &HashMap<String, String>,
    // master: &str,
    // s: &mut Store,
    vid: VolumeId,
    needle: &mut Needle,
) -> Result<u32> {
    let size: u32;
    {
        let mut s = ctx.store.lock().unwrap();
        size = s.write_volume_needle(vid, needle)?;
        if let Some(v) = params.get("type") {
            if v == "replicate" {
                return Ok(size);
            }
        }

        let v = s.find_volume_mut(vid).unwrap();
        if !v.need_to_replicate() {
            return Ok(size);
        }
    }

    let last_modified = needle.last_modified.to_string();
    let mut params: Vec<(&str, &str)> = vec![];
    params.push(("type", "replicate"));
    if needle.last_modified > 0 {
        params.push(("ts", &last_modified));
    }

    if needle.is_chunk_manifest() {
        params.push(("cm", "true"));
    }

    // don't support custom header pair like seaweed

    let res = ctx.looker.lock().unwrap().lookup(&vid.to_string())?;

    // TODO concurrent replicate
    for location in res.locations.iter() {
        util::post(&location.url, &params).map_err(|e| {
            storage::Error::String(e)
        })?;
    }

    Ok(size)
}
