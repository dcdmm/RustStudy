// trait

trait Signed {
    fn strictly_negative(self) -> bool;
}

// 数值类型i32(Primitive Type)实现trait
impl Signed for i32 {
    fn strictly_negative(self) -> bool {
        self < 0
    }
}

struct Number {
    odd: bool,
    value: i32,
}

// 运算符(The unary negation operator -.)重载
impl std::ops::Neg for Number {
    type Output = Number;
    fn neg(self) -> Self::Output {
        Number {
            value: -self.value,
            odd: self.odd,
        }
    }
}

#[test]
fn t0() {
    let n = -44;
    println!("{}", n.strictly_negative());

    let x = Number { odd: true, value: 100 };

    let y = -x; // this is only possible because we implemented `Neg`
    println!("{}", y.value);
}