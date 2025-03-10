use crate::flags::{get_configuration, Flags};
use crate::output::Output;
use crate::protocols::river_status_unstable::v1::zriver_output_status_v1::Event::{
  FocusedTags, LayoutName, LayoutNameClear, UrgentTags, ViewTags,
};
use crate::protocols::river_status_unstable::v1::zriver_output_status_v1::ZriverOutputStatusV1;
use crate::protocols::river_status_unstable::v1::zriver_seat_status_v1::Event::{
  FocusedOutput, FocusedView, Mode,
};
use crate::protocols::river_status_unstable::v1::zriver_seat_status_v1::ZriverSeatStatusV1;
use crate::protocols::river_status_unstable::v1::zriver_status_manager_v1::ZriverStatusManagerV1;
use crate::seat::Seat;
use serde::Serialize;
use std::fmt;
use wayland_client::backend::ObjectId;
use wayland_client::protocol::wl_output;
use wayland_client::protocol::wl_registry::{Event, WlRegistry};
use wayland_client::protocol::wl_seat;
use wayland_client::{Connection, Dispatch, Proxy, QueueHandle};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct State {
  #[serde(skip)]
  pub flags: Flags,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub layout: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub mode: Option<String>,
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub outputs: Vec<Output>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub seat: Option<Seat>,
  #[serde(skip)]
  pub status_manager: Option<ZriverStatusManagerV1>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub title: Option<String>,
  #[serde(skip)]
  pub updated: bool,
}

impl State {
  pub fn get_output(&mut self, id: &ObjectId) -> Option<&mut Output> {
    self.outputs.iter_mut().find(|output| output.wloutput.id() == *id)
  }

  // Destroy itself when no longer needed
  pub fn destroy(&mut self) {
    if let Some(status_mgr) = self.status_manager.take() {
      status_mgr.destroy();
    }
    for output in &self.outputs {
      if let Some(status) = &output.status {
        status.destroy();
      }
    }
  }
}

impl Default for State {
  fn default() -> Self {
    let flags = get_configuration();
    Self {
      flags,
      layout: None,
      mode: None,
      outputs: vec![],
      seat: None,
      status_manager: None,
      title: None,
      updated: false,
    }
  }
}

impl fmt::Display for State {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.flags.pretty {
      let json = match serde_json::to_string_pretty(self) {
        Ok(json) => json,
        Err(_) => String::from("{}"),
      };
      write!(f, "{}", json)
    } else {
      let json = match serde_json::to_string(self) {
        Ok(json) => json,
        Err(_) => String::from("{}"),
      };
      write!(f, "{}", json)
    }
  }
}

impl Dispatch<WlRegistry, ()> for State {
  fn event(
    state: &mut Self,
    proxy: &WlRegistry,
    event: <WlRegistry as wayland_client::Proxy>::Event,
    _: &(),
    _: &Connection,
    qhandle: &QueueHandle<Self>,
  ) {
    if let Event::Global { name, interface, version } = event {
      match &interface[..] {
        "zriver_status_manager_v1" => {
          state.status_manager =
            Some(proxy.bind::<ZriverStatusManagerV1, _, Self>(name, version, qhandle, ()));
        }
        "wl_seat" => {
          proxy.bind::<wl_seat::WlSeat, _, Self>(name, version, qhandle, ());
        }
        "wl_output" => {
          proxy.bind::<wl_output::WlOutput, _, Self>(name, version, qhandle, ());
        }
        _ => {}
      }
    }
  }
}

impl Dispatch<ZriverOutputStatusV1, ObjectId> for State {
  fn event(
    state: &mut Self,
    _: &ZriverOutputStatusV1,
    event: <ZriverOutputStatusV1 as wayland_client::Proxy>::Event,
    id: &ObjectId,
    _: &Connection,
    _: &QueueHandle<Self>,
  ) {
    match event {
      FocusedTags { tags } => {
        if state.flags.tags {
          if let Some(output) = state.get_output(id) {
            output.focused_tags = Some(tags.trailing_zeros() + 1); // convert to 1-based index
            state.updated = true;
          }
        }
      }
      LayoutName { name } => {
        if state.flags.layout {
          state.layout = Some(name);
          state.updated = true;
        }
      }
      LayoutNameClear => state.layout = None,
      UrgentTags { tags } => {
        if tags != 0 && state.flags.urgent {
          if let Some(output) = state.get_output(id) {
            output.urgent_tags = Some(tags);
            state.updated = true;
          }
        }
      }
      ViewTags { tags } => {
        if state.flags.view {
          if let Some(output) = state.get_output(id) {
            let tags: Vec<u32> = tags[0..]
              .chunks(4)
              .map(|s| {
                let buf = [s[0], s[1], s[2], s[3]];
                let tagmask = u32::from_le_bytes(buf);
                for i in 0..32 {
                  if 1 << i == tagmask {
                    return 1 + i;
                  }
                }
                0
              })
              .collect();
            output.occupied_tags = tags;
            state.updated = true;
          }
        }
      }
    }
  }
}

impl Dispatch<wl_output::WlOutput, ()> for State {
  fn event(
    state: &mut Self,
    proxy: &wl_output::WlOutput,
    event: <wl_output::WlOutput as wayland_client::Proxy>::Event,
    _: &(),
    _: &Connection,
    qhandle: &QueueHandle<Self>,
  ) {
    if let wl_output::Event::Name { name } = event {
      if match state.flags.output.as_ref() {
        Some(output) => output.eq(&name),
        None => true,
      } {
        let mut output = Output::new(name, proxy.to_owned());
        if let Some(status_mgr) = &state.status_manager {
          output.status = Some(status_mgr.get_river_output_status(proxy, qhandle, proxy.id()));
        }
        state.outputs.push(output);
      }
    }
  }
}

impl Dispatch<ZriverStatusManagerV1, ()> for State {
  fn event(
    _: &mut Self,
    _: &ZriverStatusManagerV1,
    _: <ZriverStatusManagerV1 as wayland_client::Proxy>::Event,
    _: &(),
    _: &Connection,
    _: &QueueHandle<Self>,
  ) {
  }
}

impl Dispatch<wl_seat::WlSeat, ()> for State {
  fn event(
    state: &mut Self,
    proxy: &wl_seat::WlSeat,
    event: <wl_seat::WlSeat as Proxy>::Event,
    _: &(),
    _: &Connection,
    qhandle: &QueueHandle<Self>,
  ) {
    if let wl_seat::Event::Name { name } = event {
      if match state.flags.seat.as_ref() {
        Some(seat) => seat.eq(&name),
        None => true,
      } {
        let mut seat = Seat::new(name, proxy.to_owned());
        if let Some(status_mgr) = &state.status_manager {
          seat.status = Some(status_mgr.get_river_seat_status(proxy, qhandle, ()));
        }
        state.seat = Some(seat);
      }
    }
  }
}

impl Dispatch<ZriverSeatStatusV1, ()> for State {
  fn event(
    state: &mut Self,
    _: &ZriverSeatStatusV1,
    event: <ZriverSeatStatusV1 as Proxy>::Event,
    _: &(),
    _: &Connection,
    _: &QueueHandle<Self>,
  ) {
    match event {
      FocusedOutput { output } => {
        if state.flags.focused {
          if let Some(output) = state.get_output(&output.id()) {
            output.focused = true;
            state.updated = true;
          }
        }
      }
      FocusedView { title } => {
        if state.flags.title {
          state.title = Some(title);
          state.updated = true;
        }
      }
      Mode { name } => {
        if state.flags.mode {
          state.mode = Some(name);
          state.updated = true;
        }
      }
      _ => {}
    }
  }
}
