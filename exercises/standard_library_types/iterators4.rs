// iterators4.rs

pub fn factorial(num: u64) -> u64 {
    (1..=num).fold(1, |acc, e| acc * e)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}

// In an imperative language, you might write a for loop that updates
// a mutable variable. Or, you might write code utilizing recursion
// and a match clause. In Rust you can take another functional
// approach, computing the factorial elegantly with ranges and iterators.
