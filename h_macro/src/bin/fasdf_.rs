#[test]
fn t0() {
    macro_rules! vec_strs {
        (
            // *: 0次或多次
            $(
                $element:expr
            )
            ,
            *
        ) => {
            {
                let mut v = Vec::new();
                $(
                    v.push(format!("{}", $element));
                )*
                v
            } // 表明这是一个表达式
        };
    }
    let _s = vec_strs![1, "a", true, 3.14159f32];
    // println!("{:?}", _s);
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
        "GuYu" => "123456",
        "George" => "774321"
    };
    println!("{:?}", contacts);
}

fn main() {}
