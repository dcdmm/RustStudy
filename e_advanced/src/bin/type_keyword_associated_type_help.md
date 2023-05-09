Associated types connect a type placeholder with a trait such that the trait method definitions can use these
placeholder types in their signatures. The implementor of a trait will specify the concrete type to be used instead of
the placeholder type for the particular implementation. That way, we can define a trait that uses some types without
needing to know exactly what those types are until the trait is implemented.

An associated type declaration declares a signature for associated type definitions. It is written in one of the
following forms, where Assoc is the name of the associated type, Params is a comma-separated list of type, lifetime or
const parameters, Bounds is a plus-separated list of trait bounds that the associated type must meet, and WhereBounds is
a comma-separated list of bounds that the parameters must meet:

```rust
type Assoc;
type Assoc: Bounds;
type Assoc<Params>;
type Assoc<Params>: Bounds;
type Assoc<Params> where WhereBounds;
type Assoc<Params>: Bounds where WhereBounds;
```

The identifier is the name of the declared type alias. The optional trait bounds must be fulfilled by the
implementations of the type alias. There is an implicit Sized bound on associated types that can be relaxed using the
special ?Sized bound.

An associated type definition defines a type alias for the implementation of a trait on a type. They are written
similarly to an associated type declaration, but cannot contain Bounds, but instead must contain a Type:

```rust
type Assoc = Type;
type Assoc<Params> = Type;
// the type `Type` here may reference `Params`
type Assoc<Params> = Type where WhereBounds;
```

If a type Item has an associated type Assoc from a trait Trait, then <Item as Trait>::Assoc is a type that is an alias
of the type specified in the associated type definition. Furthermore, if Item is a type parameter, then Item::Assoc can
be used in type parameters.

Associated types may include generic parameters and where clauses; these are often referred to as generic associated
types, or GATs. If the type Thing has an associated type Item from a trait Trait with the generics <'a> , the type can
be named like <Thing as Trait>::Item<'x>, where 'x is some lifetime in scope. In this case, 'x will be used wherever 'a
appears in the associated type definitions on impls..

### Associated Types Container Example

Consider the following example of a Container trait. Notice that the type is available for use in the method signatures:

```rust
trait Container {
    type E;
    fn empty() -> Self;
    fn insert(&mut self, elem: Self::E);
}
```

In order for a type to implement this trait, it must not only provide implementations for every method, but it must
specify the type E. Here's an implementation of Container for the standard library type `Vec`:

```rust
impl<T> Container for Vec<T> {
    type E = T;
    fn empty() -> Vec<T> { Vec::new() }
    fn insert(&mut self, x: T) { self.push(x); }
}
```

### Relationship between Bounds and WhereBounds

In this example:

```rust
trait Example {
    type Output<T>: Ord where T: Debug;
}
```

Given a reference to the associated type like `<X as Example>::Output<Y>`, the associated type itself must be `Ord`, and
the type `Y` must be `Debug`.

### Required where clauses on generic associated types

Generic associated type declarations on traits currently may require a list of where clauses, dependent on functions in
the trait and how the GAT is used. These rules may be loosened in the future; updates can be
found [on the generic associated types initiative repository](https://rust-lang.github.io/generic-associated-types-initiative/explainer/required_bounds.html).

In a few words, these where clauses are required in order to maximize the allowed definitions of the associated type in
impls. To do this, any clauses that can be proven to hold on functions (using the parameters of the function or trait)
where a GAT appears as an input or output must also be written on the GAT itself.

```rust
trait LendingIterator {
    type Item<'x> where Self: 'x;
    fn next<'a>(&'a mut self) -> Self::Item<'a>;
}
```

In the above, on the next function, we can prove that Self: 'a, because of the implied bounds from &'a mut self;
therefore, we must write the equivalent bound on the GAT itself: where Self: 'x.

When there are multiple functions in a trait that use the GAT, then the intersection of the bounds from the different
functions are used, rather than the union.

```rust
trait Check<T> {
    type Checker<'x>;
    fn create_checker<'a>(item: &'a T) -> Self::Checker<'a>;
    fn do_check(checker: Self::Checker<'_>);
}
```

In this example, no bounds are required on the type Checker<'a>;. While we know that T: 'a on create_checker, we do not
know that on do_check. However, if do_check was commented out, then the where T: 'x bound would be required on Checker.

The bounds on associated types also propagate required where clauses.

```rust
trait Iterable {
    type Item<'a> where Self: 'a;
    type Iterator<'a>: Iterator<Item=Self::Item<'a>> where Self: 'a;
    fn iter<'a>(&'a self) -> Self::Iterator<'a>;
}
```

Here, where Self: 'a is required on Item because of iter. However, Item is used in the bounds of Iterator, the where
Self: 'a clause is also required there.

Finally, any explicit uses of 'static on GATs in the trait do not count towards the required bounds.

```rust
trait StaticReturn {
    type Y<'a>;
    fn foo(&self) -> Self::Y<'static>;
}
```