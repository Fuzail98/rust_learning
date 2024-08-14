fn main() {
    // i8: 0 - 255
    // u8: -128 - 127
    let a: u8 = 1; 
    let b= 10_i8; 
    let c = 18 as i8;
    let d = 9i8;

    let e = 127_000_i32;
    println!("{} {} {} {} {}", a, b, c, d, e);

    let z = (a as i8) + b + c;
    println!("{}", z)
}