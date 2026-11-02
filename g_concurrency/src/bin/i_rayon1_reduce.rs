fn main() {}

#[cfg(test)]
mod tests {
    use rayon::prelude::*;
    use std::collections::HashMap;
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test01_base() {
        let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6];

        let sum: i32 = numbers.par_iter().sum();
        let max = numbers.par_iter().max();
        let min = numbers.par_iter().min();
        let even_count = numbers.par_iter().filter(|n| **n % 2 == 0).count();

        println!("和:{sum},最大值:{max:?},最小值:{min:?},偶数个数:{even_count}");
        assert_eq!(sum, 31);
        assert_eq!(max, Some(&9));
        assert_eq!(min, Some(&1));
        assert_eq!(even_count, 3);
    }

    #[test]
    fn test02_reduce() {
        // 第一个闭包返回初始值,第二个闭包合并两个结果
        let product = (1..=5u64).into_par_iter().reduce(|| 1, |a, b| a * b);
        assert_eq!(product, 120);
    }

    #[test]
    fn test03_fold() {
        let words = vec!["a", "b", "a", "c", "b", "a"];

        let counts: HashMap<&str, usize> = words
            .par_iter()
            // rayon把数据分成多段,每段使用独立的HashMap累加,不需要加锁
            // 与标准库不同,初始值是闭包,fold的结果是多个HashMap
            .fold(HashMap::new, |mut map, word| {
                *map.entry(*word).or_insert(0) += 1;
                map
            })
            // 把各段的HashMap合并为一个
            .reduce(HashMap::new, |mut merged, map| {
                for (word, count) in map {
                    *merged.entry(word).or_insert(0) += count;
                }
                merged
            });

        println!("单词计数:{counts:?}");
        assert_eq!(counts["a"], 3);
        assert_eq!(counts["b"], 2);
        assert_eq!(counts["c"], 1);
    }

    #[test]
    fn test04_find() {
        let numbers: Vec<i32> = (1..=100).collect();

        // find_first返回位置最靠前的匹配项,结果与串行find相同
        let first = numbers.par_iter().find_first(|n| **n % 10 == 0);
        println!("find_first结果:{first:?}");
        assert_eq!(first, Some(&10));

        // any找到匹配项后提前结束
        assert!(numbers.par_iter().any(|n| *n == 50));
    }

    #[test]
    fn test05_shared_state() {
        let numbers: Vec<i32> = (1..=1000).collect();

        // 闭包只能获得外部变量的共享引用,修改时需要Atomic或Mutex
        // Atomic和Mutex有同步开销,能用sum,reduce,fold时优先使用这些方法
        let even_count = AtomicUsize::new(0);
        numbers.par_iter().for_each(|n| {
            if n % 2 == 0 {
                even_count.fetch_add(1, Ordering::Relaxed);
            }
        });
        assert_eq!(even_count.load(Ordering::Relaxed), 500);

        let multiples = Mutex::new(Vec::new());
        numbers.par_iter().for_each(|n| {
            // 计算放在锁外,否则各线程只能排队执行
            let is_multiple = n % 100 == 0;
            if is_multiple {
                multiples.lock().unwrap().push(*n);
            }
        });
        let mut multiples = multiples.into_inner().unwrap();
        // push的顺序不确定,比较前先排序
        multiples.sort();
        assert_eq!(multiples.len(), 10);
        assert_eq!(multiples[0], 100);
    }
}
