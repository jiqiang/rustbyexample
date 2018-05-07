fn main() {
    let a;
    {
        let b = 1;
        a = b + 1;
        println!("{}", b);
    }

    //let c;
    //println!("{}", c);
    println!("{}", a);
}
