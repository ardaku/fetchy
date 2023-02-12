use std::borrow::Cow;

/// HTTP header
///
/// Does not include
/// [Forbidden header names](https://developer.mozilla.org/en-US/docs/Glossary/Forbidden_header_name).
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
    /// `User-Agent`
    UserAgent(Cow<'a, str>),
    /// `Accept`
    Accept(Cow<'a, str>),
    /// `Accept-Language`
    AcceptLanguage(Cow<'a, str>),
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

impl Header<'_> {
    pub(crate) fn push_name(&self, output: &mut String) {
        use Header::*;
        match self {
            ContentType(_) => output.push_str("Content-Type"),
            ContentEncoding(_) => output.push_str("Content-Encoding"),
            ContentLanguage(_) => output.push_str("Content-Language"),
            ContentLocation(_) => output.push_str("Content-Location"),
            UserAgent(_) => output.push_str("User-Agent"),
            Accept(_) => output.push_str("Accept"),
            AcceptLanguage(_) => output.push_str("Accept-Language"),
            Connection(_) => output.push_str("Connection"),
            UpgradeInsecureRequests(_) => {
                output.push_str("Upgrade-Insecure-Requests")
            }
            IfModifiedSince(_) => output.push_str("If-Modified-Since"),
            IfNoneMatch(_) => output.push_str("If-None-Match"),
            CacheControl(_) => output.push_str("Cache-Control"),
        }
    }

    pub(crate) fn push_value(&self, output: &mut String) {
        use Header::*;
        match self {
            ContentType(v) => output.push_str(v),
            ContentEncoding(v) => output.push_str(v),
            ContentLanguage(v) => output.push_str(v),
            ContentLocation(v) => output.push_str(v),
            UserAgent(v) => output.push_str(v),
            Accept(v) => output.push_str(v),
            AcceptLanguage(v) => output.push_str(v),
            Connection(v) => output.push_str(v),
            UpgradeInsecureRequests(v) => output.push_str(v),
            IfModifiedSince(v) => output.push_str(v),
            IfNoneMatch(v) => output.push_str(v),
            CacheControl(v) => output.push_str(v),
        }
    }
}
