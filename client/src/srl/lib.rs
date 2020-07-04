#![deny(clippy::all)]
#![allow(clippy::type_complexity)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod errors;
mod protocol;
