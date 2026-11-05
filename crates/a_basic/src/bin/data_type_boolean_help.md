### Logical not

| `b`     | `!b`    |
| ------- | ------- |
| `true`  | `false` |
| `false` | `true`  |

### Logical or

| `a`     | `b`     | `a      | b` |
| ------- | ------- | ------- |
| `true`  | `true`  | `true`  |
| `true`  | `false` | `true`  |
| `false` | `true`  | `true`  |
| `false` | `false` | `false` |

### Logical and

| `a`     | `b`     | `a & b` |
| ------- | ------- | ------- |
| `true`  | `true`  | `true`  |
| `true`  | `false` | `false` |
| `false` | `true`  | `false` |
| `false` | `false` | `false` |

### Logical xor

| `a`     | `b`     | `a ^ b` |
| ------- | ------- | ------- |
| `true`  | `true`  | `false` |
| `true`  | `false` | `true`  |
| `false` | `true`  | `true`  |
| `false` | `false` | `false` |

### Comparisons
| `a`     | `b`     | `a == b` |
| ------- | ------- | -------- |
| `true`  | `true`  | `true`   |
| `true`  | `false` | `false`  |
| `false` | `true`  | `false`  |
| `false` | `false` | `true`   |

| `a`     | `b`     | `a > b` |
| ------- | ------- | ------- |
| `true`  | `true`  | `false` |
| `true`  | `false` | `true`  |
| `false` | `true`  | `false` |
| `false` | `false` | `false` |

* a != b is the same as !(a == b)
* a >= b is the same as a == b | a > b
* a < b is the same as !(a >= b)
* a <= b is the same as a == b | a < b