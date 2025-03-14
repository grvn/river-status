#![allow(unused_crate_dependencies, reason = "I do not know how to fix this right now")]
#![allow(missing_docs, reason = "I ignore docs. right now")]
#![allow(clippy::print_stdout, reason = "I like using stdout and stderr")]
#![allow(clippy::print_stderr, reason = "I like using stdout and stderr")]

//! Docs for the main crate

use core::time::Duration;
use river_status::client::State;
use river_status::flags::CONFIG;
use std::{process::ExitCode, thread};
use wayland_client::{ConnectError, Connection, EventQueue};

fn main() -> ExitCode {
  if let Some(exit) = CONFIG.exitcode {
    return exit;
  }
  let Ok(conn) = Connection::connect_to_env().map_err(|e| {
    if matches!(e, ConnectError::NoCompositor) {
      eprintln!("{e}, make sure you are running river-status from a wayland desktop environment.");
    } else {
      eprint!("{e}");
    };
    ExitCode::FAILURE
  }) else {
    return ExitCode::FAILURE;
  };

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
      },
      Err(err) => {
        eprintln!("Error dispatching events: {err}");
        break;
      },
    }
  }

  println!("{state}");

  #[expect(clippy::while_immutable_condition, reason = "This cli tool has a watch-functionality")]
  while CONFIG.continuously {
    match event_queue.blocking_dispatch(&mut state) {
      // TODO: fix error handling
      Err(_) | Ok(_) => {},
    }
    if state.updated {
      println!("{state}");
      state.updated = false;
    }
    if let Some(time) = CONFIG.sleep {
      thread::sleep(Duration::from_millis(time));
    };
  }

  state.destroy();
  ExitCode::SUCCESS
}
