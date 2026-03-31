
// use fib_rs::Fib;

fn main() {
    let odd: Vec<_> = (1..=19).filter(|x| x % 2 != 0).collect(); 
    println!("{:?}", odd); 

    // let fibonacci: Vec<_> = Fib::new().iter().take(15).collect(); 
    // println!("{:?}", fibonacci); 

    let some: Vec<_> = (1..=20).filter(|x| x % 3 == 0).map(|x| x*2 ).collect(); 
    println!("{:?}", some)

}