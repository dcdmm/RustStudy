// let...else

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
    // println!("{x}"); // if let绑定的变量只在其代码块内可用

    let Some(y) = s else {
        panic!("s为None");
    };
    println!("{y}"); // let...else绑定的变量在后续代码中可用
}

#[test]
fn t2() {
    // else块必须发散

    let items = vec!["1", "two", "3"];
    let mut sum = 0;
    for item in items {
        let Ok(n) = item.parse::<i32>() else {
            continue; // 解析失败则跳过当前元素
        };
        sum += n;
    }
    assert_eq!(sum, 4);

    // let s: Option<i32> = None;
    // // error[E0308]: `else` clause of `let...else` does not diverge
    // let Some(z) = s else {
    //     println!("s为None"); // else块结束后会继续向下执行,而此时z没有值
    // };
}

#[test]
fn t3() {
    // 匹配enum变体

    #[allow(warnings)]
    enum Shape {
        Circle(f64),
        Rectangle(f64, f64),
    }

    fn rectangle_area(shape: &Shape) -> f64 {
        let Shape::Rectangle(w, h) = shape else {
            return 0.0; // 不是Rectangle变体时返回0.0
        };
        w * h
    }

    assert_eq!(rectangle_area(&Shape::Rectangle(2.0, 3.0)), 6.0);
    assert_eq!(rectangle_area(&Shape::Circle(1.0)), 0.0);
}

fn main() {}
