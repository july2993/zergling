use std;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::clone::Clone;
use std::error::Error as STDError;


use futures::future::Future;
use futures::future;
use futures;
use hyper::{self, StatusCode};
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



#[derive(Debug, Clone)]
pub struct Context {
    pub topo: Arc<Mutex<Topology>>,
    pub vg: Arc<Mutex<VolumeGrow>>,
    pub default_replica_placement: storage::ReplicaPlacement,
    pub ip: String,
    pub port: u16,
    pub cpu_pool: CpuPool,
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
}

const PHRASE: &'static str = "Hello, World!";

impl Service for Context {
    // boilerplate hooking up hyper's server types
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    // The future representing the eventual Response your call will
    // resolve to. This can change to whatever Future you need.
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        debug!(
            "request [{}] {} headers: {}",
            req.method(),
            req.path(),
            req.headers()
        );
        let method = &req.method().clone();
        let tmp = req.path().to_string();
        let path = tmp.as_str();
        match (method, path) {
            // seaweedfs will call this to check wheather master is alive
            (&Method::Get, "/stats") => Box::new(futures::future::ok(Response::new())),
            (&Method::Get, "/dir/assign") |
            (&Method::Post, "/dir/assign") => {
                let handle = assign_handler(&req, self);
                let ret = future::result(map_err(handle));
                Box::new(ret)
            }
            (&Method::Get, "/dir/lookup") |
            (&Method::Post, "/dir/lookup") => {
                let ctx = self.clone();
                let ret = self.cpu_pool.spawn_fn(move || {
                    let handle = lookup_handler(req, &ctx);
                    future::result(map_err(handle))
                    // Box::new(ret)
                });
                Box::new(ret)
            }
            (&Method::Get, "/cluster/status") => {
                let handle = culster_status_handler(&req, self);
                let ret = future::result(map_err(handle));
                Box::new(ret)
            }
            (method, path) => {
                warn!("unknow request: [{}] {}", method, path);
                Box::new(futures::future::ok(
                    Response::new()
                        .with_header(ContentLength(PHRASE.len() as u64))
                        .with_body(PHRASE),
                ))
            }
        }
    }
}



fn map_err(r: Result<Response>) -> std::result::Result<Response, hyper::Error> {
    match r {
        Ok(resp) => Ok(resp),
        Err(err) => {
            debug!("err: {:?}", err);
            let s = format!("{{\"error\": \"{}\"}}", err.description());
            Ok(
                Response::new()
                    .with_header(ContentLength(s.len() as u64))
                    .with_body(s),
            )
        }
    }
}


fn get_params(req: &Request) -> Result<HashMap<String, String>> {
    Ok(util::get_request_params(req))
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
