## Methods 

1. Define methods within impl blocks for structs
2. Understand the role of self in method definitions
3. Create associated functions that don't take self
4. Use methods to encapsulate behavior with data
    - encapsulation means bundling up data with similar attributes and method calls into an abstraction 
5. Apply method syntax for cleaner, more readable code


## Implementation is like a class in Python 

```rust 

struct Point { 
    x: f64, 
    y: f64,
}
struct Rectangle {  // store upper left and lower right points
    p1: Point,
    p2: Point,
}

impl Rectangle 
    fn area(&self) { 
        // some function 
        self.p1 
        self.p2 

        return // some value .. this example is a void function bc it is just a skeleton code
    }

    fn some_additional_method_call(&self) {
        // we can access self.p1 and self.p2 
    }

fn main() {
    let rectangle = Rectangle {
        p1: Point{x:0.0, y:0.0},
        p2: Point{x:3.0, y:4.0},
    };

    println!("Rectangle perimeter: {}", rectangle.area());
}



```

* Always call &self because it gives us attribute to the specific instance of the struct 
* We can access the attribute of the struct by doing self.p1 and self.p2 
* The method call can be access like self.some_additional_method_call() 


* We can define a builder function (which is like the init method in Python) 

```python

class Rectangle: 
    def __init__(p1, p2): 
        self.p1 = p1 
        self.p2 = p2 
```

``` rust 
struct Point { 
    x: f64, 
    y: f64,
}
struct Rectangle {  // store upper left and lower right points
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn new() -> Self { // this does not take self as a function 
        Rectangle {
            p1: 32.0 // default initializaation 
            p2: 54.0 // default initialization 
        }
    }

    fn find_area(&self) {
        // some defined method call 
    }
}


```