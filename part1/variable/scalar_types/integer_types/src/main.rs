fn main() {
    let a: i32 = -42;
    let b: u8 = 255;
    let c: i64 = 9_000_000_000; // underscores for readability
    let d: usize = 42;

    // Integer literals in different bases:
    let decimal     = 98_222;    // decimal (underscores ignored)
    let hex         = 0xff;      // hexadecimal
    let octal       = 0o77;      // octal
    let binary      = 0b1111_0000; // binary
    let byte        = b'A';      // byte literal — u8 only

    println!("{} {} {} {} {} {} {} {} {}", a, b, c, d, decimal, hex, octal, binary, byte);

    let x: u8 = 255;
    let y: u8 = x.wrapping_add(1);    // y = 0
    let z: u8 = x.saturating_add(1);  // z = 255 (saturates at max)
    let w: Option<u8> = x.checked_add(1);     // w = None (returns Option)

    println!("w: {:?}",w); // w return none
    println!("{}, {}, {}", x, y, z)
}