fn main() {}

#[cfg(test)]
mod tests {
    use rayon::prelude::*;
    use std::time::Instant;

    fn is_prime(n: u64) -> bool {
        if n < 2 {
            return false;
        }
        let mut i = 2;
        while i * i <= n {
            if n % i == 0 {
                return false;
            }
            i += 1;
        }
        true
    }

    #[test]
    fn test01_base() {
        let numbers = vec![1, 2, 3, 4];

        // 串行
        let serial: Vec<i32> = numbers.iter().map(|n| n * 2).collect();
        // 并行: iter换成par_iter,其余写法不变
        let parallel: Vec<i32> = numbers.par_iter().map(|n| n * 2).collect();
        println!("串行:{serial:?},并行:{parallel:?}");
        assert_eq!(parallel, serial);

        // into_par_iter获取所有权,对应into_iter
        let words = vec![String::from("hello"), String::from("rayon")];
        let upper: Vec<String> = words
            .into_par_iter()
            .map(|word| word.to_uppercase())
            .collect();
        assert_eq!(upper, ["HELLO", "RAYON"]);

        // Range通过into_par_iter转换为并行迭代器
        let sum: u64 = (1..=100u64).into_par_iter().sum();
        assert_eq!(sum, 5050);

        // par_iter_mut原地修改元素,对应iter_mut
        let mut values = vec![1, 2, 3];
        values.par_iter_mut().for_each(|value| *value *= 10);
        assert_eq!(values, [10, 20, 30]);
    }

    #[test]
    fn test02_speedup() {
        const LIMIT: u64 = 2_000_000;

        let serial_start = Instant::now();
        let serial = (0..LIMIT).filter(|n| is_prime(*n)).count();
        println!(
            "串行运行时间:{:.3}毫秒",
            serial_start.elapsed().as_secs_f64() * 1000.0
        );

        let parallel_start = Instant::now();
        let parallel = (0..LIMIT).into_par_iter().filter(|n| is_prime(*n)).count();
        println!(
            "并行运行时间:{:.3}毫秒",
            parallel_start.elapsed().as_secs_f64() * 1000.0
        );

        assert_eq!(parallel, serial);
        assert_eq!(parallel, 148_933);
    }

    #[test]
    fn test03_order() {
        let numbers: Vec<i32> = (0..8).collect();

        // for_each的执行顺序不确定,每次运行的输出可能不同
        numbers.par_iter().for_each(|n| {
            println!("线程{:?}处理{n}", rayon::current_thread_index());
        });

        // 结果按原顺序写入Vec,与执行顺序无关
        let doubled: Vec<i32> = numbers.par_iter().map(|n| n * 2).collect();
        assert_eq!(doubled, [0, 2, 4, 6, 8, 10, 12, 14]);
    }

    #[test]
    fn test04_closure_limits() {
        let numbers = vec![1, 2, 3, 4];

        // let mut total = 0;
        // // error[E0596]: cannot borrow `total` as mutable, as it is a captured variable in a `Fn` closure
        // numbers.par_iter().for_each(|n| total += n);

        // 由rayon汇总结果,闭包不修改外部变量
        let total: i32 = numbers.par_iter().sum();
        assert_eq!(total, 10);

        // let shared = std::cell::RefCell::new(0);
        // // error[E0277]: `RefCell<i32>` cannot be shared between threads safely
        // numbers.par_iter().for_each(|n| *shared.borrow_mut() += n);

        // 与thread::spawn不同,闭包可以直接借用外部数据,不需要Arc和move
        let offset = 10;
        let shifted: Vec<i32> = numbers.par_iter().map(|n| n + offset).collect();
        assert_eq!(shifted, [11, 12, 13, 14]);
    }

    #[test]
    fn test05_chunks() {
        let data: Vec<u64> = (1..=10).collect();

        // par_chunks把切片分成固定大小的块,最后一块可能较小
        let chunk_sums: Vec<u64> = data.par_chunks(4).map(|chunk| chunk.iter().sum()).collect();
        println!("每块的和:{chunk_sums:?}");
        assert_eq!(chunk_sums, [10, 26, 19]);

        // par_chunks_mut分块修改,各块互不重叠,不需要加锁
        // 此处模拟按行处理图像,每行4个像素
        let mut pixels = vec![0u8; 12];
        pixels
            .par_chunks_mut(4)
            .enumerate()
            .for_each(|(row, line)| line.fill(row as u8));
        assert_eq!(pixels, [0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2]);
    }

    #[test]
    fn test06_sort() {
        let mut values = vec![5, 3, 8, 1, 9, 2];

        // par_sort是稳定排序,相等元素保持原有的相对顺序
        values.par_sort();
        assert_eq!(values, [1, 2, 3, 5, 8, 9]);

        // par_sort_unstable不保证相等元素的相对顺序,通常更快
        // by后缀的版本可自定义比较规则,此处为降序
        values.par_sort_unstable_by(|a, b| b.cmp(a));
        assert_eq!(values, [9, 8, 5, 3, 2, 1]);
    }
}
