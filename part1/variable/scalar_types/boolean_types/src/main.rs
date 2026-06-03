fn main() {
    let t: bool = true;
    let f: bool = false;

    println!("{}", t && f);
    println!("{}", t || f);
    println!("{}", !t);

    let x: i32 = 5;
    let is_big: bool = x > 3;
    let is_ten: bool = x == 10;

    println!("is it big? {}", is_big);
    println!("is it ten? {}", is_ten);
}
