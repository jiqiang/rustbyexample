fn iterator1() {
    let v = vec![1, 2, 3];
    let mut v_iter = v.iter();
    println!("{:?}", v_iter.next());
    println!("{:?}", v);
}

fn iterator2() {
    let v = vec![1, 2, 3];
    let mut v_iter = v.into_iter();
    println!("{:?}", v_iter.next());
    //println!("{:?}", v);
}

fn iterator3() {
    let mut v = vec![1, 2, 3];
    let mut v_iter = v.iter_mut();
    match v_iter.next() {
        Some(i) => {
            *i += 1;
        }
        None => {
            println!("{}", -1);
        }
    }
}

fn main() {
    iterator1();
    iterator2();
    iterator3();
}
