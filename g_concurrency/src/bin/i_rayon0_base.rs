fn main() {}

#[cfg(test)]
mod tests {
    use rayon::prelude::*;
    use std::sync::Mutex;
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
        // 并行
        let parallel: Vec<i32> = numbers.par_iter().map(|n| n * 2).collect();
        println!("串行:{serial:?}, 并行:{parallel:?}");

        // into_par_iter获取所有权
        let words = vec![String::from("hello"), String::from("rayon")];
        let upper: Vec<String> = words
            .into_par_iter()
            .map(|word| word.to_uppercase())
            .collect();
        // // error[E0382]: borrow of moved value: `words`
        // println!("{:?}", words);
        assert_eq!(upper, ["HELLO", "RAYON"]);

        // par_iter_mut原地修改元素
        let mut values = vec![1, 2, 3];
        values.par_iter_mut().for_each(|value| *value *= 10);
        assert_eq!(values, [10, 20, 30]);
    }

    #[test]
    fn test02_speedup() {
        println!("Rayon线程数: {}", rayon::current_num_threads());

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
    }

    #[test]
    fn test03_order() {
        let numbers: Vec<i32> = (0..8).collect();

        // collect按原顺序写入结果,与执行顺序无关
        let doubled: Vec<i32> = numbers.par_iter().map(|n| n * 2).collect();
        assert_eq!(doubled, [0, 2, 4, 6, 8, 10, 12, 14]);

        // for_each的执行顺序不确定,每次运行的输出可能不同
        numbers.par_iter().for_each(|n| {
            // current_thread_index返回当前线程在rayon线程池中的编号
            println!("线程{:?}处理{n}", rayon::current_thread_index());
        });
    }

    #[test]
    fn test04_closure_limits() {
        let numbers1 = vec![1, 2, 3, 4];
        //
        // let mut total = 0;
        // let op1 = |n| total += n;
        // // error[E0525]: expected a closure that implements the `Fn` trait, but this closure only implements `FnMut`
        // numbers1.par_iter().for_each(op1);
        //
        // 修复:
        let total: i32 = numbers1.par_iter().sum();
        assert_eq!(total, 10);

        let numbers2 = vec![1, 2, 3, 4];
        // let shared = std::cell::RefCell::new(0);
        // let op1 = |n| *shared.borrow_mut() += n;
        // // error[E0277]: `RefCell<i32>` cannot be shared between threads safely
        // numbers2.par_iter().for_each(op1);
        //
        // 修复:
        let shared = Mutex::new(0);
        numbers2
            .par_iter()
            .for_each(|n| *shared.lock().unwrap() += n);
        assert_eq!(shared.into_inner().unwrap(), 10);

        let number3 = vec![1, 2, 3, 4];
        let offset = 10;
        // op2没有使用move,保存的是offset的引用
        let op2 = |n| n + offset;
        // thread::spawn要求'static: spawn立即返回,是否等待线程结束由调用者决定,闭包可能在当前函数结束后执行
        // map不要求'static: 执行并行迭代的操作会等待任务完成后返回,确保借用的数据在任务执行期间有效
        let shifted: Vec<i32> = number3.par_iter().map(op2).collect();
        assert_eq!(shifted, [11, 12, 13, 14]);
    }

    #[test]
    fn test05_chunks() {
        let data: Vec<u64> = (1..=10).collect();

        // par_chunks(4)每4个元素分为一块,各块可并行处理,最后一块不足4个时也会保留
        let chunk_sums: Vec<u64> = data.par_chunks(4).map(|chunk| chunk.iter().sum()).collect();
        println!("每块的和:{chunk_sums:?}");
        // [1, 2, 3, 4] → 求和 → 10
        // [5, 6, 7, 8] → 求和 → 26
        // [9, 10]      → 求和 → 19
        assert_eq!(chunk_sums, [10, 26, 19]);

        // par_chunks_mut(4)每4个元素分为一块,可并行修改各块中的原始数据
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

        // par_sort按升序排列元素,直接修改原Vec
        values.par_sort();
        assert_eq!(values, [1, 2, 3, 5, 8, 9]);

        // par_sort_by自定义比较规则,b.cmp(a)按降序排列元素
        values.par_sort_by(|a, b| b.cmp(a));
        assert_eq!(values, [9, 8, 5, 3, 2, 1]);
    }
}
