// 模式匹配(Matching Named Variables)

#[test]
fn t0()
{
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        /*
        The pattern in the second match arm introduces a new variable named y that will match any value inside a Some value.
        Because we’re in a new scope inside the match expression, this is a new y variable, not the y we declared at the beginning with the value 10.
        This new y binding will match any value inside a Some, which is what we have in x.
        Therefore, this new y binds to the inner value of the Some in x.
        That value is 5, so the expression for that arm executes and prints Matched, y = 5.
         */
        Some(y) => println!("Matched, y = {y}"),

        // If x had been a None value instead of Some(5), the patterns in the first two arms wouldn’t have matched, so the value would have matched to the underscore.
        _ => println!("Default case, x = {:?}", x),
    }

    // When the match expression is done, its scope ends, and so does the scope of the inner y.
    println!("at the end: x = {:?}, y = {y}", x);
}