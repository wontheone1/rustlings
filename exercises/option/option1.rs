// option1.rs
// Make me compile! Execute `rustlings hint option1` for hints

// you can modify anything EXCEPT for this function's sig
fn print_number(maybe_number: Option<u16>) {
    println!("printing: {}", maybe_number.unwrap());
}

fn main() {
    print_number(Some(13));
    print_number(Some(99));

    let mut numbers: [Option<u16>; 5] = [None; 5];
    for iter in 0..5 {
        let number_to_add: u16 = {
            (iter * 5) + 2
        };

        numbers[iter as usize] = Some(number_to_add);
        print_number(numbers[iter as usize]);
    }
}

// hint
// Hint 1: Check out some functions of Option:
// is_some
// is_none
// unwrap
//
// and:
// pattern matching
//
// Hint 2: There are no sensible defaults for the value of an Array; the values need to be filled before use.
