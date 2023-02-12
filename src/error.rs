use core::result::Result as CoreResult;

/// Result type alias for fetch errors
pub type Result<T = (), E = Error> = CoreResult<T, E>;

/// Error while fetching
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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

impl Error {
    pub(crate) fn new(http_code: u16) -> Self {
        use Error::*;
        match http_code {
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
