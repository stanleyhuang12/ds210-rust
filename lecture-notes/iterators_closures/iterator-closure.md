# Iter - Closures 


### filter 
```rust
let not_divisible_by_3 : Vec<_> = (0..10).filter(|x| x % 3 != 0).collect();
println!("{:?}", not_divisible_by_3);
// takes values between 0-9 inclusive and filter for cases where x is a multiple of 3 and collect it into a vector 
```

`.filter` returns an iterator that needs to be collected 

### map 
```rust 
let fibonacci_squared: Vec<_> = Fib::new().take().map(|x| x*x).collect()
```

### any 

Any returns a boolean


### fold 
Fold takes -> values and accumulates and returns a value 
```rust 
(1..=10).fold(bias, |acc, x| acc + x * x) // accumulates a single value  returns a value 
```

```rust 
enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let moves = vec![Direction::North, Direction::East, Direction::North, Direction::West];

    let pos = moves.iter().fold((0, 0) |acc, x| {

        match {    
            Direction::North => (x, y + 1), // move north (increase y)
            Direction::South => (x, y - 1), // move south (decrease y)
            Direction::East  => (x + 1, y), // move east (increase x)
            Direction::West  => (x - 1, y), // move west (decrease x)
        }

    })

}

```


### reduce 
- we can go through a sequence and accumulate a value without having to use a bias 
- returns option enum because it can run on an empty iterator, if we call it--we will get a no value back 
- fold still has an initialization value and we'll get an init value back 
- `unwrap` will panic if there is a none value.. we need to call the unwrap_or_else





