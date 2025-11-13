// This chapter is dedicated to the generics, traits and lifetimes.

use chrono::prelude::{DateTime, Utc};

// GENERICS
// ================================================================================================

// ----- 1 --------------------------------------
// Implement a generic struct `Pair<T>` that holds two values of the same type.
// Add a method `max(&self) -> &T` that returns the larger value.

// IMPLEMENT HERE:

pub struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Pair<T> {
        Pair { first: x, second: y }
    }
}

impl<T: Ord> Pair<T> {
    pub fn max(&self) -> &T {
        if self.first >= self.second {
            &self.first
        } else {
            &self.second
        }
    }
}

// TRAITS AND TRAIT BOUNDS
// ================================================================================================

// ----- 2 --------------------------------------
// Define a trait `Area` with a method `area(&self) -> f64` which calculates an area of the figure.
// Implement it for a `Rectangle` struct with fields `width` and `height`.

// IMPLEMENT HERE:

pub trait Area {
    fn area(&self) -> f64;
}

pub struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// ----- 3 --------------------------------------
// Define a trait `Summarize` with method `summary(&self) -> String`.
// Implement it for two structs:
// - `Article { title, author, content }`
// - `Tweet { username, content }`
//
// Then, write a generic function `notify<T: Summarize>(item: &T) -> String` that returns a
// formatted notification string using a `summary` method.

// IMPLEMENT HERE:

pub trait Summarize {
    fn summary(&self) -> String;
}

pub struct Article {
    title: String,
    author: String,
    content: String,
}

impl Article {
    pub fn new(title: String, author: String, content: String) -> Self {
        Self { title, author, content }
    }
}

pub struct Tweet {
    username: String,
    content: String,
}

impl Tweet {
    pub fn new(username: String, content: String) -> Self {
        Self { username, content }
    }
}

impl Summarize for Article {
    fn summary(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

impl Summarize for Tweet {
    fn summary(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

// fn iso8601(st: &std::time::SystemTime) -> String {
//     let dt: DateTime<Utc> = st.clone().into();
//     format!("{}", dt.format("%+"))
//     // formats like "2001-07-08T00:34:60.026490+09:30"
// }

pub fn notify<T: Summarize>(item: &T) -> String {
    // format!("Notification at {}\n{}", iso8601(&std::time::SystemTime::now()), item.summary())
    format!("Breaking news: {}", item.summary())
}

// LIFETIMES
// ================================================================================================

// ----- 4 --------------------------------------
// Write a function `longest_string(first: &str, second: &str) -> &str` that returns the longer of
// two string slices. Add the lifetimes where needed.

// IMPLEMENT HERE:
pub fn longest_string<'arg>(first: &'arg str , second: &'arg str) -> &'arg str {
    if first.len() >= second.len() {
        first
    } else {
        second
    }
}

// ----- 5 --------------------------------------
// Define a struct `Book` with fields:
// - title: &str
// - content: &str
//
// Implement a method `longest_word(&self) -> Option<&str>` that returns the longest word in the
// book’s content. Return `None` if the content is empty.
//
// Add the lifetimes where needed.

// IMPLEMENT HERE:

/*
 * So we may pass references to object as long as we specify
 * how long the thing pointed is alive. It is a template
 * parameter, because only compiler and user of a function
 * or a struct know how long the burrowed reference
 * used in initialization lives (it may be a local
 * variable or a global variable, it may be moved
 * somewhere and then used as a initialization for
 * a local variable or a global variable and etc).
 * 
 * Each reference (burrow) must have known lifetime
 * during compilation. It is inferred inside of functions.
 * And definitions of structs, methods and functions
 * used to figure out lifetimes or return values
 * and fields.
 * 
 * Lifetime may be (and usually is) a template parameter.
 * Which does not create a separate instance and exist
 * only during compilation stage.
 * 
 * Compiler assigns lifetimes and will bind the template
 * parameter. Then we use lifetime names as a way to bind
 * reference lifetimes. Each reference has a lifetime,
 * it may be inferred in some cases.
 */
pub struct Book<'field> {
    title: &'field str,
    content: &'field str,
}

impl<'field> Book<'field> {
    pub fn new(title: &'field str, content: &'field str) -> Self {
        Self { title, content }
    }

    /*
     * Lifetime is inferred, taken from &self, because it is a method.
     * Book instance lives not longer than it's fields, so it would be alright,
     *
     * Here extend the lifetime of return value by setting it to
     * Option<&'field str>.
     */
    // pub fn longest_word(&self) -> Option<&str>
    pub fn longest_word(&self) -> Option<&'field str>{
        self.content.split_whitespace().max_by(|word1, word2| word1.len().cmp(&word2.len()))
    }
}


