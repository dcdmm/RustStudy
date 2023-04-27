// 迭代器(自定义)

struct Counter {
    count: u32,
}

#[allow(dead_code)]
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// 实现Iterator特征
impl Iterator for Counter {
    type Item = u32;

    // 自定义next方法
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
fn test_my_obj() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

fn main() {}
