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
    pub(crate) fn as_str(&self) -> &'static str {
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
