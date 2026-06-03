fn main() {
    let x: f64 = 2.0;
    let _y: f32 = 3.0;

    let sum: f64 = x + 2.5;
    let difference: f64 = x - 1.0;
    let product: f64 = x * 3.0;
    let quotient: f64 = x / 2.0;
    let remainder: f64 = 10.0_f64 % 3.0;

    println!("sum: {}, difference: {}, product: {}, quotient: {}, remainder: {}", sum, difference, product, quotient, remainder);

    let pi: f64 = std::f64::consts::PI;
    println!("sin(π/2) = {}", (pi / 2.0).sin());
    println!("sqrt(2) = {}", 2.0_f64.sqrt());
    println!("2^10 = {}", 2.0_f64.powi(10));
}
