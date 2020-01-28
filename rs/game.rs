use std::collections::HashMap;
use yew::{Html, html, Callback};
use stdweb::web::event::ClickEvent;

pub enum ItemType {
  Weapon,
  Armor
}

pub struct Item {
  item_type: ItemType,
  item_power: String
}

impl Item {
  pub fn new(
    item_type: ItemType,
    item_power: String
  ) -> Self {
    Item {
      item_power,
      item_type
    }
  }
}

pub struct Player {
  weapon_id: String,
  armor_id: String,
  kitty_id: String,
  is_battling: bool
}

impl Player {
  pub fn new(
    weapon_id: String,
    armor_id: String,
    kitty_id: String,
  ) -> Self {
    Player {
      weapon_id,
      armor_id,
      kitty_id,
      is_battling: false
    }
  }

  pub fn set_battling(&mut self, is_battling: bool) {
    self.is_battling = is_battling;
  }
}

pub struct PlayerState {
  pub account: String,
  players: HashMap<String, Player>,
  items: HashMap<String, Item>
}

impl PlayerState {
  pub fn new(
    account: String,
    players: HashMap<String, Player>,
    items: HashMap<String, Item>
  ) -> Self {
    PlayerState {
      account,
      players,
      items
    }
  }

  pub fn has_player_for_account(&self) -> bool {
    self.players.contains_key(&self.account)
  }

  fn get_items_for_current_player(&self) -> Option<(&Item, &Item)> {
    match self.players.get(&self.account) {
      Some(items) => match (self.items.get(&items.weapon_id), self.items.get(&items.armor_id)) {
        (Some(weapon), Some(armor)) => Some((weapon, armor)),
        _ => None,
      },
      _ => None
    }
  }

  pub fn get_player_details(&self, on_join: Callback<ClickEvent>) -> Html {
    let load_finished_data = match self.get_items_for_current_player() {
      None => html! {
        <div class="join">
          <button onclick=on_join>{ "Join" }</button>
        </div>
      },
      Some((weapon, armor)) => html!{
        <div class="player-details">
          <div class="weapon">
            { format!("Weapon: {}", weapon.item_power) }
          </div>
          <div class="armor">
            { format!("Armor: {}", armor.item_power) }
          </div>
        </div>
      }
    };
    html! {
      <div class="finished">
        <p>{ format!("Hello, {}", self.account) }</p>
        { load_finished_data }
      </div>
    }
  }
}