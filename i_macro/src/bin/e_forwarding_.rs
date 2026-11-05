/*
When forwarding a matched fragment to another macro-by-example, matchers in the second macro will see an opaque AST(Abstract Syntax Tree) of the fragment type.  The ident, lifetime, and tt fragment types are an exception.
*/

#[allow(warnings)]
#[test]
fn t0() {
    macro_rules! expra_expr {
        ($lhs:expr) => {
            println!("expra : {}", stringify!($lhs));
        };
    }
    macro_rules! expra {
        ($e:expr) => {
            expra_expr!($e) // $lhs:expr可以匹配opaque AST(这里为expr类型)
        };
    }
    expra!(1 + 1);
}

#[allow(warnings)]
#[test]
fn t1() {
    macro_rules! exprb_tt_whole {
        ($lhs:tt) => {
            // invisible delimiter无文本表示,stringify!不会输出
            println!(
                "exprb tt数量: {}, 每个tt: {:?}",
                [stringify!($lhs)].len(),
                [stringify!($lhs)]
            );
            println!("exprb文本: {}", stringify!($lhs));
            println!("exprb计算结果: {}", $lhs * 2);
        };
    }
    macro_rules! exprb {
        ($e:expr) => {
            // 转发时$e被隐形括号(invisible delimiter)包裹,整体算作1个token tree
            exprb_tt_whole!($e) // $lhs:tt可以匹配opaque AST(这里为expr类型)
        };
    }
    exprb!(1 + 1); // 实际计算顺序为: (1+1)*2

    macro_rules! exprc_tt_whole {
        ($($lhs:tt)*) => {
            println!("exprc tt数量: {}, 每个tt: {:?}", [$(stringify!($lhs)),*].len(), [$(stringify!($lhs)),*]);
            println!("exprc文本: {}", stringify!($($lhs)* * 2));
            println!("exprc计算结果: {}", $($lhs)* * 2);
        };
    }
    macro_rules! exprc {
        ($e:expr) => {
            // $e整体算作1个token tree,$($lhs:tt)*仅重复1次
            exprc_tt_whole!($e) // $($lhs:tt)*可以匹配opaque AST(这里为expr类型)
        };
    }
    exprc!(1 + 1); // 实际计算顺序为: (1+1)*2
    exprc_tt_whole!(1 + 1); // 匹配3个tt;实际计算顺序为: 1+(1*2)

    // 转发多个expr: $a和$b各算1个tt,加上*共3个tt
    macro_rules! exprf_tt_many {
        ($($lhs:tt)*) => {
            println!("exprf tt数量: {}, 每个tt: {:?}", [$(stringify!($lhs)),*].len(), [$(stringify!($lhs)),*]);
            println!("exprf文本: {}", stringify!($($lhs)*));
            println!("exprf计算结果: {}", $($lhs)*);
        };
    }
    macro_rules! exprc1 {
        ($a:expr, $b:expr) => {
            exprf_tt_many!($a * $b) // 若改用$lhs:tt匹配,报错: no rules expected `*`
        };
    }
    exprc1!(1 + 1, 2 + 3); // 实际计算顺序为: (1+1)*(2+3)
}

#[allow(warnings)]
#[test]
fn t2() {
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
}

#[allow(warnings)]
#[test]
fn t3() {
    macro_rules! expre_literal {
        (xxx) => {};
    }
    macro_rules! expre {
        ($l:expr) => {
            expre_literal!($l); // opaque AST不能和token匹配
        };
    }
    // error: no rules expected `expr` metavariable
    // expre!(xxx);
}

#[test]
fn t1x() {
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
        ($($t:tt)+) => { ttb_expr!($($t)+) }; // tokens 1 + 1可以和$lhs:expr匹配(1 + 1是表达式)
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
