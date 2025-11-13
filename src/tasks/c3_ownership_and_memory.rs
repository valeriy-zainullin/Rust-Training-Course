// This chapter is dedicated to the ownership, borrowing and slices

// OWNERSHIP
// ================================================================================================

use std::fmt::Write;

// ----- 1 --------------------------------------
// Write a function `longest_owned(s1: String, s2: String) -> String` that returns the longer of
// two strings. Check that both original strings are moved into the function, and only the returned
// can still be used.
//
// You can implement the function and use it right inside the `string_ownership` function.
#[allow(dead_code)]
pub fn string_ownership() {
    let longest_owned = move |s1: String, s2: String| -> String {
        if s1.len() >= s2.len() {
            s2
        } else {
            s1
        }
    };

    let str1 = String::from("a");
    let str2 = String::from("b2");
    assert!(longest_owned(str1, str2) == "b2");
}

// BORROWING
// ================================================================================================

// ----- 2 --------------------------------------
// Write a function `print_length(s: ???)` that takes some string and prints its length without
// taking ownership. First use it with some random (censored) string, and then print this string to
// show that it was not moved and still available.
//
// You can implement the function and use it right inside the `simple_borrowing` function.
#[allow(dead_code)]
pub fn simple_borrowing() {
    let print_length = |str: &str| println!("{}", str.len());

    let str = "123";
    print_length(str);
    println!("{}", str);
}

// ----- 3 --------------------------------------
// Implement a function `append_and_return_length(string: ???, suffix: ???) -> usize` that borrows
// some string, appends a suffix to it, and returns the new length. Then call it multiple times
// to check that the string was borrowed, not moved.
//
// You can implement the function and use it right inside the `hard_borrowing` function.
#[allow(dead_code)]
pub fn hard_borrowing() {
    let append_and_return_length = |string: &mut String, suffix: &str| -> usize {
        string.write_fmt(format_args!("{}", suffix)).unwrap();
        string.len()
    };

    let mut input = String::from("123");
    assert!(append_and_return_length(&mut input, "345") == 6);
    assert!(append_and_return_length(&mut input, "678") == 9);
}

// SLICES
// ================================================================================================

// ----- 4 --------------------------------------
// Write a function last_word(s: &str) -> &str that returns the last word from a string slice.
// Assume words are separated by spaces.
pub fn last_word(slice: &str) -> &str {
    if slice.find(|c| c != ' ').is_some() {
        slice.split_whitespace().last().unwrap()
    } else {
        // Only spaces or the string is empty..
        slice
    }
}

// ----- 5 --------------------------------------
// Write a function longest_word(sentence: &str) -> &str that returns the longest word in a
// sentence (string slice). If several words have the same maximum length, return the last one.
pub fn longest_word(sentence: &str) -> &str {
    if sentence.find(|c| c != ' ').is_some() {
        sentence
            .split_whitespace()
            .max_by(|word1, word2|
                word1.len().cmp(&word2.len())
            )
            .unwrap()
    } else {
        // Only spaces or the string is empty..
        sentence
    }
}
