// 从集合中创建迭代器

/*
There are three common methods which can create iterators from a collection:

* iter(), which iterates over &T.
* iter_mut(), which iterates over &mut T.
* into_iter(), which iterates over T.

Various things in the standard library may implement one or more of the three, where appropriate.
*/

#[allow(warnings)]
use std::collections::VecDeque;

#[test]
fn iter_test() {
    let mut arr = [1, 2, 3];
    let a_iter: std::slice::Iter<'_, i32> = arr.iter();
    for i in a_iter {
        // 迭代&i32
        println!("{}", i);
    }
    let _a_iter_mut: std::slice::IterMut<'_, i32> = arr.iter_mut();

    let mut v = vec![1, 2, 3];
    let _v_iter: std::slice::Iter<'_, i32> = v.iter();
    let _v_iter_mut: std::slice::IterMut<'_, i32> = v.iter_mut();

    let mut vd = VecDeque::new();
    vd.push_back(5);
    vd.push_back(3);
    vd.push_back(4);
    let _vd_iter: std::collections::vec_deque::Iter<'_, i32> = vd.iter();
    let vd_iter_mut: std::collections::vec_deque::IterMut<'_, i32> = vd.iter_mut();
    for j in vd_iter_mut {
        // 迭代&mut i32
        *j *= 2
    }
    println!("{:?}", vd);
}

#[test]
fn into_iter_test0() {
    /*
    大多数集合提供了IntoIterator的3种不同实现(即&T、&mut T、T)
    如:
        impl<'a, T, const N: usize> IntoIterator for &'a [T; N]
        impl<'a, T, const N: usize> IntoIterator for &'a mut [T; N]
        impl<T, I, const N: usize> IndexMut<I> for [T; N]
     */

    let arr = [1, 2, 3];
    let _aii: std::array::IntoIter<i32, 3> = arr.into_iter();

    let v1 = vec![1, 2, 3];
    let vii: std::vec::IntoIter<i32> = v1.into_iter();
    for i in vii { // 迭代类型为i32
        println!("{:?}", i);
    }
    let v1_r = &vec![1, 2, 3]; // 集合的不可变(共享)引用
    let vrii = v1_r.into_iter();
    for i in vrii { // 迭代类型为&i32;实际调用了v1_r.iter()
        println!("{:?}", i);
    }
    let v1_r_mut = &mut vec![1, 2, 3]; // 集合的可变引用
    let vrmutii = v1_r_mut.into_iter();
    for i in vrmutii { // 迭代类型为&mut i32;实际调用了v1_r.iter_mut()
        println!("{:?}", i);
    }

    let mut vd = VecDeque::new();
    vd.push_back(5);
    vd.push_back(3);
    vd.push_back(4);
    let _vd_ii: std::collections::vec_deque::IntoIter<i32> = vd.into_iter();
}

#[test]
fn into_iter_test1() {
    let mut collection = vec![1, 2, 3];
    for element in &collection {  // 迭代类型为&i32;
        println!("{}", element);
    }
    for element in &mut collection { // 迭代类型为&mut i32;
        println!("{}", element);
    }
    for element in collection { // 迭代类型为i32;
        println!("{}", element);
    }
}
