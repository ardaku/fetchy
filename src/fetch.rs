use super::*;

/// A [`Notify`] for fetching data from a URL.
#[derive(Debug)]
pub struct Fetch(inner::Fetch);

impl Fetch {
    /// Create a new [`Notifier`] for fetching data from a URL.
    // FIXME: Use payload
    pub(crate) fn new(
        url: &str,
        method: Method,
        headers: Vec<Header<'_>>,
        body: Vec<u8>,
    ) -> Self {
        Self(inner::Fetch::new(url, method, headers, body))
    }

    /// Get a builder for configuring the fetch request.
    pub fn builder(url: &str) -> FetchBuilder<'_> {
        FetchBuilder::new(url)
    }

    /// Fetch the entire contents all at once.
    pub async fn all(self) -> Result<Vec<u8>> {
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

        Loop::new(&mut All(self, Vec::new()))
            .on(|s| &mut s.0, fill)
            .await
    }
}

impl Notify for Fetch {
    type Event = Result<Option<Vec<u8>>>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        task: &mut Task<'_>,
    ) -> Poll<Self::Event> {
        Pin::new(&mut self.0).poll_next(task)
    }
}
