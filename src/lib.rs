pub mod client;
pub mod network;

pub struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        client::connect();
    }

    #[test]
    fn it_must_work() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { height: 32, width: 32 };
        let smaller = Rectangle { height: 30, width: 30 };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_hold_larger() {
        let larger = Rectangle { height: 32, width: 32 };
        let smaller = Rectangle { height: 30, width: 30 };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(5, add_two(3));
    }
}
