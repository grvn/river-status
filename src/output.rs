// output as in displays|screens|monitors
use crate::protocols::river_status_unstable::v1::zriver_output_status_v1::ZriverOutputStatusV1;
use serde::Serialize;
use serde_with::skip_serializing_none;
use wayland_client::protocol::wl_output::WlOutput;

#[skip_serializing_none]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Output {
  pub focused: bool,
  pub focused_tags: Option<u32>,
  pub layout: Option<String>,
  pub name: String,
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub occupied_tags: Vec<u32>,
  #[serde(skip)]
  pub status: Option<ZriverOutputStatusV1>,
  pub urgent_tags: Option<u32>,
  #[serde(skip)]
  pub wloutput: WlOutput,
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
