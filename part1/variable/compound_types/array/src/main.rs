fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    // Access by index
    println!("{}", arr[2]);

    // Array with repeated value
    let zero_index: [i32; 10] = [0; 10];
    println!("{:?}", zero_index); 

    //length
    println!("lenght = {}", arr.len());

    // iterartion
    for element in arr {
        print!("element: {} ", element);
    }
    
}
