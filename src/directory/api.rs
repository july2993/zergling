use std;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::clone::Clone;
use std::error::Error as STDError;


use futures::future::Future;
use futures::future;
use futures;
use hyper;
use hyper::header::ContentLength;
use hyper::server::{Http, Request, Response, Service};
use hyper::{Method, StatusCode};
use url::Url;



use router::Router;
use serde_json;

use urlencoded::UrlEncodedQuery;

use directory::sequencer::{Sequencer,MemorySequencer};
use super::topology::*;
use directory::errors::Error;
use directory::{Result};
use operation::*;



#[derive(Debug, Clone)]
pub struct Context {
    pub topo: Arc<Mutex<Topology>>,    
    pub vg: Arc<Mutex<VolumeGrow>>,
}


impl Context {
    pub fn get_volume_grow_option(&self, req: &Request) -> Result<VolumeGrowOption> {
        panic!("todo");
        Ok(VolumeGrowOption::default())
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
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        match (req.method(), req.path()) {
            (&Method::Get, "/dir/assign") => {
                let handle = assign_handler(&req, self);
                let ret = future::result(map_err(handle));
                Box::new(ret)
            },
            _ => {
                Box::new(futures::future::ok(
                    Response::new()
                        .with_header(ContentLength(PHRASE.len() as u64))
                        .with_body(PHRASE))
                )
            }
        }
    }
}

fn map_err(r: Result<Response>) -> std::result::Result<Response, hyper::Error> {
    match r {
        Ok(resp) => Ok(resp), 
        Err(err) => {
            let s = format!("{{\"Error\": {}}}", err.description());    
            Ok(Response::new()
                    .with_header(ContentLength(s.len() as u64))
                    .with_body(s))
        }
    }
}

fn get_params(req: &Request) -> Result<HashMap<String, String>> {
    let s = format!("{}", req.uri());
    let url = Url::parse(&s)?;
    let pairs = url.query_pairs().into_owned();
    Ok(pairs.collect())
}

pub fn assign_handler(req: &Request, ctx: &Context) -> Result<Response> {
    let mut requestedCount: i64 = 1;

    let params = get_params(req)?;

    match params.get("count") {
       Some(value) => requestedCount = value.parse().unwrap_or(1),
       None => (),
    };

    let option = ctx.get_volume_grow_option(req)?;


    let mut topo = ctx.topo.lock().unwrap();
    if !topo.has_writable_volume(&option) {
        if topo.free_volumes() <= 0 {
            return Err(Error::NoFreeSpace);
        }

        let mut vg = ctx.vg.lock().unwrap();

        vg.grow_by_type(&option, &topo)?;
        
    }
    
    let (fid, count, dn) = topo.pick_for_write(requestedCount, &option)?;
    let assign_resp = AssignResult {
        fid: dn.id.clone(),
        url: dn.url(),
        publicUrl: dn.public_url,
        count: count,
        error: String::from(""),
    };

    let j = serde_json::to_string(&assign_resp)
        .map_err(Error::from)?;

    Ok(Response::new()
           .with_header(ContentLength(j.len() as u64))
           .with_body(j))
}
