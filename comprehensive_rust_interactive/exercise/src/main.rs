fn main() {
    // Day 1: Morning

    println!("Array Assignments:");
    array_assign();

    println!("Tuple Assignments:");
    tuple_assign();

    println!("Reference Reassignment");
    references();

    println!("Slices:");
    slices();

    println!("String vs str:");
    string_vs_str();

    println!("Function examples: Fizzbuzz");
    fizzbuzz_to(20);

    println!("Methods:");
    methods();

    println!("Function Overloading (not supported):");
    function_overloading();

    println!("Exercise 1: Implicit Conversions:");
    implicit_conversion();

    println!("Exercise 2: Arrays and for Loops");
    matrix_modifications();
}

/******* DAY 1: MORNING *******/

/* Compound Types */

fn array_assign() {
    let mut a: [i8; 10] = [42; 10]; // Values are of type i8, semicolon represents 10 of the same values
    a[5] = 0;
    println!("a: {:?}", a);
}

fn tuple_assign() {
    let t: (i8, bool) = (7, true);
    println!("1st index: {}", t.0);
    println!("2nd index: {}", t.1);
}

/* References

Rust has references, like C++. Some differences between the two are:
* `ref_x` must be dereferenced when assigning to it
* In some cases, Rust will auto-dereference (i.e. when invoking methods)
*/

fn references() {
    let mut x: i32 = 10;
    let ref_x: &mut i32 = &mut x;
    *ref_x = 20; // dereference needed when assigning to it
    println!("x: {x}");
}

/*
The following dangling_references function compiles the following error:
**************************************************************************************************
➜  exercise git:(main) ✗ cargo build
   Compiling exercise v0.1.0 (/Users/XYZ/Development/rust/rust-learning/comprehensive_rust_interactive/exercise)
error[E0597]: `x` does not live long enough
  --> src/main.rs:35:17
   |
35 |         ref_x = &x;
   |                 ^^ borrowed value does not live long enough
36 |     }
   |     - `x` dropped here while still borrowed
37 |     println!("ref_x: {ref_x}");
   |                       ----- borrow later used here

For more information about this error, try `rustc --explain E0597`.
error: could not compile `exercise` due to previous error
**************************************************************************************************
➜  exercise git:(main) ✗ rustc --explain E0597
This error occurs because a value was dropped while it was still borrowed.

Erroneous code example:

```
struct Foo<'a> {
    x: Option<&'a u32>,
}

let mut x = Foo { x: None };
{
    let y = 0;
    x.x = Some(&y); // error: `y` does not live long enough
}
println!("{:?}", x.x);
```

Here, `y` is dropped at the end of the inner scope, but it is borrowed by
`x` until the `println`. To fix the previous example, just remove the scope
so that `y` isn't dropped until after the println

```
struct Foo<'a> {
    x: Option<&'a u32>,
}

let mut x = Foo { x: None };

let y = 0;
x.x = Some(&y);

println!("{:?}", x.x);
```
*/
// fn dangling_references() {
//     let ref_x: &i32;
//     {
//         let x: i32 = 10;
//         ref_x = &x;
//     }
//     println!("ref_x: {ref_x}");
// }

/* Slices

Slices give a view into a larger collection
* Slices borrow data from the sliced type
* Question: What happens if a[3] is modified?
* Answer: `a` is an immutable array, and would not allow any of its values to be altered after initialization.
* Answer: To change this behavior, `a` would have to be assigned as follows: `let mut a: `

Two string types in Rust: String, and str.
* str -----> immutable reference to a string slice
* String --> mutable string buffer
*/

fn slices() {
    let a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");

    let s: &[i32] = &a[2..4]; // Slices borrow data from sliced type
    println!("s: {s:?}");
}

fn string_vs_str() {
    let s1: &str = "Hello"; // &str -> immutable reference to a string slice
    println!("s1: {s1}");

    let mut s2: String = String::from("Hello "); // String -> mutable string buffer
    println!("s2: {s2}");
    s2.push_str(s1);
    println!("s2: {s2}");
}

/* Functions

Rust implementation of the famous Fizzbuzz interview question
*/

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false; // Corner case, early return
    }
    lhs % rhs == 0 // Last expression is the return value
}

fn fizzbuzz(n: u32) -> () { // No return value means returning the unit type '()'
    match (is_divisible_by(n, 3), is_divisible_by(n, 5)) {
        (true,   true) => println!("fizzbuzz"),
        (true,  false) => println!("fizz"),
        (false,  true) => println!("buzz"),
        (false, false) => println!("{n}"),
    }
}

fn fizzbuzz_to(n: u32) { // `-> ()` is normally omitted
    for n in 1..=n {
        fizzbuzz(n);
    }
}

/* Methods

Methods are functions that are associated with a particular type.

First argument of a method is an instance of the type it is associated with.
*/

fn methods() {
    let mut rect = Rectangle { width: 10, height: 5 };
    println!("old area: {}", rect.area());
    rect.inc_width(5);
    println!("new area: {}", rect.area());
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn inc_width(&mut self, delta: u32) {
        self.width += delta;
    }
}

/* Function Overloading

Overloading is not supported.

* Each function has a single implementation
  * Always takes a fixed number of parameters
  * Always takes a single set of parameter types
* Default values are not supported:
  * All call sites have the same number of arguments
  * Macros are sometimes used as an alternative

Function parameters can be generic, however.
*/
fn pick_one<T>(a: T, b: T) -> T {
    if std::process::id() % 2 == 0 { a } else { b }
}

fn function_overloading() {
    println!("coin toss: {}", pick_one("heads", "tails"));
    println!("cash prize: {}", pick_one(500, 1000));
}

/* Implicit Conversions

Rust will not automatically apply implicit conversions between types (unlike C++). You can see this in a program like this:

fn multiply(x: i16, y: i16) -> i16 {
    x * y
}

fn main() {
    let x: i8 = 15;
    let y: i16 = 1000;

    println!("{x} * {y} = {}", multiply(x, y));
}

The Rust integer types all implement the From<T> and Into<T> traits to let us convert between them. 
The From<T> trait has a single from() method and similarly, the Into<T> trait has a single into() method. 
Implementing these traits is how a type expresses that it can be converted into another type.

The standard library has an implementation of From<i8> for i16, which means that we can convert a variable x of type i8 to an i16 by calling i16::from(x). 
Or, simpler, with x.into(), because From<i8> for i16 implementation automatically create an implementation of Into<i16> for i8.
1. Execute the above program and look at the compiler error.
2. Update the code above to use into() to do the conversion.
3. Change the types of x and y to other things (such as f32, bool, i128) to see which types you can convert to which other types. 
   Try converting small types to big types and the other way around. Check the standard library documentation to see if From<T> is implemented for the pairs you check.
   Standard library documentation: https://doc.rust-lang.org/std/convert/trait.From.html
*/
fn multiply(x: i16, y: i16) -> i16 {
    x * y // no semicolon at the end returns the value, instead of using the `return` keyword in other languages
}

fn implicit_conversion() {
    let x: i8 = 15;
    let y: i16 = 1000;

    println!("{x} * {y} = {}", multiply(x.into(), y.into()));
}

/* Arrays and for Loops
*/

fn matrix_modifications() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];
    
    println!("original matrix: {:?}", matrix);

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transposed: [[i32; 3]; 3] = matrix.clone();
    
    for i in 0..3 {
        for j in 0..3 {
            transposed[i][j] = matrix[j][i];
        }
    }
    
    transposed // return
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for i in matrix {
        println!(" {i:?}");
    }
}