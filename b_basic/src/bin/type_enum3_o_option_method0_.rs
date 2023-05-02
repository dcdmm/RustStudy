// std::option::Option
/*
常见方法:
is_none / is_some
 */
fn main() {}

#[test]
fn is_none_is_some_fn() {
    /*
    pub const fn is_none(&self) -> bool
        Returns true if the option is a None value.

    pub const fn is_some(&self) -> bool
        Returns true if the option is a Some value.
     */
    let x: Option<u32> = Some(2);
    assert_eq!(x.is_none(), false);

    let x: Option<u32> = None;
    assert_eq!(x.is_none(), true);
}

