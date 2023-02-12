use super::*;

/// Builder for [`Fetch`].
///
/// Created by [`Fetch::builder()`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FetchBuilder<'a> {
    url: &'a str,
    method: Method,
    headers: Vec<Header<'a>>,
    data: Vec<u8>,
}

impl<'a> FetchBuilder<'a> {
    /// Overwrite HTTP method.  Default is [`Method::Get`].
    pub fn method(mut self, method: Method) -> Self {
        self.method = method;

        self
    }

    /// Send data in the body of the request.
    pub fn body(mut self, data: impl Into<Vec<u8>>) -> Self {
        self.data = data.into();

        self
    }

    /// Add a header to the HTTP request.
    pub fn header(mut self, header: Header<'a>) -> Self {
        self.headers.push(header);

        self
    }

    /// Initiate the HTTP request.
    pub fn fetch(self) -> Fetch {
        Fetch::new(self.url, self.method, self.headers, self.data)
    }

    pub(crate) fn new(url: &'a str) -> Self {
        Self {
            url,
            method: Method::Get,
            headers: Vec::new(),
            data: Vec::new(),
        }
    }
}
