// Trait std::ops::Neg

// The unary negation operator -.

// # Required Methods
/*
fn neg(self) -> Self::Output
    Performs the unary - operation.
*/

use std::ops::Neg;

#[derive(Debug, PartialEq)]
enum Sign {
    Negative,
    Zero,
    Positive,
}

impl Neg for Sign {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Sign::Negative => Sign::Positive,
            Sign::Zero => Sign::Zero,
            Sign::Positive => Sign::Negative,
        }
    }
}

#[test]
fn t0() {
    // A negative positive is a negative.
    assert_eq!(-Sign::Positive, Sign::Negative);
    // A double negative is a positive.
    assert_eq!(-Sign::Negative, Sign::Positive);
    // Zero is its own negation.
    assert_eq!(-Sign::Zero, Sign::Zero);
}
