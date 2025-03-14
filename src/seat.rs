use crate::protocols::river_status_unstable::v1::zriver_seat_status_v1::ZriverSeatStatusV1;
use serde::Serialize;
use wayland_client::protocol::wl_seat::WlSeat;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Seat {
  pub name: String,
  #[serde(skip)]
  pub status: Option<ZriverSeatStatusV1>,
  #[serde(skip)]
  pub wlseat: WlSeat,
}

impl Seat {
  #[must_use]
  pub const fn new(name: String, wlseat: WlSeat) -> Self {
    Self { name, status: None, wlseat }
  }
}
