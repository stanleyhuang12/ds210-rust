fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    
    // Task 1: Calculate sum without taking ownership
    let total = calculate_sum(&numbers);
    
    // Task 2: Double each number in the vector
    double_values(&mut numbers);
    
    // Task 3: Print both the original and doubled values
    println!("Original sum: {}", total);
    println!("Doubled values: {:?}", numbers);
    
    // Task 4: Add new numbers to the vector
    add_numbers(&mut numbers, vec![6, 7, 8]);
    println!("After adding: {:?}", numbers);
}

fn calculate_sum(v: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for num in v {
        sum += num;
    }
    sum
}

fn double_values(v: &mut Vec<i32>) {
    for num in v {
        *num *= 2;
    }
}

fn add_numbers(v: &mut Vec<i32>, new_nums: Vec<i32>) {
    for num in new_nums {
        v.push(num);
    }
}

/*
For the first operation `calculate_sum`, we only needed an immutable reference because we aren't trying to make underlying modifications of the data. We just need to read out the values. So I added a & before the variable and changed the typing to reflect it. 

The second operation `double_values` required a modification of the data. Thus, I use &mut so that the numbers ownership does not get passed into the function which could lead to an out of scope error later when the function exits without returning the modification of the data. &mut enables "writing" or modifying the data (on the heap, in this case since it is a Vec<T>) . I also used *n to "dereference" and get the actual values of the vector instead of the pointer value. 

Finally, similar logic applies for the last operation but does not require dereferencing because it is not referencing to an individual elements within a mutable reference. 

*/