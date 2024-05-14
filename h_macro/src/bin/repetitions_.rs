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

fn main() {}
