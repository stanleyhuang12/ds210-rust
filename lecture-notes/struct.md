### Structs

Notes that I recalled: 

Structs are sort of like tuples as they can group different types of data. They allow us to define the name of each of the values. 

*We define a Struct like* 
```rust

struct Name {
    firstname: String, 
    lastname: String, 
    other_attribute: bool
}
```


We can access structs by use Name.firstname, Name.lastname, and Name.otherattribute (it is like a method call)

We can destructure structs 

```rust 
struct Name {
    firstname: String, 
    lastname: String, 
    other_attribute: bool,
}

fn main() {
    let name = Name {
        firstname: String::from("Stanley"),
        lastname: String::from("Huang"),
        other_attribute: false,
    };

    println!("{}", name.firstname);

    // Destructure
    let Name { firstname, lastname, other_attribute } = name;

    println!("{}", firstname);
}
```

```rust

struct Student {
    name: String, 
    homework: f64, 
    midterm: f64, 
    final_exam: f64
}

fn main() {
    let student: Student = Student{name:String::from("Stanley"), homework:75.5, midterm:78.9, final_exam:95.87 };  
    let average: f64 = (student.homework * 0.30) + (student.midterm * 0.30) + (student.final_exam *0.30); 
    let mut passed; 
    
    if average >= 0.60 {
        passed = "passed"; 
    } else {
        passed = "failed"; 
    }
    println!("{} averaged an {:.1} and {}", student.name, average, passed)
    
                
}
```