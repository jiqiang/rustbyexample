fn main() {
    println!("{}", 32);
    println!("{1} {0}", 2, 3);
    println!("{glenn} {joy}", joy=37, glenn=40);
    println!("{:b} {:b}", 2, 3);
    println!("{number:>width$}", number=1, width=6);
    println!("{number:>0width$}", number=1, width=6);

    #[derive(Debug)]
    struct Structure(i32);
    println!("{:#?}", Structure(3));

    let pi = 3.1415926;
    println!("{:.*}", 3, pi);
}