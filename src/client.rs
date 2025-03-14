#![allow(clippy::ref_option, reason = "Serialize forces this behavior")]
#![allow(clippy::pattern_type_mismatch, reason = "I have no clue how to fix this")]
#![allow(clippy::arithmetic_side_effects, reason = "1 addition that should not be able to create any chaos")]
#![allow(clippy::indexing_slicing, reason = "assert macro handles it")]

use crate::flags::CONFIG;
use crate::output::Output;
use crate::protocols::river_status_unstable::v1::zriver_output_status_v1::Event::{
  FocusedTags, LayoutName, LayoutNameClear, UrgentTags, ViewTags,
};
use crate::protocols::river_status_unstable::v1::zriver_output_status_v1::ZriverOutputStatusV1;
use crate::protocols::river_status_unstable::v1::zriver_seat_status_v1::Event::{
  FocusedOutput, FocusedView, Mode, UnfocusedOutput,
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

#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct State {
  #[serde(skip_serializing_if = "no_mode")]
  pub mode: Option<String>,
  #[serde(skip_serializing_if = "no_output")]
  pub outputs: Vec<Output>,
  #[serde(skip_serializing_if = "no_seat")]
  pub seat: Option<Seat>,
  #[serde(skip)]
  pub status_manager: Option<ZriverStatusManagerV1>,
  #[serde(skip_serializing_if = "no_title")]
  pub title: Option<String>,
  #[serde(skip)]
  pub updated: bool,
}

/// `no_mode` checks if there is any mode in use and if
/// the user wishes to see it
fn no_mode<T>(s: &Option<T>) -> bool {
  s.is_none() || !CONFIG.mode
}
/// `no_output` checks if there are any outputs in use and if
/// the user wishes to see it
fn no_output<T>(v: &[T]) -> bool {
  v.is_empty() || CONFIG.no_output
}
/// `no_seat` checks if there is any seat in use and if
/// the user wishes to see it
fn no_seat<T>(s: &Option<T>) -> bool {
  s.is_none() || CONFIG.no_seat
}
/// `no_title` checks if there is any title in use and if
/// the user wishes to see it
fn no_title<T>(s: &Option<T>) -> bool {
  s.is_none() || !CONFIG.title
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

impl fmt::Display for State {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if CONFIG.pretty {
      let json = serde_json::to_string(self).unwrap_or_else(|_| String::from("{}"));
      write!(f, "{json}")
    } else {
      let json = serde_json::to_string(self).unwrap_or_else(|_| String::from("{}"));
      write!(f, "{json}")
    }
  }
}

impl Dispatch<WlRegistry, ()> for State {
  fn event(
    state: &mut Self,
    proxy: &WlRegistry,
    event: <WlRegistry as Proxy>::Event,
    &(): &(),
    _: &Connection,
    qhandle: &QueueHandle<Self>,
  ) {
    if let Event::Global { name, interface, version } = event {
      match &*interface {
        "zriver_status_manager_v1" => {
          state.status_manager = Some(proxy.bind::<ZriverStatusManagerV1, _, Self>(name, version, qhandle, ()));
        },
        "wl_seat" => {
          drop::<wl_seat::WlSeat>(proxy.bind::<wl_seat::WlSeat, _, Self>(name, version, qhandle, ()));
        },
        "wl_output" => {
          drop::<wl_output::WlOutput>(proxy.bind::<wl_output::WlOutput, _, Self>(name, version, qhandle, ()));
        },
        _ => {},
      }
    }
  }
}

impl Dispatch<ZriverOutputStatusV1, ObjectId> for State {
  fn event(
    state: &mut Self,
    _: &ZriverOutputStatusV1,
    event: <ZriverOutputStatusV1 as Proxy>::Event,
    data: &ObjectId,
    _: &Connection,
    _: &QueueHandle<Self>,
  ) {
    match event {
      FocusedTags { tags } => {
        if let Some(output) = state.get_output(data) {
          output.focused_tags = Some(tags.trailing_zeros() + 1); // convert to 1-based index
          state.updated = true;
        }
      },
      LayoutName { name } => {
        if let Some(output) = state.get_output(data) {
          output.layout = Some(name);
          state.updated = true;
        }
      },
      LayoutNameClear => {
        if let Some(output) = state.get_output(data) {
          output.layout = None;
          state.updated = true;
        }
      },
      UrgentTags { tags } => {
        if tags != 0 {
          if let Some(output) = state.get_output(data) {
            output.urgent_tags = Some(tags);
            state.updated = true;
          }
        }
      },
      ViewTags { tags } => {
        if let Some(output) = state.get_output(data) {
          let taggar: Vec<u32> = tags[0..]
            .chunks(4)
            .map(|s| {
              assert!(s.len() > 3, "check that 4 values exist");
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
          output.occupied_tags = taggar;
          state.updated = true;
        }
      },
    }
  }
}

impl Dispatch<wl_output::WlOutput, ()> for State {
  fn event(
    state: &mut Self,
    proxy: &wl_output::WlOutput,
    event: <wl_output::WlOutput as Proxy>::Event,
    &(): &(),
    _: &Connection,
    qhandle: &QueueHandle<Self>,
  ) {
    if let wl_output::Event::Name { name } = event {
      if CONFIG.output.as_ref().is_none_or(|output| output.eq(&name)) {
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
    _: <ZriverStatusManagerV1 as Proxy>::Event,
    &(): &(),
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
    &(): &(),
    _: &Connection,
    qhandle: &QueueHandle<Self>,
  ) {
    if let wl_seat::Event::Name { name } = event {
      if CONFIG.seat.as_ref().is_none_or(|seat| seat.eq(&name)) {
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
    &(): &(),
    _: &Connection,
    _: &QueueHandle<Self>,
  ) {
    match event {
      FocusedOutput { output } => {
        if let Some(out) = state.get_output(&output.id()) {
          out.focused = true;
          state.updated = true;
        }
      },
      FocusedView { title } => {
        state.title = Some(title);
        state.updated = true;
      },
      Mode { name } => {
        state.mode = Some(name);
        state.updated = true;
      },
      UnfocusedOutput { .. } => {},
    }
  }
}
