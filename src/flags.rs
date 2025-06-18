#![allow(clippy::print_stderr, reason = "This is a cli tool")]
#![allow(clippy::print_stdout, reason = "This is a cli tool")]
#![allow(clippy::struct_excessive_bools, reason = "The bools represent things that need to be separate")]
use std::sync::LazyLock;
use std::env;
use std::process::ExitCode;

/// Help text to print if -h or --help is args
const HELP: &str = "\
river-status

USAGE:
  river-status [FLAGS] [OPTIONS]

  If no flag or option is given, the output will be the same as if the user hade given
  `river-status --show-outputs --show-seat`

FLAGS:
  -a,   --all             Equivalent of -f -l -m -t --tag -u --view-tags
  -f,   --focused         Prints if a view is focused
  -h,   --help            Prints help information and exit
  -l,   --layout          Prints layout name
  -m,   --mode            Prints mode name
  -p,   --pretty          Pretty print the output JSON
        --show-outputs    Explicitly print outputs even if no other flag connected to outputs is in use
        --show-seat       Explicitly print seat even if no other flag connected to seat is in use
  -T,   --tag             Prints the focused tag
  -t,   --title           Prints the focused view title
  -u,   --urgent          Prints urgent tags
  -vt,  --view-tags       Prints the tags of all views
  -V,   --version         Prints the version of river-status and exit
  -w,   --watch           Continuously prints information as it changes.

OPTIONS:
  -o    --output STRING   Select a specific output
  -s    --seat STRING     Select a specific seat
        --sleep STRING    delay (in milliseconds) between calls to river for status updates. This option is a no-op without `--watch`.
";

pub static CONFIG: LazyLock<Flags> = LazyLock::new(new);

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct Flags {
  pub continuously: bool,
  pub exitcode: Option<ExitCode>,
  pub focused: bool,
  pub layout: bool,
  pub mode: bool,
  pub show_outputs: bool,
  pub show_seat: bool,
  pub output: Option<String>,
  pub pretty: bool,
  pub seat: Option<String>,
  pub sleep: Option<u64>,
  pub tags: bool,
  pub title: bool,
  pub urgent: bool,
  pub view: bool,
}

///Instanciates an instance of Flags
fn new() -> Flags {
  let mut default = Flags::default();
  let mut args = env::args().skip(1);
  let mut args_exists = false;

  while let Some(arg) = args.next() {
    args_exists = true;
    match &*arg {
      "-h" | "--help" => {
        println!("{HELP}");
        default.exitcode = Some(ExitCode::SUCCESS);
      },
      "-a" | "--all" => {
        default.focused = true;
        default.layout = true;
        default.mode = true;
        default.show_outputs = true;
        default.show_seat = true;
        default.tags = true;
        default.title = true;
        default.urgent = true;
        default.view = true;
      },
      "-f" | "--focused" => {
        default.focused = true;
        default.show_outputs = true;
      },
      "-l" | "--layout" => {
        default.layout = true;
        default.show_outputs = true;
      },
      "-m" | "--mode" => default.mode = true,
      "-o" | "--output" => {
        default.output = args.next();
        default.show_outputs = true;
      },
      "-p" | "--pretty" => default.pretty = true,
      "-q" | "--quiet" => {
        println!("Quiet mode is not supported yet.");
        default.exitcode = Some(ExitCode::SUCCESS);
      },
      "-s" | "--seat" => {
        default.seat = args.next();
        default.show_seat = true;
      },
      "--show-outputs" => default.show_outputs = true,
      "--show-seat" => default.show_seat = true,
      "--sleep" => {
        if let Some(time) = args.next() {
          match time.parse::<u64>() {
            Ok(i) => default.sleep = Some(i),
            Err(_) => eprintln!("--sleep, faulty value, ignoring option."),
          }
        }
      },
      "-T" | "--tag" => {
        default.tags = true;
        default.show_outputs = true;
      },
      "-t" | "--title" => default.title = true,
      "-u" | "--urgent" => {
        default.urgent = true;
        default.show_outputs = true;
      },
      "-v" | "--verbose" => {
        println!("Verbose mode is not supported yet.");
        default.exitcode = Some(ExitCode::SUCCESS);
      },
      "-V" | "--version" => {
        println!("{}", env!("CARGO_PKG_VERSION"));
        default.exitcode = Some(ExitCode::SUCCESS);
      },
      "-vt" | "--view-tags" => {
        default.view = true;
        default.show_outputs = true;
      },
      "-w" | "--watch" => default.continuously = true,
      _ => {
        if arg.starts_with('-') {
          println!("Unkown argument {arg}");
        } else {
          println!("Unkown positional argument {arg}");
        }
      },
    }
  }
  if !args_exists {
    default.show_outputs = true;
    default.show_seat = true;
  }
  default
}
