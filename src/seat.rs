use crate::protocols::river_status_unstable::v1::zriver_seat_status_v1::ZriverSeatStatusV1;
use serde::Serialize;
use wayland_client::protocol::wl_seat::WlSeat;

#[derive(Serialize)]
pub struct Seat {
  pub name: String,
  #[serde(skip)]
  pub status: Option<ZriverSeatStatusV1>,
  #[serde(skip)]
  pub wlseat: WlSeat,
}

impl Seat {
  pub fn new(name: String, wlseat: WlSeat) -> Self {
    Self {
      name,
      status: None,
      wlseat,
    }
  }
}
