// std::option::Option

/*
常见方法:
is_none / is_some
 */

fn main() {
    /*
    pub const fn is_none(&self) -> bool
        Returns true if the option is a None value.

    pub const fn is_some(&self) -> bool
        Returns true if the option is a Some value.
     */
    let x: Option<u32> = Some(2);
    println!("{}", x.is_none()); // false

    let x: Option<u32> = None;
    println!("{}", x.is_none()); // true
}

