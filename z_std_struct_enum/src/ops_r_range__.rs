// Struct std::ops::Range
// Struct std::ops::RangeFrom
// Struct std::ops::RangeFull
// Struct std::ops::RangeInclusive
// Struct std::ops::RangeTo
// Struct std::ops::RangeToInclusive

#[test]
fn r_range_test() {
    // start <= x < end
    println!("{}", (3..5) == std::ops::Range { start: 3, end: 5 }); // print->true

    // x >= start
    println!("{}", (2..) == std::ops::RangeFrom { start: 2 }); // print->true

    // x >= start and x <= end
    println!("{}", (3..=5) == std::ops::RangeInclusive::new(3, 5)); // print->true

    // x < end
    println!("{}", (..5) == std::ops::RangeTo { end: 5 }); // print->true

    // x <= end
    println!("{}", (..=5) == std::ops::RangeToInclusive { end: 5 }); // print->true

    // An unbounded range (..).
    println!("{}", (..) == std::ops::RangeFull); // print->true
}
