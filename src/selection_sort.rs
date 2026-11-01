use std::collections::HashMap;

pub fn main() {
    println!("=== Selection Sort ===");

    let mut music_list = HashMap::new();
    music_list.insert("RADIOHEAD", 156);
    music_list.insert("Kishore Kumar", 141);
    music_list.insert("The Black Keys", 35);
    music_list.insert("Neutral Milk Hotel", 94);
    music_list.insert("Beck", 88);
    music_list.insert("The Strokes", 61);
    music_list.insert("WILCO", 111);

    let mut music_play_counts: Vec<i64> = music_list.values().copied().collect();
    println!("Sorted: {:?}", selection_sort(&mut music_play_counts));

    println!("Sorted: {:?}", selection_sort(&mut vec![23, 454, 56, 5, 7]));

    println!("Sorted: {:?}", selection_sort(&mut vec![1, 2, 3]));

    println!("Next Challenge: See if you can sort the HashMap/dictionary.")
}

/// Finds the smallent element in an array.
/// Steps:
/// 1. Arbitrarily selects the element at index 0 as the smallest
/// 2. Iterates the array and compares each value to current smallest
/// 3. If current value is smaller than current smallets, it updates smallest.
///
/// Returns the smallest element index.
fn find_smallest(arr: &[i64]) -> usize {
    let mut smallest = arr[0];
    let mut smallest_index = 0;

    // Iterate array and skip zero index - start at 1
    for (i, num) in arr.iter().enumerate().skip(1) {
        // Need to dereference to read the value
        if *num < smallest {
            smallest = *num;
            smallest_index = i;
        }
    }

    smallest_index
}

/// A selection sort function.
/// Uses the selection sort algorithm to sort the given array in ascending order (smallest to biggest).
/// Beware: This function mutates the passed in Vector/array, its up to the caller to maintain a copy of the original.
///
/// Returns the sorted Vector/array.
fn selection_sort(arr: &mut Vec<i64>) -> Vec<i64> {
    println!("SELECTION SORT: Array: {:?}", arr);
    // New array to store sorted elements.
    let mut new_arr = Vec::new();

    // Loop all elements in array
    for _ in 0..arr.len() {
        // Find smallest in array and adds it to new array, removing from current
        let smallest = find_smallest(arr);
        new_arr.push(arr.remove(smallest));
    }

    new_arr
}
