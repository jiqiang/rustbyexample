fn main() {
    let a = 1;
    {
        let b = 2;
        println!("{}", b);
        let a = 3;
        println!("{}", a);
    }
    //println!("{}", b);
    println!("{}", a);
    let a = 4;
    println!("{}", a);
}
