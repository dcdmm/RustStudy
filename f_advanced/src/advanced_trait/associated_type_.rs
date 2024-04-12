// Specifying Placeholder Types in Trait Definitions with Associated Types

#[test]
fn t0() {
    trait AssociatedType {
        // Associated type declaration
        type Assoc;
    }

    struct Struct;

    struct OtherStruct;

    impl AssociatedType for Struct {
        // Associated type definition
        type Assoc = OtherStruct;
    }

    impl OtherStruct {
        fn new() -> OtherStruct {
            OtherStruct
        }
    }
    // Usage of the associated type to refer to OtherStruct as <Struct as AssociatedType>::Assoc
    let _other_struct: OtherStruct = <Struct as AssociatedType>::Assoc::new();
}

#[test]
fn t1() {
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

    let mut o = Once(Some(1));
    println!("{:?}", o.next()); // print->Some(1)
    println!("{:?}", o.next()); // print->None
}