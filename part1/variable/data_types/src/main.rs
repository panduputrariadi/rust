fn main() {
    let x: i32 = 5;
    let y: f64 = 3.14;
    let z: bool = true;
    let name: &str = "Bob";

    println!("x (int): {}, y (float): {}, z (bool): {}, name (string): {}", x, y, z, name);

    let n: i32 = "42".parse().unwrap(); // must tell Rust what type to parse to
    println!("parsing string to int: {}", n);

    const MAX_SCORE: u32 = 100;// must have type annotation
    const PI: f64 = 3.14159265358979;
    println!("contant variable: {}, {}", MAX_SCORE, PI);
}
