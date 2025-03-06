use wayland_client::protocol::wl_seat::WlSeat;
use wayland_client::Proxy;
use serde::Serialize;
use crate::protocols::river_status_unstable::v1::zriver_seat_status_v1::ZriverSeatStatusV1;

pub struct Seat {
  pub name: String,
  pub status: Option<ZriverSeatStatusV1>,
  pub wlseat: WlSeat,
}

impl Seat {
  pub fn new(name: String, wlseat: WlSeat) -> Self {
    Self {
      name: name,
      status: None,
      wlseat: wlseat,
    }
  }
}

impl Serialize for Seat {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_str(&self.wlseat.id().to_string())
  }
}