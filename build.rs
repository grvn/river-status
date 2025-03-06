use std::{fs::{OpenOptions,File}, process::{Command, Stdio}, path::Path};


pub fn main() {
  // Use https://git.sr.ht/~sircmpwn/scdoc to generate docs
  match Command::new("scdoc").spawn() {
    Ok(_) => {
      let input = match File::open(Path::new(".doc.river-status.1.scd")) {
        Ok(o) => o,
        Err(_) => return,
      };
      let output = match OpenOptions::new().write(true).create(true).open(Path::new(".doc.river-status.1.gz")) {
        Ok(o) => o,
        Err(_) => return,
      };
      Command::new("scdoc").stdin(Stdio::from(input)).stdout(output).spawn().expect("Failed to execute command");
    }
    Err(_) => {}
  }
}

// fn generate(protocol: &str) {
//   let out_dir = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/protocol/"));
//   let mut protocol_dir = env!("CARGO_MANIFEST_DIR").to_string();
//   protocol_dir.push_str("/protocol/");
//   protocol_dir.push_str(protocol);
//   protocol_dir.push_str(".xml");

//   wayland_scanner::generate_client_code!("");
// }

//Path::new(&protocol_dir)