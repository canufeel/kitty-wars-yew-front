use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};
use serde_derive::{Deserialize, Serialize};
use log::*;
use crate::boundary::{boot_app, join_and_loot};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use js_sys::{JsString, Reflect, Object};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerItems {
  weapon_id: String,
  armor_id: String,
  kitty_id: String
}

impl PlayerItems {
  fn new(
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
  account: String,
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
}

pub struct State {
  player_state: Option<PlayerState>,
  load_finished: bool,
  is_joining: bool,
  load_success: Option<(bool, Option<String>)>
}

impl State {
  fn new() -> Self {
    State {
      player_state: None,
      load_finished: false,
      load_success: None,
      is_joining: false
    }
  }
}

pub struct App {
  state: State,
  link: ComponentLink<Self>
}

pub enum Msg {
  PlayerStateRequest,
  LoadSuccess(PlayerState),
  LoadFail(String),
  Join
}

fn parse_js_value_prop_as_string(obj: &JsValue, key: &str) -> Result<String, JsString> {
  let js_val = Reflect::get(obj, &JsValue::from_str(key))?;
  match js_val.as_string() {
    Some(v) => Ok(v),
    None => Err(JsString::from("Property is not a String"))
  }
}

fn parse_bootstrap_res(input: Result<JsValue, JsValue>) -> Result<PlayerState, JsString> {
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
    info!("{}", &rs_key);
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

impl Component for App {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
    let callback = link.send_back(|_| Msg::PlayerStateRequest);
    callback.emit(0);
    App {
      state: State::new(),
      link
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::PlayerStateRequest => {
        self.state.load_finished = false;
        let callback = self.link.send_back(|input| match parse_bootstrap_res(input) {
          Ok(player_state) => Msg::LoadSuccess(player_state),
          Err(e) => Msg::LoadFail(String::from(e))
        });
        spawn_local(async move {
          callback.emit(boot_app().await);
        });
      },
      Msg::LoadSuccess(player_state) => {
        info!("load success!");
        self.state.player_state = Some(player_state);
        self.state.load_success = Some((true, None));
        self.state.load_finished = true;
      },
      Msg::LoadFail(err) => {
        info!("load fail!");
        self.state.load_success = Some((false, Some(err)));
        self.state.load_finished = true;
      },
      Msg::Join => {
        let callback = self.link.send_back(|_| Msg::PlayerStateRequest);
        self.state.is_joining = true;
        spawn_local(async move {
          callback.emit(join_and_loot().await);
        });
      }
    }
    true
  }
}

enum RenderState<'a> {
  Loading,
  LoadingFailed(&'a str),
  LoadingSuccess(&'a PlayerState)
}

impl App {
  fn get_render_state(&self) -> RenderState {
    match (self.state.load_finished, self.state.load_success.as_ref()) {
      (true, Some((false, Some(msg)))) => RenderState::LoadingFailed(&msg),
      (true, Some((true, _))) => {
        let player_state = self.state.player_state.as_ref().unwrap();
        RenderState::LoadingSuccess(player_state)
      },
      (_, _) => RenderState::Loading,
    }
  }
}

impl Renderable<App> for App {
  fn view(&self) -> Html<Self> {
    info!("rendered!");
    html! {
            <div class="root">
                {
                  match self.get_render_state() {
                    RenderState::LoadingFailed(msg) => view_load_finished_fail(msg),
                    RenderState::LoadingSuccess(player_state) => view_load_finished(
                      player_state
                    ),
                    RenderState::Loading => view_loading()
                  }
                }
            </div>
        }
  }
}

fn view_loading() -> Html<App> {
  html! {
        <div class="loading">
            <p>{ "Loading" }</p>
        </div>
    }
}


fn view_load_finished_fail(msg: &str) -> Html<App> {
  html! {
        <div class="finished">
            <p>{ format!("Hello, {}", msg) }</p>
        </div>
    }
}

fn view_load_finished(
  player_state: &PlayerState,
) -> Html<App> {
  let load_finished_data = match player_state.has_player_for_account() {
    true => html!{ {""} },
    false => html! {
      <button onclick=|_| Msg::Join>{ "Join" }</button>
    }
  };
  html! {
        <div class="finished">
            <p>{ format!("Hello, {}", player_state.account) }</p>
            { load_finished_data }
        </div>
    }
}