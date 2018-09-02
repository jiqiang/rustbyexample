fn main() {
    let v: Vec<i32> = vec![1, 2, 3];

    match v.get(2) {
        Some(i) => {
            println!("{}", i);
        },
        None => {
            println!("index is out of range");
        }
    }

    // Create an empty string
    let mut s = String::new();
    s.push_str("Glenn");

    let s1 = String::from("Glenn ");
    let s2 = String::from("Ji");
    let s3 = s1 + &s2;
    println!("{}", s3);

    let s4 = String::from("Joy ");
    let s5 = String::from("Li");
    let s6 = format!("{}{}", s4, s5);
    println!("{}", s6);

    for c in "我是一個中國人".chars() {
        println!("{}", c);
    }
}
