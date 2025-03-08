use std::{env, fs};
use std::io::{Error, Write};
use std::path::PathBuf;
use std::process::{exit, Command, Stdio};

fn main() -> Result<(), Error> {
  println!("cargo:rerun-if-changed=doc");

  let target_dir = match env::var_os("OUT_DIR") {
    Some(target_dir) => target_dir,
    None => {
      eprintln!("OUT_DIR environment variable not defined.");
      exit(1);
    }
  };

  if Command::new("scdoc").spawn().is_err() {
    eprintln!("scdoc not found in PATH, skipping generating manpages from doc/");
		return Ok(());
  }

  let doc_dir = PathBuf::from(target_dir).join("doc");
  fs::create_dir_all(&doc_dir)?;

  for doc in fs::read_dir("doc")? {
    let doc = doc?;
    let cmd = Command::new("scdoc")
      .stderr(Stdio::inherit())
      .stdin(Stdio::piped())
      .stdout(Stdio::piped())
      .spawn();
    let mut cmd = cmd?;
    if let Some(mut stdin) = cmd.stdin.take() {
      let doc = fs::read(doc.path())?;
      stdin.write_all(&doc)?;
    }
    let output = cmd.wait_with_output()?;
    let doc = PathBuf::from(doc.file_name());
    let doc = doc.file_stem().unwrap();

    fs::write(doc_dir.join(doc), output.stdout)?;
  }
  Ok(())
}