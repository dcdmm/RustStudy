// Option

/*
Type Option represents an optional value: every Option is either Some and contains a value, or None, and does not. Option types are very common in Rust code, as they have a number of uses:

* Initial values
* Return values for functions that are not defined over their entire input range (partial functions)
* Return value for otherwise reporting simple errors, where None is returned on error
* Optional struct fields
* Struct fields that can be loaned or “taken”
* Optional function arguments
* Nullable pointers
* Swapping things out of difficult situations


defined by the standard library as follows:

std::option::Option
pub enum Option<T> {
    None,
    Some(T),
}

None

   * No value.

Some(T)

   * Some value of type T.
*/
fn main() {
    /*
    The type of _some_number is Option<i32>. The type of _some_char is Option<char>, which is a different type. Rust can infer these types because we’ve specified a value inside the Some variant.
    */
    let _some_number = Some(5); // 类型为:Option<i32>
    let _some_char = Some('C'); // 类型为:Option<i32>

    /*
    代码程序运行结果:
    error[E0282]: type annotations needed for `Option<T>`
      --> c_basic\src\bin\type_enum3_option_.rs:35:9
       |
    35 |     let absent_number = None;
       |         ^^^^^^^^^^^^^
       |
    help: consider giving `absent_number` an explicit type, where the type for type parameter `T` is specified
       |
    35 |     let absent_number: Option<T> = None;
       |                      +++++++++++
    */
    // let absent_number = None;

    /*
    For _absent_number, Rust requires us to annotate the overall Option type: the compiler can’t infer the type that the corresponding Some variant will hold by looking only at a None value. Here, we tell Rust that we mean for absent_number to be of type Option<i32>.
    */
    let _absent_number: Option<i32> = None;

    // Option<T> and T (where T can be any type) are different types,
    let _x: i8 = 5;
    let _y: Option<i8> = Some(5);
    /*
    代码程序运行结果:
    error[E0277]: cannot add `Option<i8>` to `i8`
      --> c_basic\src\bin\type_enum3_option_.rs:64:17
       |
    64 |     let sum = _x + _y;
       |                  ^ no implementation for `i8 + Option<i8>`
       |
       = help: the trait `Add<Option<i8>>` is not implemented for `i8`
       = help: the following other types implement trait `Add<Rhs>`:
                <&'a f32 as Add<f32>>
                <&'a f64 as Add<f64>>
                <&'a i128 as Add<i128>>
                <&'a i16 as Add<i16>>
                <&'a i32 as Add<i32>>
                <&'a i64 as Add<i64>>
                <&'a i8 as Add<i8>>
                <&'a isize as Add<isize>>
               and 48 others

    For more information about this error, try `rustc --explain E0277`.
    error: could not compile `c_basic` due to previous error 
    */
    // let sum = _x + _y;

}
