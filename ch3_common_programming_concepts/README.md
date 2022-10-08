# Chapter 3: Common Programming Concepts

##### TABLE OF CONTENTS
1. [Variables](#variables)
2. [Data Types](#data-types)
3. [Functions](#functions)
4. [Comments](#comments)
5. [Control Flow](#control-flow)
6. [Repetition with Loops](#repetition-with-loops)

#### [code](src/main.rs)

## [Variables](#table-of-contents)
By default, variables in Rust are immutable (unchanging). You _can_ make Rust variables mutable, and that can be done by the following code:
```rust
let mut x = 5;
```

### Constants
Like immutable variables, constants are variables that have values that are not allowed to change. Constants are declared with the `const` keyword instead of `let`, and the type value _must_ be annotated.

Constants can be declared in any scope (including global). Constants may be set only to a constant expression, not to the result of a function call or any other value that has to be computed.

For instance, in a Python 3 script, you could have something like the following:
```python
import requests

API_SERVER = "https://pokeapi.co/"

# Do whatever API calls below, but API_SERVER can never change
```

In Rust, this would look something like this:
```rust
const MAX_POINTS: u32 = 100_000;
```

Note that, in Rust, the naming convention is all uppercase with underscores between words for the constant name, and underscores for numeric literals to improve readability.

### Shadowing

A new variable can be declared with the same name as a previous variable, and the new variable shadows the previous variable. 

> The first variable is _shadowed_ by the second. - Rustaceans

This means that the second variable's value is what appears when the variable is used. Shadowing a variable can be done by using the same variable's name and repeating the use of the `let` keyword.
```rust
let x = 5;
let x = 6;

println!("The value of x is: {}", x);

/* Output
The value of x is: 6
*/ 
```

Shadowing is different from making a variable mutable (changing) because a compile-time error is done when accidentally reassigning an immutable variable without the `let` keyword. Another difference between the two is that, with shadowing, a "new variable" is created when using `let`, which can allow a variable to change its type while reusing the same name.

## [Data Types](#table-of-contents)

All values in Rust has some sort of data type. Two data type subsets for Rust exists: scalar and compound. Rust is a _statically typed_ language, which means that it has to know all variable types at compile time.

The compiler is usually pretty good in inferring what types are being used for some variables. In cases where different types are possible (such as user input), a type annotation will need to be added. 
```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

Without the type annotation, Rust will have an error when it comes time to compile.

### Scalar Types

A _scalar_ type represents a single value. Rust has the following scalar types:

* integers
* floating-point numbers
* booleans
* characters

#### Integer Types

An _integer_ is a number without a fractional component (2, 22, 222, etc.). Rust has many different integer types, which can be seen in the table below. By default, Rust uses `i32` for integer types.

|Length|Signed|Unsigned|
|------|------|--------|
|8-bit|i8|u8|
|16-bit|i16|u16|
|32-bit|i32|u32|
|64-bit|i64|u64|
|128-bit|i128|u128|
|arch|isize|usize|

One example of an integer type is `u32`. This indicates that the value it's associated with should be an unsigned integer that takes up 32 bits of space.

* _Signed_ integers are integers with signs prefixed to the integer. Put another way, these are positive and negative integers (i.e. `-1` or `+64`).
  * _Signed_ integers are stored using two's complement representation (??? gotta look this up)
  * Each signed variant can store numbers from $-(2^{n-1})$ to $2^{n-1}-1$ inclusive, where _n_ is the number of bits that variant uses. 
    * Example: An `i8` integer variable can store numbers between $-(2^{8-1})$ to $2^{8-1}$, which equals the range [-128, 127].
* _Unsigned_ integers are integers that will only be positive integers and be represented without a sign.
  * Unsigned variants can store numbers from 0 to $2^n -1$.
    * Example: A `u8` integer variable can store numbers between 0 to $2^8-1$, which equals the range [0, 255].

The `isize` and `usize` types depend on the kind of computer architecture the program runs on; 64 bits on a 64-bit architecture and 32 bits on a 32-bit architecture, etc.

Integer literals can be written in any of the forms shown below. Note that all number literals (except for _byte_) alows a type suffix, such as `57u8`, and a `_` as a visual separator (i.e. `1_000` for 1,000).

|Number Literals|Example|
|---------------|-------|
|Decimal|`98_222`|
|Hex|`0xff`|
|Octal|`0o77`|
|Binary|`0b1111_0000`|
|Byte (`u8` only)|`b'A'`|

#### Floating-Point Types

The _floating-point numbers_ are numbers with decimal points. They are represented according to the IEEE-754 standard. Rust has two primitive types for _floating-point numbers_:
* `f32` (32 bits; single-precision float)
* `f64` (64 bits; double precision float)

The default type is `f64`, with roughly the same speed as `f32` on modern CPUs but being capable of more precision. An example of _floating-point numbers_ is shown in the code block below.
```rust
let x = 2.0; // f64
let y: f32 = 2.0 // f32
```

#### Numeric Operations

Rust has the usual basic math operations that are expected.
```rust
let sum = 5 + 10;              // addition
let difference = 95.5 - 4.3;  // subtraction
let product = 4 * 30;        // multiplication
let quotient = 56.7 / 32.2; // division
let remainder = 13 % 7;    // modulus
```

#### Boolean

Booleans are one byte in size, and are specified in Rust with `bool`. 
```rust
let t = true;
let f: bool = false; // explicit type annotation
```

#### Characters

The `char` type in Rust is the language's most primitive alphabetic type (with char literals being specified with single quotes, where strings are specified with double quotes), and the following code shows one way to use it:
```rust
let c = 'z';
let z = 'ƶ';
```

The `char` type is 4 bytes in size and represents a Unicode Scalar Value (meaning it can represent a lot more than just ASCII). For instance, it can represent accented letters, Chinese, Japanese, Korean, emoji, and zero-width spaces. Unicode Scalar Values range from _U+0000_ to _U+D7FF_ and _U+E000_ to _U+10FFF_ inclusive.

### Compound Types

_Compound Types_ can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

#### Tuples

A tuple is a general way of grouping together some number of other values with a variety of types into one compound type. Tuples have a fixed length, and cannot grow or shrink in size once they have been declared.

A tuple can be created in Rust by writing a comma-separated list of values inside parentheses. Each position in the tuple has a type, and the types of different values in the tuple don't have to be the same. 

Tuples can be declared, and can have individual values fetched out of a tuple, with the following code:
```rust
// tuple declaration
let tup: (i32, f64, u8) = (500, 6.4, 1);

// pattern match to destructure tuple
let (x, y, z) = tup;

println!("The value of y is: {}", y);
```

Additionally, a tuple can be accessed directly by using a period followed by the index of the value to access (tuples start at the first index of 0). Code for this is shown below.
```rust
let x: (i32, f64, u8) = (500, 6.4, 1);

let five_hundred = x.0;
let six_point_four = x.1;
let one = x.2;
```

#### Arrays

Unlike a tuple, every element of an array must have the same type. Arrays in Rust are different from arrays in other languages because they have a fixed length, like tuples. Arrays can be defined in Rust like so:
```rust
let a = [1, 2, 3, 4, 5];
```

Arrays are useful for allocating data on the stack rather than the heap or for ensuring a fixed number of elements is set. 

An array isn't as flexible as the vector type, however. A vector is a similar collection type provided by the standard library that _is_ allowed to grow or shrink in size. If unsure between using an array or using a vector, use a vector.

An example of using an array over a vector would be for a program that needs to know the names of the months of the year, as it's unlikely that a new month would be spontaneously added.

An array's type can be written by using square brackets, and within the brackets include the type of each element, a semicolon, and then the number of elements in the array (i.e.):
```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

To create an array that contains the same value for each element, the initial value in question can be specified, followed by a semicolon, and then the length of the array in square brackets.
```rust
let a = [3; 5]; // 5 elements; all initially set to 3
```

##### Accessing Array Elements
An array is a single chunk of memory allocated on the stack. Elements of an array can be accessed through its index, like so:
```rust
let a = [1, 2, 3, 4, 5];

let first = a[0]; // first == 1
let second = a[1]; // second == 2
```

##### Invalid Array Element Access

If accessing an element of an array where the index is past the length of the array, Rust will compile the code but exit with an error when it runs.  An example of this is below.
```rust
let a = [1, 2, 3, 4, 5];
let index = 10;

let element = a[index];

println!("The value of element is: {}", element);
```

When attempting to access an element using indexing, Rust will check that the index specified is less than the array length. If the index is greater than or equal to the length, Rust will panic.

This example is one of Rust's safety principles in action. In many low-level languages, this check isn't done; usually when providing an incorrect index, invalid memory can be accessed. Rust protects against this kind of error by immediately exiting instead of allowing the memory access and continuing.

## [Functions](#table-of-contents)

For writing functions, Rust uses _snake case_ for function and variable names (all letters are lowercase, underscores between words). 

Rust defines functions with the `fn` keyword. Defining a basic function is similar to other programming languages:
* Language-specific function keyword
* Name of function
* Parentheses (defines if it takes attributes or not)
* Curly brackets to define code within the function

Rust doesn't care where the functions are defined (before or after the `main()` function) so long as they are defined and can be seen by the caller.

### Functions with paremeters

Functions can have parameters, which are values passed to a function to perform some logic to it. The parameters (aka arguments) *must* be defined to have a specific type. Take this function, for example:
```rust
fn main() {
    another_function(7);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
```

In this example, `another_function()` has a parameter `x` that is of type `i32`. Passing `7` to the function means that the `println!` statement will print the value that was passed to it.

Parameters requiring type declarations is a deliberate decision for Rust's design: requiring type annotations in function arguments means the compiler almost never needs to use them elsewhere in the code to figure out what type is needed for the function to perform any logic changes. Additionally, the compiler will also help give more useful and helpful error messages when knowing the types the function expects.

For functions with multiple parameters, the parameter declarations can be separated with commas, like so:
```rust
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{char}");
}
```

### Statements and Expressions

Function bodies are made up of many statements that optionally end in an expression. Since Rust is an expression-based language, it is important to note the distinction that expressions can be part of a statement. Other languages don't have the same distinction.

_Statements_ are instructions that perform some action and do not return a value.

_Expressions_ evaludate to a resulting value.

Below is a block of code that defines statements, as well as misusing statements.
```rust
let y = 6; // Statement

// 1. Will return an error!
let x = (let z = 6);
```

Statements don't return values. Because of this, a `let` statement can't be assigned to another variable (see `1.` above for an example). The `let z = 6` statement doesn't return a value, which means `x` can't bind to anything. In other languages (C, Ruby, Python3, Swift, etc.), The assignment can return the value of the assignment. For those languages, something like `x = y = 6` can work, assigning `6` to both `x` and `y`, but that can't be done in Rust.

Evaluations, on the other hand, can evaluate to a value and make up a lot of the rest of the code that normally gets written in Rust. For example, `6 + 3` evaluates to `9`. Something like this can be part of a statement. Calling a function, macro, or a new scope block created with curly brackets can be considered expressions. Here's an expression example:
```rust
fn main() {
    let y = { // block that evaluates to 7
        let x = 5;
        x + 2 // note there's no ';' character
    };

    println!("The value of y is: {y}");
}
```

**NOTE:** The block of code above that evaulates to 7 gets bound to `y` as part of the `let` statement. Also note that `x + 2` does **not** have a semicolon at the end. Expressions do not include ending semicolons. When a semicolon is added to the end of an expression, that turns it into a statement, which will **not** return a value. 

TL;DR: Semicolon? Statement. No semicolon? Expression.

### Functions with Return Values

Just like any other programming language, Rust functions can return values to code that calls them. Return values aren't named, but their types are declared after the `->` arrow syntax. Rust's return value of the function is synonymous with the value of the final expression in the block of the body of a function (i.e. usually at the very end of the function). A function can be returned early with the `return` keyword and a specified value, but most functions return the last expression implicitly.
```rust
fn five() -> i32 {
    5 // again, note that there is no semicolon there
}
```

In the `five()` function, there are no function calls, macros, or `let` statements; just the number `5` without a semicolon. This is a valid function in Rust.

## [Comments](#table-of-contents)

Comments in Rust (not documentation comments) are very simple. They can be done with the `//` characters, and can be used as shown below:
```rust
// hello, world. this is a small comment - general knoxx

// This is
// a 'multiline'
// comment

// Comments, like below, can go at the end of the line
fn main() {
    let lucky_number = 7; // I'm feeling lucky!
}

// Comments can also be placed on the line above the code. This formatting is more often seen.
fn main() {
    // I'm feeling lucky!
    let lucky_number = 7;
}
```

## [Control Flow](#table-of-contents)

If, else, it's been seen across many different programming languages. It basically helps determine if some code should be run or not, either in a loop or not at all.

### `if` Expressions

The `if` expression allows code execution depending on certain criteria. This usually entails some sort of condition to be met before the block of code contained in the `if` expression is executed. If the condition is not met, then the block of code isn't executed. For instance, the following block of code can be used to handle certain conditions:

```rust
fn main() {
    let number = 4;

    if number < 5 {
        println!("Well, the condition is true!");
    } else {
        println!("The condition was false. Oops.");
    }
}
```

The condition for control flow expressions _must_ be a `bool`. If it isn't, an error will be returned. For example, the following line of code will result in an error:
```rust
fn main() {
    let number = 3;

    if number {
        println!("number = 3");
    }
}
```

The `if` condition evaluates to `3`, which will prompt Rust to throw an error. Ruby, JavaScript, Python3, and Swift[(?) can't recall correctly on this one] will automatically convert non-Boolean types to a Boolean. In Rust, this doesn't happen. Explicitness is important, and `if` statements should always be provided a Boolean as its condition. 

### Handling multiple conditions with `else if`

Multiple conditions can be handled with `if`, `else if`, and `else` expressions.

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("divisible by 4");
    } else if number % 3 == 0 {
        println!("divisible by 3");
    } else if number % 2 == 0 {
        println!("divisible by 2");
    } else {
        println!("maybe it's a prime number? prime rib is good");
    }
}
```

When the program executes, it checks each `if` expression in turn and executes the first body for which the condition holds true. In other words, it goes through each `if` statement individually and stops when it finds the `if` expression that holds true for the Boolean value. Rust won't bother to check the rest of the statements.

If there are too many `else if` expressions, that can clutter the code. If there ends up being more than one, it's a good idea to refactor the code with `match` statements.

### Using `if` in a `let` statement

Since `if` is an expression, it can be used on the right side of a `let` statement to assign the outcome to a variable.

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 }; // Recall that expressions are in curly bracket blocks

    println!("The value of number is {number}");
}
```

The `number` variable will be bound to a value based on the `if` expression outcome. Blocks of code will evaluate to the last expression in them, and numbers by themselves are expressions. The values for both potential results _must_ be the same type. If the types are mismatched, an error will occur:
```rust
fn main() {
    let condition = true;

    let number = if condition { 5 } else { "six" };

    println!("The value of number is {number}");
}
```
```bash
# If that gets executed, the following error occurs
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
error[E0308]: `if` and `else` have incompatible types
 --> src/main.rs:4:44
  |
4 |     let number = if condition { 5 } else { "six" };
  |                                 -          ^^^^^ expected integer, found `&str`
  |                                 |
  |                                 expected because of this

For more information about this error, try `rustc --explain E0308`.
error: could not compile `branches` due to previous error
```

The expression in the `if` block evaluates to an integer, but the `else` block expression evaluates to a string. This doesn't work because variables must have a single type, and Rust needs to know at compile time what type the `number` variable is. Knowing the type of `number` lets the compiler verify the type is valid everywhere in the code that `number` is used. Rust can't do this if `number` is determined only at runtime; the compiler would be more complex and make fewer guarantees about the code if it had to figure out multiple hypothetical types for a variable.

## [Repetition with loops](#table-of-contents)

Loops. When repeating the same thing over and over again is necessary. 

Rust has three kinds of loops: `loop`, `while`, and `for`.

### Repeating code with `loop`

The `loop` keyword tells Rust to execute a block of code basically forever or until it is explicitly told to stop.

```rust
fn main() {
    loop {
        println!("again!"); // CTRL-C on the command line to interrupt the program stuck in a continuous loop
    }
}
```

Rust also provides a way to break out of a loop with code with the `break` keyword. `continue` can also be used in a loop to tell the proram to skip over any remaining code in the loop and go to the next iteration of the loop.

### Returning values from loops

One of the uses of `loop` is to retry an operation that is known to fail, such as checking whether a thread has completed its job. It's possibnle to pass the result of the operation out of the loop to the rest of the code. This can be done like so:
```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // In this case, break will yield the result of `20` to the `result` variable
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
```

### Loop labels to disambiguate between multiple loops

With loops in loops, `break` and `continue` apply to the innermost loop at that point. A `loop label` can be specified on a loop that can be used with `break` or `continue` to specify that the keywords apply to the labeled loop instead of the innermost loop. Loop labels must begin with a single quote.
```rust
fn main() {
    let mut count = 0;

    'counting_up: loop { // outer loop with label
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            
            if remaining == 9 {
                break; // break from inner loop
            }

            if count == 2 {
                break 'counting_up; // breaks the outer loop that's labeled
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");
}
```

The outer loop has the label `'counting_up`, and counts up from 0 to 2. The inner loop without a label counts down from 10 to 9. The first `break` that doesn't specify a label will exit the inner loop only. It's the `break 'counting_up` statement that exits the outer loop.

### Conditional loops with `while`

Often, a program will need to loop under a certain condition. While the condition holds true, the loop runs. When the condition is no longer true, the program calls `break` to stop the loop. This _could_ technically be done with `loop`, `if`, `else`, and `break`, but `while` loops are more commonplace to use.
```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

### Looping through a collection with `for`

The `while` construct _can_ be used to loop over the elements of a collection such as an array, but more often than not this will be made easier with the `for` loop construct. Below is an example with `while`:
```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);

        index += 1;
    }
}
```

The `while` approach can be error prone; the program could end up panicing if the index value or test conditions are incorrect. For instance if the `a` array had 4 elements but still checks for 5, the code panics. It would also be slow, because the compiler adds runtime code to perform the conditional check of whether the index is within the bounds of the array on every iteration through the loop.

A `for` loop can be a more concise alternative to use. It executes code for each item in a collection regardless of the collection size, which ultimately increases the safety of the code and eliminates the chance of bugs that might result from going beyond the end of the array or not far enough into an array.
```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```

The safety and conciseness of `for` loops make them the most commonly used loop construct in Rust. Even in situations in which running code a certain number of time is necessary (such as the countdown from the `while` loop earlier), most Rustaceans end up using a `for` loop. The way that this can be done is with the `Range` keyword (provided by the standard library), which generates all numbers in sequence starting from one number and ending before another number.
```rust
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
```

##### [back to parent readme](../README.md)