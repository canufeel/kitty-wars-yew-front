use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use yew::{Html, html};

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerItems {
  weapon_id: String,
  armor_id: String,
  kitty_id: String
}

impl PlayerItems {
  pub fn new(
    weapon_id: String,
    armor_id: String,
    kitty_id: String
  ) -> Self {
    PlayerItems {
      weapon_id,
      armor_id,
      kitty_id
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerState {
  pub account: String,
  players: HashMap<String, PlayerItems>
}

impl PlayerState {
  pub fn new(
    account: String,
    players: HashMap<String, PlayerItems>
  ) -> Self {
    PlayerState {
      account,
      players
    }
  }

  pub fn has_player_for_account(&self) -> bool {
    self.players.contains_key(&self.account)
  }

  pub fn get_player_details(&self) -> Html {
    html!{ {""} }
  }
}