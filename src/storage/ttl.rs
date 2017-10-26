
use super::{Result};
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
        match *self {
            Unit::Minute => String::from("m"),
            Unit::Hour => String::from("h"),
            Unit::Day => String::from("d"),
            Unit::Week => String::from("w"),
            Unit::Month => String::from("M"),
            Unit::Year => String::from("y"),
            _ => String::from(""),
        }
    }
}

#[derive(Debug, Copy, Default)]
pub struct TTL {
    pub count: u8,
    pub unit: Unit,
}

impl Clone for TTL {
    fn clone(&self) -> TTL {
        TTL {
            count: self.count,
            unit: self.unit,
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
    pub fn new(s: &str) -> Result<TTL> {
        if s.is_empty() {
            return Err(ParseTTL(String::from(s)));
        }

        let bytes = s.as_bytes();

        let mut unit = bytes[bytes.len() - 1];
        let mut count_bytes = &bytes[..bytes.len() - 1];

        if unit >= '0'  as u8 && unit <= '9' as u8 {
            unit = 'm' as u8;
            count_bytes = bytes;
        }

        if let Ok(count) = String::from_utf8(count_bytes.to_vec()).unwrap().parse::<u8>() {
            if let Some(unit) = Unit::new(unit) {
                let ttl = TTL {
                    count: count,
                    unit: unit,
                };
                return Ok(ttl);
            }

        }

        return Err(ParseTTL(String::from(s)));
    }

    pub fn string(&self) -> String {
        if self.count == 0 {
            return String::from("");
        }

        let mut s = self.count.to_string();

        s = s.add(&self.unit.string());

        s
    }


}
