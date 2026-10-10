// Conditionally Implement

use std::fmt::Display;

#[allow(warnings)]
struct Pair<T> {
    x: T,
    y: T,
}

// 没有额外约束
#[allow(warnings)]
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Pair<T> only implements the cmp_display method if its inner type T implements the PartialOrd trait that enables comparison and the Display trait that enables printing.
#[allow(warnings)]
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

#[test]
fn t0() {
    let ps = Pair { x: 1, y: 2 };
    ps.cmp_display(); // i32实现了Display + PartialOrd trait

    let ps1 = Pair {
        x: vec![1, 2],
        y: vec![3, 4],
    };
    // the method `cmp_display` exists for struct `Pair<Vec<{integer}>>`, but its trait bounds were not satisfied
    // ps1.cmp_display();
}

#[test]
fn t1() {
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

    struct Moive {
        title: String,
    }

    impl PrintBook for Book {
        fn print_book(&self) {
            println!("{}", self.title);
        }
    }

    let book = Book {
        title: String::from("Rust"),
    };
    let container = Container { item: book };
    container.print_container(); // Book实现了PrintBook特质,故可以为Container<T>实现PrintContainer特质

    let moive = Moive {
        title: String::from("大话西游"),
    };
    let container_m = Container { item: moive };
    // error[E0599]: the method `print_container` exists for struct `Container<Moive>`, but its trait bounds were not satisfied
    // container_m.print_container(); // Moive没有实现PintBook特质
}
