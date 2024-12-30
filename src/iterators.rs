fn main() {
    // Iterating over an array
    let array = [1, 2, 3, 4, 5];
    println!("Iterating over an array:");
    for &item in &array {
        println!("Value: {}", item);
    }

    // Iterating over a vector with mutable access
    let mut vector = vec![10, 20, 30, 40, 50];
    println!("\nMutably iterating over a vector:");
    for item in &mut vector {
        *item += 1; // Mutate each element
        println!("Updated Value: {}", item);
    }

    // Using iterator methods on a vector
    println!("\nUsing map and filter:");
    let transformed: Vec<_> = vector.iter()
        .map(|x| x * 2) // Double each element
        .filter(|x| x % 3 == 0) // Keep only elements divisible by 3
        .collect();
    println!("Transformed Vector: {:?}", transformed);

    // Iterating over a range
    println!("\nIterating over a range:");
    (1..5).for_each(|x| println!("Range value: {}", x));

    // Using fold for reduction
    println!("\nUsing fold to sum elements:");
    let sum: i32 = array.iter().fold(0, |acc, &x| acc + x);
    println!("Sum of array: {}", sum);

    // Consuming iterator
    println!("\nConsuming iterator:");
    vector.into_iter().enumerate().for_each(|(index, value)| {
        println!("Index: {}, Value: {}", index, value);
    });

    // Creating an infinite iterator (lazily evaluated)
    println!("\nUsing an infinite iterator:");
    let first_five: Vec<_> = (1..).take(5).collect(); // Take the first 5 elements
    println!("First 5 natural numbers: {:?}", first_five);

    // Custom closure in an iterator
    println!("\nUsing a custom closure:");
    let odd_numbers: Vec<_> = (1..10).filter(|x| x % 2 != 0).collect();
    println!("Odd numbers: {:?}", odd_numbers);
}
