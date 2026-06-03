fn main() {
    let tup: (i32, f64, bool, char) = (500, 6.4, true, 'z');

    // Access by index (0-based):
    let five_hundred: i32 = tup.0;
    let six_point_four: f64 = tup.1;
    let is_true: bool = tup.2;
    let char: char = tup.3;

    println!("{} {} {} {}", five_hundred, six_point_four, is_true, char);

    // Destructuring:
    let (x, y, z, w) = tup;
    println!("{} {} {} {}", x, y, z, w);

    // Nested tuple
    let nested: ((i32, i32), (i32, i32)) = ((1, 2), (3,4));
    println!("{}", nested.0.1);
    println!("{}", nested.1.0);

    let nums: [i32; 8] = [3, 1, 4, 1, 5, 9, 2, 6];
    let (min, max) = min_max(&nums);
    println!("min={}, max={}", min, max);
}

fn min_max(numbers: &[i32]) -> (i32, i32) {
    let min: i32 = *numbers.iter().min().unwrap();
    let max: i32 = *numbers.iter().max().unwrap();

    (min, max)
}
