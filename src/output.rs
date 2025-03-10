// output as in displays|screens|monitors
#![allow(clippy::trivially_copy_pass_by_ref)]
use crate::flags::CONFIG;
use crate::protocols::river_status_unstable::v1::zriver_output_status_v1::ZriverOutputStatusV1;
use serde::Serialize;
use wayland_client::protocol::wl_output::WlOutput;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Output {
  #[serde(skip_serializing_if = "no_focused")]
  pub focused: bool,
  #[serde(skip_serializing_if = "no_focused_tags")]
  pub focused_tags: Option<u32>,
  #[serde(skip_serializing_if = "no_layout")]
  pub layout: Option<String>,
  pub name: String,
  #[serde(skip_serializing_if = "no_occupied_tags")]
  pub occupied_tags: Vec<u32>,
  #[serde(skip)]
  pub status: Option<ZriverOutputStatusV1>,
  #[serde(skip_serializing_if = "no_urgent_tags")]
  pub urgent_tags: Option<u32>,
  #[serde(skip)]
  pub wloutput: WlOutput,
}

fn no_focused(_: &bool) -> bool {
  !CONFIG.focused
}

fn no_focused_tags<T>(u: &Option<T>) -> bool {
  u.is_none() || !CONFIG.tags
}

fn no_layout<T>(s: &Option<T>) -> bool {
  s.is_none() || !CONFIG.layout
}

fn no_occupied_tags<T>(v: &[T]) -> bool {
  v.is_empty() || !CONFIG.view
}

fn no_urgent_tags<T>(u: &Option<T>) -> bool {
  u.is_none() || !CONFIG.urgent
}

impl Output {
  pub fn new(name: String, wloutput: WlOutput) -> Self {
    Self {
      focused: false,
      focused_tags: None,
      layout: None,
      name,
      occupied_tags: vec![],
      status: None,
      urgent_tags: None,
      wloutput,
    }
  }
}
