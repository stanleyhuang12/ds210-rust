struct Container<T> {
    value: T,
}

impl<T:Copy> Container<T> {
    // TODO: Implement a constructor that creates a new Container
    fn new(value: T) -> Container<T> {
        Container { 
            value
        }
    }

    // TODO: Implement a method that returns a reference to the value
    fn get(&self) -> &T {
        &self.value 
    }

    // TODO: Implement a method that replaces the value and returns the old one
    fn replace(&mut self, new_value: T) -> T {
        // Your code here
        let old_value = self.value; 
        self.value = new_value; 
        old_value 
    }
}

fn main() {
    let mut container = Container::new(42);
    println!("Value: {:?}", container.get());

    let old_value = container.replace(100);
    println!("Old value: {}, New value: {:?}", old_value, container.get());
}