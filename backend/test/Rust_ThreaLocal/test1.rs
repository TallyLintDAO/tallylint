use std::cell::RefCell;

// Define a thread-local struct that contains a username and a password
thread_local! {
    static CREDENTIALS: RefCell<Credentials> = RefCell::new(Credentials::default());
}

// A simple struct that represents a username and a password
#[derive(Debug)]
struct Credentials {
    username: String,
    password: String,
}

impl Credentials {
    // A default constructor that creates an empty credentials
    fn default() -> Self {
        Self {
            username: String::new(),
            password: String::new(),
        }
    }

    // A method that sets the username and the password
    fn set(&mut self, username: &str, password: &str) {
        self.username = username.to_owned();
        self.password = password.to_owned();
    }
}

// A function that simulates adding an item to the cart
fn add_to_cart(item: &str) {
    // Access the thread-local credentials using the with method
    CREDENTIALS.with(|c| {
        // Borrow the credentials as immutable
        let c = c.borrow();
        // Print a message with the username and the item
        println!("{} added {} to the cart", c.username, item);
    });
}

// A function that simulates making a payment
fn payment(amount: f64) {
    // Access the thread-local credentials using the with method
    CREDENTIALS.with(|c| {
        // Borrow the credentials as immutable
        let c = c.borrow();
        // Print a message with the username, the password and the amount
        println!(
            "{} paid {} with password {}",
            c.username, amount, c.password
        );
    });
}

fn main() {
    // Access the thread-local credentials using the with method
    CREDENTIALS.with(|c| {
        // Borrow the credentials as mutable and set them
        let mut c = c.borrow_mut();
        c.set("Alice", "secret");
        // Print the credentials
        println!("{:?}", c);
    });

    // Call the add_to_cart function with some item
    add_to_cart("book");

    // Call the payment function with some amount
    payment(10.0);
}
