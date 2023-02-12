// Copyright © 2022 The Fetchy Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).
//
//! Async HTTPS fetch API built on pasts.

#[cfg(feature = "web")]
mod web;
#[cfg(feature = "web")]
use web as inner;

// FIXME
/*
#[cfg(feature = "rustls")]
mod tls;
#[cfg(feature = "rustls")]
use tls as inner;
*/

use pasts::{prelude::*, Join};

/// Result type alias for fetch errors
pub type Result<T> = std::result::Result<T, Error>;

/// Method for fetching
pub enum Method {
    /// GET: Retrieve data
    Get = 0,
    /// HEAD: GET without response body
    Head = 1,
    /// POST: Add new data (may have additional side-effects)
    Post = 2,
    /// PUT: Set existing data (no additional side-effects)
    Put = 3,
    /// DELETE: Delete data
    Delete = 4,
}

impl Method {
    fn as_str(&self) -> &'static str {
        use Method::*;
        match self {
            Get => "GET",
            Head => "HEAD",
            Post => "POST",
            Put => "PUT",
            Delete => "DELETE",
        }
    }
}

/// A [`Notifier`] for fetching data from a URI.
pub struct Fetch(inner::Fetch);

impl Fetch {
    /// Create a new [`Notifier`] for fetching data from a URI.
    // FIXME: Use payload
    pub fn new(url: &str, method: Method, _payload: Vec<u8>) -> Self {
        Self(inner::Fetch::new(url, method, _payload))
    }

    /// Fetch the entire contents all at once.
    pub async fn all(
        url: &str,
        method: Method,
        payload: Vec<u8>,
    ) -> Result<Vec<u8>> {
        struct All(Fetch, Vec<u8>);

        fn fill(
            all: &mut All,
            data: Result<Option<Vec<u8>>>,
        ) -> Poll<Result<Vec<u8>>> {
            match data {
                Ok(Some(buf)) => {
                    all.1.extend(&buf);
                    Pending
                }
                Ok(None) => {
                    let mut buf = Vec::new();
                    core::mem::swap(&mut buf, &mut all.1);
                    Ready(Ok(buf))
                }
                Err(e) => Ready(Err(e)),
            }
        }

        Join::new(&mut All(Self::new(url, method, payload), Vec::new()))
            .on(|s| &mut s.0, fill)
            .await
    }
}

impl Notifier for Fetch {
    type Event = Result<Option<Vec<u8>>>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Exec<'_>,
    ) -> Poll<Self::Event> {
        Pin::new(&mut self.0).poll_next(cx)
    }
}

impl From<u16> for Error {
    fn from(other: u16) -> Self {
        use Error::*;
        match other {
            400 => Request,
            401 => Auth,
            403 => Permission,
            404 => Missing,
            405 => Method,
            406 => Unacceptable,
            407 => Proxy,
            408 => Timeout,
            409 => Conflict,
            410 => Gone,
            411 => Length,
            412 => Header,
            413 => Limit,
            414 => Uri,
            415 => Type,
            416 => Range,
            417 => Expectation,
            418 => Teapot,
            421 => Response,
            422 => Unprocessable,
            425 => Early,
            426 => Version,
            428 => Precondition,
            429 => Rate,
            431 => Oversized,
            451 => Illegal,
            500 => Server,
            501 => Unimplemented,
            502 => Gateway,
            503 => Unavailable,
            504 => Time,
            505 => Http,
            506 => Variant,
            507 => Storage,
            508 => Loop,
            510 => Extensions,
            511 => Network,
            _ => Unknown,
        }
    }
}

/// Error while fetching
pub enum Error {
    /// Network error
    Network,
    /// 400: Bad Request: Client error
    Request,
    /// 401: Unauthorized: Authentication error
    Auth,
    /// 403: Forbidden: Permission error
    Permission,
    /// 404: Not Found
    Missing,
    /// 405: Method not allowed
    Method,
    /// 406: Not acceptable
    Unacceptable,
    /// 407: Proxy Authentication
    Proxy,
    /// 408: Request timeout
    Timeout,
    /// 409: Conflict
    Conflict,
    /// 410: Content deleted
    Gone,
    /// 411: Length required
    Length,
    /// 412: Client's header preconditions failed
    Header,
    /// 413: Payload Too Large
    Limit,
    /// 414: URI too long
    Uri,
    /// 415: Unsupported media type
    Type,
    /// 416: Range not satisfiable
    Range,
    /// 417: Expectation
    Expectation,
    /// 418: I'm a teapot
    Teapot,
    /// 421: Misdirected Request
    Response,
    /// 422: Unprocessable Entity
    Unprocessable,
    /// 425: Too early
    Early,
    /// 426: Upgrade Required
    Version,
    /// 428: Precondition Required
    Precondition,
    /// 429: Too Many Requests (Rate limited)
    Rate,
    /// 431: Request Header Fields Too Large
    Oversized,
    /// 451: Unavailable For Legal Reasons
    Illegal,
    /// 500: Internal server error
    Server,
    /// 501: Not implemented
    Unimplemented,
    /// 502: Bad gateway
    Gateway,
    /// 503: Service Unavailable
    Unavailable,
    /// 504: Gateway Timeout
    Time,
    /// 505: HTTP Version Not Supported
    Http,
    /// 506: Variant Also Negotiates
    Variant,
    /// 507: Insufficient Storage
    Storage,
    /// 508: (Infinite) Loop detected
    Loop,
    /// 510: Not extended
    Extensions,
    /// 511: Network auth required
    Net,
    /// None of the other ones
    Unknown,
}
