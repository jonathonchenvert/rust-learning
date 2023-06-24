# Chapter 6: Enums and Pattern Matching

NOTE: Contents of this README are abridged; see [main.rs](src/main.rs) for this chapter for code examples and commenting

##### TABLE OF CONTENTS
1. [Defining an Enum](#defining-an-enum)
    * [Enum Values](#enum-values)
    * [The Option Enum and Its Advantages Over Null Values](#the-option-enum-and-its-advantages-over-null-values)
2. [The _match_ Control Flow Construct](#the-match-control-flow-construct)
    * [Patterns That Bind to Values](#patterns-that-bind-to-values)
    * [Matching with `Option<T>`](#matching-with-optiont)
    * [Matches Are Exhaustive](#matches-are-exhaustive)
    * [Catch-all Patterns and the _ Placeholder](#catch-all-patterns-and-the-_-placeholder)
3. [Concise Control Flow with `if let`](#concise-control-flow-with-if-let)
4. [Summary](#summary)

## [Defining an Enum](#table-of-contents)

While structs gave a way of grouping together related fields and data (like a `Rectangle` with `width` and `height`),
enums give a way of saying a value is one of a possible set of values. For example, a `Rectangle` is one of a set of possible shapes, as is a `Triangle`, a `Hexagon`, etc. To do this, Rust allows encoding these possibilities as an enum.

One example of enums can be IP addresses. There are currently (as of `April 7, 2023`) two major standards for IP addresses: IPv4 and IPv6.
Since these are the only possibilities for an IP address, this can be enumerated across all possible variants.

Any IP address can be either a version four or a version six address, but not both at the same time. The property of IP addresses makes the enum data 
structure appropriate because an enum value can only be one of its variants. Both variants are still fundamentally IP addresses, so they should be 
treated as the same type when the code is handling situations that apply to any kind of IP address.

### [Enum Values](#table-of-contents)

An IP address enum can be expressed as follows:
```rust
enum IpAddrKind {
    V4,
    V6,
}
```

One problem with the above enum is that there's no actual way to store IP address data; instead, there's only options for what IP address can be present.
One approach could be implementing the enum in a struct, like below.
```rust
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
```

The same concept can also be represented within an enum, like so:
```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));
```

The data can be attached to each variant of the enum directly, so there's no need for an extra struct. It's also easier to see another detail of how
enums work: the name of each variant that is defined also becomes a function that constructs an instance of the enum. 
That is, `IpAddr::V4()` is a function call that takes a `String` argument and returns an instance of the `IpAddr` type. The constructor function 
is automatically defined as a result of defining the enum.

Another advantage of using an enum rather than a struct is that each variant can have different types and amounts of associated data. IPv4 addresses will
always have four numeric components that have values between 0 and 255. If it's desired to store `V4` addresses as four `u8` values but still express `V6` 
addresses as one `String` value, that wouldn't be possible with a struct.
```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

The standard library has definitions that can be used for storing IP addresses. The standard library defines `IpAddr` with the 
exact enum and variants defined above, but embeds the address data inside the variants in the form of two different structs, as seen below:
```rust
struct Ipv4Addr {
    // snip
}

struct Ipv6Addr {
    // snip
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

This code illustrates that any kind of data can be put inside an enum variant: strings, numeric types, or structs. These can also include
another enum. Standard library types are often not much more complicated than this, or custom creations.

Even though the standard library contains a definition for `IpAddr`, a developer can still create and use their own definition without conflict
because the standard library's definition would not yet have been brought into scope (see chapter 7 for scopes).

Another type of enum can be seen below.
```rust
enum Message {
    Quit,                        // No data associated with it at all
    Move { x: i32, y: i32 },     // Named fields, like a struct would have
    Write(String),               // A single String
    ChangeColor(i32, i32, i32),  // Includes three i32 values
}
```

Defining an enum with variants (such as above) is similar to defining different kinds of struct definitions, except that the enum doesn't 
use the `struct` keyword and all the variants are grouped together under the `Message` type. The following structs could hold the same data 
that the preceding enum variants hold:
```rust
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
```

If other structs were used (with each one being its own type), then defining a function wouldn't be as easy to take any of these types of messages
as can be done with the `Message` enum.

There's one more similarity between enums and structs: just as methods can be defined on structs with `impl`, methods can also be defined for enums.
An example is below for the `Message` enum:
```rust
impl Message {
    fn call(&self) {
        // method body is defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

The body of the method would use `self` to get the value that called the method.

### [The Option Enum and Its Advantages Over Null Values](#table-of-contents)

`Option` is another enum defined by the standard library. The `Option` type encodes the very common scenario in which
a value could be something or it could be nothing.

For example, if the first item of a non-empty list is requested, a value would be obtained. If requesting the first
item in an empty list, nothing would be obtained. Expressing this concept in terms of the type system means the compiler
can check whether all the cases that should be handled have been handled; this functionality can prevent bugs that are
extremely common in other programming languages.

Programming language design is often thought of in terms of which features are included, but the features that are excluded
are just as important. Rust doesn't have the null feature that other languages have. _Null_ is a value that means there is 
no value there. In languages with null, variables can be in one of two states: null or not-null.

The problem with null values is that if a null value were to be used as a not-null value, an error of some kind will have been produced.
Because the null or not-null property is pervasive, it's easy to make this kind of error.

The concept that null is trying to express is still useful: a null value is a value that is currently invalid or absent for some reason.

The problem isn't really with the concept but with the particular implementation. As such, Rust does not have nulls, but it does have
an enum that can encode the concept of a value being present or absent. The enum is `Option<T>`, and is defined by the standard library
as follows:
```rust
enum Option<T> {
    None,
    Some(T),
}
```

The `Option<T>` enum is so useful that it's even included in the prelude; it doesn't need to be brought into scope explicitly.
Its variants are also included in the prelude: it can use `Some` and `None` directly without the `Option::` prefix. The `Option<T>`
enum is still just a regular enum, and `Some(T)` and `None` are still variants of type `Option<T>`.

The `<T>` syntax is a feature of rust called a generic type parameter (more in chapter 10). For now, know that `<T>` means that the `Some`
variant of the `Option` enum can hold one piece of data of any type, and that each concrete type that gets used in place of `T` makes the 
overall `Option<T>` type a different type. Some examples are below:
```rust
let some_number = Some(5);:

let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
```

If this is attempted to be run, the error message below appears:
```bash
$ cargo run
   Compiling ch6_enums_and_pattern_matching v0.1.0 (/home/me/development/rust-learning/ch6_enums_and_pattern_matching)
error[E0277]: cannot add `Option<i8>` to `i8`
  --> src/main.rs:30:17
   |
30 |     let sum = x + y;
   |                 ^ no implementation for `i8 + Option<i8>`
   |
   = help: the trait `Add<Option<i8>>` is not implemented for `i8`
   = help: the following other types implement trait `Add<Rhs>`:
             <&'a i8 as Add<i8>>
             <&i8 as Add<&i8>>
             <i8 as Add<&i8>>
             <i8 as Add>

For more information about this error, try `rustc --explain E0277`.
error: could not compile `ch6_enums_and_pattern_matching` due to previous error
```

The error message means that Rust doesn't understand how to add an `i8` and an `Option<i8>` because they're different types.
With a value of a type like `i8` in Rust, the compiler will ensure that a valid value will always be present, and can thus proceed
confidently without having to check for null before using that value. Only when an `Option<i8>` (or any other type being worked with)
is present would the worry about possibly not having a value be present, and the compiler will make sure that case is handled before using
the value.

In other words, `Option<T>` has to be converted to a `T` before performing `T` operations with it. Generally, this helps catch one of the most
common issues with null: assuming that something isn't null when it actually is.

Eliminating the risk of incorrectly assuming a not-null value helps to be more confident in the written code. In order to have a value
that can possibly be null, it must be explicitly opted in by making the type of that value `Option<T>`. Then, when that value is used, it's 
required to explicitly handle the case when the value is null. Everywhere that a value has a type that isn't an `Option<T>` _can_ safely be assumed
that the value isn't null. This was a deliberate design decision for Rust to limit null's pervasiveness and increase the safety of Rust code.

How does one get the `T` value out of a `Some` variant from a value of `Option<T>` so that the value can be used? The `Option<T>` enum has a large
number of methods that are useful in a variety of situations that can be checked out in the 
[enum Option documentation](https://doc.rust-lang.org/stable/std/option/enum.Option.html). 
Becoming familiar with the methods on `Option<T>` will be extremely useful in working with Rust.

In general, in order to use an `Option<T>` value, there will need to be some code that will handle each variant. Some code will be needed that
will run only when there's a `Some(T)` value, and the code is allowed to use the inner `T`. Some other code will also be needed for a `None` value,
and that code doesn't have a `T` value available. The `match` expression is a control flow construct that does just this when used with enums: it
will run different code depending on which variant of the enum it has, and that code can use the data inside the matching value.

## [The _match_ Control Flow Construct](#table-of-contents)

Rust has a powerful control flow construct called `match` that allows for comparing a value against a series of patterns and then
execute code based on which pattern matches. Patterns can be made up of literal values, variable names, wildcards, and many other things 
(chapter 18 covers all different kinds of patterns and what they do). The power of `match` comes from the expressiveness of the patterns 
and the fact that the compiler confirms that all possible cases are handled.

Think of a `match` expression as being like a coin-sorting machine: coins slide down a track with variously sized holes along it, and
each coin falls through the first hole it encounters that it fits into. In the same way, values go through each pattern in a `match`, and
at the first pattern the value "fits", the value falls into the associated code block to be used during execution.

Below is a coin-based example, with a function that takes an unknown US coin and determines which coin it is and returns its value in cents.
```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

The `match` keyword is followed by an expression (in this case, the `coin` value). This looks similar to an `if` statement, but there's a difference:
with `if`, the condition needs to evaluate to a Boolean value, but here it can be any type. The type of `coin` in this example is the `Coin` enum.

Next is the `match` arms, which has two parts: a pattern and some code. The first arm has a pattern that is the value of `Coin::Penny`, and the `=>` 
operator that separates the pattern and the code to run. In this case, the code is just the value 1. Each arm is separated from the next with a comma.

When the `match` expression executes, it compares the resultant value against the pattern of each arm in order. If a pattern matches the value, the code
associated with that pattern is executed. If it doesn't match, execution continues to the next arm. `match` expressions can have as many values as possible.

The code associated with each arm is an expression, and the resultant value of the expression in the matching arm is the value that gets returned for the
entire `match` expression.

Curly brackets aren't typically used if the `match` arm code is short. If it's desired to run multiple lines of code in a `match` arm, curly brackets
(and the comma following the arm) is then optional, like so:
```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

### [Patterns That Bind to Values](#table-of-contents)

Another useful feature of `match` arms is that they can bind to the parts of the values that match the pattern. This is how
values can be extracted out of enum variants.

For example, from 1999-2008, the US minted quarters with different designs for each of the 50 states on one side. 
No other coins got state designs, so only quarters have this extra value. This information can be added to the `enum` by
changing the `Quarter` variant to include a `UsState` value stored inside it. An example of this is below. 
```rust
#[derive(Debug)] // so the state can be inspected
enum UsState {
    Alabama,
    Alaska,
    // etc.
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
```

If someone were collecting all 50 state quarters, the `match` expression could be used to call out the name of the state associated with
each quarter so that if it's one that isn't known to that person, it can be added to the collection. The `match` expression can be 
retrofitted for this, as such.
```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
```

Calling `value_in_cents(Coin::Quarter(UsState::Alaska));` would yield the `coin` to be `Coin::Quarter(UsState::Alaska)`. The binding for `state`
in the `Coin::Quarter` in the `match` expression would yield the value `UsState::Alaska`, then use the binding in the `println!` expression, which
gets the inner state value out of the `Coin` enum variant for `Quarter`.

### [Matching with `Option<T>`](#table-of-contents)

`Option<T>` can also be handled using `match`, as was done with the `Coin` enum. Instead of comparing coins, the variants of `Option<T>` are
compared, but the `match` expression works remain the same.

For instance, a function that takes an `Option<i32>` can add 1 to the value if it's present. If there isn't a value inside, the function
should return the `None` value and not attempt to perform any operations.
```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

When calling `plus_one(five)`, the value `x` in the body of `plus_one` will have the value `Some(5)`. It's then compared against each `match` arm.
The `i` binds to the value contained in `Some`, so `i` takes the value `5`. 1 is then added to the value of `i` and creates a new `Some` value with
`6` inside.

Combining `match` and enums is useful in many situations. This pattern is seen a lot in Rust code: `match` against an `enum`, bind a variable to
the data inside, and then execute code based on it. It's a bit tricky at first, but developers will eventually get used to.

### [Matches Are Exhaustive](#table-of-contents)

There's one other aspect of `match`: the arms' patterns must cover all possibilities. Consider this version of the previous `plus_one` function,
which has a bug and won't compile:
```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}
```

The `None` case here isn't handled, so this code will cause a bug. Luckily, it's a bug Rust knows how to catch. If attempting to compile this code,
the following error will result:
```bash
$ cargo run
   Compiling enums v0.1.0 (file:///projects/enums)
error[E0004]: non-exhaustive patterns: `None` not covered
 --> src/main.rs:3:15
  |
3 |         match x {
  |               ^ pattern `None` not covered
  |
note: `Option<i32>` defined here
  = note: the matched value is of type `Option<i32>`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
  |
4 ~             Some(i) => Some(i + 1),
5 ~             None => todo!(),
  |

For more information about this error, try `rustc --explain E0004`.
error: could not compile `enums` due to previous error
```

Rust knows that every possible case wasn't covered, and even knows which pattern was forgotten. Matches in Rust are _exhaustive_: every
last possiblility must be exhausted in order for the code to be valid. Especially in the case of `Option<T>`, when Rust prevents from
forgetting to explicitly handle the `None` case. This protects from assuming that a value is present when it's possible to get null.

### [Catch-all Patterns and the _ Placeholder](#table-of-contents)

Using enums, special actions can also be taken for a few particular values, but for all other values take one default action. For example,
imagine a game where rolling a 3 doesn't doesn't move a player, but instead gets a new fancy hat. Rolling a 7 causes the player to lose
the fancy hat, and any other values let the player move that number of spaces on the game board. The `match` code for this logic is below,
with the result of the dice roll hardcoded rather than a random value, and all other logic represented by functions without bodies.
```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
```

For the first two arms, the patterns are the literal values `3` and `7`. The last arm that covers every possible value was chosen to be
named `other`. The code that runs for the `other` arm uses the variable by passing it to the `move_player` function.

The code compiles, even though it hasn't been listed with all the possible values a `u8` can have, because the last pattern will match all 
values not specifically listed. This catch-all pattern meets the requirement that `match` must be exhaustive. Note that the catch-all arm 
must be placed last because the patterns are evaluated in order. If the catch-all arm was placed earlier, the other arms would never run.
Rust will warn if arms are added after a catch-all.

Rust also has a pattern that can be used when using a catch-all statement but the value isn't used in the catch-all pattern: `_` is a special 
pattern that matches any value and does not bind to that value. This tells Rust that the value isn't going to be used, so Rust won't warn
about an unused variable.

Changing the above game example, if anything other than a 3 or 7 is rolled, a reroll must occur. With that, there's no longer a need to
use the catch-all value, so the code can be changed to use `_` instead of the vairable named `other`.
```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}
```

This example also meets the exhaustiveness requirement because all other values in the last arm are explicitly being ignored. Nothing has
been forgotten.

Changing the above game example again, nothing happens on the player's turn if rolling anything other than a 3 or a 7. This can be expressed
by using the unit value (the empty tuple type) as the code that goes with the `_` arm:
```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => (),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
```

Here, Rust is being told explicitly that no other value is going to be used that didn't match a pattern already defined, and that no code
should be run in this case.

There's more about patterns and matching in chapter 18. There is also the `if let` syntax, which can be useful in situations where the `match`
expression can get wordy.

## [Concise Control Flow with `if let`](#table-of-contents)

The `if let` syntax combines `if` and `let` into a less verbose way to handle values that match one pattern while ignoring the rest. Consider
the following code below that matches on an `Option<u8>` value in the `config_max` variable but only wants to execute code if the value
is the `Some` variant:
```rust
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The maximum is configured to be {}", max),
    _ => (),
}
```

If the value is `Some`, the value in the `Some` variant is printed by binding the value to the variable `max` in the pattern. Nothing is done
with the `None` value. Satisfying the `match` expression involves adding the `_ => ()` arm after processing just one variant.

Instead, the above code can be written in a shorter way with `if let`.
```rust
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
}
```

The `if let` syntax takes a pattern and an expression separated by an equal sign. It works the same way as a `match`, where the expression is
given to the `match` and the pattern is its first arm. In this case, the pattern is `Some(max)`, and the `max` binds to the value inside the `Some`.
`max` can then be used in the body of the `if let` block in the same way it's used in the `match` arm. The `if let` code block isn't run if the value
doesn't match the pattern.

Using `if let` means less typing, less indentation, and less boilerplate code. However, this loses out on the exhaustive checking that `match` enforces.
Choosing between `match` and `if let` depends on what's being done in code and whether gaining conciseness is an appropriate trade-off for losing exhaustive
checking. The `if let` syntax can be thought of as a simpler (and cleaner) version of `match` that runs code when the value matches one pattern and then
ignores all other values.

An `else` can also be included with an `if let` statement. The block of code that goes with the `else` is the same as the block of code that would go with
the `_` case in the `match` expression that is equivalent to the `if let` and `else`. From the previous `Coin` enum example above, the `Quarter` variant also
held a `UsState` value. If it was desired to coint all non-quarter coins that were seen while also announcing the state of the quarters, that can be done with
a `match` expression, like so:
```rust
let mut count = 0; 
match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count += 1,
}
```

Or an `if let` and `else` expression can be written for the same scenario above.
```rust
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```

If a situation calls for a program that has logic too verbose to express using a `match`, `if let` is a useful substitute for that.

## [Summary](#table-of-contents)

Enums were covered to create custom types that can be one of a set of enumerated values. The standard library's `Option<T>` type helps use
the type system to prevent errors. When enum values have data inside them, `match` or `if let` can be used to extract and use those values, 
depending on the number of cases needed to be handled.

##### [back to parent readme](../README.md)