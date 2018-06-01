struct BinarySearch {}

impl BinarySearch {
    fn rank(key: i32, a: &[i32]) {
        let lo: i32 = 0;
        let hi: i32 = a.len() as i32 - 1;
        println!("{} {} {}", lo, hi, key);
    }
}

fn main() {
    BinarySearch::rank(2, &[2, 3, 4]);
}
