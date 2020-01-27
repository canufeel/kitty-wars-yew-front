use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{JsFuture, spawn_local};
use js_sys::Promise;
use futures::executor::block_on;

#[wasm_bindgen(raw_module = "../js/contracts/setup")]
extern "C" {
  fn loot();
  fn join();
}

#[wasm_bindgen(raw_module = "../js/actions/bootstrap")]
extern "C" {
  fn appBoot() -> Promise;
}

pub async fn boot_app() -> Result<JsValue, JsValue> {
  let promise = appBoot();
  let future = JsFuture::from(promise);
  future.await
}