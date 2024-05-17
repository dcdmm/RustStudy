// Implementing an unsafe trait

#[test]
fn t0() {
    unsafe trait TrustedLen {
        fn len(&self) -> usize;
    }

    struct SafeVec<T> {
        elements: Vec<T>,
    }

    unsafe impl<T> TrustedLen for SafeVec<T> {
        fn len(&self) -> usize {
            self.elements.len()
        }
    }

    let my_vec = SafeVec {
        elements: vec![1, 2, 3, 4, 5],
    };

    println!("{}", my_vec.len());
}
