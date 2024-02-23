// 切片(Primitive Type slice)

/*
常见方法:
len / is_empty
get
split
join
 */

#[test]
fn len_is_empyt_test() {
    let a: &[i32] = &[1, 2, 3];
    assert_eq!(a.len(), 3);

    let a = [1, 2, 3];
    assert!(!a.is_empty());
}

#[test]
fn get_test() {
    /*
    Returns a reference to an element or subslice depending on the type of index.
    
    * If given a position, returns a reference to the element at that position or None if out of bounds.
    * If given a range, returns the subslice corresponding to that range, or None if out of bounds.
    */
    
    let v = [10, 40, 30];
    assert_eq!(Some(&40), v.get(1));
    assert_eq!(Some(&[10, 40][..]), v.get(0..2));
    assert_eq!(None, v.get(3));
    assert_eq!(None, v.get(0..4));
}

#[test]
fn split_test() {
    let slice = [10, 40, 33, 20];
    // Returns an iterator over subslices separated by elements that match pred. The matched element is not contained in the subslices.
    let mut iter = slice.split(|num| num % 3 == 0);
    assert_eq!(iter.next().unwrap(), &[10, 40]);
    assert_eq!(iter.next().unwrap(), &[20]);
    assert!(iter.next().is_none());

    // If the first element is matched, an empty slice will be the first item returned by the iterator. Similarly, if the last element in the slice is matched, an empty slice will be the last item returned by the iterator:
    let slice = [10, 40, 33];
    let mut iter = slice.split(|num| num % 3 == 0);
    assert_eq!(iter.next().unwrap(), &[10, 40]);
    assert_eq!(iter.next().unwrap(), &[]);
    assert!(iter.next().is_none());

    // If two matched elements are directly adjacent, an empty slice will be present between them:
    let slice = [10, 6, 33, 20];
    let mut iter = slice.split(|num| num % 3 == 0);
    assert_eq!(iter.next().unwrap(), &[10]);
    assert_eq!(iter.next().unwrap(), &[]);
    assert_eq!(iter.next().unwrap(), &[20]);
    assert!(iter.next().is_none());
}

#[test]
fn join_test() {
    // Flattens a slice of T into a single value Self::Output, placing a given separator between each.
    assert_eq!(["hello", "world"].join(" "), "hello world");
    assert_eq!([[1, 2], [3, 4]].join(&0), [1, 2, 0, 3, 4]);
    assert_eq!([[1, 2], [3, 4]].join(&[0, 0][..]), [1, 2, 0, 0, 3, 4]);
}

fn main() {}
