// output as in displays|screens|monitors
#![allow(clippy::trivially_copy_pass_by_ref, reason = "Serialize forces this behavior")]
#![allow(clippy::ref_option, reason = "Serialize forces this behavior")]
use crate::flags::CONFIG;
use crate::protocols::river_status_unstable::v1::zriver_output_status_v1::ZriverOutputStatusV1;
use serde::Serialize;
use wayland_client::protocol::wl_output::WlOutput;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
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

/// `no_focused` checks if the user wishes to see the focused tags
fn no_focused(_: &bool) -> bool {
  !CONFIG.focused
}

/// `no_focused_tags` checks if there are any focused tags in use and if
/// the user wishes to see them
fn no_focused_tags<T>(u: &Option<T>) -> bool {
  u.is_none() || !CONFIG.tags
}

/// `no_layout` checks if there are a layout in use and if
/// the user wishes to see it
fn no_layout<T>(s: &Option<T>) -> bool {
  s.is_none() || !CONFIG.layout
}

/// `no_occupied_tags` checks if there are any tags in use and if
/// the user wishes to see them
fn no_occupied_tags<T>(v: &[T]) -> bool {
  v.is_empty() || !CONFIG.view
}

/// `no_urgent_tags` checks if there are any urgent tags and if
/// the user wishes to see them
fn no_urgent_tags<T>(u: &Option<T>) -> bool {
  u.is_none() || !CONFIG.urgent
}

impl Output {
  #[must_use]
  pub const fn new(name: String, wloutput: WlOutput) -> Self {
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
