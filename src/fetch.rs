use super::*;

/// A [`Notifier`] for fetching data from a URL.
#[derive(Debug)]
pub struct Fetch(inner::Fetch);

impl Fetch {
    /// Create a new [`Notifier`] for fetching data from a URL.
    // FIXME: Use payload
    pub fn new(url: &str, method: Method, _payload: Vec<u8>) -> Self {
        Self(inner::Fetch::new(url, method, _payload))
    }

    /// Get a builder for configuring the fetch request.
    pub fn builder(url: &str) -> FetchBuilder<'_> {
        FetchBuilder::new(url)
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
        task: &mut Task<'_>,
    ) -> Poll<Self::Event> {
        Pin::new(&mut self.0).poll_next(task)
    }
}