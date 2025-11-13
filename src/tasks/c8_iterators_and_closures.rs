// This chapter is dedicated to the iterators and closures.

use std::collections::HashMap;

// ITERATORS
// ================================================================================================

// ----- 1 --------------------------------------
// Write a function `word_frequencies(text: &str) -> Vec<(String, usize)>` that:
// - Splits the input text into words.
// - Normalizes them to lowercase.
// - Counts how many times each word appears.
// - Returns the result as a vector of `(word, count)` tuples, sorted by descending frequency.
// If some words have the same frequency, return them in alphabetical order.

fn get_frequencies<T: std::cmp::Eq + Ord + std::hash::Hash + Clone, Iter: Iterator<Item = T>>(iter: Iter) -> Vec<(T, usize)> {
    let mut counter = HashMap::<T, usize>::new();
    for item in iter {
        match counter.get_mut(&item) {
            None => {
                counter.insert(item, 1);
            }

            Some(amount) => {
                *amount += 1;
            }
        }
    }

    let mut result: Vec<_> = counter.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
    result.sort_by(|(item1, count1), (item2, count2)|
        count1.cmp(count2).reverse().then(item1.cmp(item2)));
    result
}

pub fn word_frequencies(text: &str) -> Vec<(String, usize)> {
    let normalized_words = text.split_whitespace().map(|word| word.to_lowercase());
    get_frequencies(normalized_words)
}

// ----- 2 --------------------------------------
// Write a function `top_k_most_common_letters(text: &str, k: usize) -> Vec<(char, usize)>` that:
// - Counts the frequency of letters only (ignore spaces/punctuation).
// - Case-insensitive.
// - Sorts by descending frequency.
// - Returns the vector with top `k` letters.
// If some letters have the same frequency, return them in alphabetical order.

pub fn top_k_most_common_letters(text: &str, k: usize) -> Vec<(char, usize)> {
    let normalized_chars = text.chars()
        .filter(|chr|
            !chr.is_whitespace() && !chr.is_ascii_punctuation()
        )
        .map(|chr| chr.to_ascii_lowercase());

    let frequencies = get_frequencies(normalized_chars);

    frequencies[0..k].iter().cloned().collect::<Vec<_>>()
}

// CLOSURES
// ================================================================================================

// ----- 3 --------------------------------------
// Write a function
// `filter_and_sort_names(names: Vec<String>, minimum_length: usize) -> Vec<String>` that:
// - Filters out names shorter than minimum_length.
// - Sorts the remaining names alphabetically (case-insensitive).
// - Returns the result.
// You must use closures in filtering and sorting.

pub fn filter_and_sort_names(names: Vec<String>, minimum_length: usize) -> Vec<String> {
    let mut filtered = names.iter()
        .filter(|name| name.len() >= minimum_length)
        .cloned()
        .collect::<Vec<_>>();
    filtered.sort_by(|name1, name2|
        /*
         * Not optimal perfomance-wise to make names lowercase on each comparison,
         * but it is code is easier. We do not need a more complicated solution
         * for now.
         */
        name1.to_lowercase().cmp(&name2.to_lowercase())
    );

    filtered
}

// ----- 4 --------------------------------------
// Create a function `group_students_by_grade` that:
// - Accepts a `Vec<(String, u32)>` where each tuple is `(student_name, grade)`.
// - Groups students into some map where the key is the grade and the value is a vector of names.
// - Uses closures for grouping logic.
// - Returns the grouped map, sorted internally by student names.

pub fn group_students_by_grade(students: Vec<(String, u32)>) -> HashMap<u32, Vec<String>> {
    students.into_iter().fold(
        HashMap::new(),
        |accumulator, item| {
            let mut acc_moved = accumulator;
            let (student, grade) = item;
            match acc_moved.get_mut(&grade) {
                None => {
                    acc_moved.insert(grade, vec![student]);
                }

                Some(students) => {
                    students.push(student);
                }
            }

            acc_moved
        }
    )
}
