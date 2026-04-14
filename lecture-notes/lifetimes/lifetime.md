



- Dangling reference errors are references that points to
 the reference that does not exist anymore 


```rust
let s: &'static str = "I have a static lifetime.";

// static str live for the scope of the entire programming  
```

Also including combining lifetimes with generic types 

```rust
fn longest_with_announce<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,   // T must implement the Display trait
```













