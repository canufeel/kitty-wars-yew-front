use js_sys::{JsString, Reflect, Object};
use crate::game::{PlayerState, Player, ItemType, Item};
use wasm_bindgen::JsValue;
use std::collections::HashMap;

pub fn parse_js_value_prop_as_string(obj: &JsValue, key: &str) -> Result<String, JsString> {
  let js_val = Reflect::get(obj, &JsValue::from_str(key))?;
  match js_val.as_string() {
    Some(v) => Ok(v),
    None => Err(JsString::from("Property is not a String"))
  }
}

fn get_hash_interface<T, F>(
  state: &JsValue,
  outer_key: &str,
  mut prop_parser: F
) -> Result<HashMap<String, T>, JsString>
  where T: Sized,
    F: FnMut(&JsValue, String, &mut HashMap<String, T>) -> Result<(), JsString>
{
  let players_val = Reflect::get(&state, &JsValue::from_str(outer_key))?;
  let obj = match Object::try_from(&players_val) {
    Some(obj) => Ok(obj),
    None => Err(JsString::from(format!("{} is not an object", outer_key)))
  }?;
  let keys_vec = Object::keys(obj).to_vec();
  let mut hash = HashMap::new();
  for key in keys_vec {
    let value = Reflect::get(&obj, &key)?;
    let rs_key = match key.as_string() {
      Some(key_str) => Ok(key_str),
      None => Err(JsString::from(format!("Inner key {:?} is not a string", key)))
    }?;
    prop_parser(&value, rs_key, &mut hash)?;
  }
  Ok(hash)
}

fn parse_players(state: &JsValue) -> Result<HashMap<String, Player>, JsString> {
  let parser = |value: &JsValue, rs_key: String, hash: &mut HashMap<String, Player>| -> Result<(), JsString> {
    let weapon_id = parse_js_value_prop_as_string(&value, "weaponId")?;
    let armor_id = parse_js_value_prop_as_string(&value, "armorId")?;
    let kitty_id = parse_js_value_prop_as_string(&value, "kittyId")?;
    let player = Player::new(
      weapon_id,
      armor_id,
      kitty_id
    );
    hash.insert(rs_key, player);
    Ok(())
  };
  get_hash_interface(state, "players", parser)
}

fn parse_items(state: &JsValue) -> Result<HashMap<String, Item>, JsString> {
  let parser = |value: &JsValue, rs_key: String, hash: &mut HashMap<String, Item>| -> Result<(), JsString> {
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
    hash.insert(rs_key, item);
    Ok(())
  };
  get_hash_interface(state, "items", parser)
}

fn parse_is_battling(state: &JsValue, players_hash: &mut HashMap<String, Player>) -> Result<(), JsString> {
  let parser = |value: &JsValue, rs_key: String, _hash: &mut HashMap<String, ()>| -> Result<(), JsString> {
    let bool_value = match value.as_bool() {
      Some(v) => Ok(v),
      None => Err(JsString::from("Property is not a bool"))
    }?;
    match players_hash.get_mut(&rs_key) {
      Some(player) => { player.set_battling(bool_value); },
      _ => {}
    };
    Ok(())
  };
  get_hash_interface(state, "isBattling", parser)?;
  Ok(())
}

pub fn parse_bootstrap_res(input: Result<JsValue, JsValue>) -> Result<PlayerState, JsString> {
  let res = input?;
  let account_val = parse_js_value_prop_as_string(&res, "account")?;
  let mut players_hash = parse_players(&res)?;
  parse_is_battling(&res, &mut players_hash)?;
  let items_hash = parse_items(&res)?;
  Ok(PlayerState::new(account_val, players_hash, items_hash))
}