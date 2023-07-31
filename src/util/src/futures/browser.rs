use std::future::Future;
use wasm_bindgen::JsValue;

pub struct SendableJsFuture {
    inner: wasm_bindgen_futures::JsFuture,
}

impl From<js_sys::Promise> for SendableJsFuture {
    fn from(value: js_sys::Promise) -> Self {
        Self {
            inner: wasm_bindgen_futures::JsFuture::from(value),
        }
    }
}

impl Future for SendableJsFuture {
    type Output = Result<JsValue, JsValue>;
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        std::pin::pin!(self.inner).poll(cx)
    }
}

unsafe impl Send for SendableJsFuture {
}