use::std::collections::HashSet; 

fn main() {
    
    let mut developer1 = HashSet::new(); 
    let developer1_knows = ["Rust", "Python", "JavaScript", "C++", "Go"]; 
    for language in developer1_knows {
        let status = developer1.insert(language); 
        if status == false { 
            println!("Failed to add {}", language); 
        }
    }

    let mut developer2 = HashSet::new(); 
    let developer2_knows = ["Python", "Java", "JavaScript", "Ruby", "Go"]; 
    for language in developer2_knows {
        let status = developer2.insert(language); 
        if status == false { 
            println!("Failed to add {}", language); 
        }
    }

    let pintersection = developer1.intersection(&developer2).collect::<Vec<_>>(); 
    let pdiff = developer1.difference(&developer2).collect::<Vec<_>>(); 
    let punion = developer1.union(&developer2).collect::<Vec<_>>(); 
    let psymdiff = developer1.symmetric_difference(&developer2).collect::<Vec<_>>(); 

    println!("Intersection: {:?}", pintersection); 
    println!("Difference: {:?}", pdiff); 
    println!("Union {:?}", punion); 
    println!("Symmetric Difference {:?}", psymdiff); 

}