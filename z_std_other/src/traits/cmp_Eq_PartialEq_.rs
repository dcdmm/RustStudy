// Trait std::cmp::PartialEq/Trait std::cmp::Eq

// # Trait std::cmp::PartialEq
/*
Trait for comparisons using the equality operator.

Implementing this trait for types provides the == and != operators for those types.

x.eq(y) can also be written x == y, and x.ne(y) can be written x != y.

This trait can be used with #[derive]. When derived on structs, two instances are equal if all fields are equal, and not equal if any fields are not equal. When derived on enums, two instances are equal if they are the same variant and all fields are equal.
*/

// # Trait std::cmp::Eq
/*
Trait for comparisons corresponding to equivalence relations.

This means, that in addition to a == b and a != b being strict inverses, the relation must be (for all a, b and c):
    * reflexive: a == a;
    * symmetric: a == b implies b == a (required by PartialEq as well); and
    * transitive: a == b and b == c implies a == c (required by PartialEq as well).

This property cannot be checked by the compiler, and therefore Eq implies PartialEq, and has no extra methods.

This trait can be used with #[derive]. When derived, because Eq has no extra methods, it is only informing the compiler that this is an equivalence relation rather than a partial equivalence relation. Note that the derive strategy requires all fields are Eq, which isn’t always desired.
*/

// Required Methods
/*
fn eq(&self, other: &Rhs) -> bool
    This method tests for self and other values to be equal, and is used by ==.
*/

#[test]
fn t0() {
    enum BookFormat {
        Paperback,
        Hardback,
        Ebook,
    }

    struct Book {
        isbn: i32,
        format: BookFormat,
    }

    // 手动实现
    impl PartialEq for Book {
        fn eq(&self, other: &Self) -> bool {
            self.isbn == other.isbn
        }
    }

    // 手动实现
    impl Eq for Book {}

    let b1 = Book {
        isbn: 3,
        format: BookFormat::Paperback,
    };
    let b2 = Book {
        isbn: 3,
        format: BookFormat::Ebook,
    };
    let b3 = Book {
        isbn: 10,
        format: BookFormat::Paperback,
    };

    println!("{}", b1 == b2); // print->true
    println!("{}", b1.eq(&b2)); // 与上等价
    println!("{}", b1 != b3); // print-true
    println!("{}", b1.ne(&b3)); // 与上等价
}

#[test]
fn t1() {
    // derive属性自动实现
    #[derive(PartialEq, Eq)]
    enum BookFormat {
        Paperback,
        Hardback,
        Ebook,
    }

    enum BookFormat1 {
        Paperback,
        Hardback,
        Ebook,
    }

    #[derive(PartialEq, Eq)]
    struct Book {
        isbn: i32,
        format: BookFormat,
    }

    // Implement <Book> == <BookFormat> comparisons
    impl PartialEq<BookFormat> for Book {
        fn eq(&self, other: &BookFormat) -> bool {
            self.format == *other
        }
    }

    // Implement <BookFormat> == <Book> comparisons
    impl PartialEq<Book> for BookFormat {
        fn eq(&self, other: &Book) -> bool {
            *self == other.format
        }
    }

    let b1 = Book {
        isbn: 3,
        format: BookFormat::Paperback,
    };

    let b2 = Book {
        isbn: 4,
        format: BookFormat::Paperback,
    };

    println!("{}", b1 == BookFormat::Paperback); // print->true
    println!("{}", BookFormat::Ebook != b1); // print->true
    println!("{}", b1 == b2); // print->false
    println!("{}", BookFormat::Paperback == b1.format); // print->true
                                                        // error[E0277]: can't compare `cmp_Eq_PartialEq_::t1::BookFormat` with `BookFormat1`
                                                        // println!("{}", BookFormat::Paperback == BookFormat1::Paperback); // `==`: no implementation for `cmp_Eq_PartialEq_::t1::BookFormat == BookFormat1`
}
