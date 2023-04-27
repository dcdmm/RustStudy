// Trait std::iter::Iterato
/*
常见方法:
next
sum / max / min
map
filter
zip
skip
collect
 */

fn main() {}

// Required Methods
#[test]
fn next_fn() {
    let a = [1, 2, 3];

    let mut iter = a.iter();

    /*
    Advances the iterator and returns the next value.

    Returns None when iteration is finished.
    Individual iterator implementations may choose to resume iteration, and so calling next() again may or may not eventually start returning Some(Item) again at some point.
     */

    // A call to next() returns the next value...
    assert_eq!(Some(&1), iter.next());
    assert_eq!(Some(&2), iter.next());
    assert_eq!(Some(&3), iter.next());

    // ... and then None once it's over.
    assert_eq!(None, iter.next());

    // More calls may or may not return `None`. Here, they always will.
    assert_eq!(None, iter.next());
    assert_eq!(None, iter.next());
}

#[test]
fn sum_max_min_fn() {
    let a = [1, 2, 3];
    let sum: i32 = a.iter().sum();

    assert_eq!(sum, 6);

    // If several elements are equally maximum, the last element is returned. If the iterator is empty, None is returned.
    let b: Vec<u32> = Vec::new();

    assert_eq!(a.iter().max(), Some(&3));
    assert_eq!(b.iter().max(), None);

    // f several elements are equally minimum, the first element is returned. If the iterator is empty, None is returned.
    assert_eq!(a.iter().min(), Some(&1));
    assert_eq!(b.iter().min(), None);
}

#[test]
fn map_fn() {
    /*
    Takes a closure and creates an iterator which calls that closure on each element.

    map() transforms one iterator into another, by means of its argument: something that implements FnMut. It produces a new iterator which calls this closure on each element of the original iterator.
     */
    let a = [1, 2, 3];

    let mut iter = a.iter().map(|x| 2 * x);

    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(4));
    assert_eq!(iter.next(), Some(6));
    assert_eq!(iter.next(), None);
}

#[test]
fn filter_fn() {
    /*
    Creates an iterator which uses a closure to determine if an element should be yielded.

    Given an element the closure must return true or false. The returned iterator will yield only the elements for which the closure returns true.
     */

    let a = [0i32, 1, 2];

    let mut iter = a.iter().filter(|x| x.is_positive());

    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), None);
}

#[test]
fn zip_fn() {
    /*
    ‘Zips up’ two iterators into a single iterator of pairs.

    zip() returns a new iterator that will iterate over two other iterators, returning a tuple where the first element comes from the first iterator, and the second element comes from the second iterator.

    In other words, it zips two iterators together, into a single one.

    If either iterator returns None, next from the zipped iterator will return None. If the zipped iterator has no more elements to return then each further attempt to advance it will first try to advance the first iterator at most one time and if it still yielded an item try to advance the second iterator at most one time.
     */
    let a1 = [1, 2, 3];
    let a2 = [4, 5, 6];

    let mut iter = a1.iter().zip(a2.iter());

    assert_eq!(iter.next(), Some((&1, &4)));
    assert_eq!(iter.next(), Some((&2, &5)));
    assert_eq!(iter.next(), Some((&3, &6)));
    assert_eq!(iter.next(), None);

    // Since the argument to zip() uses IntoIterator, we can pass anything that can be converted into an Iterator, not just an Iterator itself. For example, slices (&[T]) implement IntoIterator, and so can be passed to zip() directly:
    let s1 = [1, 2, 3];
    let s2 = [4, 5, 6];

    let mut iter = s1.iter().zip(s2);

    assert_eq!(iter.next(), Some((&1, 4)));
    assert_eq!(iter.next(), Some((&2, 5)));
    assert_eq!(iter.next(), Some((&3, 6)));
    assert_eq!(iter.next(), None);
}

#[test]
fn skip_fn() {
    /*
    Creates an iterator that skips the first n elements.

    skip(n) skips elements until n elements are skipped or the end of the iterator is reached (whichever happens first). After that, all the remaining elements are yielded. In particular, if the original iterator is too short, then the returned iterator is empty.

    Rather than overriding this method directly, instead override the nth method.
     */
    let a = [1, 2, 3];

    let mut iter = a.iter().skip(2);

    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), None);
}

#[test]
fn collect_fn() {
    let a = [1, 2, 3];

    // Transforms an iterator into a collection.
    let doubled: Vec<i32> = a.iter()
        .map(|&x| x * 2)
        .collect();

    assert_eq!(vec![2, 4, 6], doubled);

    use std::collections::VecDeque;

    let doubled: VecDeque<i32> = a.iter().map(|&x| x * 2).collect();

    assert_eq!(2, doubled[0]);
    assert_eq!(4, doubled[1]);
    assert_eq!(6, doubled[2]);


    let chars = ['g', 'd', 'k', 'k', 'n'];

    let hello: String = chars.iter()
        .map(|&x| x as u8)
        .map(|x| (x + 1) as char)
        .collect();

    assert_eq!("hello", hello);

}