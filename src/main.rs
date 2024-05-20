use crate::cmd_processor::CommandProcessor;
use clap::{
  builder::{
    styling::{AnsiColor, Effects},
    Styles,
  },
  value_parser, Arg, ArgAction, Command,
};
use std::path::PathBuf;

pub mod cmd_processor;
pub mod file_crypto;
pub mod log;
pub mod utils;

pub const DEFAULT_ENCRYPTED_FILE_EXT: &str = ".enc";

fn main() {
  let styles = Styles::styled()
    .header(AnsiColor::Green.on_default() | Effects::BOLD)
    .usage(AnsiColor::Green.on_default() | Effects::BOLD)
    .literal(AnsiColor::BrightCyan.on_default() | Effects::BOLD)
    .placeholder(AnsiColor::Cyan.on_default());

  let matches = Command::new("store")
    .styles(styles)
    .bin_name("store")
    .about("Store to keep files secure using Cryptography")
    .subcommand_required(true)
    .arg_required_else_help(true)
    .subcommand(
      Command::new("encrypt").about("Encrypt a file").args([
        Arg::new("force")
          .short('f')
          .long("force")
          .help("Use force to overwrite the files if exists")
          .action(ArgAction::SetTrue),
        Arg::new("input")
          .short('i')
          .long("src")
          .help("Path to source file")
          .required(true)
          .num_args(1)
          .value_parser(value_parser!(PathBuf)),
        Arg::new("output")
          .short('o')
          .long("dest")
          .help("Path to destination file")
          .required(false)
          .num_args(1)
          .value_parser(value_parser!(PathBuf)),
      ]),
    )
    .subcommand(
      Command::new("decrypt").about("Decrypt a file").args([
        Arg::new("force")
          .short('f')
          .long("force")
          .help("Use force to overwrite the files if exists")
          .action(ArgAction::SetTrue),
        Arg::new("input")
          .short('i')
          .long("src")
          .help("Path to source file")
          .required(true)
          .num_args(1)
          .value_parser(value_parser!(PathBuf)),
        Arg::new("output")
          .short('o')
          .long("dest")
          .help("Path to destination file")
          .required(false)
          .num_args(1)
          .value_parser(value_parser!(PathBuf)),
      ]),
    )
    .get_matches();

  match matches.subcommand() {
    Some(("encrypt", m)) => {
      let src = m.get_one::<PathBuf>("input").unwrap();
      let dest = m.get_one::<PathBuf>("output");
      let force = m.get_flag("force");

      CommandProcessor::encrypt(src, dest, force);
    }

    Some(("decrypt", m)) => {
      let src = m.get_one::<PathBuf>("input").unwrap();
      let dest = m.get_one::<PathBuf>("output");
      let force = m.get_flag("force");

      CommandProcessor::decrypt(src, dest, force);
    }

    _ => {}
  }
}
