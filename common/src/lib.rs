#![warn(clippy::pedantic)]

#[macro_use]
extern crate serde;
#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate async_trait;

pub mod config;
pub mod eventing;
pub mod logging;

// TODO: Write common logger setup
