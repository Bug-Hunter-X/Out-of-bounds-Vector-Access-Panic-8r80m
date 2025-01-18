fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let index = 5;

    // Safe way to access vector elements: returns an Option
    match vec.get(index) {
        Some(value) => println!("Value at index {}: {}", index, value),
        None => println!("Index {} is out of bounds", index),
    }
} 