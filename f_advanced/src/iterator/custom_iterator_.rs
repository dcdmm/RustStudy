// 自定义迭代器

struct Counter {
    count: u32,
}

#[allow(warnings)]
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// 实现Iterator特质
impl Iterator for Counter {
    // associated type
    type Item = u32; // The type of the elements being iterated over.

    // 自定义next方法(必须实现,没有默认实现)
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn next_method_test() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);

    let counter1 = Counter::new();
    for i in counter1 {
        println!("{}", i);
    }
}

#[test]
fn other_method_test() {
    let sum: u32 = Counter::new()
        // 其他包含默认实现的方法示例
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    println!("{}", sum);
}
