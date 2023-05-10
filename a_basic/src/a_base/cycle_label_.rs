// 循环标签

#[test]
fn t0() {
    let mut count = 0;
    /*
    If you have loops within loops, break and continue apply to the innermost loop at that point.
    You can optionally specify a loop label on a loop that you can then use with break or continue to specify that those keywords apply to the labeled loop instead of the innermost loop. Loop labels must begin with a single quote.
    */
    'counting_up: loop { // 类似Java带标签的循环
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            };
            if count == 2 {
                break 'counting_up;
            };
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    'out1: for i in 1..5 {
        println!("i = {i}");
        for j in 2..6 {
            println!("j = {j}");
            if j >= 3 && i > 2 {
                break 'out1;
            }
        }
    }
}
