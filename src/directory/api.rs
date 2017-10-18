use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::clone::Clone;


use iron::prelude::*;
use iron::response::Response;
use iron::request::Request;
use iron::status;
use iron;
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

// unsafe impl<S: Sequencer> Send for Context<S>{}
// unsafe impl<S: Sequencer> Sync for Context<S>{}

// impl<S: Sequencer> Clone for Context<S> {
//     fn clone(&self) -> Context<S> {
//         Context {
//             topo: self.topo.clone(),
//         }
//     }
// }


impl Context {
    pub fn get_volume_grow_option(&self, req: &mut Request) -> Result<VolumeGrowOption> {
        panic!("todo");
        Ok(VolumeGrowOption::default())
    }
}

pub fn test_handler(req: &mut Request, ctx: &Context) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>().unwrap().find("test_handler").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}

pub fn assign_handler(req: &mut Request, ctx: &Context) -> IronResult<Response> {
    let mut requestedCount: i64 = 1;

    match req.get_ref::<UrlEncodedQuery>() {
        Ok(hashmap) => {
            match hashmap.get("count") {
                Some(values) => requestedCount = values[0].parse().unwrap_or(1),
                None => (),
            };
        },
        Err(e) => {
            return Err(iron::IronError::new(e, status::BadRequest));
        }
    };

    let option = ctx.get_volume_grow_option(req)?;

    let mut topo = ctx.topo.lock().unwrap();
    if !topo.has_writable_volume(&option) {
        if topo.free_space() <= 0 {
            return Err(IronError::from(Error::NoFreeSpace));
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
        .map_err(|err| iron::IronError::new(err, status::BadRequest))?;

    Ok(Response::with((status::Ok, j)))
}
