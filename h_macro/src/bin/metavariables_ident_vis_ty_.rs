#![allow(warnings)]

#[test]
fn ident_test0() {
    macro_rules! meta_ident {
        // ident: an identifier (this includes keywords)
        ($var:ident) => {
            println!("{:?}", $var);
        };
    }
    let x = 666;
    meta_ident!(x); // print->666
    let y = "hello python";
    meta_ident!(y); // print->"hello python"
}

#[test]
fn ident_test1() {
    macro_rules! meta_ident {
        ($var:ident) => {
            $var {
                name: "satori",
                age: 17,
            }
        };
    }

    #[derive(Debug)]
    struct Girl {
        name: &'static str,
        age: u8,
    }

    let mm = meta_ident!(Girl);
    // println!("{:?}", meta_ident!(Girl)); // print->Girl { name: "satori", age: 17 }
}

#[test]
fn complex_test0() {
    macro_rules! create_function {
    // vis: a possibly empty Visibility qualifier
    // ty: a Type
    ($vis:vis $func_keyword:ident $func_name:ident, $param1:ident: $type1:ty, $param2:ident: $type2:ty, $body:expr) => {
        $vis fn $func_name($param1: $type1, $param2: $type2) -> i32 {
            $body
        }
    };
}

    create_function!(pub fn add, x: i32, y: i32, x + y);
    create_function!(fn subtract, x: i32, y: i32, x - y); // subtract为私有函数
    println!("3 + 4 = {}", add(3, 4));
    println!("3 - 4 = {}", subtract(3, 4));
}

#[test]
fn complex_test1() {
    macro_rules! create_struct_and_methods {
        ($struct_name:ident, $field1_name:ident: $field1_type:ty, $field2_name:ident: $field2_type:ty) => {
            #[derive(Debug)]
            struct $struct_name<T, U> {
                $field1_name: T,
                $field2_name: U,
            }

            impl<T, U> $struct_name<T, U> {
                fn new($field1_name: T, $field2_name: U) -> Self {
                    $struct_name {
                        $field1_name,
                        $field2_name,
                    }
                }

                fn get_first_field(&self) -> &T {
                    &self.$field1_name
                }

                fn get_second_field(&self) -> &U {
                    &self.$field2_name
                }
            }
        };
    }

    create_struct_and_methods!(Pair, first: i32, second: f64);

    let pair = Pair::new(42, 3.14159);
    println!("{:?}", pair);
    println!("{}", pair.get_first_field());
    println!("{}", pair.get_second_field());
    let _pair = Pair {
        first: "hello",
        second: "world",
    };
}

fn main() {}
