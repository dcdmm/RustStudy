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
    }

    #[test]
    fn test02_reduce() {
        // // reduce函数签名
        // fn reduce<OP, ID>(self, identity: ID, op: OP) -> Self::Item
        // where
        //     OP: Fn(Self::Item, Self::Item) -> Self::Item + Sync + Send,
        //     ID: Fn() -> Self::Item + Sync + Send,

        // 1. 切分:
        //           [1,2,3,4,5]
        //          /           \
        //      [1,2]          [3,4,5]
        //      /   \          /     \
        //    [1]   [2]      [3]    [4,5]
        //                          /   \
        //                        [4]   [5]
        // 2. 分别计算[1],[2],[3],[4],[5]: 先调用|| 1得到初始值1,再用|a, b| a * b乘以其中的元素
        //    [1]: 初始值1, 1*1=1
        //    [2]: 初始值1, 1*2=2
        //    [3]: 初始值1, 1*3=3
        //    [4]: 初始值1, 1*4=4
        //    [5]: 初始值1, 1*5=5
        // 3. 合并结果: 用|a, b| a * b把树中左右两边的结果相乘,从下往上逐层合并
        //    [4]和[5]: 4*5=20
        //    [1]和[2]: 1*2=2
        //    [3]和[4,5]: 3*20=60
        //    [1,2]和[3,4,5]: 2*60=120
        let product = (1..=5u64).into_par_iter().reduce(|| 1, |a, b| a * b);
        assert_eq!(product, 120);

        // 1. 切分:
        //          [-3,-7,-1,-5]
        //          /           \
        //      [-3,-7]       [-1,-5]
        //      /    \        /    \
        //   [-3]   [-7]   [-1]   [-5]
        // 2. 分别计算[-3],[-7],[-1],[-5]: 先调用|| i32::MIN得到初始值i32::MIN,再用|a, b| a.max(b)取它和元素中较大的一个
        //    [-3]: 初始值i32::MIN, i32::MIN.max(-3)=-3
        //    [-7]: 初始值i32::MIN, i32::MIN.max(-7)=-7
        //    [-1]: 初始值i32::MIN, i32::MIN.max(-1)=-1
        //    [-5]: 初始值i32::MIN, i32::MIN.max(-5)=-5
        // 3. 合并结果: 用|a, b| a.max(b)取树中左右两边结果中较大的一个,从下往上逐层合并
        //    [-3]和[-7]: (-3).max(-7)=-3
        //    [-1]和[-5]: (-1).max(-5)=-1
        //    [-3,-7]和[-1,-5]: (-3).max(-1)=-1
        let max = vec![-3, -7, -1, -5]
            .into_par_iter()
            .reduce(|| i32::MIN, |a, b| a.max(b));
        assert_eq!(max, -1);
    }

    #[test]
    fn test03_fold() {
        // // fold函数签名
        // fn fold<T, ID, F>(self, identity: ID, fold_op: F) -> Fold<Self, ID, F>
        // where
        //     F: Fn(T, Self::Item) -> T + Sync + Send,
        //     ID: Fn() -> T + Sync + Send,
        //     T: Send,

        // 1. 切分(最少3个元素):
        //          [1,2,3,4,5,6,7,8]
        //          /               \
        //     [1,2,3,4]         [5,6,7,8]
        // 2. 分别计算[1,2,3,4]和[5,6,7,8]: 先调用|| 0得到初始值0,再用|acc, x|把其中的元素逐个相加
        //    [1,2,3,4]: 初始值0 → 0+1=1 → 1+2=3 → 3+3=6 → 6+4=10
        //    [5,6,7,8]: 初始值0 → 0+5=5 → 5+6=11 → 11+7=18 → 18+8=26
        // 3. collect: 把2个结果按原顺序放进Vec,得到[10, 26]
        let parts: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8]
            .par_iter()
            .with_min_len(3)
            .fold(|| 0, |acc, x| acc + x)
            .collect();
        println!("{:?}", parts);

        let words = vec!["a", "b", "a", "c", "b", "a"];
        // 1. 切分(最少3个元组):
        //          [a,b,a,c,b,a]
        //          /           \
        //      [a,b,a]       [c,b,a]
        // 2. 分别计算[a,b,a]和[c,b,a],每个都先执行fold,再把fold的结果交给reduce
        //    fold: 先调用fold的HashMap::new得到初始值{},再用fold的闭包|mut map, word|把其中的单词逐个计入
        //        [a,b,a]: 初始值{} → 计入a得到{a: 1} → 计入b得到{a: 1, b: 1} → 计入a得到{a: 2, b: 1}
        //        [c,b,a]: 初始值{} → 计入c得到{c: 1} → 计入b得到{b: 1, c: 1} → 计入a得到{a: 1, b: 1, c: 1}
        //    reduce: 先调用reduce的HashMap::new得到初始值{},再用reduce的闭包|mut merged, map|把fold的结果合并进来,结果不变
        //        [a,b,a]: 初始值{}和{a: 2, b: 1}合并为{a: 2, b: 1}
        //        [c,b,a]: 初始值{}和{a: 1, b: 1, c: 1}合并为{a: 1, b: 1, c: 1}
        // 3. 合并结果,只有reduce参与: 用reduce的闭包|mut merged, map|把左右两边的HashMap合并,相同单词的计数相加
        //    [a,b,a]和[c,b,a]: {a: 2, b: 1}和{a: 1, b: 1, c: 1}合并为{a: 3, b: 2, c: 1}
        let counts_fold = words
            .par_iter()
            .with_min_len(3)
            .fold(HashMap::new, |mut map, word| {
                *map.entry(*word).or_insert(0) += 1;
                map
            });
        let counts: HashMap<&str, usize> = counts_fold.reduce(HashMap::new, |mut merged, map| {
            for (word, count) in map {
                *merged.entry(word).or_insert(0) += count;
            }
            merged
        });
        println!("单词计数:{counts:?}");
    }

    #[test]
    fn test04_find() {
        let numbers: Vec<i32> = (1..=100).collect();

        let first = numbers.par_iter().find_first(|n| **n % 10 == 0);
        println!("find_first结果:{first:?}");
        
        let have = numbers.par_iter().any(|n| *n == 50);
        println!("any结果:{}", have)
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
