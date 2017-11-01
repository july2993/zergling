
use super::{Result};
use storage::errors::Error::ParseTTL;
use std::ops::Add;
// use std::marker::Clone;

#[derive(Copy, Clone, Debug)]
pub enum Unit {
    Empty = 0,
    Minute = 1,
    Hour = 2,
    Day = 3,
    Week = 4,
    Month = 5,
    Year = 6,
}

impl Default for Unit {
    fn default() -> Unit {
        Unit::Empty
    }
}

impl Unit {
    fn from_u8_idx(u: u8) -> Option<Unit> {
        match u {
            0 => Some(Unit::Empty),
            1 => Some(Unit::Minute),
            2 => Some(Unit::Hour),
            3 => Some(Unit::Day),
            4 => Some(Unit::Week),
            5 => Some(Unit::Month),
            6 => Some(Unit::Year),
            _ => None,
        }
    }

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

    pub fn minutes(&self)  -> u32 {
        match self.unit {
            Unit::Empty => 0,
            Unit::Minute => self.count as u32,
            Unit::Hour => self.count as u32 * 60,
            Unit::Day => self.count as u32 * 60 * 24,
            Unit::Week => self.count as u32 * 60 * 24 * 7,
            Unit::Month => self.count as u32 * 60 * 24 * 31,
            Unit::Year => self.count as u32 * 60 * 24 * 365,
        }
    }

}

impl From<u32> for TTL {
    fn from(u: u32) -> Self {
        let mut vec: Vec<u8> = vec![];
        vec.push((u % 0xff) as u8);
        vec.push(((u >> 8) % 0xff) as u8);

        TTL::from(vec)
    }
}

impl From<Vec<u8>> for TTL {
    fn from(u: Vec<u8>) -> Self {
        TTL {
            count: u[0],
            unit: Unit::from_u8_idx(u[1]).unwrap(),
        }
    }
}

