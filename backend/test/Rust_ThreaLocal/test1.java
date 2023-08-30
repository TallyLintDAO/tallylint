import java.util.ArrayList;

// A simple class that represents a username and a password
class Credentials {
    private String username;
    private String password;

    // A default constructor that creates an empty credentials
    public Credentials() {
        this.username = "";
        this.password = "";
    }

    // A method that sets the username and the password
    public void set(String username, String password) {
        this.username = username;
        this.password = password;
    }

    // A method that returns the username
    public String getUsername() {
        return this.username;
    }

    // A method that returns the password
    public String getPassword() {
        return this.password;
    }

    // A method that returns a string representation of the credentials
    public String toString() {
        return "Credentials{" +
                "username='" + username + '\'' +
                ", password='" + password + '\'' +
                '}';
    }
}

// A wrapper class that provides access to the thread local value
class ThreadLocalRef<T> {
    private ThreadLocal<T> inner;

    // A constructor that takes a thread local key and creates a wrapper
    public ThreadLocalRef(ThreadLocal<T> inner) {
        this.inner = inner;
    }

    // A method that returns the thread local value
    public T get() {
        return this.inner.get();
    }

    // A method that sets the thread local value
    public void set(T value) {
        this.inner.set(value);
    }
}

public class test1 {

    // Define a thread-local object that contains a username and a password
    private static final ThreadLocal<Credentials> CREDENTIALS = new ThreadLocal<>();

    // A function that simulates adding an item to the cart
    public static void add_to_cart(String item) {
        // Get the thread local credentials using the wrapper
        ThreadLocalRef<Credentials> c = new ThreadLocalRef<>(CREDENTIALS);
        // Print a message with the username and the item
        System.out.println(c.get().getUsername() + " added " + item + " to the cart");
    }

    // A function that simulates making a payment
    public static void payment(double amount) {
        // Get the thread local credentials using the wrapper
        ThreadLocalRef<Credentials> c = new ThreadLocalRef<>(CREDENTIALS);
        // Print a message with the username, the password and the amount
        System.out.println(
                c.get().getUsername() + " paid " + amount + " with password " + c.get().getPassword()
        );
    }

    public static void main(String[] args) {
        // Create a wrapper for the thread local credentials
        ThreadLocalRef<Credentials> c = new ThreadLocalRef<>(CREDENTIALS);

        // Set the credentials using the wrapper
        c.set(new Credentials());
        c.get().set("Alice", "secret");

        // Print the credentials using the wrapper
        System.out.println(c.get());

        // Call the add_to_cart function with some item
        add_to_cart("book");

        // Call the payment function with some amount
        payment(10.0);
    }
}
