// recursion.rs
// a program that uses recursion to find a key in nested boxes.

pub fn main() {
    println!("=== Recursion ===");

    let key = Item::new(Kind::Key, Vec::new());
    let box_with_items = vec![Item::new(
        Kind::Box,
        vec![Item::new(Kind::Box, vec![Item::new(Kind::Box, vec![key])])],
    )];
    println!(
        "Box {:?} | Found Key: {:?}",
        box_with_items,
        look_for_key(&box_with_items)
    );

    let key = Item::new(Kind::Key, Vec::new());
    println!(
        "Box {:?} | Found Key: {:?}",
        box_with_items,
        look_for_key(&vec![key])
    );

    let key = Item::new(Kind::Key, Vec::new());
    let box_with_more_items = vec![
        Item::new(
            Kind::Box,
            vec![Item::new(Kind::Box, vec![Item::new(Kind::Box, vec![])])],
        ),
        Item::new(
            Kind::Box,
            vec![Item::new(Kind::Box, vec![Item::new(Kind::Box, vec![key])])],
        ),
    ];
    println!(
        "Box {:?} | Found Key: {:?}",
        box_with_items,
        look_for_key(&box_with_more_items)
    );

    println!("Factorial Expected: {} | Actual: {}", 120, factorial(5));
    println!("Factorial Expected: {} | Actual: {}", 6, factorial(3));
    println!("Factorial Expected: {} | Actual: {}", 5040, factorial(7));
}

/// Describes the kind of object an item is.
#[derive(Debug, PartialEq)]
enum Kind {
    Key,
    Box,
}

/// Represents an item which can be a box with other items inside it or just an item like a key.
#[derive(Debug)]
struct Item {
    kind: Kind,
    items: Vec<Item>,
}

impl Item {
    fn new(kind: Kind, items: Vec<Item>) -> Self {
        Self { kind, items }
    }

    fn is_a_box(&self) -> bool {
        self.kind == Kind::Box
    }

    fn is_a_key(&self) -> bool {
        self.kind == Kind::Key
    }
}

/// Uses recursion to find a key in nested boxes.
///
/// Returns true if key is found and false if otherwise.
fn look_for_key(box_with_items: &Vec<Item>) -> bool {
    println!("Looking for Key in: {:?}", box_with_items);
    for item in box_with_items {
        if item.is_a_box() {
            look_for_key(&item.items);
        } else if item.is_a_key() {
            println!("Found Key!");
            return true;
        }
    }
    false
}

/// Calculates the factorial.
/// A factorial is the product of all whole numbers for a given number down to 1.
/// Formular: n! = n * !(n - 1)
///
/// Returns the factorial number.
fn factorial(num: i64) -> i64 {
    if num == 1 {
        // Base case
        num
    } else {
        // Recursive case
        num * factorial(num - 1)
    }
}
