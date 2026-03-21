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
    fn new() -> Self { // this does not take self as a function as it is a constructor 
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


# Methods II 

1. Distinguish between self, &self, and &mut self parameter types
2. Understand when methods take ownership vs. borrow references
3. Design method APIs that appropriately handle ownership and mutability
4. Apply method call syntax with different reference types
5. Recognize the implications of different self parameter choices


```rust
#[derive(Debug)]
struct Road {
    intersection_1: u32,
    intersection_2: u32,
    max_speed: u32,
}

impl Road {
    // constructor
    fn new(i1:u32,i2:u32,speed:u32) -> Road {
        Road {
            intersection_1: i1,
            intersection_2: i2,
            max_speed: speed,
        }
    }
    // note &self: immutable reference
    fn display(&self) {
        println!("{:?}",*self);
    }
}
// You can invoke the display method on the road instance
// or on a reference to the road instance.

fn main() {
    let mut road = Road::new(1,2,35);

    road.display();
    &road.display(); 
    (&road).display();
}
```


```rust
impl Road {
    fn update_speed(&self, new_speed:u32) {
        self.max_speed = new_speed; 
    }
} // WILL ERROR 
```

- When we pass &self as the first parameter to a method API call, the underlying Struct is **not modifiable**. 

- When we pass &mut self, then the underlying Struct is modifiable. 

- passing `self` or `&self`  as parameters: 
    - self will move ownership to inside the function 
    - &self will not move ownership but provide an immutable reference 

- We initialize an implementation 
```rust 

Student::new() and pass in the parameters 
```

```rust
#![allow(unused)]

#[derive(Debug)]
struct Student {
    name: String,
    id: u64,
    grades: Vec<f32>,
    num_grades: i32,
}

impl Student {
    fn new(name: String, id: u64) -> Student {
        Student {
            name,
            id,
            grades: Vec::new(),
            num_grades: 0,
        }
    }

    fn add_grade(&mut self, grade: f32) {
        self.grades.push(grade);
        self.num_grades += 1;
    }

    fn average(&self) -> f64 {
        let sum: f64 = self.grades.iter().map(|&g| g as f64).sum(); // we need to dereference each grade value 
        if self.num_grades == 0 {
            0.0
        } else {
            sum / self.num_grades as f64
        }
    }

    fn letter_grade(&self) -> &str {
        let average = self.average();
        if average >= 90.0 {
            "A"
        } else if average >= 80.0 {
            "B"
        } else if average >= 70.0 {
            "C"
        } else {
            "F"
        }
    }

    fn display(&self) {
        println!("{:?}", self);
        println!("Average: {:.2}", self.average());
        println!("Grade: {}", self.letter_grade());
    }
}

fn main() {
    let mut student = Student::new(String::from("Stanley"), 12456);

    student.add_grade(85.5);
    student.add_grade(92.0);
    student.add_grade(78.5);
    student.add_grade(88.0);

    student.display();
    println!();
}