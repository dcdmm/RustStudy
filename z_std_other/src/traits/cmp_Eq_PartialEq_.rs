// Trait std::cmp::PartialEq/Trait std::cmp::Eq

/*
Trait for comparisons corresponding to equivalence relations.

This means, that in addition to a == b and a != b being strict inverses, the relation must be (for all a, b and c):
    * reflexive: a == a;
    * symmetric: a == b implies b == a (required by PartialEq as well); and
    *transitive: a == b and b == c implies a == c (required by PartialEq as well).

This property cannot be checked by the compiler, and therefore Eq implies PartialEq, and has no extra methods.
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

    impl PartialEq for Book {
        fn eq(&self, other: &Self) -> bool {
            self.isbn == other.isbn
        }
    }

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
    println!("{}", b1 != b3); // print-true
}

#[test]
fn t1() {
    // The derive implements <BookFormat> == <BookFormat> comparisons
    #[derive(PartialEq)]
    enum BookFormat {
        Paperback,
        Hardback,
        Ebook,
    }

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

    assert!(b1 == BookFormat::Paperback);
    assert!(BookFormat::Ebook != b1);
}
