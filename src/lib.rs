//! Async HTTPS fetch API built on pasts.

#![doc(
    html_logo_url = "https://ardaku.github.io/mm/logo.svg",
    html_favicon_url = "https://ardaku.github.io/mm/icon.svg",
    html_root_url = "https://docs.rs/fetchy"
)]
#![forbid(unsafe_code)]
#![warn(
    anonymous_parameters,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_qualifications,
    variant_size_differences,

)]

mod error;
mod fetch;
mod method;
mod builder;
mod header;

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
    builder::FetchBuilder,
    method::Method,
    header::Header,
};
