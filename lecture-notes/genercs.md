# Generics 

1. We can define generic types in structs and method call functions. Generic types are basically ways to allow a variable to have any types to pass through

    - This could be efficient by removing duplicate codes. A find largest value function can be written for both parsing and finding the highest-valued integer in a Vec<T> and characters in a list[T]. If the function has a parameter that it takes with a   type is T, they can take in both vectors/arrays/list of characters or numbers to perform operations. 

2. An example of a commonly used generic data type is 

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
// Result could be a number, a sentence, or something else, where as error could be an Exception enum, a string, or something else. It enables all of it to get passed 
```

3. Generics won't reduce code performance because of a process known as *monomorphization of code generics*

    -  When Rust compiles the code, Rust will look for all the cases where a generic data type is used, check for how it is defined, i.e. as an f32, f65, String or any other way, and then expand the code to support multiple specific data types 
        - Convert use of generic data types -> specific data types (all done in the background) 

        e.g. 

        we do this... 
        
        ```rust
        let integer = Some(5);
        let float = Some(5.0);
        ```

        but Rust actually does something similar to this in the compiled file 
         ```rust
        enum Option_i32 {
            Some(i32),
            None,
        }   

        enum Option_f64 {
            Some(f64),
            None,
        }

        fn main() {
            let integer = Option_i32::Some(5);
            let float = Option_f64::Some(5.0);
        }
        ```
