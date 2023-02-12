//! Async HTTPS fetch API built on pasts.

mod error;
mod fetch;
mod method;

#[cfg(feature = "web")]
mod web;

// FIXME
/*
#[cfg(feature = "rustls")]
mod tls;
#[cfg(feature = "rustls")]
use tls as inner;
*/

use pasts::{prelude::*, Join};
#[cfg(feature = "web")]
use web as inner;

pub use self::{
    error::{Error, Result},
    fetch::Fetch,
    method::Method,
};
