#[test]
fn t0() {
    macro_rules! print_token {
        // tt: TokenTree (a single token or tokens in matching delimiters (), [], or {})
        ($token:tt) => {
            println!("{}", stringify!($token));
        };
    }
    print_token!(*); // print->*
    print_token!(Hello); // print->Hello
    print_token!({ HELLO }); // print->{ HELLO }

    // error: no rules expected the token `+`
    // print_token!(a + b + (c + d[0]) + e);

    /*
    a + b + (c + d[0]) + e
    would be parsed into the following token trees:
    <<a>> <<+>> <<b>> <<+>> <<( <<c>> <<+>> <<d>> <<[ <<0>> ]>>  )>> <<+>> <<e>>
     */
    print_token!((a + b + (c + d[0]) + e)); // print->(a + b + (c + d[0]) + e)

    // error: no rules expected the token `+`
    // print_token!((a + b + (c + d[0]) + e) + f);

    macro_rules! print_many_token {
        ($($token:tt)*) => {
            println!("{}", stringify!($($token)*));
        };
    }
    print_many_token!(*); // print->*
    print_many_token!(Hello); // print->Hello
    print_many_token!({ HELLO }); // print->{ HELLO }
    print_many_token!(a + b + (c + d[0]) + e); // print->a + b + (c + d[0]) + e
    print_many_token!((a + b + (c + d[0]) + e)); // print->(a + b + (c + d[0]) + e)
    print_many_token!((a + b + (c + d[0]) + e) + f); // print->(a + b + (c + d[0]) + e) + f
}

#[test]
fn t1() {
    macro_rules! create_iter {
        ($array:ident, $($iter_ops:tt)*) => {{
            let iter = $array.$($iter_ops)*;
            iter
        }};
    }

    let a1 = [0, 1, 2, 3];
    let a2 = [4, 5, 6];

    let result = create_iter!(a1, iter().zip(a2.iter())); // 链式调用
    for (i, j) in result {
        println!("{} {}", i, j)
    }
}

#[test]
fn t2() {
    macro_rules! dynamic_call {
        ($func:ident($($args:tt)*)) => {{
            $func($($args)*);
        }};
    }

    fn print_details(name: &str, age: u32, city: &str) {
        println!("Name: {}, Age: {}, City: {}", name, age, city);
    }

    fn add_numbers(a: i32, b: i32) {
        println!("Sum of {} and {} is {}", a, b, a + b);
    }

    // 任意数量的参数
    dynamic_call!(print_details("Alice", 30, "New York"));
    dynamic_call!(add_numbers(10, 20));
}

fn main() {}
