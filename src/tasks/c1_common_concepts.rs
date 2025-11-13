// This chapter is dedicated to the common programming concepts, like variables and their
// mutability, data types, functions and control flow stuff

// MUTABILITY
// ================================================================================================

// ----- 1 --------------------------------------
// Write a function that declares a mutable integer variable, assigns it the value 5, then changes
// it to 10, and prints both values.
#[allow(dead_code)]
pub fn simple_mutability() {
    let mut var = 5;
    println!("{}", var);
    var = 10;
    println!("{}", var);
}

// DATA TYPES
// ================================================================================================

// ----- 2 --------------------------------------
// Create variables of types `i32``, `f64``, `bool``, and `char``, assign them values, and print
// them.
#[allow(dead_code)]
pub fn simple_data_types() {
    let vi32: i32 = 1;
    let vf64: f64 = 2.0;
    let vbool: bool = 3 == 3;
    let vchar: char = '4';

    println!("{}", vi32);
    println!("{}", vf64);
    println!("{}", vbool);
    println!("{}", vchar);
}

// FUNCTIONS
// ================================================================================================

// ----- 3 --------------------------------------
// Write a function `square` that takes a `u32` integer and returns its square as `u32`.

// IMPLEMENT HERE:
#[allow(dead_code)]
pub fn square(input: u32) -> u32 {
    input * input
}

// ----- 4 --------------------------------------
// Write a recursive function `factorial` that computes the factorial of a number (n!) as `u32`.

// IMPLEMENT HERE:
pub fn factorial(index: u32) -> u32 {
    if index == 0 {
        1
    }
    else {
        factorial(index - 1) * index
    }
}

// CONTROL FLOW
// ================================================================================================

// ----- 5 --------------------------------------
// Write a program that prints whether a provided signed integer number is positive, negative, or
// zero using `if` statement.
pub fn sign_checker(number: i32) -> &'static str {
    if number < 0 {
        "negative"
    } else if number > 0 {
        "positive"
    } else {
        "zero"
    }
}

// ----- 6 --------------------------------------
// Write a program that finds the largest number in an array of 5 integers using a for or while
// loop.
pub fn find_biggest_number(some_array: [u32; 5]) -> u32 {
    let mut max_value: Option<u32> = None;
    for item in some_array {
        if max_value.is_none() || max_value.unwrap() < item {
            max_value = Some(item);
        }
    }

    max_value.unwrap()
}
