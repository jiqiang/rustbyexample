struct BinarySearch {}

impl BinarySearch {
    fn rank(key: i32, a: &[i32]) -> i32 {
        let mut lo: i32 = 0;
        let mut hi: i32 = a.len() as i32 - 1;
        while lo <= hi {
            let mid: i32 = lo + (hi - lo) / 2;
            println!("{} {} {}", a[lo as usize], a[mid as usize], a[hi as usize]);
            if key < a[mid as usize] {
                hi = mid - 1;
            } else if key > a[mid as usize] {
                lo = mid + 1;
            } else {
                return mid;
            }
        }
        -1
    }
}

fn main() {
    let result = BinarySearch::rank(2, &[2, 4, 6, 8, 10, 12, 14, 16, 18]);
    println!("{}", result);
}
