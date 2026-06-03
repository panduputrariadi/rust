fn main() {
    let x = 5;
    println!("x = {}", x);  // x = 5

    let x = x + 1; // new variable 'x', shadows the old one
    println!("x = {}", x);  // x = 6

    {
        let x = x * 2;// shadows only within this block
        println!("x = {}", x);// x = 12
    }

    println!("x = {}", x);// x = 6  (inner shadow gone)
}
