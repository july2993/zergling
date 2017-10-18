
use super::{Error, Result};
use storage::errors::Error::ParseTTL;
use std::ops::Add;
// use std::marker::Clone;

#[derive(Copy, Clone, Debug)]
pub enum Unit {
    Empty,
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Year,
}

impl Default for Unit {
    fn default() -> Unit {
        Unit::Empty
    }
}

impl Unit {
    fn new(u: u8) -> Option<Unit> {
        match char::from(u) {
            'm' => Some(Unit::Minute),
            'h' => Some(Unit::Hour),
            'd' => Some(Unit::Day),
            'w' => Some(Unit::Week),
            'M' => Some(Unit::Month),
            'y' => Some(Unit::Year),
            _ => None,
        }
    }

    fn string(&self) -> String {
        match self {
            Minute => String::from("m"),
            Hour => String::from("h"),
            Day => String::from("d"),
            Week => String::from("w"),
            Month => String::from("M"),
            Year => String::from("y"),
            _ => String::from(""),
        }
    }
}

#[derive(Debug, Copy, Default)]
pub struct TTL {
    pub Count: u8,
    pub Unit: Unit,
}

impl Clone for TTL {
    fn clone(&self) -> TTL {
        TTL {
            Count: self.Count,
            Unit: self.Unit,
        }
    }
}


// default: m
// 3m
// 4h
// 5d
// 6w
// 7M
// 8y

impl TTL {
    fn new(s: String) -> Result<TTL> {
        if s.is_empty() {
            return Err(ParseTTL(String::from(s)));
        }

        let bytes = s.as_bytes();

        let mut unit = bytes[bytes.len() - 1];
        let mut countBytes = &bytes[..bytes.len() - 1];

        if unit >= '0'  as u8 && unit <= '9' as u8 {
            unit = 'm' as u8;
            countBytes = bytes;
        }

        if let Ok(count) = String::from_utf8(countBytes.to_vec()).unwrap().parse::<u8>() {
            if let Some(unit) = Unit::new(unit) {
                let ttl = TTL {
                    Count: count,
                    Unit: unit,
                };
                return Ok(ttl);
            }

        }

        return Err(ParseTTL(s.clone()));
    }

    pub fn string(&self) -> String {
        if self.Count == 0 {
            return String::from("");
        }

        let mut s = self.Count.to_string();

        s = s.add(&self.Unit.string());

        s
    }


}
