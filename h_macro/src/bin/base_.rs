#[test]
fn t0() {
    // 宏规则: MacroMatcher => MacroTranscriber
    // Each rule has two parts: a matcher, describing the syntax that it matches, and a transcriber, describing the syntax that will replace a successfully matched invocation.

    // 单个宏规则
    macro_rules! rule_single {
        () => {
            1 + 3
        };
    }
    let a = rule_single!();
    println!("{a}"); // print->4
    let _b = rule_single! {}; // 与上等价
    let _c = rule_single![]; // 与上等价

    // 多个宏规则(从上到下的顺序依次匹配)
    macro_rules! rule_many {
        () => {
            1 + 3
        }; // 规则1
        ($e:expr) => {
            1 * $e
        }; // 规则2
        ($a:expr, $b:expr, $c:expr) => {
            $a * ($b + $c)
        }; // 规则3
        () => {}; // 规则4
    }
    let aa = rule_many!(); // 对应规则1
    println!("{aa}"); // print->4
    let bb = rule_many!(5); // 对应规则2
    println!("{bb}"); // print->5
    let cc = rule_many!(1, 2, 3); // 对应规则3
    println!("{cc}"); // print->5
}

#[test]
fn t1() {
    macro_rules! repeat {
        ($e:expr) => {
            $e;
            $e;
            $e;
        };
    }

    repeat!(println!("Hello"));

    let mut x = 0;
    repeat!(x += 1);
    println!("{x}"); // print->3

    let mut vec = vec![];
    repeat!(vec.push("a"));
    println!("{:?}", vec); // print->["a", "a", "a"]
}

#[allow(warnings)]
#[test]
fn t2() {
    macro_rules! repeat0 {
        ($e:expr) => {
            $e;
            "hello rust!"
        };
    }
    let mut x = 0;
    // error: macro expansion ignores token `"hello rust!"` and any following
    // let xr = repeat0!(x += 1);

    macro_rules! repeat1 {
        ($e:expr) => {
            // 代码块
            {
                $e;
                "hello rust!" // 表达式
            }
        };
    }
    let mut y = 0;
    /*
    展开宏调用,即:
    let yr = {
        y+=1;
        "hello rust!"
    };
     */
    let yr = repeat1!(y += 1);
    println!("{}", yr); // print->hello rust!
    println!("{y}"); // print->1
}

fn main() {}
