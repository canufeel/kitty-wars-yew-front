use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{JsFuture};
use js_sys::Promise;

#[wasm_bindgen(raw_module = "../js/contracts/setup")]
extern "C" {
  fn joinAndLoot() -> Promise;
  fn createBattle() -> Promise;
  fn joinBattle(battle_id: u32) -> Promise;
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

pub async fn join_and_loot() -> Result<JsValue, JsValue> {
  let promise = joinAndLoot();
  let future = JsFuture::from(promise);
  future.await
}


pub async fn crate_battle() -> Result<JsValue, JsValue> {
  let promise = createBattle();
  let future = JsFuture::from(promise);
  future.await
}


pub async fn join_battle(battle_id: u32) -> Result<JsValue, JsValue> {
  let promise = joinBattle(battle_id);
  let future = JsFuture::from(promise);
  future.await
}