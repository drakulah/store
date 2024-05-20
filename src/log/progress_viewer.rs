use super::progress_value::ProgressValue;

pub struct ProgressViewer {
  prev_text_size: usize,
}

impl ProgressViewer {
  pub fn new() -> ProgressViewer {
    ProgressViewer { prev_text_size: 0 }
  }

  pub fn update(&mut self, value: &ProgressValue) {
    let new_text_size = value.get_text_size();
    let spaces = if self.prev_text_size <= new_text_size {
      0
    } else {
      self.prev_text_size - new_text_size
    };
    print!("\r{}{}", value.get_text(), " ".repeat(spaces));
    self.prev_text_size = new_text_size;
  }
}
