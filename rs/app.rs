use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender, Callback};
use serde_derive::{Deserialize, Serialize};
use log::*;
use crate::boundary::boot_app;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use js_sys::JsString;

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerState {
  account: String
}

impl PlayerState {
  pub fn new(account: String) -> Self {
    PlayerState {
      account
    }
  }
}

pub struct State {
  player_state: Option<PlayerState>,
  load_finished: bool,
  load_success: Option<(bool, Option<String>)>
}

impl State {
  fn new() -> Self {
    State {
      player_state: None,
      load_finished: false,
      load_success: None
    }
  }
}

pub struct App {
  state: State,
  // link: ComponentLink<Self>
}

pub enum Msg {
  LoadSuccess(PlayerState),
  LoadFail(String)
}

fn parse_bootstrap_res(input: Result<JsValue, JsValue>) -> Result<String, JsString> {
  let res = input?;
  let account_val = js_sys::Reflect::get(&res, &JsValue::from_str("account"))?;
  match account_val.as_string() {
    Some(v) => Ok(v),
    None => Err(JsString::from("Account is not a string"))
  }
}

impl Component for App {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
    let callback: Callback<Result<JsValue, JsValue>> = link.send_back(|input: Result<JsValue, JsValue>| match parse_bootstrap_res(input) {
      Ok(account) => Msg::LoadSuccess(PlayerState::new(account)),
      Err(e) => Msg::LoadFail(String::from(e))
    });
    spawn_local(async move {
      callback.emit(boot_app().await);
    });
    App {
      state: State::new(),
      // link
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
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
      }
    }
    true
  }
}

impl Renderable<App> for App {
  fn view(&self) -> Html<Self> {
    info!("rendered!");
    html! {
            <div class="root">
                {
                  match (self.state.load_finished, self.state.load_success.as_ref()) {
                    (true, Some((false, Some(msg)))) => view_load_finished(msg),
                    (true, Some((true, _))) => view_load_finished(&self.state.player_state.as_ref().unwrap().account),
                    (_, _) => view_loading(),
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

fn view_load_finished(msg: &str) -> Html<App> {
  html! {
        <div class="finished">
            <p>{ format!("Hello, {}", msg) }</p>
        </div>
    }
}