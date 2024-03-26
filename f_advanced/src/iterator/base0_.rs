// 从集合中创建迭代器

/*
There are three common methods which can create iterators from a collection:

* iter(), which iterates over &T.
* iter_mut(), which iterates over &mut T.
* into_iter(), which iterates over T.
*/

#[allow(warnings)]
use std::collections::VecDeque;

#[test]
fn iter_test() {
    let mut arr = [1, 2, 3];
    let a_iter: std::slice::Iter<'_, i32> = arr.iter(); 
    for i in a_iter { // 迭代&i32
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
    for j in vd_iter_mut { // 迭代&mut i32
        *j *= 2
    }
    println!("{:?}", vd);
}

#[test]
fn into_iter_test() {
    let arr = [1, 2, 3];
    let _aii: std::array::IntoIter<i32, 3> = arr.into_iter();

    let v1 = vec![1, 2, 3];
    let vii: std::vec::IntoIter<i32> = v1.into_iter();
    for i in vii { // 迭代i32
        println!("{:?}", i); 
    }

    let mut vd = VecDeque::new();
    vd.push_back(5);
    vd.push_back(3);
    vd.push_back(4);
    let _vd_ii: std::collections::vec_deque::IntoIter<i32> = vd.into_iter();
}
