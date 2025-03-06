use std::fs::{OpenOptions,File};
use std::path::Path;
use std::process::{Command, Stdio};

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