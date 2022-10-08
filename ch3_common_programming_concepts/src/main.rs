fn main() {
    let mut y = 5; // Mutable variable, as Rust has all variables immutable by default
    let x = 5;
    println!("The value of x is: {}", x);

    let x = x + 1; // Shadowing
    println!("The value of x after the first shadowing is: {}", x);

    let x = x * 2;
    println!("The value of x after the second shadowing is: {}", x);

    another_function();

    // Function with multiple parameters
    print_my_nums(12, 15, 'u');

    // Expressional assignment

    let b = {
        let a = 5;
        a + 2 // returns 7, also note no semicolon (see README for more info)
    };

    println!("What is b? {b}");

    let five = five();

    println!("five is {five}");

    // Function with parameters, control statements, and the return of a boolean value
    let greater_than = positive(five);
    let negative_one = positive(-1);

    println!("Is {five} a positive number? {greater_than}");
    println!("Is -1 a positive number? {negative_one}");

    // loop-de-loop
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

    println!("\nEnd count = {count}");

    // recursive fibonacci implementation in Rust

    let num = 6;
    let fib = fibonacci(num);

    println!("fibonacci sequence of {num} = {fib}");

    // self-made Twelve Days of Christmas jingle (half-baked atm)

    twelve_days_of_christmas();

}

fn another_function() {
    println!("Another function.");
}

// Function with parameters
fn print_my_nums(x: i32, y: i32, letter: char) {
    println!("The value of x is: {x}\nThe value of y is: {y}\nYou smell funny.\nno {letter}");
}

// Function with a return value
fn five() -> i32 {
    5
}

// Can return a value at the very end, or even at the beginning to prevent unintended consequences in functions, when any sort of processing is done
fn positive(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    true
}

fn fibonacci(n: i32) -> i32 {

    if n < 0 {
        return 0;
    }

    if n == 0 {
        return 0;
    }

    if n == 1 || n == 2 {
        return 1;
    }

    fibonacci(n - 1) + fibonacci(n - 2)
}

fn twelve_days_of_christmas() {

    // let days = 12;
    let mut current_day = 1;

    for days in 1..12 {
        println!("On day {current_day} of Christmas, my true love gave to me...");

        let mut day = current_day;
        for current in 1..current_day {
            println!("{day} items of blank");
            day -= 1;
        }
        
        
        if current_day == 1 {
            println!("A partridge in a pear tree!");
        } else {
            println!("And a partridge in a pear tree!");
        }

        current_day += 1;

    }

}