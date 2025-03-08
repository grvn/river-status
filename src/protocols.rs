pub mod river_status_unstable {
  pub mod v1 {
    use wayland_client;
    use wayland_client::protocol::{wl_output, wl_seat};

    pub mod __interfaces {
      use wayland_client::backend as wayland_backend;
      use wayland_client::protocol::__interfaces::{
        WL_OUTPUT_INTERFACE, WL_SEAT_INTERFACE, wl_output_interface, wl_seat_interface,
      };
      wayland_scanner::generate_interfaces!("./river/protocol/river-status-unstable-v1.xml");
    }

    use self::__interfaces::{
      ZRIVER_OUTPUT_STATUS_V1_INTERFACE, ZRIVER_SEAT_STATUS_V1_INTERFACE,
      ZRIVER_STATUS_MANAGER_V1_INTERFACE,
    };
    wayland_scanner::generate_client_code!("./river/protocol/river-status-unstable-v1.xml");
  }
}
