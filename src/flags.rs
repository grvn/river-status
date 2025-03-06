use std::env;

const HELP: &str = "\
river-status

USAGE:
  river-status [FLAGS] [OPTIONS]

FLAGS:
  -a, --all             Equivalent of -f -l -t --tag -u --view-tags
  -f, --focused         Prints if a view is focused
  -h, --help            Prints help information and exit
  -l, --layout          Prints layout name
      --tag             Prints the focused tag
  -t, --title           Prints the focused view title
  -u, --urgent          Prints urgent tags
      --view-tags       Prints the tags of all views
  -w, --watch           Continuously prints information as it changes

OPTIONS:
  -o --output STRING    Select a specific output
  -s --seat STRING      Select a specific seat
";

#[derive(Debug)]
pub struct Flags {
  pub continuously: bool,
  pub focused: bool,
  pub layout: bool,
  pub mode: bool,
  pub output: Option<String>,
  pub pretty: bool,
  pub seat: Option<String>,
  pub tags: bool,
  pub title: bool,
  pub urgent: bool,
  pub view: bool,
}

// Set the default values for Flags
impl Default for Flags {
  fn default() -> Flags {
    Flags {
      continuously: false,
      focused: false,
      layout: false,
      mode: false,
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

pub fn get_configuration() -> Flags {
  let mut default= Flags::default();
  let mut args = env::args().skip(1);

  while let Some(arg) = args.next() {
    match &arg[..] {
      "-h" | "--help" => {
        print!("{}", HELP);
        std::process::exit(0);
      }
      "-a" | "--all" => {
        default.focused = true;
        default.layout = true;
        default.tags = true;
        default.title = true;
        default.urgent = true;
        default.view = true;
      }
      "-f" | "--focused" => default.focused = true,
      "-l" | "--layout" => default.layout = true,
      "-o" | "--output" => default.output = args.next(),
      "-p" | "--pretty" => default.pretty = true,
      "-q" | "--quiet" => println!("Quiet mode is not supported yet."),
      "-s" | "--seat" => default.seat = args.next(),
      "--tag" => default.tags = true,
      "-t" | "--title" => default.title = true,
      "-u" | "--urgent" => default.urgent = true,
      "-v" | "--verbose" => println!("Verbose mode is not supported yet."),
      "--version" => println!("Version info is not supported yet."),
      "--view-tags" => default.view = true,
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