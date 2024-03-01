// trait(Using Trait Bounds to Conditionally Implement Methods)

use std::fmt::Display;

trait PrintBook {
    fn print_book(&self);
}

trait PrintContainer {
    fn print_container(&self);
}

struct Container<T> {
    item: T,
}

impl<T: PrintBook> PrintContainer for Container<T> {
    fn print_container(&self) {
        println!("Container holding:");
        self.item.print_book();
    }
}

struct Book {
    title: String,
}

impl PrintBook for Book {
    fn print_book(&self) {
        println!("{}", self.title);
    }
}

trait Printable {
    fn print(&self);
}

impl<T: Display> Printable for T {
    fn print(&self) {
        println!("{}", self);
    }
}

fn main() {
    let book = Book {
        title: String::from("Rust"),
    };
    let container = Container { item: book };
    container.print_container(); // Book实现了PrintBook特质,所以Container<Book>也实现了PrintContainer特质

    let my_string = "Hello, world!".to_string();
    my_string.print(); // String实现了Display特质,所以也实现了Printable特质
    let my_number = 42;
    my_number.print(); // i32实现了Display特质,所以也实现了Printable特质
}
