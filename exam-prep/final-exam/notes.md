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


6. In main, use iterator methods to compute and print the sum of squares of the even numbers in 1..=10.



```rust
fn main() {
    
    let sum = (1..=10).filter(|x| x % 2 == 0).map(|x| x * x).sum::<i32>();
    let sum2 = (1..=10).filter(|x| x % 2 == 0).fold(0, |acc, x| acc + (x * x)); 
    println!("Sum of squares of even numbers from 1-10 inclusive: {} = {}", sum, sum2)
}

```

7. In main, use VecDeque<i32> as a FIFO queue. Enqueue the numbers 1 through 5 (in order), then dequeue them all and print each dequeued value on its own line.

```rust
use std::collections::VecDeque; 
fn main() {
    let mut queue: VecDeque<i32> = VecDeque::new(); 
    for v in 1..=5 {
        queue.push_back(v); 
    }
    println!("Queue: {:?}", queue); 
    
    while let Some(val) = queue.pop_front() {
        println!("{}", val); 
    }
}
```

8. In main, use a BinaryHeap<i32> to find and print the three largest values in vec![4, 10, 2, 8, 15, 7, 1], in descending order, separated by spaces.

```rust
use std::collections::BinaryHeap; 

fn main() {
    let vector: Vec<i32> = vec![4, 10, 2, 8, 15, 7, 1]; 
    let mut heap: BinaryHeap<i32> = BinaryHeap::from(vector); 
    
    for _ in 0..=2 {
        print!("{} ", heap.pop().unwrap()); 
    }
}
```

9. In main, build a BTreeMap<&str, i32> from the pairs ("banana", 3), ("apple", 5), ("cherry", 2), and then iterate and print each name score pair on its own line.

```rust
use std::collections::BTreeMap; 

fn main() {
    let mut map = BTreeMap::from([
        ("banana", 3),
        ("apple", 5),
        ("cherry", 2),
    ]);
    
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

}
```

10. In main, start with let nums = vec![1, 2, 2, 3, 1, 4, 3, 5]; and use a HashSet<i32> to count how many distinct values appear. Print the count.

```rust
use std::collections::HashSet; 

fn main() {
    let nums = [1, 2, 2, 3, 1, 4, 3, 5]; 
    let set: HashSet<i32> = HashSet::from(nums); 
    
    println!("Number of distinct values: {}", set.len()); 
}

```

11. Write a function char_count(s: &str) -> HashMap<char, usize> that returns a map from each character in s to the number of times it appears.

Call it in main with "rust" and print the resulting map with {:?}.

```rust
use std::collections::HashMap; 

fn char_count(s: &str) -> HashMap<char, usize> {
    let mut hashtable: HashMap<char, usize> = HashMap::new(); 
    for c in s.chars() {
        *hashtable.entry(c).or_insert(0) += 1; 
    }
    
    hashtable
}

fn main() {
    println!("Character counts: {:?}", char_count("rust")); 
}
```

