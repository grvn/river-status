use river_status::client::State;
use river_status::flags::CONFIG;
use std::process::exit;
use wayland_client::{ConnectError, Connection, EventQueue};

fn main() {
  let Ok(conn) = Connection::connect_to_env().map_err(|e| match e {
    ConnectError::NoCompositor => {
      eprintln!("{e}, make sure you are running river-status from a wayland desktop environment.");
      exit(1);
    }
    _ => {
      eprintln!("{e}");
      exit(1)
    }
  });

  let display = conn.display();
  let mut event_queue: EventQueue<State> = conn.new_event_queue();
  let qh = event_queue.handle();
  let _registry = display.get_registry(&qh, ());

  let mut state = State::default();

  loop {
    match event_queue.roundtrip(&mut state) {
      Ok(num_events) => {
        if num_events == 0 {
          break;
        }
      }
      Err(err) => {
        eprintln!("Error dispatching events: {:?}", err);
        break;
      }
    }
  }

  println!("{}", state);

  #[allow(clippy::while_immutable_condition)]
  while CONFIG.continuously {
    match event_queue.blocking_dispatch(&mut state) {
      Ok(_) => {} // TODO: fix error handling
      Err(_) => {}
    }
    if state.updated {
      println!("{}", state);
      state.updated = false;
    }
  }

  state.destroy();
}
