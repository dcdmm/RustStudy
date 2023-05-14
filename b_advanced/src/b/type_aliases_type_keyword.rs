// Creating Type Synonyms with Type Aliases

/*
Define an alias for an existing type.

The syntax is type Name = ExistingType;.
 */

#[test]
fn t0() {
    // type does not create a new type:

    type Meters = u32;
    type Kilograms = u32;

    let m: Meters = 3;
    let k: Kilograms = 3;

    assert_eq!(m, k);
}
