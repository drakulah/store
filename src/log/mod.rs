use console::style;

pub mod progress_value;
pub mod progress_viewer;

pub fn error(msg: &str) {
  println!("{} {}", style("Error:").red().bright().bold(), msg);
}

pub fn warning(msg: &str) {
  println!("{} {}", style("Warning:").yellow().bright(), msg);
}

pub fn abort(msg: &str) {
  println!("{} {}", style("Abort:").magenta().bright().bold(), msg);
}

pub fn info(msg: &str) {
  println!("{} {}", style("Info:").blue().bright().bold(), msg);
}

pub fn success(msg: &str) {
  println!("{} {}", style("Success:").green().bright().bold(), msg);
}
