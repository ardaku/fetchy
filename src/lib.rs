use pasts::{prelude::*, Join};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::Headers;

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
pub struct Fetch {
    /// Promise backed notifier for initiating fetch request
    init: Local<'static, std::result::Result<JsValue, JsValue>>,
    /// Promise backed notifier for next incoming data
    read: Option<Local<'static, std::result::Result<JsValue, JsValue>>>,
    /// Function to produce next read promise
    next: Option<js_sys::Function>,
    /// Value for JS `this` to pass to the function
    this: JsValue,
}

impl Fetch {
    /// Create a new [`Notifier`] for fetching data from a URI.
    // FIXME: Use payload
    pub fn new(url: &str, method: Method, _payload: Vec<u8>) -> Self {
        // unwrap: Should always be a window in the DOM.
        let window = web_sys::window().unwrap();

        let mut init = web_sys::RequestInit::new();
        let headers = Headers::new().unwrap();
        let init = init.method(method.as_str()).headers(&headers);
        let promise = window.fetch_with_str_and_init(url, init);
        let future = wasm_bindgen_futures::JsFuture::from(promise);
        let init = Box::<Option<_>>::pin(future.fuse());
        let read = None;
        let next = None;
        let this = JsValue::NULL;

        Self {
            init,
            read,
            next,
            this,
        }
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

impl Notifier for Fetch {
    type Event = Result<Option<Vec<u8>>>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Exec<'_>,
    ) -> Poll<Self::Event> {
        if let Some(ref mut read) = self.read {
            // Streaming
            let params = if let Ready(params) = Pin::new(read).poll_next(cx) {
                params.unwrap()
            } else {
                return Pending;
            };

            let object: js_sys::Object = params.clone().dyn_into().unwrap();
            let list = js_sys::Object::entries(&object);
            let found = list.find(&mut |value, _index, _array| {
                let array: js_sys::Array = value.dyn_into().unwrap();
                array.at(0) == JsValue::from_str("done")
            });
            let found: js_sys::Array = found.dyn_into().unwrap();
            let done: js_sys::Boolean = found.at(1).dyn_into().unwrap();
            let done = done.value_of();

            if done {
                Ready(Ok(None))
            } else {
                // Get new promise
                let promise: js_sys::Promise = self
                    .next
                    .as_ref()
                    .unwrap()
                    .call0(&self.this)
                    .unwrap()
                    .into();
                let future = wasm_bindgen_futures::JsFuture::from(promise);
                self.read = Some(Box::pin(future.fuse()));

                // Return data
                let found = list.find(&mut |value, _index, _array| {
                    let array: js_sys::Array = value.dyn_into().unwrap();
                    array.at(0) == JsValue::from_str("value")
                });
                let found: js_sys::Array = found.dyn_into().unwrap();
                let data: js_sys::Uint8Array = found.at(1).dyn_into().unwrap();
                Ready(Ok(Some(data.to_vec())))
            }
        } else if let Ready(response) = Pin::new(&mut self.init).poll_next(cx) {
            // Connected, and ready to stream
            if let Ok(response) = response {
                let response = web_sys::Response::from(response);
                if !response.ok() {
                    return Ready(Err(Error::from(response.status())));
                }
                let reader = if let Some(body) = response.body() {
                    body.get_reader()
                } else {
                    return Ready(Ok(None));
                };
                let proto = js_sys::Object::get_prototype_of(&reader);
                let read_value = js_sys::Object::get_own_property_descriptor(
                    &proto,
                    &JsValue::from_str("read"),
                );

                let read_value = js_sys::Object::entries(
                    &js_sys::Object::try_from(&read_value).unwrap(),
                );

                let found = read_value.find(&mut |value, _index, _array| {
                    let array: js_sys::Array = value.dyn_into().unwrap();
                    array.at(0) == JsValue::from_str("value")
                });
                let array: js_sys::Array = found.dyn_into().unwrap();
                let found = array.at(1);
                let read_fn: js_sys::Function = found.dyn_into().unwrap();

                let promise: js_sys::Promise =
                    read_fn.call0(&reader).unwrap().into();
                let future = wasm_bindgen_futures::JsFuture::from(promise);

                self.read = Some(Box::pin(future.fuse()));
                self.next = Some(read_fn);
                self.this = (*reader).clone();
                self.poll_next(cx)
            } else {
                Ready(Err(Error::Network))
            }
        } else {
            Pending
        }
    }
}
