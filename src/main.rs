//1. Display Inventory, 
//2. Apply Discount of 10% to Electronics product
//3. Display expensive items with price>20.0
//4. Increase the price by 2X in new Inventory

#[derive(Debug, Clone)]
enum Category {
    Electronics,
    Clothing,
    Grocery,
}

#[derive(Debug, Clone)]
struct Item<T> {
    name: String,
    price: T,
    category: Category
}

trait Discount {
    fn apply_discount(&mut self, percentage: f64);
}

impl Discount for Item<f64> {
    fn apply_discount(&mut self, percentage: f64){
        self.price -= self.price*(percentage/100.0);
    }
}

fn main(){

    let mut inventory: Vec<Item<f64>> = vec![
        Item{name: "Laptop".to_string(), price: 1200.00, category: Category::Electronics},
        Item{name: "T-Shirt".to_string(), price: 200.00, category: Category::Clothing},
        Item{name: "Vegetable".to_string(), price: 50.00, category: Category::Grocery},
        ];


    println!("\nInventory List is...");
    for item in &inventory {
        println!("{:?}", item );
    };

    for item in inventory.iter_mut() { //The .iter_mut() method creates an iterator that yields mutable references to the items in the vector.
        if let Category::Electronics = item.category {
            item.apply_discount(10.0);
        }
    };

    println!("\nExpensive item list is");
    let expensive_items: Vec<&Item<f64>> = inventory.iter().filter(|item| item.price>50.0).collect();
    for item in expensive_items{
        println!("{:?}", item);
    };

    let updated_price: Vec<Item<f64>> = inventory
    .iter()
    .map(|item| Item{
        name: item.name.clone(),
        price: item.price*2.0,
        category: item.category.clone()
    })
    .collect();

    println!("\nUpdated price list is");

    for item in updated_price {
        println!("{:?}",item );
    };

    println!("\nDiscounted inventory is..");
    for item in &inventory {
        println!("{:?}", item );
    };

    
}