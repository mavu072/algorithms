// binary_search.rs
// A program that uses a binary search algorithm to accurately guess a number.

pub fn main() {
    println!("=== Binary Search ===");

    // Binary Search
    let list = vec![1, 2, 3, 4, 5];
    let item = 5;
    check_guess(&list, &item, &binary_search(&list, &item));

    let item = 3;
    check_guess(&list, &item, &binary_search(&list, &item));

    let item = 1;
    check_guess(&list, &item, &binary_search(&list, &item));
}

/// Checks if the guess is accurate.
/// Returns true if accurate and false if not.
fn check_guess(list: &[i32], item: &i32, found_at: &usize) -> bool {
    let found = list[*found_at];
    if found == *item {
        println!(
            "Item: {} was found! Guess: {} at index: {}",
            item, found, found_at
        );
        return true;
    }
    false
}

/// A binary search function.
/// Uses the binary search algorithm to find the specified number in the fewest steps,
/// by beginning the search in the middle.
/// Returns the found element index.
fn binary_search(list: &Vec<i32>, item: &i32) -> usize {
    println!("BINARY SEARCH: Item: {} | List: {:?}", item, list);
    // Derefence item to get the value inside.
    let target: i32 = *item;

    // Get low and high values.
    let mut low = 0;
    let mut high = list.len() - 1;

    // Count number of steps to find item. For debugging.
    let mut step = 0;

    // While its not narrowed down to one element.
    while low <= high {
        // Start guessing at the middle.
        let mid = (low + high) / 2;
        let guess = list[mid];
        step += 1;

        // Show the steps
        println!("Step: {} | Guess: {} | Index: {}", step, guess, mid);

        if guess == target {
            println!("Found in {} steps.", step);
            return mid;
        } else if guess > target {
            // If high: ignore upper half of the array.
            high = mid - 1;
        } else {
            // If low: ignore lower half of the array.
            low = mid + 1;
        }
    }

    panic!("Item Not found!");
}
