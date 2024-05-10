#[test]
fn t0() {
    // 单个匹配规则
    macro_rules! rule_single {
        () => {
            1 + 3
        };
    }
    let a = rule_single!();
    println!("{a}"); // print->4
    let _b = rule_single! {}; // 与上等价
    let _c = rule_single![]; // 与上等价

    // 多个匹配规则
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
    let aa = rule_many!(); // 对应匹配规则1
    println!("{aa}"); // print->4
    let bb = rule_many!(5); // 对应匹配规则2
    println!("{bb}"); // print->5
    let cc = rule_many!(1, 2, 3); // 对应匹配规则3
    println!("{cc}"); // print->5
}

fn main() {}
