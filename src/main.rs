use std::process::ExitCode;
use wayland_client::{Connection, EventQueue};
use river_status::client::State;

fn main() -> ExitCode {
  let conn = Connection::connect_to_env().expect("Failed to connect to the Wayland server!");
  let display = conn.display();
  let mut event_queue: EventQueue<State> = conn.new_event_queue();
  let qh = event_queue.handle();
  let _registry = display.get_registry(&qh, ());

  let mut state = State::new();

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

  while state.flags.continuously {
    match event_queue.blocking_dispatch(&mut state) {
      Ok(_) => {} // TODO: fix error handling
      Err(_) => {}
    }
    if state.updated == true {
      println!("{}", state);
      state.updated = false;
    }
  }

  state.destroy();
  return ExitCode::SUCCESS;
}
