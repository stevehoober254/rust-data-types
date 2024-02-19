// Tuples
fn tuples_example() {
    // Define a tuple
    let person: (&str, i32) = ("Alice", 30);

    // Access tuple elements
    let (name, age) = person;

    // Print tuple elements
    println!("Name: {}, Age: {}", name, age);
}

// Arrays
fn arrays_example() {
    // Define an array
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // Access array elements
    let first_number = numbers[0];

    // Print array elements
    println!("First number: {}", first_number);
}

// Structs
struct Person {
    name: String,
    age: i32,
}

fn structs_example() {
    // Create an instance of Person
    let person = Person {
        name: String::from("Bob"),
        age: 25,
    };

    // Access struct fields
    println!("Name: {}, Age: {}", person.name, person.age);
}

fn main() {
    // Example usage of compound data types
    tuples_example();
    arrays_example();
    structs_example();
}
