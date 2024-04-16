// Creating Type Synonyms with Type Aliases

/*
Define an alias for an existing type.

The syntax is type Name = ExistingType;.
 */

// type does not create a new type:
#[test]
fn t0() {
    type Meters = u32;
    type Kilograms = u32;

    let m: Meters = 3;
    let k: Kilograms = 3;

    assert_eq!(m, k);
}

// A type can be generic:
#[test]
fn t1() {
    struct Point<T> {
        x: T,
        y: T,
    }

    type PBox<T> = Box<Point<T>>;

    let point_box: PBox<i32> = Box::new(Point { x: 10, y: 20 });

    fn create_point<T>(x: T, y: T) -> PBox<T> {
        Box::new(Point { x, y })
    }
}

// In traits, type is used to declare an associated type:
#[test]
fn t2() {
    trait Iterator {
        // associated type declaration
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }

    struct Once<T>(Option<T>);

    impl<T> Iterator for Once<T> {
        // associated type definition
        type Item = T;
        fn next(&mut self) -> Option<Self::Item> {
            self.0.take()
        }
    }
}
