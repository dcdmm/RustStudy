#[allow(warnings)]
#[test]
fn expr_test0() {
    macro_rules! expressions {
        // expr: 表达式
        ($e:expr) => {};
    }

    expressions!(8);
    expressions!(3.14159);
    expressions!(false);
    expressions!("hello rust");
    expressions!([1, 2, 3, 4]);
    expressions!([1, 2, 3, 4][1]);
    expressions!((1, 2, 3, 4));
    expressions!((1, 2, 3, 4).0);
    // Range expressions
    expressions!(1..2);
    // Range expressions
    expressions!(std::ops::Range { start: 0, end: 10 });
    expressions!(Point { x: 10.0, y: 20.0 });
    expressions!(IpAddrkind::V4);
    expressions!(Vec::<i32>::push);
    expressions!(3 + 4);
    expressions!(funcall());
    expressions!(a.name);
    expressions!(b.len());
    // Closure expressions
    expressions!(|z| z == x1);
    expressions!(if x == 4 {
        println!("x is four");
    } else if x == 3 {
        println!("x is three");
    } else {
        println!("x is something else");
    });
    let sum = 0;
    expressions!(for n in 1..11 {
        sum += n;
    });
    // match expressions
    let x = 1;
    expressions!(match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("something else"),
    });
    // Block expressions
    expressions!({
        fn_call();
        5
    });
}

#[test]
fn expr_test1() {
    macro_rules! expressions {
        // $expr后面只可以跟`,`、`;`和`=>`
        ($e0:expr, $e1:expr) => {
            // 引用元变量
            $e0 + $e1
        };
        ($e0:expr; $e1:expr) => {
            $e0 + $e1
        };
        ($e0:expr => $e1:expr) => {
            $e0 + $e1
        };
    }

    let a = expressions!(1, 2);
    let b = expressions!(1; 2);
    let c = expressions!(1 => 2);
    println!("{}/{}/{}", a, b, c); // print->3/3/3
}

fn main() {}
