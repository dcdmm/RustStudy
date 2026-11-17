//! rayon::join: 分治算法的并行
//!
//! 对外的函数都直接使用u64而不是泛型: 泛型函数是在调用方的crate中实例化和编译的,
//! 开发模式下src-tauri没有开启优化,泛型版本的排序会按debug模式编译,耗时对比就失真了

use std::cmp::Ordering;

use rayon::prelude::*;

/// 单线程版本: 标准库的排序
pub fn sort_seq(v: &mut [u64]) {
    v.sort_unstable();
}

/// 用rayon::join实现的并行快速排序
///
/// rayon::join(a, b): a和b可能在两个线程上同时执行,两个都完成后才返回
/// 快速排序把数组分成左右两部分后,两部分互不影响,正好可以用join同时排序
/// rayon的par_iter、par_sort等底层都是基于join实现的
pub fn quick_sort_par(v: &mut [u64]) {
    // 粒度控制: 数据量小时,并行的调度开销大于收益,改用单线程排序
    if v.len() <= SEQUENTIAL_THRESHOLD {
        v.sort_unstable();
        return;
    }
    let (lt, gt) = partition3(v);
    // 分成三段: left < 基准值 | equal == 基准值 | right > 基准值
    // equal已经在最终位置上,只需要对left和right继续排序,两者互不影响
    let (left, rest) = v.split_at_mut(lt);
    let (_equal, right) = rest.split_at_mut(gt - lt);
    rayon::join(|| quick_sort_par(left), || quick_sort_par(right));
}

const SEQUENTIAL_THRESHOLD: usize = 8 * 1024;

// 三路分区: 以中间元素为基准值,返回(lt, gt),使得v[..lt] < 基准值,v[lt..gt] == 基准值,v[gt..] > 基准值
//
// 普通的两路分区遇到大量重复值时,相等的元素都被分到同一边,
// 递归深度退化成O(n),数据量大时会栈溢出;三路分区把相等的元素单独分出来,不再参与递归
fn partition3<T: Ord>(v: &mut [T]) -> (usize, usize) {
    v.swap(0, v.len() / 2);
    let (mut lt, mut i, mut gt) = (0, 1, v.len());
    // 循环中始终满足: v[..lt] < 基准值,v[lt..i] == 基准值,v[gt..] > 基准值
    // v[lt]始终是一个等于基准值的元素,用它作为比较对象
    while i < gt {
        match v[i].cmp(&v[lt]) {
            Ordering::Less => {
                v.swap(lt, i);
                lt += 1;
                i += 1;
            }
            Ordering::Greater => {
                gt -= 1;
                v.swap(i, gt);
            }
            Ordering::Equal => i += 1,
        }
    }
    (lt, gt)
}

/// rayon内置的并行排序,实际项目中直接用这个即可
pub fn sort_par(v: &mut [u64]) {
    v.par_sort_unstable();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01_quick_sort_par() {
        let mut v = crate::sample::numbers(100_000);
        let mut expected = v.clone();
        expected.sort_unstable();

        quick_sort_par(&mut v);
        assert_eq!(v, expected);
    }

    #[test]
    fn test02_edge_cases() {
        let mut empty: Vec<u64> = vec![];
        quick_sort_par(&mut empty);
        assert!(empty.is_empty());

        // 大量重复值、已经有序、逆序
        let mut duplicates: Vec<u64> = (0..50_000).map(|i| i % 3).collect();
        let mut sorted: Vec<u64> = (0..50_000).collect();
        let mut reversed: Vec<u64> = (0..50_000).rev().collect();
        for v in [&mut duplicates, &mut sorted, &mut reversed] {
            quick_sort_par(v);
            assert!(v.is_sorted());
        }
    }

    #[test]
    fn test03_partition3() {
        let mut v = [3, 1, 3, 5, 3, 0, 4];
        // 中间元素v[3] = 5是基准值
        let (lt, gt) = partition3(&mut v);
        assert!(v[..lt].iter().all(|&x| x < 5));
        assert!(v[lt..gt].iter().all(|&x| x == 5));
        assert!(v[gt..].iter().all(|&x| x > 5));
        assert_eq!((lt, gt), (6, 7));
    }

    #[test]
    fn test04_sort_par() {
        let mut v = crate::sample::numbers(100_000);
        sort_par(&mut v);
        assert!(v.is_sorted());
    }
}
