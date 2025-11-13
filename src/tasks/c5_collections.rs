// This chapter is dedicated to some collections: vectors, strings and hash maps

use std::{collections::{HashMap, HashSet}, mem::swap};

// VECTORS
// ================================================================================================

// ----- 1 --------------------------------------
// Write a function `second_largest(vec: &[i32]) -> Option<i32>` that returns the second largest
// element in the array. If the array has fewer than 2 elements, return `None`.

pub fn second_largest(vec: &[i32]) -> Option<i32> {
    let mut items = vec.iter().cloned().collect::<HashSet<_>>().iter().cloned().collect::<Vec<_>>();
    items.sort();

    if items.len() >= 2 {
        Some(items[items.len() - 2])
    } else {
        None
    }
}

// ----- 2 --------------------------------------
// Write a function `longest_increasing_subsequence(vec: &[i32]) -> Vec<i32>`` that finds the
// longest strictly increasing subsequence (not necessarily contiguous) in the array.
//
// For the simplicity, assume that there is only one longest increasing subsequence.

pub fn longest_increasing_subsequence(init_sequence: &[i32]) -> Vec<i32> {
    struct SubsequenceDesc {
        prev_idx: Option<usize>,
        length: usize,
    }

    impl Clone for SubsequenceDesc {
        fn clone(&self) -> Self {
            Self { prev_idx: self.prev_idx.clone(), length: self.length.clone() }
        }
    }
    let mut longest_subsequence_with_end = Vec::<SubsequenceDesc>::with_capacity(init_sequence.len());

    for item_idx in 0..init_sequence.len() {
        let item = init_sequence[item_idx];

        longest_subsequence_with_end.push(SubsequenceDesc { prev_idx: None, length: 1 });

        for smaller_prev_idx in 0..item_idx {
            let smaller_prev = init_sequence[smaller_prev_idx];
            if smaller_prev >= item {
                continue;
            }

            let mut candidate = longest_subsequence_with_end[smaller_prev_idx].clone();
            candidate.length += 1;
            candidate.prev_idx = Some(smaller_prev_idx);

            if candidate.length > longest_subsequence_with_end[item_idx].length { 
                longest_subsequence_with_end[item_idx] = candidate;
            }
        }
    }

    let mut longest_subseq_end = longest_subsequence_with_end.iter().enumerate().max_by(
        |subseq1, subseq2| subseq1.1.length.cmp(&subseq2.1.length)
    );

    let mut result = Vec::<i32>::new();

    while longest_subseq_end.is_some() {
        result.push(init_sequence[longest_subseq_end.unwrap().0]);
        longest_subseq_end = match longest_subseq_end.unwrap().1.prev_idx {
            None => None,
            Some(prev_idx) => Some((prev_idx, &longest_subsequence_with_end[prev_idx])),
        };
    }

    result.reverse();


    result
}

// STRINGS
// ================================================================================================

// ----- 3 --------------------------------------
// Write a function `reverse_words(sentence: &str) -> String` that reverses the order of words in a
// sentence but does not reverse the characters inside each word.

pub fn reverse_words(sentence: &str) -> String {
    let mut words: Vec<&str> = sentence.split(' ').collect();
    words.reverse();

    words.join(" ")
}

// ----- 4 --------------------------------------
// Write a function `normalize_and_capitalize(sentence: &str) -> String` that:
// - Trims extra spaces at the beginning and end.
// - Converts multiple spaces between words into a single space.
// - Makes the first letter of every word uppercase, and every other letter lowercase, for example
//   "пРеВеД МеДвЕд -> Превед Медвед"

pub fn normalize_and_capitalize(sentence: &str) -> String {
    let trimmed = sentence.trim();
    let words = 
        trimmed
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            let start = chars.next().unwrap().to_uppercase().collect::<String>();
            let remainder = chars.map(|chr|
                chr
                    .to_lowercase()
                    .collect::<String>()
            ).collect::<String>();

            start + remainder.as_str()
        })
        .collect::<Vec<String>>();

    let single_spaced = words.join(" ");

    single_spaced
}

// HASH SET
// ================================================================================================

// ----- 5 --------------------------------------
// Write a function `unique_chars(s: &str) -> bool` that returns true if a string has all unique
// characters (ignoring case), and false otherwise.

pub fn unique_chars(s: &str) -> bool {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort();

    for pos in 1..chars.len() {
        if chars[pos] == chars[pos - 1] {
            return false;
        }
    }

    true
}

// HASH MAP
// ================================================================================================

// ----- 6 --------------------------------------
// Write a function `top_k_frequent(nums: Vec<i32>, k: usize) -> Vec<i32>` that returns the `k` most
// frequent numbers in the vector. If `k` is greater than the total number of unique elements in the
// vector, return all of them.

pub fn top_k_frequent(nums: Vec<i32>, k: usize) -> Vec<i32> {
    let mut counter = HashMap::<i32, usize>::new();

    for num in nums {
        match counter.get_mut(&num) {
            None => {
                counter.insert(num, 1);
            }
            Some(value) => {
                *value += 1;
            }
        }
    }

    let mut unique_nums: Vec<i32> = counter.keys().cloned().collect();
    unique_nums
        .sort_by(|num1, num2| 
            counter[num1].cmp(&counter[num2]).reverse()
        );

    unique_nums.iter().take(k).cloned().collect()
}
