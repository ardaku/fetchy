use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Blob, Headers};

use super::*;

#[derive(Debug)]
pub(crate) struct Fetch {
    /// Promise backed notifier for initiating fetch request
    init: LocalBoxNotify<'static, Result<JsValue, JsValue>>,
    /// Promise backed notifier for next incoming data
    read: Option<LocalBoxNotify<'static, Result<JsValue, JsValue>>>,
    /// Function to produce next read promise
    next: Option<js_sys::Function>,
    /// Value for JS `this` to pass to the function
    this: JsValue,
}

impl Fetch {
    pub(crate) fn new(
        url: &str,
        method: Method,
        headers: Vec<Header<'_>>,
        body: Vec<u8>,
    ) -> Self {
        // unwrap: Should always be a window in the DOM.
        let window = web_sys::window().unwrap();

        let mut init = web_sys::RequestInit::new();
        let headers = {
            let headers_js = Headers::new().unwrap();
            let mut name = String::new();
            let mut value = String::new();

            for header in headers.iter() {
                header.push_name(&mut name);
                header.push_value(&mut value);
                headers_js.append(&name, &value).unwrap();
                name.clear();
                value.clear();
            }

            headers_js
        };
        let body = (!body.is_empty()).then(|| -> JsValue {
            let body = js_sys::Uint8Array::from(body.as_slice());
            let array = js_sys::Array::new();

            array.push(&body.into());

            Blob::new_with_u8_array_sequence(&array).unwrap().into()
        });
        let init = init
            .method(method.as_str())
            .headers(&headers)
            .body(body.as_ref());
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
}

impl Notify for Fetch {
    type Event = Result<Option<Vec<u8>>>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Task<'_>,
    ) -> Poll<Self::Event> {
        if let Some(ref mut read) = self.read {
            // Streaming
            let params = if let Ready(params) = Pin::new(read).poll_next(cx) {
                params.unwrap()
            } else {
                return Pending;
            };

            let object: js_sys::Object = params.dyn_into().unwrap();
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
                    return Ready(Err(Error::new(response.status())));
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
                    js_sys::Object::try_from(&read_value).unwrap(),
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
                web_sys::console::error_2(
                    &"[fetchy]".to_string().into(),
                    &response.unwrap_err(),
                );
                Ready(Err(Error::Network))
            }
        } else {
            Pending
        }
    }
}
