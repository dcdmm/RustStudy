`as` can be used to explicitly perform coercions, as well as the following additional casts. Any cast that does not fit
either a coercion rule or an entry in the table is a compiler error. Here *T means either `*const T` or `*mut T`. m stands
for optional mut in reference types and `mut` or `const` in pointer types.

| Type of `e`                                                                         | `U`                                                                                 | Cast performed by `e as U`             |
|-------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------|----------------------------------------|
| Integer or Float type                                                               | Integer or Float type                                                               | Numeric cast                           |
| Enumeration                                                                         | Integer type                                                                        | Enum cast                              |
| `bool` or `char`                                                                    | Integer type                                                                        | Primitive to integer cast              |
| `u8`                                                                                | `char`                                                                              | `u8` to `char` cast                    |
| `*T`                                                                                | `*V` where `V: Sized` *                                                             | Pointer to pointer cast                |
| `*T` where `T: Sized`                                                               | Integer type                                                                        | Pointer to address cast                |
| Integer type                                                                        | `*V` where `V: Sized`                                                               | Address to pointer cast                |
| `&m₁ T`                                                                             | `*m₂ T` **                                                                          | Reference to pointer cast              |
| `&m₁ [T; n]`                                                                        | `*m₂ T` **                                                                          | Array to pointer cast                  |
| [Function item](https://doc.rust-lang.org/reference/types/function-item.html)       | [Function pointer](https://doc.rust-lang.org/reference/types/function-pointer.html) | Function item to function pointer cast |
| [Function item](https://doc.rust-lang.org/reference/types/function-item.html)       | `*V` where `V: Sized`                                                               | Function item to pointer cast          |
| [Function item](https://doc.rust-lang.org/reference/types/function-item.html)       | Integer                                                                             | Function item to address cast          |
| [Function pointer](https://doc.rust-lang.org/reference/types/function-pointer.html) | `*V` where `V: Sized`                                                               | Function pointer to pointer cast       |
| [Function pointer](https://doc.rust-lang.org/reference/types/function-pointer.html) | Integer                                                                             | Function pointer to address cast       |
| Closure ***                                                                         | Function pointer                                                                    | Closure to function pointer cast       |

`*` or T and V are compatible unsized types, e.g., both slices, both the same trait object.

`**` only when m₁ is mut or m₂ is const. Casting mut reference to const pointer is allowed.

`***` only for closures that do not capture (close over) any local variables

### Numeric cast

* Casting between two integers of the same size (e.g. i32 -> u32) is a no-op (Rust uses 2's complement for negative
  values of fixed integers)
* Casting from a larger integer to a smaller integer (e.g. u32 -> u8) will truncate
* Casting from a smaller integer to a larger integer (e.g. u8 -> u32) will
    * zero-extend if the source is unsigned
    * sign-extend if the source is signed
* Casting from a float to an integer will round the float towards zero
    * NaN will return 0
    * Values larger than the maximum integer value, including INFINITY, will saturate to the maximum value of the
      integer type.
    * Values smaller than the minimum integer value, including NEG_INFINITY, will saturate to the minimum value of the
      integer type.
* Casting from an integer to float will produce the closest possible float *
    * if necessary, rounding is according to roundTiesToEven mode ***
    * on overflow, infinity (of the same sign as the input) is produced
    * note: with the current set of numeric types, overflow can only happen on u128 as f32 for values greater or equal
      to f32::MAX + (0.5 ULP)

* Casting from an f32 to an f64 is perfect and lossless
* Casting from an f64 to an f32 will produce the closest possible f32 **
    * if necessary, rounding is according to roundTiesToEven mode ***
    * on overflow, infinity (of the same sign as the input) is produced

`*` if integer-to-float casts with this rounding mode and overflow behavior are not supported natively by the hardware,
these casts will likely be slower than expected.

`**` if f64-to-f32 casts with this rounding mode and overflow behavior are not supported natively by the hardware, these
casts will likely be slower than expected.

`***` as defined in IEEE 754-2008 §4.3.1: pick the nearest floating point number, preferring the one with an even least
significant digit if exactly halfway between two floating point numbers.

### Enum cast

Casts an enum to its discriminant, then uses a numeric cast if needed. Casting is limited to the following kinds of
enumerations:

* [Unit-only enums](https://doc.rust-lang.org/reference/items/enumerations.html#unit-only-enum)
* [Field-less](https://doc.rust-lang.org/reference/items/enumerations.html#field-less-enum) enums
  without [explicit discriminants](https://doc.rust-lang.org/reference/items/enumerations.html#explicit-discriminants),
  or where only unit-variants have explicit discriminants

### Primitive to integer cast

* false casts to 0, true casts to 1
* char casts to the value of the code point, then uses a numeric cast if needed.

### u8 to char cast

Casts to the char with the corresponding code point.

### Pointer to address cast

Casting from a raw pointer to an integer produces the machine address of the referenced memory. If the integer type is
smaller than the pointer type, the address may be truncated; using usize avoids this.

### Address to pointer cast
Casting from an integer to a raw pointer interprets the integer as a memory address and produces a pointer referencing that memory.

Warning: This interacts with the Rust memory model, which is still under development. A pointer obtained from this cast may suffer additional restrictions even if it is bitwise equal to a valid pointer. Dereferencing such a pointer may be undefined behavior if aliasing rules are not followed.