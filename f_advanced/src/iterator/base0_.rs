// 从集合中创建迭代器

/*
There are three common methods which can create iterators from a collection:
* iter(), which iterates over &T.
* iter_mut(), which iterates over &mut T.
* into_iter(), which iterates over T.

大多数集合对IntoIterator提供了3种(即&T、&mut T、T)不同实现
如:
    impl<'a, T> IntoIterator for &'a [T] // 切片
    impl<'a, T> IntoIterator for &'a mut [T]

    impl<'a, T, const N: usize> IntoIterator for &'a [T; N] // 数组
    impl<'a, T, const N: usize> IntoIterator for &'a mut [T; N]
    impl<T, I, const N: usize> IndexMut<I> for [T; N]

    impl<'a, T, A> IntoIterator for &'a Vec<T, A>
    impl<'a, T, A> IntoIterator for &'a mut Vec<T, A>
    impl<T, A> IntoIterator for Vec<T, A>

    impl<'a, T, A> IntoIterator for &'a VecDeque<T, A>
    impl<'a, T, A> IntoIterator for &'a mut VecDeque<T, A>
    impl<T, A> IntoIterator for VecDeque<T, A>
*/

#[allow(warnings)]
use std::collections::HashMap;
#[allow(warnings)]
use std::collections::HashSet;
#[allow(warnings)]
use std::collections::VecDeque;

// Slices are either mutable(&mut [T]) or shared(&[T]),故没有IntoIterator的[T]实现
#[test]
fn slice_test() {
    let x = &[1, 2, 3, 4];
    // Returns an iterator over the slice.
    let x_iter: std::slice::Iter<'_, i32> = x.iter(); // 变量为&[T]、&mut [T]、[T; N]、mut [T; N]、&[T; N]、&mut [T; N]、Vec[T, A]、mut Vec[T, A]、&Vec[T, A]、&mut Vec[T, A]时调用该方法
    for i in x_iter {
        // 迭代类型为&i32
        println!("{}", i)
    }

    let y = &mut vec![1, 2, 3, 4];
    // Returns an iterator that allows modifying each value.
    let y_iter_mut: std::slice::IterMut<'_, i32> = y.iter_mut(); // 变量为&mut [T]、mut [T; N]、&mut [T; N]、mut Vec[T, A]、&mut Vec[T, A]调用该方法
    for elem in y_iter_mut {
        // 迭代类型为&mut i32
        *elem += 2
    }
    println!("{:?}", y);

    let n = &[1, 2, 3, 4][..];
    let _nii = n.into_iter(); // 迭代类型为&i32;实际调用了n.iter()

    let m = &mut [1, 2, 3, 4][..];
    let _mii = m.into_iter(); // 迭代类型为&mut i32;实际调用了m.iter_mut()
}

#[test]
fn array_test() {
    let a0 = [1, 2, 3, 4];
    /*
    impl<T, const N: usize> IntoIterator for [T; N]
        fn into_iter(self) -> <[T; N] as IntoIterator>::IntoIter

    Creates a consuming iterator, that is, one that moves each value out of the array (from start to end). The array cannot be used after calling this unless T implements Copy, so the whole array is copied.
     */
    let a0ii = a0.into_iter();
    for i in a0ii {
        // 迭代类型为i32
        println!("{}", i)
    }
    println!("{:?}", a0);

    let z0 = &mut [1, 2, 3, 4];
    let _z0ii: std::slice::IterMut<'_, i32> = z0.into_iter(); // 迭代类型为&mut i32;实际调用了z0.iter_mut()

    let z1 = &[1, 2, 3, 4];
    let _z0ii: std::slice::Iter<'_, i32> = z1.into_iter(); // 迭代类型为&i32;实际调用了z1.iter()
}

#[test]
fn vec_test() {
    let v = vec![1, 2, 3];
    // Creates a consuming iterator, that is, one that moves each value out of the vector (from start to end). The vector cannot be used after calling this.
    let vii: std::vec::IntoIter<i32> = v.into_iter();
    for i in vii {
        // 迭代类型为i32
        println!("{:?}", i);
    }
    let v_r = &vec![1, 2, 3]; // 集合的不可变(共享)引用
    let v_rii = v_r.into_iter();
    for i in v_rii {
        // 迭代类型为&i32;实际调用了v_r.iter()
        println!("{:?}", i);
    }
    let v_r_mut = &mut vec![1, 2, 3]; // 集合的可变引用
    let v_rmutii = v_r_mut.into_iter();
    for i in v_rmutii {
        // 迭代类型为&mut i32;实际调用了v_r.iter_mut()
        println!("{:?}", i);
    }
}

// 不能直接迭代&str和String(UTF-8是一种变长编码,字符可由一个到四个字节组成)
#[test]
fn str_string_test() {
    let s = "你好，世界！";
    for c in s.chars() {
        println!("{}", c);
    }
    for b in s.bytes() {
        println!("{}", b);
    }

    let s1 = String::from("你好，中国！");
    for c in s1.chars() {
        println!("{}", c)
    }
    for b in s1.bytes() {
        println!("{}", b)
    }
}

#[test]
fn vec_deque_test() {
    let mut vd = VecDeque::from(vec![1, 2, 3]);
    // Returns a front-to-back iterator.
    let _vd_iter: std::collections::vec_deque::Iter<'_, i32> = vd.iter();
    // Returns a front-to-back iterator that returns mutable references.
    let _vd_iter_mut: std::collections::vec_deque::IterMut<'_, i32> = vd.iter_mut();

    // Consumes the deque into a front-to-back iterator yielding elements by value.
    let _vd_ii: std::collections::vec_deque::IntoIter<i32> = vd.into_iter();

    let vd_r = &VecDeque::from(vec![1, 2, 3]);
    let _vd_rii = &vd_r.into_iter(); // 实际调用vd_r.iter()

    let vd_r_mut = &mut VecDeque::from(vec![1, 2, 3]);
    let _vd_rmutii = &vd_r_mut.into_iter(); // 实际调用vd_r.iter()
}

#[test]
fn hash_map_test() {
    let mut map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    // An iterator visiting all key-value pairs in arbitrary order. The iterator element type is (&'a K, &'a V).
    let _map_iter = map.iter();

    // An iterator visiting all key-value pairs in arbitrary order, with mutable references to the values. The iterator element type is (&'a K, &'a mut V).
    for (_, val) in map.iter_mut() {
        *val *= 2;
    }
    for (key, val) in &map {
        println!("key: {key} val: {val}");
    }

    // Creates a consuming iterator, that is, one that moves each key-value pair out of the map in arbitrary order. The map cannot be used after calling this.
    let _map_ii = map.into_iter();

    let map_r = &HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    let _map_r_ii = map_r.into_iter(); // 实际调用map_r.iter()

    let map_r_mut = &mut HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    let _map_rmut_ii = map_r_mut.into_iter(); // 实际调用map_r.iter_mut()
}

// 不支持通过可变引用迭代HashSet,故没有iter_mut方法和没有IntoIterator的&mut HashSet<T, S>实现
#[test]
fn hash_set_test() {
    let set = HashSet::from([1, 2, 3]);
    // An iterator visiting all elements in arbitrary order. The iterator element type is &'a T.
    let _set_iter = set.iter();

    // Creates a consuming iterator, that is, one that moves each value out of the set in arbitrary order. The set cannot be used after calling this.
    let _set_ii = set.into_iter();

    let set_r = &HashSet::from([1, 2, 3]);
    let _set_rii = set_r.into_iter(); // 实际调用set_r.iter()
}
