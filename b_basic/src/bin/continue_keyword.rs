// continue关键字

fn main() {
    for i in 1..10 {
        if i == 5 {
            println!("i={}", i);
            continue;  // 跳过本次循环
        }
        println!("i != 5");
    }
}