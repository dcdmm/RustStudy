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
            exprf_tt_many!($a * $b)
        };
    }
    exprc1!(1 + 1, 2 + 3); // 实际计算顺序为: (1+1)*(2+3)

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
fn t2() {
    macro_rules! expr_literal {
        (xxx) => {};
    }
    macro_rules! expr {
        // $l:expr将xxx解析为expr类型的opaque AST
        ($l:expr) => {
            expr_literal!($l); // opaque AST不能和token匹配
        };
    }
    // error: no rules expected `expr` metavariable
    // expr!(xxx);

    macro_rules! tt_literal {
        (xxx) => {
            println!("tt_literal匹配成功: {}", stringify!(xxx));
        };
    }
    macro_rules! tt {
        // $l:tt不解析xxx,转发后仍是原始ident token xxx(ident,lifetime,tt属于例外)
        ($l:tt) => {
            tt_literal!($l); // token xxx可以和token xxx匹配
        };
    }
    tt!(xxx);
}

fn main() {}
