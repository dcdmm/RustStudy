// Specifying Placeholder Types in Trait Definitions with Associated Types

/*
Associated types connect a type placeholder with a trait such that the trait method definitions can use these placeholder types in their signatures. The implementor of a trait will specify the concrete type to be used instead of the placeholder type for the particular implementation. That way, we can define a trait that uses some types without needing to know exactly what those types are until the trait is implemented.
*/

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
    let _other_struct = <Struct as AssociatedType>::Assoc::new();

    // 与上等价
    let _other_struct1 = OtherStruct::new();
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