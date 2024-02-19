// Slices
fn slices_example() {
    // Define an array
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // Create a slice
    let slice = &numbers[1..3];

    // Print slice elements
    println!("Slice: {:?}", slice);
}

fn main() {
    slices_example();
}
