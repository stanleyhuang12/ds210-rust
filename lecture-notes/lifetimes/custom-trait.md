
## Custom Trait Definitions 

```rust
trait Person {
    // method header specifications
    // must be implemented by any type that implements the trait
    fn get_name(&self) -> String; // getter methods 
    fn get_age(&self) -> u32; // getter methods

    // default implementation of a method
    fn description(&self) -> String {
        format!("{} ({})",self.get_name(),self.get_age())
    }
}
```

- get_name and get_age any struct that implements this trait needs to define these get_name and get_age 
- get_description is default implementation 

we can have a function signature that takes in a specific struct that implements a specific trait 

```rust
// sample function accepting object implementing trait
fn long_description(person: &impl Person) {
    println!("{}, who is {} years old", person.get_name(), person.get_age());
}

```
