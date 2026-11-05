// `$(...)*`: 匹配0次或多次,没有分隔符
// `$(...),*`: 匹配0次或多次,以逗号分隔
// `$(...);*`: 匹配0次或多次,以分号分隔
// `$(...)+`: 匹配1次或多次,没有分隔符
// `$(...),+`: 匹配1次或多次,以逗号分隔
// `$(...);+`: 匹配1次或多次,以分号分隔
// `$(...)?`: 匹配0次或1次,没有分隔符

#[test]
fn t0() {
    macro_rules! print_values {
        ($($value:expr);*) => {
            // 打印$value中每个表达式
            $(
                println!("{:?}", $value);
            )*
        }
    }
    print_values!(false; "hello rust!"; vec![1, 2, 3]);
}

#[test]
fn t1() {
    macro_rules! map {
        ($($k:expr => $v:expr), *) => {
            {
                let mut map = std::collections::HashMap::new();
                $(
                    map.insert($k, $v);
                )*
                map
            }
        };
    }

    let contacts = map! {
        "a" => "1",
        "b" => "2",
        "c" => "3"
    };
    println!("{:?}", contacts); // print->{"b": "2", "a": "1", "c": "3"}
}

#[test]
fn t2() {
    // `$(,)?`: 允许末尾多一个逗号(尾随逗号)
    macro_rules! my_vec {
        ($($x:expr),* $(,)?) => {
            vec![$($x),*]
        };
    }

    let v0 = my_vec![1, 2, 3];
    let v1 = my_vec![1, 2, 3,]; // 末尾的逗号由`$(,)?`匹配
    println!("{:?}", v0); // print->[1, 2, 3]
    println!("{:?}", v1); // print->[1, 2, 3]
}

#[test]
fn t3() {
    // `$(...)?`: 可选的语法片段,写或不写均可匹配
    macro_rules! var {
        ($name:ident $(: $t:ty)? = $val:expr) => {
            let $name $(: $t)? = $val;
        };
    }

    var!(x = 1); // 省略类型,展开为`let x = 1;`
    var!(y: u8 = 2); // 指定类型,展开为`let y: u8 = 2;`
    println!("{x} {y}"); // print->1 2
}

fn main() {}
