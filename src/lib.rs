/// Determine whether a sentence is a pangram.
use std::collections::HashSet;
use unicode_segmentation::UnicodeSegmentation;

// fn

pub fn is_pangram(sentence: &str) -> bool {
    // todo!("Is {sentence} a pangram?");

    if sentence.trim().is_empty() {
        return false;
    }

    let unicode_letters: Vec<&str> = sentence
        .graphemes(true)
        .into_iter()
        .filter(|s| *s != " " && *s != "_")
        .collect();

    let ascii_letters: HashSet<char> = HashSet::from_iter(
        unicode_letters
            .into_iter()
            .filter(|s| s.is_ascii())
            .map(|s| s.chars().nth(0))
            .flatten()
            .map(|c| c.to_lowercase().nth(0).unwrap())
            .filter(|s| ('a'..='z').contains(s))
            .map(|s| s.to_string())
            .reduce(|acc, s| format!("{}{}", acc, s))
            .unwrap()
            .to_lowercase()
            .chars(),
    );

    let ascii_letters_as_vec = ascii_letters.into_iter().collect::<Vec<char>>();
    let sorted_letters = insert_sort(&ascii_letters_as_vec);
    let all_ascii_letters: Vec<char> = ('a'..='z').collect();
    if sorted_letters.len() != all_ascii_letters.len() {
        return false;
    }
    println!("sorted_letters: {:?}", sorted_letters);
    println!("all_ascii_letters: {:?}", all_ascii_letters);
    for (i, el) in all_ascii_letters.into_iter().enumerate() {
        if *sorted_letters[i] != el {
            return false;
        }
    }

    true
}

fn insert_sort<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> Vec<&T> {
    let list_to_pass: Vec<&T> = list.into_iter().collect();
    let sorted = insert_sort_rec_outer(list_to_pass.to_vec(), 1);
    sorted
}

fn insert_sort_rec_outer<T: std::cmp::PartialOrd + Copy>(list: Vec<&T>, i: usize) -> Vec<&T> {
    if i >= list.len() {
        return list.to_owned();
    }
    let new_list = insert_sort_rec_inner(list, i as i32);
    let new_i = i + 1;
    insert_sort_rec_outer(new_list, new_i)
}

fn insert_sort_rec_inner<T: std::cmp::PartialOrd + Copy>(list: Vec<&T>, j_input: i32) -> Vec<&T> {
    if j_input <= 0 {
        return list;
    }
    let j = j_input as usize;
    if !(list[j - 1] > list[j]) {
        return list;
    }
    let new_j: i32 = j as i32 - 1;
    let (j_el, j_minus1_el) = (list[j - 1], list[j]);
    let new_list: Vec<&T> = list
        .into_iter()
        .enumerate()
        .map(|(i, x)| match i {
            i if i == j => j_el,
            i if i == j - 1 => j_minus1_el,
            _ => x,
        })
        .collect();
    insert_sort_rec_inner(new_list, new_j)
}
