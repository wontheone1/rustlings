// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` for hints!!

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];
    // let a = [1, 2, 3, 4, 5].deref(); // Q: Why this doesn't work?
    // A: the coercion from arrays to slices doesn't happen via dereference
    // it happens via unsizing, a completely different mechanism
    // the coercion [T; N] -> [T] happens through the same means as Type -> dyn Trait
    // refer https://doc.rust-lang.org/std/marker/trait.Unsize.html

    let nice_slice:[i32; 3] = [2,3,4]; // this works too
    let nice_slice = &a[1..4]; // this works too

    assert_eq!([2, 3, 4], nice_slice)
}
