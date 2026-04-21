fn main() {
    use std::collections::HashMap;

    let mut countmap = HashMap::new(); 
    let value = "rust is awesome rust is fast rust is safe"; 

    for v in value.split_whitespace() {
        let current_count = countmap.entry(v).or_insert(0); 
        *current_count += 1
    }

    println!("{:?}", countmap)
}