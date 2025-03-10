use once_cell::sync::Lazy;
use std::env;
use std::process::exit;

const HELP: &str = "\
river-status

USAGE:
  river-status [FLAGS] [OPTIONS]

FLAGS:
  -a,   --all             Equivalent of -f -l -m -t --tag -u --view-tags
  -f,   --focused         Prints if a view is focused
  -h,   --help            Prints help information and exit
  -l,   --layout          Prints layout name
  -m,   --mode            Prints mode name
        --no-output       Explicitly remove all outputs from print
        --no-seat         Explicitly remove seat from print
  -p,   --pretty          Pretty print the output JSON
  -T,   --tag             Prints the focused tag
  -t,   --title           Prints the focused view title
  -u,   --urgent          Prints urgent tags
  -vt,  --view-tags       Prints the tags of all views
  -V,   --version         Prints the version of river-status and exit
  -w,   --watch           Continuously prints information as it changes

OPTIONS:
  -o    --output STRING   Select a specific output
  -s    --seat STRING     Select a specific seat
";

pub static CONFIG: Lazy<Flags> = Lazy::new(get_configuration);

#[derive(Debug, Default)]
pub struct Flags {
  pub continuously: bool,
  pub focused: bool,
  pub layout: bool,
  pub mode: bool,
  pub no_output: bool,
  pub no_seat: bool,
  pub output: Option<String>,
  pub pretty: bool,
  pub seat: Option<String>,
  pub tags: bool,
  pub title: bool,
  pub urgent: bool,
  pub view: bool,
}

// Set the default values for Flags
impl Flags {
  fn default() -> Flags {
    Flags {
      continuously: false,
      focused: false,
      layout: false,
      mode: false,
      no_output: false,
      no_seat: false,
      output: None,
      pretty: false,
      seat: None,
      tags: false,
      title: false,
      urgent: false,
      view: false,
    }
  }
}

fn get_configuration() -> Flags {
  let mut default = Flags::default();
  let mut args = env::args().skip(1);

  while let Some(arg) = args.next() {
    match &arg[..] {
      "-h" | "--help" => {
        println!("{}", HELP);
        exit(0);
      }
      "-a" | "--all" => {
        default.focused = true;
        default.layout = true;
        default.mode = true;
        default.tags = true;
        default.title = true;
        default.urgent = true;
        default.view = true;
      }
      "-f" | "--focused" => default.focused = true,
      "-l" | "--layout" => default.layout = true,
      "--no-output" => default.no_output = true,
      "--no-seat" => default.no_seat = true,
      "-m" | "--mode" => default.mode = true,
      "-o" | "--output" => default.output = args.next(),
      "-p" | "--pretty" => default.pretty = true,
      "-q" | "--quiet" => {
        println!("Quiet mode is not supported yet.");
        exit(0)
      }
      "-s" | "--seat" => default.seat = args.next(),
      "-T" | "--tag" => default.tags = true,
      "-t" | "--title" => default.title = true,
      "-u" | "--urgent" => default.urgent = true,
      "-v" | "--verbose" => {
        println!("Verbose mode is not supported yet.");
        exit(0)
      }
      "-V" | "--version" => {
        println!("{}", env!("CARGO_PKG_VERSION"));
        exit(0)
      }
      "-vt" | "--view-tags" => default.view = true,
      "-w" | "--watch" => default.continuously = true,
      _ => {
        if arg.starts_with('-') {
          println!("Unkown argument {}", arg);
        } else {
          println!("Unkown positional argument {}", arg);
        }
      }
    }
  }
  default
}
