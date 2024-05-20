use crate::utils::format_size_unit;
use console::style;

pub struct ProgressValue {
  pub value: usize,
  pub size: usize,
  pub prefix: String,
  pub suffix: String,
}

impl ProgressValue {
  pub fn new() -> ProgressValue {
    ProgressValue {
      value: 0,
      size: 1,
      prefix: String::new(),
      suffix: String::new(),
    }
  }

  pub fn set_prefix<S: AsRef<str>>(&mut self, prefix: S) {
    self.prefix = prefix.as_ref().to_string();
  }

  pub fn set_suffix<S: AsRef<str>>(&mut self, suffix: S) {
    self.suffix = suffix.as_ref().to_string();
  }

  pub fn set_size_current(&mut self, value: usize) {
    self.value = value.min(self.size);
  }

  pub fn set_size_max(&mut self, size: usize) {
    self.size = if size == 0 { 1 } else { size };
  }

  fn get_percentage(&self) -> String {
    format!(
      "{:.2}%",
      ((self.value as f64) / (self.size as f64) * 100_f64).min(100_f64)
    )
  }

  pub fn get_text(&self) -> String {
    format!(
      "{} {} {} of {} [{}] {}",
      style("Info:").blue().bright().bold(),
      self.prefix,
      style(format_size_unit(self.value)).yellow(),
      style(format_size_unit(self.size)).yellow(),
      style(self.get_percentage()).yellow(),
      self.suffix
    )
  }

  pub fn get_text_size(&self) -> usize {
    let percent = (self.value as f64) / (self.size as f64) * 100_f64;

    return 19
      + self.prefix.chars().count()
      + self.suffix.chars().count()
      + format_size_unit(self.value).chars().count()
      + format_size_unit(self.size).chars().count()
      + if percent < 10_f64 {
        1
      } else if percent == 100_f64 {
        3
      } else {
        2
      };
  }
}
