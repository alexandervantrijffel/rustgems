#[cfg(test)]
mod tests {
  use std::cell::RefCell;
  /// Use ``RefCell`` to mutate a variable in a closure
  #[test]
  fn test_mutate_in_closure() {
    let my_string = RefCell::new("hello".to_string());
    let mutate = |new_val| {
      *my_string.borrow_mut() = new_val;
    };
    // mutate in closure
    mutate("hej".to_string());
    assert_eq!(*my_string.borrow(), "hej");

    *my_string.borrow_mut() = "bonjour".to_string();
    assert_eq!(*my_string.borrow(), "bonjour");
  }
}
