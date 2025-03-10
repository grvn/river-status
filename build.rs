use clap::{crate_authors,crate_version};
use clap_complete::generate_to;
use clap_complete::shells::{Bash, Elvish, Fish, PowerShell, Zsh};
use clap_mangen::Man;
use std::io::Error;
use std::path::PathBuf;
use std::process::exit;
use std::{env, fs};

fn main() -> Result<(), Error> {
  println!("cargo::rerun-if-changed=build.rs");

  let target_dir = match env::var_os("OUT_DIR") {
    Some(target_dir) => target_dir,
    None => {
      eprintln!("OUT_DIR environment variable not defined.");
      exit(1);
    }
  };

  let man_dir = PathBuf::from(target_dir).join("man");
  fs::create_dir_all(&man_dir)?;

  let app_name = "river-status";

  let mut cmd = clap::Command::new(app_name)
  .about("A status information client for river")
  .arg(clap::arg!(-a --all "Equivalent of -f -l -m -t --tag -u --view-tags"))
  .arg(clap::arg!(-f --focused "Print information about the focused tags"))
  .arg(clap::arg!(-l --layout "Print layout name of the focused tags."))
  .arg(clap::arg!(-m --mode "Print mode name."))
  .arg(clap::arg!(--"no-output" "Force information about all outputs to be omitted from the output"))
  .arg(clap::arg!(--"no-seat" "Force information about all seats to be omitted from the output."))
  .arg(clap::arg!(-o --output <OUTPUT> "Select the output to display information about."))
  .arg(clap::arg!(-p --pretty "Pretty print JSON."))
  .arg(clap::arg!(-s --seat <SEAT> "Select the seat to display information about."))
  .arg(clap::arg!(-T --tag "Output the key *focusedTags* and numerical value representing which tag is focused for each output."))
  .arg(clap::arg!(-t --title "Print the title of the focused view."))
  .arg(clap::arg!(-u --urgent "Print information about urgent tags if there are any."))
  .arg(clap::arg!(--"view-tags" "Prints the tags of all views."))
  .arg(clap::arg!(-w --watch "Continuously print information as it changes."))
  .author(crate_authors!("\n"))
  .long_about("river-status is a CLI tool that displays information about running instancies of river.
The output is always in JSON, but you can select to have it pretty printed for human readablility.

By default, it will display the names of each output and the current seat river is using.

NOTE: this tool uses the `river-status-unstable-v1` protocol which might be subject to significant breaking changes before river sees a stable 1.0 release.
")
  .version(crate_version!())
  ;

  generate_to(Bash, &mut cmd, app_name, "completions/bash")?;
  generate_to(Elvish, &mut cmd, app_name, "completions/elvish")?;
  generate_to(Fish, &mut cmd, app_name, "completions/fish")?;
  generate_to(PowerShell, &mut cmd, app_name, "completions/powershell")?;
  generate_to(Zsh, &mut cmd, app_name, "completions/zsh")?;

  let man = Man::new(cmd);
  let mut buffer: Vec<u8> = Default::default();
  man.render(&mut buffer)?;
  std::fs::write(man_dir.join("river-status.1"), buffer)?;

  Ok(())
}
