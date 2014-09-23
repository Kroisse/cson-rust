// This is a part of CSON-rust.
// Written by Kang Seonghoon. See README.md for details.

//! An internal representation of CSON data.

use std::str::MaybeOwned;
use std::collections::TreeMap;
use serialize::json;
use serialize::json::ToJson;

#[deriving(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Slice<'a>(&'a str);

impl<'a> Slice<'a> {
    pub fn new(base: &'a str, start: uint, end: uint) -> Slice<'a> {
        Slice(base.slice(start, end))
    }
}

impl<'a> Str for Slice<'a> {
    fn as_slice<'a>(&'a self) -> &'a str {
        let Slice(slice) = *self;
        slice
    }
}

#[deriving(PartialEq, Show, Clone)]
pub enum Atom<'a> {
    Null,
    True,
    False,
    //UnparsedF64(Slice<'a>),
    I64(i64),
    U64(u64),
    F64(f64),
    //UnparsedString(Slice<'a>),
    //ParsedString(Slice<'a>),
    OwnedString(String),
    List(AtomList<'a>),
    Object(AtomObject<'a>),
}

pub type Key<'a> = MaybeOwned<'a>;
pub type AtomList<'a> = Vec<Atom<'a>>;
pub type AtomObject<'a> = TreeMap<Key<'a>, Atom<'a>>;

impl<'a> Atom<'a> {
    pub fn from_json<T: ToJson>(jsonlike: &T) -> Atom<'a> {
        Atom::from_owned_json(jsonlike.to_json())
    }

    pub fn from_owned_json(json: json::Json) -> Atom<'a> {
        match json {
            json::I64(v) => I64(v),
            json::U64(v) => U64(v),
            json::F64(v) => F64(v),
            json::String(s) => OwnedString(s),
            json::Boolean(true) => True,
            json::Boolean(false) => False,
            json::List(l) => List(l.into_iter().map(Atom::from_owned_json).collect()),
            json::Object(o) =>
                Object(o.into_iter().map(|(k,v)| (k.into_maybe_owned(),
                                                  Atom::from_owned_json(v))).collect()),
            json::Null => Null,
        }
    }

    pub fn into_parsed(self) -> Atom<'a> {
        match self {
            Null => Null,
            True => True,
            False => False,
            I64(v) => I64(v),
            U64(v) => U64(v),
            F64(v) => F64(v),
            OwnedString(s) => OwnedString(s),
            List(l) => List(l.into_iter().map(|e| e.into_parsed()).collect()),
            Object(o) => Object(o.into_iter().map(|(k,v)| (k,v.into_parsed())).collect()),
        }
    }

    pub fn into_owned(self) -> Atom<'static> {
        match self {
            Null => Null,
            True => True,
            False => False,
            I64(v) => I64(v),
            U64(v) => U64(v),
            F64(v) => F64(v),
            OwnedString(s) => OwnedString(s),
            List(l) => List(l.into_iter().map(|e| e.into_owned()).collect()),
            Object(o) => Object(o.into_iter().map(|(k,v)| (k.into_string().into_maybe_owned(),
                                                           v.into_owned())).collect()),
        }
    }
}

impl<'a> ToJson for Atom<'a> {
    fn to_json(&self) -> json::Json {
        match *self {
            Null => json::Null,
            True => json::Boolean(true),
            False => json::Boolean(false),
            I64(v) => json::I64(v),
            U64(v) => json::U64(v),
            F64(v) => json::F64(v),
            OwnedString(ref s) => json::String(s.clone()),
            List(ref l) => json::List(l.iter().map(|e| e.to_json()).collect()),
            Object(ref o) => json::Object(o.iter().map(|(k,v)| (k.to_string(),
                                                                v.to_json())).collect()),
        }
    }
}

