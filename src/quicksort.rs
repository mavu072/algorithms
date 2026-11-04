// quicksort.rs
// a program that uses quicksort to sort a collection.

pub fn main() {
    println!("=== Quicksort ===");

    let arr = vec![3];
    println!("Sorted: {:?} | List: {:?}", quicksort(&arr), arr);

    let arr = vec![84, 63];
    println!("Sorted: {:?} | List: {:?}", quicksort(&arr), arr);

    let arr = vec![1, 2, 3];
    println!("Sorted: {:?} | List: {:?}", quicksort(&arr), arr);

    let arr = vec![4, 3, 2, 1];
    println!("Sorted: {:?} | List: {:?}", quicksort(&arr), arr);

    let arr = vec![23, 454, 56, 5, 7];
    println!("Sorted: {:?} | List: {:?}", quicksort(&arr), arr);
}

/// Uses the quicksort algorithm to sort an array.
///
/// Returns a sorted array.
fn quicksort(arr: &Vec<i64>) -> Vec<i64> {
    if arr.len() < 2 {
        // Base case: array with 0 or 1 element is already sorted.
        arr.clone()
    } else {
        // Recursive case: array with 2 or more elements.
        let pivot = arr[0]; // Choosing first element as pivot, can also use mid element with `arr[arr.len() / 2]`.
        println!("Array: {:?} | Pivot: {}", arr, pivot);

        // Sub-array of elements smaller than the pivot.
        let less: Vec<i64> = arr.iter().filter(|&&i| i < pivot).map(|&i| i).collect();
        println!("Less: {:?}", less);

        // Sub-array of elements greater than the pivot.
        let greater: Vec<i64> = arr.iter().filter(|&&i| i > pivot).map(|&i| i).collect();
        println!("Greater: {:?}", greater);

        // Combine the sorted sub-arrays and the pivot into one sorted array.
        let mut sorted: Vec<i64> = vec![];
        sorted.append(&mut quicksort(&less)); // Recurse call quicksort on less sub-array
        sorted.push(pivot); // Pivot in the middle
        sorted.append(&mut quicksort(&greater)); // Recurse call quicksort on greater sub-array

        sorted
    }
}
