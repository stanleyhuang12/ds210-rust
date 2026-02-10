fn main() {
 let n: u8 = 77;
 let x: f32 = 1.25;
 
 println!("{}", n);
 println!("{}", x);
 let n = n as f64; 
 let x = x as f64;
 println!("{} x {} = {}", n, x, n*x);
}