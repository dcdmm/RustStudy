// 迭代器基本方法
/*
1. min/max/sum
2. all/any
3. collect
4. map
5. filter
6. enumerate
 */

#[cfg(test)]
mod tests {
    #[test]
    fn min_max_sum_test() {
        // Returns the minimum element of an iterator.
        let a = [1, 2, 3];
        let b: Vec<u32> = Vec::new();

        assert_eq!(a.iter().min(), Some(&1));
        assert_eq!(b.iter().min(), None);

        // Returns the maximum element of an iterator.
        assert_eq!(a.iter().max(), Some(&3));
        assert_eq!(b.iter().max(), None);

        // Sums the elements of an iterator.
        let a_iter = a.iter();
        println!("{:?}", a_iter);
        let sum: i32 = a_iter.sum();
        assert_eq!(sum, 6);
    }

    #[test]
    fn all_any_test() {
        // Tests if every element of the iterator matches a predicate.
        // all() takes a closure that returns true or false. It applies this closure to each element of the iterator, and if they all return true, then so does all(). If any of them return false, it returns false.
        // all() is short-circuiting; in other words, it will stop processing as soon as it finds a false, given that no matter what else happens, the result will also be false.
        // An empty iterator returns true.

        let a = [1, 2, 3];

        assert!(a.iter().any(|&x| x > 0));
        assert!(!a.iter().any(|&x| x > 5));

        let b = [1, 2, 3];
        let mut iter = b.iter();
        assert!(!iter.all(|&x| x != 2));
        // we can still use `iter`, as there are more elements.
        assert_eq!(iter.next(), Some(&3));
    }

    #[test]
    fn collect_test() {
        // Transforms an iterator into a collection.

        let a = [1, 2, 3];
        // Basic usage:
        let doubled: Vec<i32> = a.iter().map(|&x| x * 2).collect();
        assert_eq!(vec![2, 4, 6], doubled);

        let chars = ['g', 'd', 'k', 'k', 'n'];
        // Using collect() to make a String:
        let hello: String = chars
            .iter()
            .map(|&x| x as u8)
            .map(|x| (x + 1) as char)
            .collect();

        assert_eq!("hello", hello);
    }

    #[test]
    fn zip_test() {
        // ‘Zips up’ two iterators into a single iterator of pairs.

        // zip() returns a new iterator that will iterate over two other iterators, returning a tuple where the first element comes from the first iterator, and the second element comes from the second iterator.

        // In other words, it zips two iterators together, into a single one.

        // If either iterator returns None, next from the zipped iterator will return None. If the zipped iterator has no more elements to return then each further attempt to advance it will first try to advance the first iterator at most one time and if it still yielded an item try to advance the second iterator at most one time.
        let a1 = [1, 2, 3];
        let a2 = [4, 5, 6];

        let mut iter = a1.iter().zip(a2.iter());

        assert_eq!(iter.next(), Some((&1, &4)));
        assert_eq!(iter.next(), Some((&2, &5)));
        assert_eq!(iter.next(), Some((&3, &6)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn map() {
        // 函数签名:
        // fn map<B, F>(self, f: F) -> Map<Self, F> ⓘ
        // where
        //     Self: Sized,
        //     F: FnMut(Self::Item) -> B, // 闭包
        // Takes a closure and creates an iterator which calls that closure on each element.

        // map() transforms one iterator into another, by means of its argument: something that implements FnMut. It produces a new iterator which calls this closure on each element of the original iterator.

        // If you are good at thinking in types, you can think of map() like this: If you have an iterator that gives you elements of some type A, and you want an iterator of some other type B, you can use map(), passing a closure that takes an A and returns a B.

        // map() is conceptually similar to a for loop. However, as map() is lazy, it is best used when you’re already working with other iterators. If you’re doing some sort of looping for a side effect, it’s considered more idiomatic to use for than map().
        let a = [1, 2, 3];

        let mut iter = a.iter().map(|x| 2 * x);

        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), Some(6));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn filter_test() {
        // 函数签名:
        // fn filter<P>(self, predicate: P) -> Filter<Self, P> ⓘ
        // where
        //     Self: Sized,
        //     P: FnMut(&Self::Item) -> bool, // 闭包

        // Creates an iterator which uses a closure to determine if an element should be yielded.

        // Given an element the closure must return true or false. The returned iterator will yield only the elements for which the closure returns true.
        let a = [0i32, 1, 2];

        let mut iter = a.iter().filter(|x| x.is_positive());

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), None);

        let b = [0, 1, 2];

        // Because the closure passed to filter() takes a reference, and many iterators iterate over references, this leads to a possibly confusing situation, where the type of the closure is a double reference:
        let mut iter1 = b.iter().filter(|x| **x > 1); // need two *s!

        assert_eq!(iter1.next(), Some(&2));
        assert_eq!(iter1.next(), None);
    }

    #[test]
    fn enumerate_test() {
        // Creates an iterator which gives the current iteration count as well as the next value.

        // The iterator returned yields pairs (i, val), where i is the current index of iteration and val is the value returned by the iterator.
        let a = ['a', 'b', 'c'];

        let mut iter = a.iter().enumerate();

        assert_eq!(iter.next(), Some((0, &'a')));
        assert_eq!(iter.next(), Some((1, &'b')));
        assert_eq!(iter.next(), Some((2, &'c')));
        assert_eq!(iter.next(), None);
    }
}

fn main() {}
