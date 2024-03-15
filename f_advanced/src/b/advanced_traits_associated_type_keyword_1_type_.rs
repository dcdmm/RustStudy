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
    println!("{:?}", o.next()); // Some(1)
    println!("{:?}", o.next()); // None
}

// An example of associated types with generics and where clauses:
#[test]
fn t2() {
    struct ArrayLender<'a, T>(&'a mut [T; 16]);

    trait Lend {
        // Generic associated type declaration
        type Lender<'a> where Self: 'a;
        fn lend(&mut self) -> Self::Lender<'_>;
    }

    impl<T> Lend for [T; 16] {
        // Generic associated type definition
        type Lender<'a> = ArrayLender<'a, T> where Self: 'a;

        fn lend(&mut self) -> Self::Lender<'_> {
            ArrayLender(self)
        }
    }

    fn borrow<T: Lend>(array: &mut T) -> <T as Lend>::Lender<'_> {
        array.lend()
    }

    let mut array = [0usize; 16];
    let _lender = borrow(&mut array);
}