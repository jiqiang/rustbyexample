use std::fmt;
use std::mem;

struct Matrix(f32, f32, f32, f32);
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})\n({}, {})", self.0, self.1, self.2, self.3)
    }
}

fn transpose(m: Matrix) -> Matrix {
    Matrix(m.0, m.2, m.1, m.3)
}

fn main() {
    println!("1 - 2 = {}", 1i32 -2);
    println!("{}", 0b0011u32 & 0b0101);
    println!("{}", 1u32 << 5);
    println!("{}", 1_000_000u32);

    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    println!("{}", Matrix(1.1, 1.2, 2.1, 2.2));
    println!("{}", transpose(Matrix(1.1, 1.2, 2.1, 2.2)));

    let a: [i32; 5] = [0; 5];
    println!("{:?}", a);
    println!("{}", a.len());
    println!("{}", mem::size_of_val(&a));

    let slice = &a[1..4];
    println!("{:?}", slice);
}
