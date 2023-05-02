// std::option::Option

/*
Type Option represents an optional value: every Option is either Some and contains a value, or None, and does not.
Option types are very common in Rust code, as they have a number of uses:

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

None: No value.

Some(T): Some value of type T.
*/
fn main() {
    /*
    The type of _some_number is Option<i32>. The type of _some_char is Option<char>, which is a different type.
    Rust can infer these types because we’ve specified a value inside the Some variant.
    */
    let _some_number = Some(5); // 类型为:Option<i32>
    let _some_char = Some('C'); // 类型为:Option<i32>

    /*
    For _absent_number, Rust requires us to annotate the overall Option type: the compiler can’t infer the type that the corresponding Some variant will hold by looking only at a None value.
    Here, we tell Rust that we mean for absent_number to be of type Option<i32>.
    */
    // 报错:type annotations needed for `Option<T>`
    // let absent_number = None;
    let _absent_number: Option<i32> = None;

    let _x: i8 = 5;
    let _y: Option<i8> = Some(5);
    // 报错:cannot add `Option<i8>` to `i8`
    /*
    Option<T> and T (where T can be any type) are different types

    In other words, you have to convert an Option<T> to a T before you can perform T operations with it.
    Generally, this helps catch one of the most common issues with null: assuming that something isn’t null when it actually is.
     */
    // let sum = _x + _y;
}
