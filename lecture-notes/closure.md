

# Closures 


### Closures are anonymous, in-line functions that 

- They have lexical scope (in-line scope)
- Do something if it is relevant 
- Syntax: |params| {the closure line}
- Lazily evaluated. In other words, the closure does not create outputs until it is explicitly called 
- Performs type inference once and then locks the parameter or return type to a specific typing
    
e.g. 
```rust
store.giveaway.unwrap_or_else(|count| self.most_stocked_for(count))
```
e.g.
```rust
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
```

### Closures are like functions

```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;

let res = add_one_v4(5); 

```

### Compile infer types for closure during compile time.

```rust
let example_closure = |x| x;

let s = example_closure(String::from("hello"));
let n = example_closure(5);
// example_closure(5) would error because it is not a String as inferred in the line before the last line 
```

### Move with closures 

Move transfers the ownership to the closure functions 
Move is important because the closure might outlive the scope (?)

### Closures can implement three different fn traits 

1. FnOnce: only allows the closure to be applied once 
2. FnMut: does not move the value to the closure.. since this function likely only mutates the data 
3. Fn: closures that do not need ownership of the data or mutate captured value 