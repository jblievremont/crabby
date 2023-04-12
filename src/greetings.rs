pub fn greets(who: &str) -> String {
  format!("Hello, {who}!")
}

#[cfg(test)]
mod tests {
    use super::greets;

    #[test]
    fn test_greet() {
      assert_eq!("Hello, Kitty!", greets("Kitty"));
    }
}
