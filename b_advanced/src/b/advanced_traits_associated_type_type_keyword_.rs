// Specifying Placeholder Types in Trait Definitions with Associated Types

fn main() {
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