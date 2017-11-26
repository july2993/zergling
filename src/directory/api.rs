use std;
use std::{thread, time};
use std::boxed::FnBox;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::clone::Clone;
use std::error::Error as STDError;
use futures::future::Future;
use futures::future;
use futures;
use futures::sync::oneshot;
use hyper::{self, StatusCode};
use serde::Serialize;
use hyper::header::ContentLength;
use hyper::server::{Request, Response, Service};
use hyper::Method;
use operation::ClusterStatusResult;
use util;
use serde_json;
use storage;
use super::topology::*;
use directory::errors::Error;
use directory::Result;
use futures_cpupool::CpuPool;
use operation::*;
use metrics::*;

pub type APICallback = Box<FnBox(Result<Response>) + Send>;

pub enum Msg {
    API { req: Request, cb: APICallback },
}

pub fn make_callback()
    -> (Box<FnBox(Result<Response>) + Send>, oneshot::Receiver<Result<Response>>)
{
    let (tx, rx) = oneshot::channel();
    let callback = move |resp| { tx.send(resp).unwrap(); };
    (Box::new(callback), rx)
}

#[derive(Debug, Clone)]
pub struct Context {
    pub topo: Arc<Mutex<Topology>>,
    pub vg: Arc<Mutex<VolumeGrow>>,
    pub default_replica_placement: storage::ReplicaPlacement,
    pub ip: String,
    pub port: u16,
    pub cpu_pool: CpuPool,
}

#[derive(Clone)]
pub struct HTTPContext {
    pub sender: Sender<Msg>,
}


impl Context {
    pub fn get_volume_grow_option(
        &self,
        params: &HashMap<String, String>,
    ) -> Result<VolumeGrowOption> {
        let mut option = VolumeGrowOption::default();
        option.replica_placement = self.default_replica_placement;
        if let Some(rp) = params.get("replication") {
            option.replica_placement = storage::ReplicaPlacement::new(rp)?;
        }

        if let Some(ttl) = params.get("ttl") {
            option.ttl = storage::TTL::new(ttl)?;
        }

        if let Some(preallocate) = params.get("preallocate") {
            option.preallocate = preallocate.parse()?;
        }

        Ok(option)
    }

    pub fn run(&mut self, receiver: Receiver<Msg>) {
        let alrecv = Arc::new(Mutex::new(receiver));

        let mut threads = vec![];
        for _i in 0..16 {
            let mut ctx = self.clone();
            let recv = alrecv.clone();
            let t = thread::spawn(move || loop {
                match recv.lock().unwrap().recv() {
                    Ok(msg) => ctx.handle_msg(msg),
                    Err(err) => {
                        info!("recv msg err: {}", err);
                        return;
                    }
                };
            });
            threads.push(t);
        }
        for t in threads {
            t.join().unwrap();
        }
        info!("handle msg quit...");
    }


    fn handle_msg(&mut self, msg: Msg) {
        match msg {
            Msg::API { req, cb } => {
                debug!("hanle msg: [{}] {}", req.method(), req.path());
                let method = req.method().clone();

                let path = req.path().to_string();
                match (&method, path.as_ref()) {
                    // seaweedfs will call this to check wheather master is alive
                    (&Method::Get, "/stats") => cb(Ok(Response::new())),
                    (&Method::Get, "/dir/assign") |
                    (&Method::Post, "/dir/assign") => {
                        let handle = assign_handler(&req, self);
                        cb(handle);
                    }
                    (&Method::Get, "/dir/lookup") |
                    (&Method::Post, "/dir/lookup") => {
                        let handle = lookup_handler(req, self);
                        cb(handle);
                    }
                    (&Method::Get, "/dir/status") => {
                        let handle = dir_status_handler(req, self);
                        cb(handle);
                    }
                    (&Method::Get, "/cluster/status") => {
                        let handle = culster_status_handler(&req, self);
                        cb(handle);
                    }
                    (&Method::Get, "/metrics") => {
                        let handle = Ok(util::metrics_handler(&req));
                        cb(handle);
                    }
                    (method, path) => {
                        warn!("unknow request: [{}] {}", method, path);
                        let res = Ok(
                            Response::new()
                                .with_header(ContentLength(PHRASE.len() as u64))
                                .with_body(PHRASE),
                        );
                        cb(res);
                    }
                }
            }
        }
    }
}

const PHRASE: &'static str = "Hello, World!";


fn get_params(req: &Request) -> Result<HashMap<String, String>> {
    Ok(util::get_request_params(req))
}

fn dir_status_handler(_req: Request, ctx: &Context) -> Result<Response> {
    // no clone will not impl Serialize for MutexGard***...
    let topo = ctx.topo.lock().unwrap().clone();
    let j = serde_json::to_string(&topo)?;
    Ok(
        Response::new()
            .with_header(ContentLength(j.len() as u64))
            .with_body(j),
    )
}

fn lookup_handler(req: Request, ctx: &Context) -> Result<Response> {
    let params = util::get_form_params(req);
    let mut vid = match params.get("volumeId") {
        Some(s) => s.clone(),
        None => {
            return Ok(util::error_json_response("no volumeId params"));
        }
    };

    let idx = vid.rfind(",");
    if idx.is_some() {
        vid = vid[..idx.unwrap()].to_string();
    }

    let collection = params
        .get("collection")
        .map(|v| v.clone())
        .unwrap_or_default();

    let mut topo = ctx.topo.lock().unwrap();
    let mut locations = vec![];
    if let Some(nodes) = topo.lookup(collection, vid.parse::<u32>()?) {
        for ncell in nodes.iter() {
            // let a: u8 = ncell;
            let n = ncell.borrow();
            locations.push(Location {
                url: n.url(),
                public_url: n.public_url.clone(),
            });
        }

        let result = LookupResult {
            volume_id: vid,
            locations: locations,
            error: String::new(),
        };
        util::json_response(StatusCode::Accepted, &result).map_err(Error::from)
    } else {
        return Ok(util::error_json_response("cant't find any locations"));
    }
}

pub fn assign_handler(req: &Request, ctx: &Context) -> Result<Response> {
    let mut requested_count: u64 = 1;

    let params = get_params(req)?;

    match params.get("count") {
        Some(value) => requested_count = value.parse().unwrap_or(1),
        None => (),
    };

    let option = ctx.get_volume_grow_option(&params)?;
    debug!("get option: {:?}", option);

    let mut topo = ctx.topo.lock().unwrap();
    if !topo.has_writable_volume(&option) {
        debug!("no writable volume");
        if topo.free_volumes() <= 0 {
            return Err(Error::NoFreeSpace(String::from("no writable volume")));
        }

        let vg = ctx.vg.lock().unwrap();

        vg.grow_by_type(&option, &mut topo)?;
    }

    let (fid, count, node) = topo.pick_for_write(requested_count, &option)?;

    let dn = node.borrow();
    let assign_resp = AssignResult {
        fid: fid,
        url: dn.url(),
        publicUrl: dn.public_url.clone(),
        count: count,
        error: String::from(""),
    };

    let j = serde_json::to_string(&assign_resp).map_err(Error::from)?;

    Ok(
        Response::new()
            .with_header(ContentLength(j.len() as u64))
            .with_body(j),
    )
}

pub fn culster_status_handler(_req: &Request, ctx: &Context) -> Result<Response> {
    let res = ClusterStatusResult {
        IsLeader: true,
        Leader: format!("{}:{}", ctx.ip, ctx.port),
        Peers: vec![],
    };

    let j = serde_json::to_string(&res).map_err(Error::from)?;

    Ok(
        Response::new()
            .with_header(ContentLength(j.len() as u64))
            .with_body(j),
    )
}

impl Service for HTTPContext {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;

    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let method = req.method().clone();
        HTTP_REQ_COUNTER_VEC
            .with_label_values(&["all", method.as_ref()])
            .inc_by(1.0)
            .unwrap();
        let timer = HTTP_REQ_HISTOGRAM_VEC
            .with_label_values(&["all", method.as_ref()])
            .start_coarse_timer();

        let (cb, future) = make_callback();
        self.sender.send(Msg::API { req, cb }).unwrap();


        let future = future
            .map_err(|_err| {
                // _err is futures::Canceled
                hyper::Error::Timeout
            })
            .map(|v| match v {
                Ok(resp) => resp,
                Err(err) => {
                    debug!("err: {:?}", err);
                    let s = format!("{{\"error\": \"{}\"}}", err.description());
                    Response::new()
                        .with_status(StatusCode::NotAcceptable)
                        .with_header(ContentLength(s.len() as u64))
                        .with_body(s)
                }
            }).map(move |v| {
                timer.observe_duration();
                v
            });
        Box::new(future)
    }
}
