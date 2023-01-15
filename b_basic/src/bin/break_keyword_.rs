// break关键字

fn main() {
    for i in 1..10 {
        if i == 5 {
            println!("i={}", i);
            break;  // 跳出当前整个循环
        }
        println!("i != 5");
    }
}