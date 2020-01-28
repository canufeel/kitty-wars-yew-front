use js_sys::{JsString, Reflect, Object};
use crate::game::{PlayerState, PlayerItems};
use wasm_bindgen::JsValue;
use std::collections::HashMap;

pub fn parse_js_value_prop_as_string(obj: &JsValue, key: &str) -> Result<String, JsString> {
  let js_val = Reflect::get(obj, &JsValue::from_str(key))?;
  match js_val.as_string() {
    Some(v) => Ok(v),
    None => Err(JsString::from("Property is not a String"))
  }
}

pub fn parse_bootstrap_res(input: Result<JsValue, JsValue>) -> Result<PlayerState, JsString> {
  let res = input?;
  let account_val = parse_js_value_prop_as_string(&res, "account")?;
  let players_val = Reflect::get(&res, &JsValue::from_str("players"))?;
  let players_obj = match Object::try_from(&players_val) {
    Some(obj) => Ok(obj),
    None => Err(JsString::from("Players is not an object"))
  }?;
  let keys_vec = Object::keys(players_obj).to_vec();

  let mut players_hash = HashMap::new();

  for key in keys_vec {
    let value = Reflect::get(&players_obj, &key)?;
    let rs_key = match key.as_string() {
      Some(key_str) => Ok(key_str),
      None => Err(JsString::from("Players address is not a string"))
    }?;
    let weapon_id = parse_js_value_prop_as_string(&value, "weaponId")?;
    let armor_id = parse_js_value_prop_as_string(&value, "armorId")?;
    let kitty_id = parse_js_value_prop_as_string(&value, "kittyId")?;
    let player_items = PlayerItems::new(
      weapon_id,
      armor_id,
      kitty_id
    );
    players_hash.insert(rs_key, player_items);
  }

  Ok(PlayerState::new(account_val, players_hash))
}