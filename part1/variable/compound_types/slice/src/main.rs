fn main() {
    let array: [i32; 4] = [1,2,3,4];

    let slice: &[i32] = &array[1..3];
    let from_start: &[i32] = &array[..3];
    let until_end: &[i32] = &array[2..];
    let whole: &[i32] = &array[..];

    println!("{:?}", slice);
    println!("from start:{:?}", from_start);
    println!("until end from spesific index:{:?}", until_end);
    println!("whole slice:{:?}", whole);
    println!("length = {}", slice.len());

    fn summary(sum: &[i32]) -> i32 {
        let mut total: i32 = 0;
        for &x in sum {
            total += x;
        }
        total
    }

    println!("sum = {}", summary(&array));
    println!("sum = {}", summary(&array[1..4]));
}
