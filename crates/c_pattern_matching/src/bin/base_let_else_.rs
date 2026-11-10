// let...else

/*
If an else block is not present, the pattern must be irrefutable. If an else block is present, the pattern may be refutable. If the pattern does not match (this requires it to be refutable), the else block is executed. The else block must always diverge (evaluate to the never type).
*/

fn parse_point_if_let(s: &str) -> Option<(i32, i32)> {
    if let Some((x_str, y_str)) = s.split_once(',') {
        if let Ok(x) = x_str.trim().parse::<i32>() {
            if let Ok(y) = y_str.trim().parse::<i32>() {
                return Some((x, y));
            }
        }
    }
    None
}

// 与上等价
fn parse_point_let_else(s: &str) -> Option<(i32, i32)> {
    let Some((x_str, y_str)) = s.split_once(',') else {
        return None;
    };
    let Ok(x) = x_str.trim().parse::<i32>() else {
        return None;
    };
    let Ok(y) = y_str.trim().parse::<i32>() else {
        return None;
    };
    Some((x, y))
}

#[test]
fn t0() {
    assert_eq!(parse_point_if_let("3, 5"), Some((3, 5)));
    assert_eq!(parse_point_let_else("3, 5"), Some((3, 5)));

    assert_eq!(parse_point_let_else("3"), None); 
    assert_eq!(parse_point_let_else("3, a"), None);
}

#[test]
fn t1() {
    // 绑定变量的作用域

    let s = Some(5);

    if let Some(x) = s {
        println!("{x}");
    }
    // // error[E0425]: cannot find value `x` in this scope
    // println!("{x}"); // if let绑定的变量仅在其代码块内可用

    let Some(y) = s else {
        panic!("s为None");
    };
    println!("{y}"); // let ... else绑定的变量在当前作用域的后续代码中可用
}

#[test]
fn t2() {
    // else块必须发散: 执行后不会回到下一行,即else块的类型为!(never type)
    // 1. return: 离开当前函数
    // 2. break/continue: 结束循环/进入下一轮循环
    // 3. panic!,unreachable!,todo!等: 触发panic
    // 4. 返回类型为!的函数: 如std::process::exit
    // 5. break 'label: 离开带标签的代码块

    let s = Some(5);

    // 1. return
    fn get_or_zero(s: Option<i32>) -> i32 {
        let Some(n) = s else {
            return 0;
        };
        n
    }
    assert_eq!(get_or_zero(None), 0);

    // 2. break/continue
    let mut sum = 0;
    for v in [Some(1), None, Some(3)] {
        let Some(n) = v else {
            continue; 
        };
        sum += n;
    }
    assert_eq!(sum, 4);

    // 4. 返回类型为!的函数
    fn fail(msg: &str) -> ! {
        panic!("{msg}")
    }
    let Some(a) = s else {
        println!("s为None"); // 发散前可以先执行其他语句
        fail("s为None");
    };
    assert_eq!(a, 5);

    // 5. break 'label
    let doubled = 'blk: {
        let Some(n) = s else {
            break 'blk 0; // 只离开'blk代码块,doubled为0
        };
        n * 2
    };
    assert_eq!(doubled, 10);

    // error[E0308]: `else` clause of `let...else` does not diverge
    // let Some(z) = s else {
    //     println!("s为None"); // println!的类型为()
    // };
}

fn main() {}
