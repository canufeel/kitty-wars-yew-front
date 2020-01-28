use js_sys::{JsString, Reflect, Object};
use crate::game::{PlayerState, PlayerItems, ItemType, Item};
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
  let items_val = Reflect::get(&res, &JsValue::from_str("items"))?;
  let items_obj = match Object::try_from(&items_val) {
    Some(obj) => Ok(obj),
    None => Err(JsString::from("Items is not an object"))
  }?;
  let players_obj = match Object::try_from(&players_val) {
    Some(obj) => Ok(obj),
    None => Err(JsString::from("Players is not an object"))
  }?;
  let items_keys_vec = Object::keys(items_obj).to_vec();
  let players_keys_vec = Object::keys(players_obj).to_vec();
  let mut players_hash = HashMap::new();
  let mut items_hash = HashMap::new();

  for key in players_keys_vec {
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

  for key in items_keys_vec {
    let value = Reflect::get(&items_obj, &key)?;
    let rs_key = match key.as_string() {
      Some(key_str) => Ok(key_str),
      None => Err(JsString::from("Item's id is not a string"))
    }?;
    let item_type = parse_js_value_prop_as_string(&value, "itemType")?;
    let item_power = parse_js_value_prop_as_string(&value, "itemPower")?;
    let item_type_enum = match item_type.as_str() {
      "0" => Ok(ItemType::Weapon),
      "1" => Ok(ItemType::Armor),
      _ => Err(JsString::from("Invalid item type"))
    }?;
    let item = Item::new(
      item_type_enum,
      item_power
    );
    items_hash.insert(rs_key, item);
  }

  Ok(PlayerState::new(account_val, players_hash, items_hash))
}