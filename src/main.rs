fn main() {
    println!("Life, the universe + the rest = {}", add(40, 2));
}

fn add(a: u16, b: u16 ) -> u16 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::add;

    #[test]
    fn test_add() {
        assert_eq!(add(12, 5), 17);
    }

    #[test]
    fn test_add_large() {
        assert_eq!(add(255, 1), 256);
    }
}
