// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.
pub struct Order {
    pub product_name: String,
    pub quantity: u64,
    pub unit_price: u64,
}

impl Order {
    pub fn new(name: String, quantity: u64, unit_price: u64) -> Order {
        if name.is_empty() {
            panic!("Product name should not be empty!");
        }
        if name.len() > 300 {
            panic!("Product name is too long, should be 300 bytes");
        }
        if quantity == 0 {
            panic!("Zero quantity!");
        }
        if unit_price == 0 {
            panic!("Zero price!");
        }

        Order {
            product_name: name,
            quantity: quantity,
            unit_price: unit_price,
        }
    }

    pub fn product_name(&self) -> &String {
        &self.product_name
    }

    pub fn quantity(&self) -> &u64 {
        &self.quantity
    }

    pub fn unit_price(&self) -> &u64 {
        &self.unit_price
    }

    pub fn set_product_name(&mut self, name: String) {
        if name.is_empty() {
            panic!("Product name should not be empty!");
        }
        if name.len() > 300 {
            panic!("Product name is too long, should be 300 bytes");
        }
        self.product_name = name;
    }

    pub fn set_quantity(&mut self, quantity: u64) {
        if quantity == 0 {
            panic!("Zero quantity!");
        }
        self.quantity = quantity;
    }

    pub fn set_unit_price(&mut self, price: u64) {
        if price == 0 {
            panic!("Zero price!");
        }
        self.unit_price = price;
    }

    pub fn total(&self) -> u64 {
        self.quantity * self.unit_price
    }
}
