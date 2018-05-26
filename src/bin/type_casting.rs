fn main() {
    let a: f32 = 45.678;
    let b: u8 = a as u8;
    let c: char = b as char;
    println!("casting {} -> {} -> {}", a, b, c);

    println!("1000 as a u16 is: {}", 1000 as u16);
    println!("1000 as a u8 is : {}", 1000 as u8);
    println!("  -1 as a u8 is : {}", (-1i8) as u8);
    println!("1000 mod 256 is : {}", 1000 % 256);
    println!(" 128 as a i16 is: {}", 128 as i16);
    println!(" 128 as a i8 is : {}", 128 as i8);
    println!("1000 as a u8 is : {}", 1000 as u8);
    println!(" 232 as a i8 is : {}", 232 as i8);
}