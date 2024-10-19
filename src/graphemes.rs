#![allow(dead_code)]

use unicode_segmentation::UnicodeSegmentation;

trait Trim {
  fn trim_count(&self, count: usize) -> String;
}

impl Trim for str {
  fn trim_count(&self, count: usize) -> String {
    UnicodeSegmentation::graphemes(self, true).take(count).collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_graphemes() {
    let s = "a̐éö̲\r\n";
    let result = s.trim_count(2);
    assert_eq!(result, "a̐é");
  }
}
