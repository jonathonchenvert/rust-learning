# Chapter 4: Understanding Ownership

##### TABLE OF CONTENTS
1. [What is Ownership?](#what-is-ownership)
    * [Ownership Rules](#ownership-rules)
    * [Variable Scope](#variable-scope)
    * [The String Type](#the-string-type)
    * [Memory and Allocation](#memory-and-allocation)
        * [Ways That Variables and Data Interact: Move](#ways-that-variables-and-data-interact-move)
        * [Ways That Variables and Data Interact: Clone](#ways-that-variables-and-data-interact-clone)
        * [Stack-Only Data: Copy](#stack-only-data-copy)
    * [Ownership and Functions](#ownership-and-functions)
    * [Return Values and Scope](#return-values-and-scope)
2. [References and Borrowing](#references-and-borrowing)
    * [Mutable References]()
    * [Dangling References]()
    * [The Rules of References]()
3. [The Slice Type](#the-slice-type)
    * [String Slices]()
        * [String Literals Are Slices]()
        * [String Slices as Parameters]()
    * [Other Slices]()

## [What is Ownership?](#table-of-contents)

A unique feature of the Rust programming language is _ownership_. It is straightforward, but has deep implications for the language. All programs have to manage the way a computer's memory is used while running. Some examples include:

1. Garbage collection, which constantly looks for no longer used memory as the program runs
2. Explicit allocation and freeing up the usage of the memory, which is managed by the programmer.

Rust has a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks at compile time. No ownership features slow down the program while it's running.

### [Ownership Rules](#table-of-contents)

1. Each value in Rust has a variable that's called its _owner_.
2. There can be only one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

### [Variable Scope](#table-of-contents)

A variable's _scope_ is the range within a program for which an item is valid. Take the following line of code, for example:
```rust
{                     // s is not valid here, since it has not yet been declared
    let s = "hello"; // s is valid from this point forward

    // do stuff with s
}                   // this scope is now over, and s is no longer valid
```

The variable _s_ refers to a string literal, where the value of the string is hardcoded into the text of the program. The variable is valid from the point where it's declared until the end of the current _scope_. There are a few important things to note here:

* When _s_ comes _into scope_, it is valid
* _s_ continues to be valid until it goes _out of scope_

The relationship between scopes and when variables are valid is similar to how other programming languages refer to scopes of variables.

### [The String Type](#table-of-contents)

The types covered previously are all stored on the stack and popped off the stack when their scope is over, but what about data that is stored on the heap? Rust knows when to clean that data up.

A `String` is an example. The aspects covered below will focus on how `String` relates to ownership, and can also apply to other complex data types provided by the standard library and ones created by other developers, including oneself.

String literals have already been seen, with a string value being hardcoded into the program. String literals are convenient, but not suitable for every situation where text may be used. String literals:

* are immutable, and
* not every string value can be known while writing code
    * e.g. what about user providing input? Where is that stored?

With these situations, Rust has a second string type (`String`). This type is allocated on the heap and can store an amount of text that is unknown at compile time. A `String` can be created from a string literal using the _from_ function:
```rust
let s = String::from("hello");
```

Note that the double colon (`::`) is an operator that allows usage of the `from` function under the `String` type, rather than using something like `string_from`. The string example from above _can_ be mutated:
```rust
let mut s = String::from("hello");

s.push_str(", world!"); // push_str() appends a literal to a String

println!("{s}"); // prints `hello, world!`
```

Why can `String` be mutated but string literals can't? It has to do with how these two types deal with memory.

### [Memory and Allocation](#table-of-contents)

With string literals, the contents are known at compile time, so the text is hardcoded directly into the final executable. This is the reason why string literals are fast and efficient, which only come from a string literal's immutability. A blob of memory can't be put into binary for each piece of text whose size is unknown at compile time and whose size might change while running the program.

With the `String` type, the ability to support mutability and alterations of a string require allocating an amount of memory on the heap, where the size is unknown at compile time. This means:

* the memory must be requested from the operating system at runtime, and
* there needs to be a way of returning this memory to the operating system when done with the `String`.

The first part is done when calling `String::from`, where its implementation requests the memory it needs. The second part is different. Languages that have _garbage collection_ (_GC_) uses the GC to keep track and clean up memory that isn't being used anymore, and isn't thought about much by developers. Without a GC, it's usually the responsibility of the developers to identify when memory is no longer being used and call code to explicitly return it, just as it was to request it. Doing this correctly has historically been a difficult programming problem. If developers forget to free memory, then memory is wasted. If it's done too early, an invalid variable will exist (which could crash programs). Freeing memory twice can also be considered a bug. Exactly one `allocate` needs to be paired with exactly one `free`.

Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope.

```rust
{                                   // s is not valid here, since it has not yet been declared
    let s = String::from("hello"); // s is valid from this point forward

    // do stuff with s
}                                // this scope is now over, and s is no longer valid
```

There is a natural point at which the memory that `String` needs can be returned to the operating system: once `s` goes out of scope. Rust calls a special function (`drop`) for the developer when the variable goes out of scope, and it's where the author of `String` can put the code to return the memory. Rust calls `drop` automatically at the closing curly bracket.

> In C++, this pattern of deallocating resources at the end of an item's lifetime is sometimes called Resource Acquisition Is Initialization (RAII). The `drop` function in Rust will be familiar to those that have used RAII patterns.

This pattern has a profound impact on how Rust code is written. It looks simple now, but the behavior of code can be unexpected in more complicated situations when having multiple variables using the data allocated on the heap.

#### [Ways That Variables and Data Interact: Move](#table-of-contents)

Multiple variables can interact with the same data in different ways in Rust. Take the code below for example:
```rust
let x = 5;
let y = x;
```

The code above is binding the value 5 to `x`, then making a copy of the value in `x` and binding it to `y`. Both variables equal 5. The integers are simple values with a known, fixed size and the two `5` values are pushed onto the stack. The code below replicates this approach, but from a `String` perspective.
```rust
let s1 = String::from("hello");
let s2 = s1;
```

One would assume that it would work the same as the integer example above, but that isn't entirely true. The figure below shows what happens with `String` under the hood. 

![figure4_1](img/trpl04-01.svg)


A `String` is made up of three parts:

1. A pointer to the memory that holds the contents of the string,
2. a length, and
3. a capacity.

The group of data is stored on the stack. On the right is the memory on the heap that holds the contents.

The length is how much memory (in bytes) the contents of the `s1` `String` is currently using. The capacity is the total amount of memory (in bytes) that it has received from the operating system. The difference between length and capacity matters (except in the context of the image above).

When assigning `s1` to `s2`, the `String` data is copied, meaning the pointer, length, and capacity from the lefthand side stack. The data on the heap that the pointer refers to is not copied. This process will look like the figure below.

![figure4_2](img/trpl04-02.svg)

The representation does **not** look like the figure below, which is what memory would look like if Rust instead copied the heap data. If Rust were to do this in the `let s2 = s1;` line of code, runtime performance would take a large hit if the data on the heap were large.

![figure4_3](img/trpl04-03.svg)

From an earlier mention: when a variable goes out of scope, Rust automatically calls the `drop` function and cleans up the heap memory for that variable. In the second image above, both data pointers are pointing to the same location in the heap memory. This will cause a problem when both `s1` and `s2` go out of scope, as they both will try to free the same memory. This is known as a _double free_ error, and is one of the memory safety bugs previously mentioned. Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.

To ensure memory safety, there's one more detail to what happens in Rust. Instead of trying to copy the allocated memory, Rust considers `s1` to no longer be valid, which means Rust doesn't need to free anything when `s1` goes out of scope. The code below will trigger an error, as Rust will prevent the developer from using the invalidated reference.

```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);
```
```bash
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:5:28
  |
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 | 
5 |     println!("{}, world!", s1);
  |                            ^^ value borrowed here after move
  |
  = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0382`.
error: could not compile `ownership` due to previous error
```

Other languages have terms called _shallow copy_ and _deep copy_. The concept of copying the pointer, length, and capacity without copying the data sounds like a shallow copy. But because Rust also invalidates the first variable, instead of being called a shallow copy, it's known as a _move_. In the code example above, `s1` was _moved_ to `s2`. What actually occurs is in the image below, where the grayed out `s1` is invalidated.

![figure4_4](img/trpl04-04.svg)

This solves the problem of freeing memory, as `s2` is the only valid variable. When `s2` goes out of scope, only `s2` will have its memory released.

This also implies a design choice: Rust will never automatically create "deep" copies of data. Therefore, _automatic_ copying can be assumed to be inexpensive in terms of runtime performance.

#### [Ways That Variables and Data Interact: Clone](#table-of-contents)

If it is desired to deeply copy the heap data of the `String` (not just the stack data), a `clone` method can be used instead. Here's an example of it in use:
```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {s1}, s2 = {s2}");
```

This works fine and explicitly produces the behavior shown in the third image above, where the heap data _does_ get copied, in addition to the stack data.

When a call to `clone` is seen, some arbitrary code is being executed and that code may be expensive. It's a visual indicator that something else is going on.

#### [Stack-Only Data: Copy](#table-of-contents)

Sample code previously shown above doesn't entirely fit into this category either.
```rust
let x = 5;
let y = x;
println!("x = {x}, y = {y}");
```

This code contradicts what was previously known, which is that a call to `clone` isn't made, yet `x` is still valid and didn't move to `y`.

The reason is that scalar types, such as integers, have a known size at compile time and are stored entirely on the stack. This means that copies of the actual values are quick to make. Because of this, there isn't really any reason to prevent `x` from being valid after creating the variable `y`. For these types of variables, there's no difference between deep and shallow copying, so calling `clone` wouldn't do anything different from the usual shallow copying.

Rust has a special annotation called the `Copy` trait that can be placed on types like integers that are stored on the stack. If a type has the `Copy` trait, an older variable is still usable after assignment. Rust doesn't allow annotating a type with the `Copy` trait if the type (or any of its parts) has implemented the `Drop` trait. If the type needs something special to happen when the value goes out of scope and the `Copy` annotation is added to that type, a compile-time error will occur.

What types are `Copy`? Documentation has this handled fairly well, but generally any group of simple scalar values can be `Copy`, and nothing that requires allocation or is some form of resource is `Copy`. Some types can include:

* All the integer types, such as `u32`
* The Boolean type `bool`
* The character type `char`
* All floating point types, such as `f64`
* Tuples, if they only contain types that are also `Copy`
  * Ex: `(i32, i32)` is `Copy`, but `(i32, String)` is not, since `String` doesn't implement `Copy`

### [Ownership and Functions](#table-of-contents)

### [Return Values and Scope](#table-of-contents)

## [References and Borrowing](#table-of-contents)

### [Mutable References](#table-of-contents)

### [Dangling References](#table-of-contents)

### [The Rules of References](#table-of-contents)

## [The Slice Type](#table-of-contents)

### [String Slices](#table-of-contents)

#### [String Literals Are Slices](#table-of-contents)

#### [String Slices as Parameters](#table-of-contents)

### [Other Slices](#table-of-contents)


##### [back to parent readme](../README.md)