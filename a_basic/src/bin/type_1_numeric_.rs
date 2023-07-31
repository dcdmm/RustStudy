// 数值类型(Primitive Type)与算术运算,比较运算

// 参考:https://doc.rust-lang.org/reference/types/numeric.html

/*
# Integer types:

The unsigned integer types consist of:
Type	Minimum	Maximum
u8	    0	    2^{8}-1
u16	    0	    2^{16}-1
u32	    0	    2^{32}-1
u64	    0	    2^{64}-1
u128	0	    2^{128}-1

The signed two's complement integer types consist of:
Type	Minimum	    Maximum
i8	    -(2^7)	    2^7-1
i16	    -(2^{15})	2^{15}-1
i32	    -(2^{31})	2^{31}-1
i64	    -(2^{63})	2^{63}-1
i128	-(2^{127})	2^{127}-1


# Floating-point types

The IEEE 754-2008 "binary32" and "binary64" floating-point types are f32 and f64, respectively.


# Machine-dependent integer types

The usize type is an unsigned integer type with the same number of bits as the platform's pointer type. It can represent every memory address in the process.

The isize type is a signed integer type with the same number of bits as the platform's pointer type. The theoretical upper bound on object and array size is the maximum isize value. This ensures that isize can be used to calculate differences between pointers into an object or array and can address every byte within an object along with one byte past the end.

usize and isize are at least 16-bits wide.

Note: Many pieces of Rust code may assume that pointers, usize, and isize are either 32-bit or 64-bit. As a consequence, 16-bit pointer support is limited and may require explicit care and acknowledgment from a library to support.
 */

fn main() {
    let _x = 5; // 整型默认使用`i32`类型
    let _f0 = 5.34; // 浮点型默认使用`f64`类型
    let _f1: f32 = 5.34;
    let f2 = 5.34e2; // 科学计数法
    println!("{}", f2);

    println!("{}", 98_222); // Decimal
    println!("{}", 0xff); // Hex
    println!("{}", 0o77); // Octal
    println!("{}", 0b1111_0000); // Binary
    println!("{}", b'A'); // Byte (u8 only)

    // 算法运算符: +, -, *, /, %
    println!("{}", 5 + 10); // 加法
    println!("{}", 5 / 10); // 除法
    println!("{}", 11 / 5); // 整数与整数相除仍为整数
    println!("{}", 43 % 6); // 取余

    let mut x = 5;
    /*
    赋值运算符:
    +=: fn add_assign(&mut self, rhs: Rhs)
    -=: fn sub_assign(&mut self, rhs: Rhs)
    *=: fn mul_assign(&mut self, rhs: Rhs)
    /=: fn div_assign(&mut self, rhs: Rhs)
    %=: fn rem_assign(&mut self, rhs: Rhs)
    */
    x += 1;
    println!("{}", x);

    // 比较运算符:>, >=, <, <=, ==, !=
    println!("{}", 5 != 34)
}
