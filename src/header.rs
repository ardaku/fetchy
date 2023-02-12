use std::borrow::Cow;

/// HTTP header
// FIXME: Better type-safety for headers
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Header<'a> {
    /// [`Content-Type`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Type)
    ContentType(Cow<'a, str>),
    /// [`Content-Encoding`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Encoding)
    ContentEncoding(Cow<'a, str>),
    /// [`Content-Language`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Language)
    ContentLanguage(Cow<'a, str>),
    /// [`Content-Location`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Location)
    ContentLocation(Cow<'a, str>),
    /// `Host`
    Host(Cow<'a, str>),
    /// `User-Agent`
    UserAgent(Cow<'a, str>),
    /// `Accept`
    Accept(Cow<'a, str>),
    /// `Accept-Language`
    AcceptLanguage(Cow<'a, str>),
    /// `Accept-Encoding`
    AcceptEncoding(Cow<'a, str>),
    /// `Referer`
    Referer(Cow<'a, str>),
    /// `Connection`
    Connection(Cow<'a, str>),
    /// `Upgrade-Insecure-Requests`
    UpgradeInsecureRequests(Cow<'a, str>),
    /// `If-Modified-Since`
    IfModifiedSince(Cow<'a, str>),
    /// `If-None-Match`
    IfNoneMatch(Cow<'a, str>),
    /// `Cache-Control`
    CacheControl(Cow<'a, str>),
}
