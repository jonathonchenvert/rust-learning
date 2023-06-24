// Tuple structs without named fields
// Useful when giving the whole tuple a name and
// making a different type from other tuples
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

/* Unit-like Structs
Behave similarly to `()`, the unit type of Tuples

Useful when implementing a trait on some type but don't have any data 
to be stored in the type itself (more on traits later).
*/
struct AlwaysEqual;

struct User {
    active: bool, // struct field
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)] // Without this, println!("{:?}"); won't work (specifically, :?)
struct Rectangle {
    width: u32,
    height: u32,
}

/* Associated Functions

Methods, defined specifically for the Rectangle struct
Methods are defined within the context of a struct, enum, or trait
everything in the impl block will be associated with the Rectangle struct

Getter methods are not implemented automatically for structs like other languages might do
Getters can be useful for making fields private but methods public (read-only) (i.e. width())

All functions in `impl` blocks are called associated functions because they're associated with the type
named after the `impl`. Associated functions can be defined that don't have a `self` as the first parameter
(and thus are not methods) because they don't need an instance of the type to work with. One function that
behaves like this is `String::from`, defined on the `String` type.

Associated functions that aren't methods are often used for constructors that will return a new
instance of the struct. They're often called `new`, but `new` isn't a special name and isn't built into 
the language. For example, an associated function can be provided named `square` that would have a one dimension
parameter and use that as both width and height, thus making it easier to create a square `Rectangle`, rather than
specifying the same value twice.
*/
impl Rectangle {
    // Methods can take ownership of self, borrow self immutably or mutably
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Can be the same name as one of the struct's fields
    fn width(&self) -> bool {
        self.width > 0
    }

    fn square(size: u32) -> Self {
        // `Self` is an alias for the 'Rectangle' type, in this case (written after impl)
        // Calling this would use the `::` syntax with the struct name
        // i.e. let sq = Rectangle::square(3);
        Self {
            width: size,
            height: size,
        }
    }
}

/* Multiple impl Blocks

Each struct is allowed to have multiple impl blocks.

There's no reason to separate these methods into multiple impl blocks here, but this is valid syntax.
Multiple impl blocks can be useful (see chapter 10 on generic types and traits).

*/
impl Rectangle {
    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.width > rectangle.width && self.height > rectangle.height
    }
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// borrow struct instead of taking it
// This is a function, not a method
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    // Previous call to main was testing the waters for Structs
    // This main focuses on an example of using Structs in a real scenario
    // i.e. area of a rectangle
    // was_main();

    // let rect1 = (30, 50);
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3: Rectangle = Rectangle {
        width: 60,
        height: 45,
    };

    // The :? inside the curly brackets tells println! to use an output format called Debug
    // Useful for printing out structs in a way that's useful for developers
    // :#? is prettified debug output
    println!("rect1 is:\n{:#?}", rect1); // won't print unless std::fmt::Display is implemented for Rectangle

    // let width1 = 30;
    // let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        // area(width1, height1)
        // area(&rect1)
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width of {}", rect1.width);
    }

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // Additionally, dbg! macro is another debug print (stderr, not stdout) 
    // Also takes ownership of an expression, prints the file and line number 
    // where the macro call comes in code
    // along with the value, and returns ownership

    let scale = 2;
    let rect9 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect9);

}

fn was_main() {
    let user1 = User {
        active: true,
        username: String::from("hiuser"),
        email: String::from("hiuser@example.com"),
        sign_in_count: 3,
    };
    let mut user2 = User {
        active: true,
        username: String::from("hiuser2"),
        email: String::from("hiuser2@example.com"),
        sign_in_count: 3,
    };
    user2.email = String::from("someotheremail@example.com");

    let user3 = build_user(String::from("hi2@gmail.com"), "user3".to_string());
    println!("Hello, world! This is user {:?}", user1.username);

    // Create a new user, but still has the same values as user2
    // let user4 = User {
    //     active: user2.active,
    //     username: user2.username,
    //     email: String::from("user4@example.com"),
    //     sign_in_count: user2.sign_in_count
    // };

    // Struct update syntax
    // Less code to effectively 'clone' the user created, with 
    // the only differing value being the email

    // NOTE: This breaks if user4 is uncommented out, as user4 effectively takes the values, rendering user2 useless
    let user5 = User {
        email: String::from("user5@example.com"),
        ..user2
    };

    let black  = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Unit-like struct
    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}


/* `build_user` can also be written as below 

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

This only works if the function parameters are exactly the same as the struct field names
*/

/* Ownership of Struct Data

The uncommented `User` struct in the code above uses the owned `String` type rather than
a &str string slice type. This is because each instance of the struct was made to own all of its data
and for the data to be valid for as long as the struct is valid.

Structs can store references to data owned by something else, but that requires 
the use of lifetimes (more on that in Chapter 10). Lifetimes ensure that the 
data referenced by a struct is valid for as long as the struct is.

The struct implementation below will not work because it does not have lifetimes specified.

struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: "someusername123",
        email: "someone@example.com",
        sign_in_count: 1,
    };
}

Below is what the compiler error will look like.

$ cargo run
   Compiling structs v0.1.0 (file:///projects/structs)
error[E0106]: missing lifetime specifier
 --> src/main.rs:3:15
  |
3 |     username: &str,
  |               ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 ~ struct User<'a> {
2 |     active: bool,
3 ~     username: &'a str,
  |

error[E0106]: missing lifetime specifier
 --> src/main.rs:4:12
  |
4 |     email: &str,
  |            ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 ~ struct User<'a> {
2 |     active: bool,
3 |     username: &str,
4 ~     email: &'a str,
  |

For more information about this error, try `rustc --explain E0106`.
error: could not compile `structs` due to 2 previous errors

*/