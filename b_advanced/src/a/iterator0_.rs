// 迭代器

// ======> 参考Z_std_trait/src/bin/iter_i_iterator_.rs

fn main() {
    let v = vec!["a".to_string(), "b".to_string()];
    println!("{:?}", v);
    // Creates a consuming iterator, that is, one that moves each value out of the vector (from start to end). The vector cannot be used after calling this.
    let mut v_iter = v.into_iter();
    // 报错:error[E0382]: borrow of moved value: `v`
    // println!("{:?}", v);

    let first_element: Option<String> = v_iter.next();

    assert_eq!(first_element, Some("a".to_string()));
    assert_eq!(v_iter.next(), Some("b".to_string()));
    assert_eq!(v_iter.next(), None);


    let x = vec![1, 2, 4];
    /*
    Returns an iterator over the slice.

    The iterator yields all items from start to end.
     */
    let mut iterator = x.iter();

    assert_eq!(iterator.next(), Some(&1));
    assert_eq!(iterator.next(), Some(&2));
    assert_eq!(iterator.next(), Some(&4));
    assert_eq!(iterator.next(), None);

    println!("{:?}", x);

    let x = &mut [1, 2, 4];
    /*
    Returns an iterator that allows modifying each value.

    The iterator yields all items from start to end.
     */
    for elem in x.iter_mut() {
        *elem += 2;
    }
    assert_eq!(x, &[3, 4, 6]);
}

