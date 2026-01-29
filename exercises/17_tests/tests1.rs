// Tests are important to ensure that your code does what you think it should
// do.

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    println!("8 is an even number: {}", is_even(8));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert() {
        assert!(!is_even(77), "expected 77 to be false");
        assert!(is_even(8), "expected 8 to be true");
    }
}
