use lru::LruCache;
use std::time::{SystemTime, Duration};
use super::{Result, Error};
use util;
use serde_json;
use std::collections::HashMap;


#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub url: String,
    pub public_url: String,
}


#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LookupResult {
    pub volume_id: String,
    pub locations: Vec<Location>,
    pub error: String,
}


pub struct Looker {
    server: String,
    cache: HashMap<String, (LookupResult, SystemTime)>,
    timeout: Duration,
}

impl Looker {
    pub fn new(server: &str) -> Looker {
        Looker {
            server: String::from(server),
            // should bigger than volume number
            cache: HashMap::new(),
            timeout: Duration::new(600, 0),
        }
    }

    pub fn lookup(&mut self, vid: &str) -> Result<LookupResult> {
        let now = SystemTime::now();
        if let Some(c) = self.cache.get(&String::from(vid)) {
            if now.duration_since(c.1).unwrap().lt(&self.timeout) {
                return Ok(c.0.clone());
            }
        }

        match self.do_lookup(vid) {
            Ok(res) => {
                let res2 = res.clone();
                self.cache.insert(String::from(vid), (res, now));
                Ok(res2)
            }
            Err(e) => Err(e),
        }
    }

    fn do_lookup(&mut self, vid: &str) -> Result<LookupResult> {
        let mut params: Vec<(&str, &str)> = vec![];
        params.push(("volumeId", vid));
        let body = util::post(&format!("http://{}/dir/lookup", self.server), &params)?;
        let res: LookupResult = serde_json::from_slice(&body)?;
        Ok(res)
    }
}
