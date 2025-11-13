// This chapter is dedicated to the structs, methods, enums, and pattern matching

// STRUCTS
// ================================================================================================

// ----- 1 --------------------------------------
// Define a struct `Point` with fields `x` and `y` (both `u32`). Create a function `new_point(x, y)`
// that returns a `Point` instance.

// IMPLEMENT HERE:

use std::time::SystemTime;

use chrono::Datelike;

pub struct Point {
    x: u32,
    y: u32,
}

pub fn new_point(x: u32, y: u32) -> Point {
    Point { x, y }
}

// uncomment once implemented
pub fn point_checker() {
    let point = new_point(3, 4);
    assert_eq!((3, 4), (point.x, point.y));
}

// ----- 2 --------------------------------------
// Define a struct `Rectangle` with width and height. Implement a function
// `can_hold(r1: &Rectangle, r2: &Rectangle) -> bool` that returns true if `r1` can completely
// contain `r2`.

// IMPLEMENT HERE:

pub struct Rectangle {
    width: u32,
    height: u32,
}

pub fn can_hold(outer: &Rectangle, inner: &Rectangle) -> bool {
    (inner.width <= outer.width && inner.height <= outer.height) ||
    (inner.height <= outer.width && inner.width <= outer.height)
}

// uncomment once implemented
pub fn rectangle_checker() {
    let big = Rectangle { width: 10, height: 8 };
    let small = Rectangle { width: 5, height: 4 };

    assert!(can_hold(&big, &small));
    assert!(!can_hold(&small, &big));
}

// METHODS
// ================================================================================================

// ----- 3 --------------------------------------
// Create a `Company` struct with `name: String`, `date_of_origin: u32` and `annual_income: u64`
// fields. Implement `new(...) -> Self` constructor for it and a `total_income(...) -> u64`
// method that would calculate how much money this company earned since it was established
// (excluding taxes).

// IMPLEMENT HERE:

pub struct Company {
    name: String,
    date_of_origin: u32,
    annual_income: u64,
}

impl Company {
    pub fn new(name: String, date_of_origin: u32, annual_income: u64) -> Self {
        Self { name, date_of_origin, annual_income }
    }

    pub fn total_income(&self) -> u64 {
        self.annual_income * ((chrono::Utc::now().year() as u64) - (self.date_of_origin as u64))
    }
}

// ----- 4 --------------------------------------
// Define a struct BankAccount with `owner: String` and `balance: u64` fields.
// Implement basic `new(...) -> Self` constructor.
// Implement methods:
// - `deposit(&mut self, amount: u64)` which adds the specified amount to the balance.
// - `withdraw(&mut self, amount: u64) -> bool` which removes the specified amount from the balance
//   and returns `true`, or just returns `false` if there are insufficient funds.
// - `balance(&self) -> u64` which returns the current balance.

// IMPLEMENT HERE:

pub struct BankAccount {
    owner: String,
    balance: u64,
}

impl BankAccount {
    pub fn new(owner: String, balance: u64) -> Self {
        Self { owner, balance }
    }

    pub fn deposit(&mut self, amount: u64) {
        self.balance += amount;
    }

    pub fn withdraw(&mut self, amount: u64) -> bool {
        if self.balance <= amount {
            false
        } else {
            self.balance -= amount;
            true
        }
    }

    pub fn balance(&mut self) -> u64 {
        self.balance
    }
}

// ENUMS
// ================================================================================================

// ----- 5 --------------------------------------
// Define an enum `TrafficLight` with variants `Red`, `Yellow`, and `Green`. Implement a
// `next(light: &TrafficLight) -> TrafficLight` method for it that returns the next light in
// sequence.

// IMPLEMENT HERE:

pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    pub fn next(&self) -> Self {
        match self {
            TrafficLight::Red => TrafficLight::Green,
            TrafficLight::Green => TrafficLight::Yellow,
            TrafficLight::Yellow => TrafficLight::Red,
        }
    }
}

// ----- 6 --------------------------------------
// Define an enum `Operation` with variants `Add(i32, i32)`, `Subtract(i32, i32)`,
// `Multiply(i32, i32)` and `Divide(i32, i32)`. Implement a method
// `apply(self) -> Option<i32>` for it that computes the result and returns `None` if
// dividing by zero (you can use `match` for convenience)

// IMPLEMENT HERE:

pub enum Operation {
    Add(i32, i32),
    Subtract(i32, i32),
    Multiply(i32, i32),
    Divide(i32, i32),
}

impl Operation {
    pub fn apply(self) -> Option<i32> {
        match self {
            Operation::Add(x, y) => Some(x + y),
            Operation::Subtract(x, y) => Some(x - y),
            Operation::Multiply(x, y) => Some(x * y),
            Operation::Divide(x, y) => match y {
                0 => None,
                divider => Some(x / divider),
            }
        }
    }
}

// PATTERN MATCHING
// ================================================================================================

// ----- 7 --------------------------------------
// Write a enum `WeirdLengthMeasures`, containing `Inch`, `Foot`, `Yard` and `Mile` variants.
// Implement a `convert_to_human_format(&self) -> f64` method, which returns the length of of the
// provided weirdo lengths in meters using pattern matching (with `match`).
// Use provided values:
// - Inch -> 0.0254 m
// - Foot -> 0.3048 m
// - Yard -> 0.9144 m
// - Mile -> 1609.344 m

// IMPLEMENT HERE:

pub enum WeirdLengthMeasures {
    Inch,
    Foot,
    Yard,
    Mile,
}

impl WeirdLengthMeasures {
    pub fn convert_to_human_format(&self) -> f64 {
        match self {
            WeirdLengthMeasures::Inch => 0.0254,
            WeirdLengthMeasures::Foot => 0.3048,
            WeirdLengthMeasures::Yard => 0.9144,
            WeirdLengthMeasures::Mile => 1609.344,
        }
    }
}

// ----- 8 --------------------------------------
// Write a function `fizzbuzz(n: u32) -> Vec<String>` that returns a vector of strings from 1 to n
// where:
// - Multiples of 2 are "Fizz",
// - Multiples of 3 are "Buzz",
// - Multiples of both 2 and 3 are "FizzBuzz",
// - Otherwise the number itself.

pub fn fizzbuzz(n: u32) -> Vec<String> {
    let mut result = Vec::new();
    result.resize(n as usize, String::new());

    for i in 1..=n {
        result[(i-1) as usize] = if i % 6 == 0 {
            String::from("FizzBuzz")
        } else if i % 2 == 0 {
            String::from("Fizz")
        } else if i % 3 == 0 {
            String::from("Buzz")
        } else {
            i.to_string()
        };
    }

    result
}
