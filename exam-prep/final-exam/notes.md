1. In main, start with let mut scores = vec![("alice", 85), ("bob", 92), ("carol", 78)]; and use sort_by with a closure to sort the vector by score in descending order. Print the resulting vector with {:?}.

```rust
fn main() {
    let mut scores = vec![("alice", 85), ("bob", 92), ("carol", 78)]; 
    scores.sort_by(|a, b| b.1.cmp(&a.1)); 
    
    println!("Scores ordered in descending order: {:?}", scores); 
}
```

2. Write a generic function largest<T: PartialOrd + Copy>(list: &[T]) -> T that returns the largest element of the slice. Assume the slice is non-empty.

Call it from main once with &[10, 3, 7, 25, 4] and once with &[1.5_f64, 2.5, 0.5] and print both results.

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T { 
    let mut largest_value = list[0]; 
    for l in list { // we need to dereference here because we are iterating through a reference? 
        if *l > largest_value {
            largest_value = *l; 
        }
    } 
    largest_value
}

fn main() { 
    let ex1 = &[10, 3, 7, 25, 4]; 
    let ex2 = &[1.5_f64, 2.5, 0.5]; 
    
    println!("Result for ex1: {}", largest(ex1)); 
    println!("Result for ex2: {}", largest(ex2)); 
} 
```


3. Write a function first_long<'a>(words: &'a [&'a str], min_len: usize) -> Option<&'a str> that returns the first word in words whose length is at least min_len, or None if there isn't one.

Call it from main with &["hi", "ok", "rust", "hello"] and min_len = 4, and print the result using {:?}.

```rust

fn first_long<'a>(words: &'a[ &'a str], min_len: usize) -> Option<&'a str> {
    for word in words {
        if word.len() >= min_len {
            return Some(word); 
        }
    }
    None
}   

fn main() {
    let words = &["hi", "ok", "rust", "hello"]; 
    let result = first_long(words, 4); 
    println!("Results: {:?}", result); 
}

```

4. Given let fruits = vec!["apple", "banana", "apple", "cherry", "banana", "apple"];, in main build a HashMap<&str, i32> using the entry API that inserts each key and increments the count for each key and print how many times "apple" appears.

```rust
use std::collections::HashMap; 

fn main() {
    let fruits = vec!["apple", "banana", "apple", "cherry", "banana", "apple"]; 
    let mut hashtable: HashMap<&str, i32> = HashMap::new(); 
    for fruit in fruits {
        *hashtable.entry(fruit).or_insert(0) += 1; 
    }
    
    println!("Number of times referenced apple: {}", hashtable.get("apple").unwrap_or(&0)); 
}

```

5. In main, given vec![3, 1, 4, 1, 5, 9, 2, 6, 5], use .partition(...) to split the values into two Vec<i32>s: those less than 5 and those greater than or equal to 5. Then print the lengths of the two resulting vectors.

```rust

fn main() {
    let mut vector = vec![3, 1, 4, 1, 5, 9, 2, 6, 5]; 
    let (vec1, vec2): (Vec<i32>, Vec<i32>) = vector.into_iter().partition(|x| *x < 5); 
    
    println!("Length of the first vector: {}", vec1.len()); 
    println!("Length of the second vector: {}", vec2.len()); 
}
```

