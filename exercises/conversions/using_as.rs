// Type casting in Rust is done via the usage of the `as` operator.
// Please note that the `as` operator is not only used when type casting.
// It also helps with renaming imports.

// The goal is to make sure that the division does not fail to compile
fn average(values: &[f64]) -> f64 {
    let total = values
        .iter()
        .fold(0.0, |a, &b| a + b);
    total / values.len() as f64
}

fn main() {
    let values = [3.5, 0.5, 11.0];
    println!("{}", average(&values));
}

// Use the `as` operator to cast one of the operands in the last line of the
// `average` function into the expected return type.
