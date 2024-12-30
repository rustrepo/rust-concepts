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

use std::fmt;  // Import the fmt module for formatting output
    

    #[derive(Debug)]
    enum Category {  // Enum for item categories
        Electronics,
        Clothing,
        Grocery,
    }
/*
Enum Category:

enum Category defines different categories for items (e.g., Electronics, Clothing, Grocery).
The fmt::Display implementation is necessary to print out these categories as strings in a user-friendly format.

The fmt::Display trait provides a standardized way to convert a custom type (like Category) into a user-friendly string.
This allows enums and structs to integrate seamlessly with Rust's formatting macros like println!, write!, etc.
*/

// Implementing fmt::Display trait for Category to format the output as a string

    impl fmt::Display for Category {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

            let category = match self {
                Category::Electronics => "Electronics",
                Category::Clothing => "Clothing",
                Category::Grocery => "Grocery",
            };
            
            write!(f, "{}", category) //// Format the output. A return expression cannot have ";", otherwise it will throw error.
            
        }
    }

    // Struct for an inventory item
    struct Item<T> {
        name: String,
        price: T, // The price of the item, using a generic type T (can be any type, e.g., f64 for floating-point numbers)
        category: Category,
    }

    // Implementing methods for Item<T>
    // Method to display item details. T must implement fmt::Display to be used here

    impl<T: fmt::Display> Item<T> {
        fn display(&self) {
            println!("Item: {}, Price: {}, Category: {}", self.name, self.price, self.category );
        }
    }


    //Trait for discount

    trait Discount {
        fn apply_discount(&mut self, percentage: f64); // Method to apply discount, modifies the item (mut)
    }

    // Implementing Discount trait for Item<f64>, meaning only items with a price of type f64 will use this implementation
    impl Discount for Item<f64> {
        fn apply_discount(&mut self, percentage: f64){
            self.price -= self.price * (percentage/100.0);
        }
    }



    fn main(){
        // Vector to hold items, specifically items with a price of type f64 (floating-point numbers)
        let mut inventory: Vec<Item<f64>> = vec! [
            Item {
                name: "Laptop".to_string(),
                price: 1200.0, // The price is a floating-point number (f64)
                category: Category::Electronics // Category is set to Electronics
            },

            Item {
                name: "T-Shirt".to_string(),
                price: 60.0,
                category: Category::Clothing
            },
            Item {
                name: "Vegetables".to_string(),
                price: 10.0,
                category: Category::Grocery
            },

        ];

         // Loop through the inventory and display items
         println!("Original Inventory :\n ");

         for item in &inventory {
            item.display(); //this why we need method, without it, i would have written this line 3 times
         };

         // Apply a discount to all electronics using a mutable iterator
         println!("\nApplying 10% discount to Electronics.. ", );

         for item in inventory.iter_mut() { //iter_mut() allows us to mutate the item
            if let Category::Electronics = item.category { // Match the category to identify electronics
                item.apply_discount(10.0); // Apply a 10% discount to electronics
            }
         };


         // Filter items using a closure and collect results into a new vector
         println!("\nFiltered Items (Price > 20):");

         let expensive_items: Vec<&Item<f64>> = inventory 
         .iter() // Create an iterator over the inventory
         .filter(|item| item.price > 20.0) // Use a closure to filter out items with a price greater than 20.0
         .collect(); // Collect the filtered items into a new vector of references

         /*
         Closures and Iterators:

A closure |item| item.price > 20.0 is used within the filter method to filter out items that cost more than 20.
This makes the code concise and readable.
iter_mut() and iter() are used to iterate over the vector, with iter_mut() allowing for mutable access to items 
(useful for applying discounts).
*/

         for item in &expensive_items {
            item.display();
         };

         // Match usage to count categories
         let mut category_counts = vec![0,0,0]; //create an empty vector

         for item in &inventory {
            match item.category {
                Category::Electronics => category_counts[0] += 1, // Increment the Electronics counter
                Category::Clothing => category_counts[1] += 1,
                Category::Grocery => category_counts[0] += 1,
            }
         };
         println!("Total Electronics : {}", category_counts[0]); 
         println!("Total Clothing : {}", category_counts[1]);
         println!("Total Grocery : {}", category_counts[2]);
        
         // Display updated inventory (after applying discounts)
         println!("\nUpdated Inventory:");

         for item in &inventory {
            item.display();
         }


    }



