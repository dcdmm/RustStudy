// Trait std::default::Default

// A trait for giving a type a useful default value.

// # Required Methods
/*
fn default() -> Self
    Returns the “default value” for a type.

    Default values are often some kind of initial value, identity value, or anything else that may make sense as a default.
*/

use std::default;

#[test]
fn t0() {
    // This trait can be used with #[derive] if all of the type’s fields implement Default. When derived, it will use the default value for each field’s type.
    #[derive(Default, Debug)]
    struct SomeOptions {
        foo: i32,
        bar: f32,
    }

    let default_v = SomeOptions::default();
    let default_v1: SomeOptions = Default::default(); // 遇上等价
    println!("{:?}", default_v);
}

#[test]
fn t1() {
    // When using #[derive(Default)] on an enum, you need to choose which unit variant will be default. You do this by placing the #[default] attribute on the variant.
    #[derive(Default, Debug)]
    enum Kind {
        #[default]
        A,
        B,
        C,
    }

    let default_v = Kind::default();
    println!("{:?}", default_v);
}

#[test]
fn t2() {
    #[derive(Debug)]
    struct Settings {
        brightness: u8,
        contrast: u8,
        hue: u8,
        volume: u8,
    }

    // 手动实现
    impl Default for Settings {
        fn default() -> Self {
            Settings {
                brightness: 50,
                contrast: 50,
                hue: 50,
                volume: 70,
            }
        }
    }

    let default_v = Settings::default();
    println!("{:?}", default_v);
}
