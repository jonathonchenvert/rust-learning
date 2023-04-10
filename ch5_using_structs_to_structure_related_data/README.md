# Chapter 5: Using Structs to Structure Related Data

NOTE: Contents of this README are abridged; see [main.rs](src/main.rs) for this chapter for code examples and commenting

##### TABLE OF CONTENTS
1. [Where's the -> Operator?](#wheres-the---operator)
2. [Summary](#summary)

## [Where's the -> Operator?](#table-of-contents)

C and C++ have two different operators that are used for calling methods:
1. `.` for calling a method on the object directly, and
2. `->` for calling a method on a pointer to the object and need to dereference the pointer first.
    1. If `object` is a pointer, `object->something()` is similar to `(*object).something()`.

Rust doesn't have an equivalent to the `->` operator; instead, it has a feature called
_automatic referencing and dereferencing_. Calling methods in Rust is one of the few places
in Rust that has this behavior.

When a method is called with `object.something()`, Rust automatically adds in `&`, `&mut`, or `*`
so `object` matches the signature of the method. The following code is functionally the same:
```rust
p1.distance(&p2);
(&p1).distance(&p2);
```

The first one looks cleaner. The automatic referencing behavior works because methods have a clear receiver - the `self` type.
Given the receiver and name of a method, Rust can figure out definitively whether the method is reading (`&self`),
mutating (`&mut self`), or consuming (`self`). The fact that Rust makes borrowing implicit for method receivers is a big 
part of making ownership ergonomic in practice.

## [Summary](#table-of-contents)

Structs allow creation of custom types that are meaningful for an application. By using structs, associated pieces of data can be
kept connected to each other and name each piece for clear code. In `impl` blocks, functions can be defined that are associated
with the struct, and methods are a kind of associated function that can specify the behavior that struct instances have.

Structs aren't the only means of creating custom types: `enum`s are another way (see chapter 6).

##### [back to parent readme](../README.md)