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



# Long coding challenges 
1. 

```rust 

use std::collections::HashMap;

fn count_word_freq(sentence: &str) -> HashMap<&str, usize> {
    let mut hash = HashMap::new(); 
    let word_vec = sentence.split_whitespace().collect::<Vec<&str>>(); 
    for word in word_vec {
        *hash.entry(word).or_insert(0) += 1;
    }
    hash
}

fn main() {
    let sentence = "the quick brown fox jumps over the lazy dog the fox runs fast over the hill"; 
    let hash = count_word_freq(sentence); 
    println!("{:?}", hash); 
    let mut vecs = hash.into_iter().collect::<Vec<(&str, usize)>>(); 
    println!("{:?}", vecs); 

    vecs.sort_by(|a, b| {
        b.1.cmp(&a.1).then(a.0.cmp(b.0))
    }); 
    
    for v in 0..=2 {
        println!("{} {}", vecs[v].0, vecs[v].1); 
    
    }

}
```


2. Write a program that prints, in sorted order, the values that appear in both input vectors:

let a = vec![1, 2, 3, 4, 5, 6];
let b = vec![4, 5, 6, 7, 8, 9];
Use HashSet for the membership checks, then sort the resulting common values before printing them (one per line).

```rust

use std::collections::HashSet; 

fn main() {
    let a = vec![1, 2, 3, 4, 5, 6]; 
    let b = vec![4, 5, 6, 7, 8, 9]; 
    let ahash = a.iter().collect::<HashSet<_>>(); 
    let bhash = b.iter().c
    
    ollect::<HashSet<_>>(); 
    
    let mut intersection: Vec<_> = ahash.intersection(&bhash).collect(); 
    intersection.sort(); 
    for i in intersection {
        println!("{}", i)
    }
}

```

Build a small "grade book"

Given let entries = vec![("carol", 72), ("alice", 95), ("bob", 58), ("alice", 88), ("bob", 80)];,
insert each pair into a BTreeMap<&str, Vec<i32>> keyed by name, with the list of that student's scores as the value.
Then iterate the map (which will give names in alphabetical order)
and print each student's name followed by their average score, formatted to one decimal place.
Hint: To print 1 decimal place, use {:.1} in the println! macro.

Expected output:

```rust
use std::collections::BTreeMap; 

fn calculate_mean_grade(vector: &Vec<i32>) -> f64 {
    vector.iter().sum::<i32>() as f64 / vector.len() as f64
}

fn main() {
    let entries = vec![("carol", 72), ("alice", 95), ("bob", 58), ("alice", 88), ("bob", 80)];
    let mut map: BTreeMap<&str, Vec<i32>> = BTreeMap::new(); 
    for (name, score) in entries {
        map.entry(name).or_insert(Vec::new()).push(score); 
    }
    for (name, score) in &map {
        println!("{} {:.1}", name, calculate_mean_grade(score)); 
    }
    
}



```


```rust
fn is_balanced(s: &str) -> bool {
    let mut vec: Vec<char> = Vec::new(); 
    for c in s.chars() {
        match c {
            '(' | '[' | '{' => vec.push(c), 
            ')' | ']' | '}' => {
                let counter = match c {
                    ')' => '(', 
                    ']' => '[', 
                    '}' => '{', 
                    _ => unreachable!()
                }; 
                // now we have to make sure the vec.pop matches the counter 
                
                match vec.pop() {
                    Some(val) if val == counter => {},
                    _ => return false
                }
            }, 
            _ => {}
        }
    }
    vec.is_empty()
}

fn main() {
    let cases = ["()[]{}", "([{}])", "(]", "((()"];
    for case in cases {
        println!("{}", is_balanced(case)); 
    }
}

```

Implement the following:

Write a function moving_avg(values: &[f64], window: usize) -> Vec<f64> that computes the rolling average over a sliding window of size window.
Use a VecDeque<f64> to track the window.
Only emit an average once the window is fully populated, so the output length is values.len() - window + 1 (or empty if values.len() < window).**
Call the function from main with values = &[1.0, 2.0, 3.0, 4.0, 5.0] and window = 3, and print each result on its own line, formatted to one decimal place.

Expected output:
```rust
use std::collections::VecDeque;

fn moving_avg(values: &[f64], window: usize) -> Vec<f64> {
    if window > values.len() {
        return Vec::new();
    }
    let mut deque: VecDeque<f64> = VecDeque::new();
    let mut running_sum: f64 = 0.;
    let mut result: Vec<f64> = Vec::new();

    for &val in values {
        deque.push_back(val);
        running_sum += val;

        if deque.len() == window {
            result.push(running_sum / window as f64);
            let removed = deque.pop_front().unwrap();
            running_sum -= removed;
        }
    }
    result
}

fn main() {
    let values = &[1.0, 2.0, 3.0, 4.0, 5.0];
    for avg in moving_avg(values, 3) {
        println!("{:.1}", avg);
    }
}
```


Implement the following article and tweet structs and trait:

Define a trait Summary with a single method summary(&self) -> String.
Define two structs: Tweet { user: String, text: String } and Article { title: String, body: String }.
Implement Summary for the two structs.
The tweet summary should be "@user: text", and
the article summary should be "title — first 20 chars of body"
(use .chars().take(20).collect::<String>() to get the first 20 characters)
In main, create:
A Tweet with user "alice" and text "hello rust!"
An Article with title "DS210" and body "Rust is a systems language."
Call the summary method for each and print the result.
Hint: Use the format! macro to create the summary strings.

Expected output:
@alice: hello rust!
DS210 — Rust is a systems language.
```rust
trait Summary {
    fn summary(&self) -> String; 
}
struct Tweet {
    user: String, 
    text: String, 
}

struct Article {
    title: String,
    body: String, 
}

impl Summary for Tweet {
    fn summary(&self) -> String {
        format!("@{}: {}", self.user, self.text)
    }
}

impl Summary for Article {
    fn summary(&self) -> String {
        let blurb = self.body.chars().take(20).collect::<String>(); 
        format!("{} – {}", self.title, blurb)
    }
}

fn main() {
    let tweet: Tweet = Tweet { user: String::from("alice"), text: String::from("hello rust!") }; 
    let article: Article = Article { title: String::from("DS210"), body: String::from("Rust is a system language.") } ; 
    println!("{}", tweet.summary()); 
    println!("{}", article.summary()); 
        
}

```


Write:

a function kth_largest(nums: &[i32], k: usize) -> Option<i32> that returns the k-th largest value in the slice (1-indexed).
If k is zero or larger than the slice length, return None.
Use a BinaryHeap to find the answer
Call the function from main with &[7, 2, 9, 4, 11, 3] and k = 3
Print the result with {:?}.
Expected output:

```rust
use std::collections::BinaryHeap;
fn kth_largest(nums: &[i32], k: usize) -> Option<i32> {
    if k > nums.len() || k == 0  {
        return None; 
    }
    let mut heap = BinaryHeap::from(nums.to_vec());
    let mut result = None; 
    for _ in 0..k {
        result = heap.pop(); 
    }
    result
}

fn main() {
    println!("{:?}", kth_largest(&[7, 2, 9, 4, 11, 3], 3)); 
}

```

Write a program that:

prints every city whose temperature is above 75°F,
with its temperature converted to Celsius (formula: (f - 32.0) * 5.0 / 9.0),
one per line.
Sort the output alphabetically by city.
Use iterator methods (filter, map, collect, sort_by_key, ...) rather than explicit loops.
Each printed line should look like Phoenix 39.2 (temperature formatted to one decimal place).


    reading_vec.iter().for_each(|x| println!("{} {:.1}", x.0, x.1)); 

```rust
fn main() {
    let readings = vec![
        ("Boston", 48.0_f64), ("Phoenix", 102.5),
        ("Denver", 77.0), ("Miami", 88.0), ("Anchorage", 36.0),
    ];
    
    let mut reading_vec = readings
        .into_iter()
        .filter(|(_city, temp)| *temp > 75.)
        .map(|(city, temp)| (city, (temp - 32.0) * 5.0/9.0) )
        .collect::<Vec<(_, _)>>(); 
    
    reading_vec.sort_by(|a, b| a.0.cmp(&b.0)); 
    reading_vec.iter().for_each(|x| println!("{} {:.1}", x.0, x.1)); 
}
```



Define a struct Rectangle { width: u32, height: u32 } with three methods:

area(&self) -> u32
perimeter(&self) -> u32
can_hold(&self, other: &Rectangle) -> bool — returns true if self is strictly larger than other in both dimensions
In main, create let a = Rectangle { width: 10, height: 5 }; and let b = Rectangle { width: 4, height: 3 };. Print a.area(), a.perimeter(), and a.can_hold(&b) each on their own line.

```rust
struct Rectangle { 
    width: u32, 
    height: u32 
} 

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height 
    }
    fn perimeter(&self) -> u32 {
        (self.width * 2) + (self.height * 2)
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        if self.area() > other.area() && self.perimeter() > other.perimeter() {
            true
        } else {
            false
        }
    }
}

fn main() { 
    let a = Rectangle { width: 10, height: 5 }; 
    let b = Rectangle { width: 4, height: 3 }; 
    println!("Area of a: {}", a.area()); 
    println!("Perimeter of a: {}", a.perimeter()); 
    println!("A can hold B: {}", a.can_hold(&b)); 
} 

```

Implement the following:

You are given let scores = vec![("alice", 90), ("bob", 75), ("alice", 82), ("carol", 88), ("bob", 91), ("alice", 79)];
Using the Entry API on a HashMap<&str, (i32, i32)> (where the value is (total_points, count)),
compute each student's average.
Then print a line for each student in alphabetical order, formatted name avg with the average rounded to one decimal place.
Expected output:


alice 83.7
bob 83.0
carol 88.0

```rust
use std::collections::HashMap; 

fn main() {
    let scores = vec![("alice", 90), ("bob", 75), ("alice", 82), ("carol", 88), ("bob", 91), ("alice", 79)];
    
    let mut map: HashMap<&str, (i32, i32)> = HashMap::new(); 
    
    for (name, score) in scores {
        let entry = map.entry(name).or_insert((0, 0));  
        entry.0 += score; 
        entry.1 += 1; 
    }
    
    map.iter().for_each(|(name, (score, count))| {
        println!("{} {:.1}", name, *score as f64 / *count as f64)
    })
}
```