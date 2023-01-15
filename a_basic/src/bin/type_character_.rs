// 字符(Primitive Type char)类型

// The char type represents a single character.
fn main() {
    /*
    Note that we specify char literals with single quotes, as opposed to string literals, which use double quotes.

    Rust’s char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII. Accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are all valid char values in Rust. Unicode Scalar Values range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive.
    */
    let c = 'z';
    let z = 'ℤ';
    let g = '国';
    let heart_eyed_cat = '😻';

    println!("{}", c);
    println!("{}", z);
    println!("{}", g);
    println!("{}", heart_eyed_cat);
}
