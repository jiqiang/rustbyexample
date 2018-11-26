fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];
    println!("2 in v1: {}", v1.iter().any(|&x| x == 2));
    println!("2 in v2: {}", v2.into_iter().any(|x| x == 2));

    let a1 = [1, 2, 3];
    let a2 = [4, 5, 6];
    println!("2 in a1: {}", a1.iter().any(|&x| x == 2));
    println!("2 in a2: {}", a2.iter().any(|&x| x == 2));
}
