use console::Term;

const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
const UNIT_FACTOR: usize = 1024;

pub fn getch() -> char {
  if let Ok(c) = Term::stdout().read_char() {
    return c;
  }

  return '\0';
}

pub fn format_size_unit(bytes: usize) -> String {
  let mut value = bytes as f64;
  let mut unit_index = 0;

  while value >= UNIT_FACTOR as f64 && unit_index < UNITS.len() - 1 {
    value /= UNIT_FACTOR as f64;
    unit_index += 1;
  }

  format!("{:.2}{}", value, UNITS[unit_index])
}

pub fn digit_count(number: usize) -> usize {
  let mut n = number;
  if n == 0 {
    return 1;
  }

  let mut count = 0;
  while n > 0 {
    n /= 10;
    count += 1;
  }
  count
}
