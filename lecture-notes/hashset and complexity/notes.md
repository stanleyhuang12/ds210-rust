#### amoritized inference

- initialize a vectory with properly allocated space from the beginning for o(1)

### Hash maps 

The `insert` method enables us to overwrite the method 
```rust

wins.insert(String::from("Boston University"),24);

wins.entry(String::from("Boston University")).or_insert(24)

// update the value needs to dereference 
let entry = wins.entry(String::from("Boston University")).or_insert(10);
*entry += 50;
```

The `entry` method first checks if the key is present which returns an Option<&v> which either returns Some(&V) or None. 

The `get` function tries to retrieve a value returned in the Option enum

Entry provides a mutable reference. Powerfully you can `or_insert` which will insert depending on on whether there is an Some or None 
