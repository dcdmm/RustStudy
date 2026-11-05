// 字符(Primitive Type char)类型

fn main() {
    let ascii = 'a';
    let non_ascii = '❤';

    // Checks if the value is within the ASCII range.
    println!("{}", ascii.is_ascii()); // print->true
    println!("{}", non_ascii.is_ascii()); // print->false

    // Checks if the value is an ASCII decimal digit: U+0030 ‘0’ ..= U+0039 ‘9’.
    println!("{}", '1'.is_ascii_digit()); // print->true
    println!("{}", 'g'.is_ascii_digit()); // print->false

    /*
    Checks if the value is an ASCII alphabetic character:
        * U+0041 ‘A’ ..= U+005A ‘Z’, or
        * U+0061 ‘a’ ..= U+007A ‘z’.
     */
    println!("{}", 'g'.is_ascii_alphabetic()); // print->true
    println!("{}", '\n'.is_ascii_alphabetic()); // print->false
    println!("{}", 'G'.is_ascii_alphabetic()); // print->ture
    println!("{}", '%'.is_ascii_alphabetic()); // print->false

    // Checks if the value is an ASCII lowercase character: U+0061 ‘a’ ..= U+007A ‘z’.
    println!("{}", 'a'.is_ascii_lowercase()); // print->true
    println!("{}", 'A'.is_ascii_lowercase()); // print->false
                                              // Checks if the value is an ASCII uppercase character: U+0041 ‘A’ ..= U+005A ‘Z’.
    println!("{}", 'a'.is_ascii_uppercase()); // print->false
    println!("{}", 'A'.is_ascii_uppercase()); // print->true

    // Returns true if this char has the White_Space property.
    println!("{}", ' '.is_whitespace()); // print->true
    println!("{}", '\n'.is_whitespace()); // print->true
    println!("{}", '\t'.is_whitespace()); // print->true
}
