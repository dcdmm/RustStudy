### Integer types:

The unsigned integer types consist of:
| Type   | Minimum | Maximum |
| ------ | ------- | ------- |
| `u8`   | 0       | 2^8-1   |
| `u16`  | 0       | 2^16-1  |
| `u32`  | 0       | 2^32-1  |
| `u64`  | 0       | 2^64-1  |
| `u128` | 0       | 2^128-1 |

The signed two's complement integer types consist of:

| Type   | Minimum | Maximum |
| ------ | ------- | ------- |
| `i8`   | -(27)   | 2^7-1   |
| `i16`  | -(215)  | 2^15-1  |
| `i32`  | -(231)  | 2^31-1  |
| `i64`  | -(263)  | 2^63-1  |
| `i128` | -(2127) | 2^127-1 |

### Floating-point types

The IEEE 754-2008 "binary32" and "binary64" floating-point types are f32 and f64, respectively.

### Machine-dependent integer types

The primary situation in which you’d use isize or usize is when indexing some sort of collection.

#### Primitive Type usize

The pointer-sized unsigned integer type.

The size of this primitive is how many bytes it takes to reference any location in memory. For example, on a 32 bit target, this is 4 bytes and on a 64 bit target, this is 8 bytes.

#### Primitive Type isize

The pointer-sized signed integer type.

The size of this primitive is how many bytes it takes to reference any location in memory. For example, on a 32 bit target, this is 4 bytes and on a 64 bit target, this is 8 bytes.