// 迭代器适配器

// Functions which take an Iterator and return another Iterator are often called ‘iterator adapters’, as they’re a form of the ‘adapter pattern’.

/*
常见方法:
map
filter
flatten
take
skip
enumerate
 */

#[test]
fn map_test() {
    let a = [1, 2, 3];
    // Tkes a closure and creates an iterator which calls that closure on each element.
    let iter = a.iter().map(|x| 2 * x);
    for i in iter {
        print!("{}\t", i)
    }
}

#[test]
fn filter_test() {
    let a = [0i32, 1, 2];
    /*
    Creates an iterator which uses a closure to determine if an element should be yielded.

    Given an element the closure must return true or false. The returned iterator will yield only the elements for which the closure returns true.
     */
    let iter = a.iter().filter(|x| x.is_positive());
    for i in iter {
        print!("{}\t", i)
    }
}

#[test]
fn flatten_test() {
    let data = vec![vec![1, 2, 3, 4], vec![5, 6]];
    /*
    Creates an iterator that flattens nested structure.

    This is useful when you have an iterator of iterators or an iterator of things that can be turned into iterators and you want to remove one level of indirection.
     */
    let flattened = data.into_iter().flatten();
    for i in flattened {
        println!("{}", i);
    }
}

#[test]
fn take_test() {
    let a = [1, 2, 3];
    /*
    Creates an iterator that yields the first n elements, or fewer if the underlying iterator ends sooner.

    take(n) yields elements until n elements are yielded or the end of the iterator is reached (whichever happens first). The returned iterator is a prefix of length n if the original iterator contains at least n elements, otherwise it contains all of the (fewer than n) elements of the original iterator.
     */
    let iter = a.iter().take(2);
    for i in iter {
        println!("{}", i);
    }

    let v = [-1, -2];
    let iter_v = v.into_iter().take(5);
    for i in iter_v {
        println!("{}", i);
    }
}

#[test]
fn skip_test() {
    let a = [1, 2, 3, 4, 5];
    /*
    Creates an iterator that skips the first n elements.

    skip(n) skips elements until n elements are skipped or the end of the iterator is reached (whichever happens first). After that, all the remaining elements are yielded. In particular, if the original iterator is too short, then the returned iterator is empty.
     */
    let iter = a.iter().skip(2);
    for i in iter {
        println!("{}", i);
    }
}

#[test]
fn enumerate_test() {
    let a = ['a', 'b', 'c'];
    /*
    Creates an iterator which gives the current iteration count as well as the next value.

    The iterator returned yields pairs (i, val), where i is the current index of iteration and val is the value returned by the iterator.
     */
    let iter = a.iter().enumerate();
    for (i, j) in iter {
        println!("{} {}", i, j);
    }
}

#[test]
fn chain_test() {
    let a1 = [1, 2, 3];
    let a2 = [4, 5, 6, 7];
    /*
    Takes two iterators and creates a new iterator over both in sequence.

    chain() will return a new iterator which will first iterate over values from the first iterator and then over values from the second iterator.
     */
    let iter = a1.iter().chain(a2.iter());
    for i in iter {
        println!("{}", i);
    }
}

#[test]
fn zip_test() {
    let a1 = [0, 1, 2, 3];
    let a2 = [4, 5, 6];
    /*
    ‘Zips up’ two iterators into a single iterator of pairs.

    zip() returns a new iterator that will iterate over two other iterators, returning a tuple where the first element comes from the first iterator, and the second element comes from the second iterator.

    In other words, it zips two iterators together, into a single one.

    If either iterator returns None, next from the zipped iterator will return None. If the zipped iterator has no more elements to return then each further attempt to advance it will first try to advance the first iterator at most one time and if it still yielded an item try to advance the second iterator at most one time.
     */
    let iter = a1.iter().zip(a2.iter());
    for (i, j) in iter {
        println!("{} {}", i, j)
    }
}
