// 消耗迭代器方法

/*
常见方法:
count/sum/product/min/max
fold
nth
last
find
 */

#[test]
fn count_sum_product_min_max_() {
    let a = [1, 2, 3];
    let a_iter = a.iter();
    println!("{}", a_iter.count()); // `count()`:  ------- `a_iter` moved due to this method call

    // error[E0382]: use of moved value: `a_iter`
    // et a_sum: i32 = a_iter.sum(); // 迭代器a_iter已经被消耗

    let b = [1, 2, 3];
    let _bsum: i32 = b.iter().sum(); // 必须显式指定类型
    let _bpro: i32 = b.iter().product(); // 必须显式指定类型
    let _bmin = b.iter().min();
    let _bmax = b.iter().max();
}

#[test]
fn fold_test() {
    let a = [1, 2, -3, 5];
    /*
    Folds every element into an accumulator by applying an operation, returning the final result.

    fold() takes two arguments: an initial value, and a closure with two arguments: an ‘accumulator’, and an element. The closure returns the value that the accumulator should have for the next iteration.
     */
    /*
    sum结果解析:
    step0: -1 + 1 = 0 // -1为初始值
    step1: 0 + 2 = 2
    step2: 2 + -3 = -1
    step3: -1 + 5 = 4
     */
    let a_iter = a.iter();
    let sum = a_iter.fold(-1, |acc, x| acc + x);
    println!("{}", sum);

    let numbers = [1, 2, 3, 4, 5];
    let zero = "0".to_string();
    /*
    result结果解析:
    step0: (0 + 1)
    step1: ((0 + 1) + 2)
    step2: (((0 + 1) + 2) + 3)
    step3: ((((0 + 1) + 2) + 3) + 4)
    step4: (((((0 + 1) + 2) + 3) + 4) + 5)
     */
    let result = numbers
        .iter()
        .fold(zero, |acc, &x| format!("({acc} + {x})"));
    println!("{}", result);
}

#[test]
fn nth_test() {
    let a = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    let mut a_iter = a.iter();  // 更改迭代器内部状态(即当前位置),故迭代器变量必须是可变的
    /*
    Returns the nth element of the iterator.

    Like most indexing operations, the count starts from zero, so nth(0) returns the first value, nth(1) the second, and so on.

    Note that all preceding elements, as well as the returned element, will be consumed from the iterator. That means that the preceding elements will be discarded, and also that calling nth(0) multiple times on the same iterator will return different elements.

    nth() will return None if n is greater than or equal to the length of the iterator.
     */
    println!("{:?}", a_iter.nth(3)); // print->Some(3)
    println!("{:?}", a_iter.nth(1)); // 元素0,1,2,3已经被消耗;print->Some(5)
    println!("{:?}", a_iter.nth(1)); // 元素5,6又被消耗;print->Some(7)
    println!("{:?}", a_iter.nth(1)); // print->None
}

#[test]
fn last_test() {
    let a = [1, 2, 3];
    let a_iter = a.iter();
    // Consumes the iterator, returning the last element.
    println!("{:?}", a_iter.last());
    
    let b = [1, 2, 3, 4, 5];
    assert_eq!(b.iter().last(), Some(&5));
}

#[test]
fn find_test() {
    let a = [1, 2, 3];
    /*
    Searches for an element of an iterator that satisfies a predicate.
     */
    println!("{:?}", a.iter().find(|&&x| x == 2)); // print->Some(2)
    println!("{:?}", a.iter().find(|&&x| x == 5)); // print->None

    let b = [1, 2, 3, 4, 2, 5, 6];
    let mut b_iter = b.iter(); // 更改迭代器内部状态(即当前位置),故迭代器变量必须是可变的
    println!("{:?}", b_iter.find(|&&x| x == 2)); // 元素1,2已经被消耗;print->Some(2)
    println!("{:?}", b_iter.next()); // 元素3已经被消耗;print->Some(3)
    println!("{:?}", b_iter.find(|&&x| x == 2)); // 元素4,2已经被消耗;print->Some(2)
    println!("{:?}", b_iter.next()); // 元素5已经被消耗;print->Some(5)
    println!("{:?}", b_iter.next()); // 元素6已经被消耗;print->Some(6)
    println!("{:?}", b_iter.next()); // print->None
}