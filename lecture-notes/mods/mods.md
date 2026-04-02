# Modules help us define external interfaces

We can expose model APIs publicly with using `pub` and we can keep certain interfaces private with `private`. We create a namespace by using mods (modularizing code), we can be deliberate with what we expose

```rust
mod things_to_say {
    pub fn say_hi() {
        say("Hi");
    }
    
    pub fn say_bye() {
        say("Bye");
    }
    
    fn say(what: &str) {
        println!("{}!",what);
    }
}
fn main() {
    things_to_say::say_hi(); 
}
```


In semantic versioning (`2.0.1`), major changes (the first date) will introduce incompatible API-breaking changes that breaks programs. Minor numbers add functionality in a backwards-compatible manner (i.e., adding new APIs, change a bit of the function) [2nd number], patch makes bug fixes [3rd number]

# Nesting modules 
- Nested modules are by default private and so we have to make sure it is public first appending `pub mod` 

```rust
mod level_1 {

    pub mod level_2_1 {

        pub mod level_3 {

            pub fn where_am_i() {println!("3");}

        }

        pub fn where_am_i() {println!("2_1");}
        
    }
    
    pub mod level_2_2 {
        
        pub fn where_am_i() {println!("2_2");}
        
    }
    
    pub fn where_am_i() {println!("1");}
    
}

fn main() {
    level_1::level_2_2::where_am_i();
}

```

We can search in the namespace of the module by doing `super::` [1 level up] and `super::super::` [2 levels up]
We can return to the 1st level by calling `crate::`


# Creating public accessor function on private data 
```rust
pub mod test { 
    #[derive(Debug)]
    pub struct Point {
       x: i32, // makes x private 
       pub y: i32, // makes y public so we can update via point.y but not point.x 
    }

    impl Point {
        pub fn create(x:i32,y:i32) -> Point {
            Point{x,y}
        } // a public function 

        pub fn update_x(*mut self, x:i32) { // we create a mutable reference so the ownership doesn't transfer to the variables inside the function 
            self.x = x; 
        }
    }

}

use test::Point;

fn main() {
    let mut p = Point::create(2,3);
    println!("{:?}",p);

    p.x = 3;  // Error: try commenting this out
    p.y = 4;  // Why does this work? we can access indirectly 
    p.update_x(3); // this will work 
    println!("{:?}",p); 
}

```