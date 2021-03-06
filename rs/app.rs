use yew::{html, Component, ComponentLink, Html, ShouldRender};
use log::*;
use crate::{
  boundary::{boot_app, join_and_loot},
  game::PlayerState,
  parsing
};
use wasm_bindgen_futures::spawn_local;

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


impl Component for App {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    let callback = link.callback(|_| Msg::PlayerStateRequest);
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
        let callback = self.link.callback(|input| match parsing::parse_bootstrap_res(input) {
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
        let callback = self.link.callback(|_| Msg::PlayerStateRequest);
        self.state.is_joining = true;
        spawn_local(async move {
          callback.emit(join_and_loot().await);
        });
      }
    }
    true
  }

  fn view(&self) -> Html {
    info!("rendered!");
    let on_join = self.link.callback(|_| Msg::Join);
    html! {
      <div class="root">
        {
          match self.get_render_state() {
            RenderState::LoadingFailed(msg) => view_load_finished_fail(msg),
            RenderState::LoadingSuccess(player_state) => player_state.get_player_details(on_join),
            RenderState::Loading => view_loading()
          }
        }
      </div>
    }
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

fn view_loading() -> Html {
  html! {
    <div class="loading">
      <p>{ "Loading" }</p>
    </div>
  }
}


fn view_load_finished_fail(msg: &str) -> Html {
  html! {
    <div class="finished">
      <p>{ format!("Hello, {}", msg) }</p>
    </div>
  }
}
