/// This code models an inventory system for a store, showcasing various Rust features:
/// - `Vector`: for managing lists of items
/// - `Struct`: to define item properties
/// - `Methods`: to define behaviors of the struct
/// - `Generic`: to work with different data types
/// - `Trait`: for shared behavior
/// - `Closure`: for filtering items
/// - `Match`: for categorizing items
/// - `Enum`: for representing item categories
/// - `Loops` and `Iterator`: for processing items

#[derive(Debug)]
enum Category {  // Enum for item categories
    Electronics,
    Clothing,
    Grocery,
}

#[derive(Debug)]
struct Item<T> {
    name: String,
    price: T, // The price of the item, using a generic type T (can be any type, e.g., f64 for floating-point numbers)
    category: Category,
}

trait Discount {
    fn apply_discount(&mut self, percentage: f64); // Method to apply discount, modifies the item (mut)
}

// Implementing Discount trait for Item<f64>, meaning only items with a price of type f64 will use this implementation
impl Discount for Item<f64> {
    fn apply_discount(&mut self, percentage: f64) {
        self.price -= self.price * (percentage / 100.0);
    }
}

fn main() {
    // Vector to hold items, specifically items with a price of type f64 (floating-point numbers)
    let mut inventory: Vec<Item<f64>> = vec![
        Item {
            name: "Laptop".to_string(),
            price: 1200.0, // The price is a floating-point number (f64)
            category: Category::Electronics, // Category is set to Electronics
        },
        Item {
            name: "T-Shirt".to_string(),
            price: 60.0,
            category: Category::Clothing,
        },
        Item {
            name: "Vegetables".to_string(),
            price: 10.0,
            category: Category::Grocery,
        },
    ];

    // Loop through the inventory and display items
    println!("Original Inventory:\n");
    for item in &inventory {
        println!("{:?}", item);
    }

    // Apply a discount to all electronics using a mutable iterator
    println!("\nApplying 10% discount to Electronics..",);
    for item in inventory.iter_mut() {
        if let Category::Electronics = item.category {
            item.apply_discount(10.0); // Apply a 10% discount to electronics
        }
    }

    // Filter items using a closure and collect results into a new vector
    println!("\nFiltered Items (Price > 20):");
    let expensive_items: Vec<&Item<f64>> = inventory
        .iter()
        .filter(|item| item.price > 20.0) // Use a closure to filter out items with a price greater than 20.0
        .collect();

    for item in &expensive_items {
        println!("{:?}", item);
    }

    // Match usage to count categories
    let mut category_counts = vec![0, 0, 0]; // Create an empty vector
    for item in &inventory {
        match item.category {
            Category::Electronics => category_counts[0] += 1, // Increment the Electronics counter
            Category::Clothing => category_counts[1] += 1,
            Category::Grocery => category_counts[2] += 1,
        }
    }
    println!("Total Electronics: {}", category_counts[0]);
    println!("Total Clothing: {}", category_counts[1]);
    println!("Total Grocery: {}", category_counts[2]);

    // Display updated inventory (after applying discounts)
    println!("\nUpdated Inventory:");
    for item in &inventory {
        println!("{:?}", item);
    }
}
