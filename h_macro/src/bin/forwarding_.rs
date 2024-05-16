/*
When forwarding a matched fragment to another macro-by-example, matchers in the second macro will see an opaque AST(Abstract Syntax Tree) of the fragment type.  The ident, lifetime, and tt fragment types are an exception.
*/

#[allow(warnings)]
#[test]
fn t0() {
    macro_rules! expra_expr {
        ($lhs:expr) => {};
    }
    macro_rules! expra {
        ($e:expr) => {
            expra_expr!($e) // $lhs:expr可以匹配opaque AST(这里为expr类型)
        };
    }
    expra!(1 + 1);

    macro_rules! exprb_tt_whole {
        ($lhs:tt) => {};
    }
    macro_rules! exprb {
        ($e:expr) => {
            exprb_tt_whole!($e) // $lhs:tt可以匹配opaque AST(这里为expr类型)
        };
    }
    exprb!(1 + 1);

    macro_rules! exprc_tt_whole {
        ($($lhs:tt)*) => {};
    }
    macro_rules! exprc {
        ($e:expr) => {
            exprc_tt_whole!($e) // $($lhs:tt)*可以匹配opaque AST((这里为expr类型))
        };
    }
    exprc!(1 + 1);

    macro_rules! exprd_tt_split {
        ($lhs:tt $op:tt $rhs:tt) => {};
    }
    macro_rules! exprd {
        ($e:expr) => {
            exprd_tt_split!($e) // opaque AST不能被分解
        };
    }
    // error: unexpected end of macro invocation
    // exprd!(1 + 1);

    macro_rules! expre_literal {
        (xxx) => {};
    }
    macro_rules! expre {
        ($l:expr) => {
            expre_literal!($l); // opaque AST不能和token匹配
        };
    }
    // error: no rules expected the token `xxx`
    // expre!(xxx);
}

#[test]
fn t1() {
    macro_rules! tta_tt {
        ($lhs:tt $op:tt $rhs:tt) => {};
    }
    macro_rules! tta {
        ($($t:tt)+) => { tta_tt!($($t)+) }; // tokens 1 + 1可以和$lhs:tt $op:tt $rhs:tt匹配
    }
    tta!(1 + 1);

    macro_rules! ttb_expr {
        ($lhs:expr) => {};
    }
    macro_rules! ttb {
        ($($t:tt)+) => { ttb_expr!($($t)+) }; // tokens 1 + 1可以和$expr匹配(1 + 1是表达式)
    }
    ttb!(1 + 1);

    macro_rules! ttc_literal {
        (xxx) => {};
    }
    macro_rules! ttc {
        ($l:tt) => {
            ttc_literal!($l); // token xxx可以和token xxx匹配
        };
    }
    ttc!(xxx);
}

fn main() {}
