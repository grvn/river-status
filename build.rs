#![allow(missing_docs, reason = "I ignore docs. right now")]
#![allow(clippy::cognitive_complexity, reason = "I do not think this has high complexity right now")]
#![allow(clippy::print_stderr, reason = "I like using stdout and stderr")]
#![allow(unused_results, reason = "I do not care about the result for generating completions")]

//! # Build crate
//!
//! This crate creates documentation and shell completions

use clap::{Command, crate_authors, crate_version};
use clap_complete::Shell::{Bash, Elvish, Fish, PowerShell, Zsh};
use clap_complete::generate_to;
use clap_mangen::Man;
use core::error::Error;
use std::path::PathBuf;
use std::{env, fs};

///The name of the binary
static APP_NAME: &str = "river-status";

/// Create the docs and shell completions
fn main() {
  println!("cargo::rerun-if-changed=build.rs");

  let cmd = Command::new(APP_NAME)
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
  .arg(clap::arg!(--"sleep" "optional delay (in milliseconds) between calls to river for status updates. This option is a no-op without `--watch`."))
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

  if let Err(e) = generate_man_pages(cmd.clone()) {
    println!("cargo::warning=Error generating man pages: {e}");
  }
  if let Err(e) = generate_shell_completions(cmd) {
    println!("cargo::warning=Error generating shell completions: {e}");
  }
}

/// generates the man pages
fn generate_man_pages(cmd: Command) -> Result<(), Box<dyn Error>> {
  let man_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("man");
  let mut buffer = Vec::<u8>::new();
  Man::new(cmd).render(&mut buffer)?;
  fs::create_dir_all(&man_dir)?;
  fs::write(man_dir.join(APP_NAME.to_owned() + ".1"), buffer)?;
  Ok(())
}

/// generates the shell completions
fn generate_shell_completions(mut cmd: Command) -> Result<(), Box<dyn Error>> {
  let cmp_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("completions");
  fs::create_dir_all(&cmp_dir)?;
  for shell in [(Bash, "bash"), (Elvish, "elvish"), (Fish, "fish"), (PowerShell, "powershell"), (Zsh, "zsh")] {
    generate_to(shell.0, &mut cmd, APP_NAME, cmp_dir.join(shell.1))?;
  }
  Ok(())
}
