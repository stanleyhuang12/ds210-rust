fn main() {
    let x = 12; 
    let y = -5; 
    
    println!("12: {:08b}", {x});
    println!("-5: {:08b}", {y});
    
    println!("12 & -5: {:08b}", {x & y}); 
    println!("12 & -5: {:08b}", {x | y});
    println!("!12: {:08b}", {!x}); 
    println!("!-5: {:08b}", {!y}); 
}