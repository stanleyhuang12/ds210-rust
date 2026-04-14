
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


#### TODO: 
```rust
// TODO: Define the Describable trait
trait Describable {
    fn describe(&self) -> String; 
}

struct Book {
    title: String,
    author: String,
    pages: u32,
}

impl Describable for Book { 
    fn describe(&self) -> String {
        format!("'{}' by {} ({} pages)", self.title, self.author, self.pages)
    }
} 

fn main() {
    let book = Book {
        title: String::from("The Rust Book"),
        author: String::from("Steve Klabnik"),
        pages: 500,
    };

    println!("{}", book.describe());
}
```


## trait bounds 
```rust
use std::fmt::Debug;

// three options, useful for different settings

// This is good if you want to pass many parameters to the function
// and the parameters are of different types
fn multiple_1(person: &(impl Person + Debug)) {
    println!("{:?}",person);
    println!("Age: {}",person.get_age());
}

// This is better if you want all your parameters to be of the same type
fn multiple_2<T: Person + Debug>(person: &T) {
    println!("{:?}",person);
    println!("Age: {}",person.get_age());
}

// This is like option 2 but easier to read if your parameter
// combines many traits
fn multiple_3<T>(person: &T)
    where T: Person + Debug
{
    println!("{:?}",person);
    println!("Age: {}",person.get_age());
}


    multiple_1(&mlk);
    multiple_2(&mlk);
    multiple_3(&mlk);

```