mod std_ops;

fn fn_once<F>(func: F)
where
    F: FnOnce(usize) -> bool,// 改动在这里
{
    println!("{}", func(3));
    // println!("{}", func(4));
}

fn main() {
    let x = vec![1, 2, 3];
    fn_once(|z|{z == x.len()});
    println!("{:?}", x);
}
