pub mod client;
pub mod network;

#[cfg(test)]
mod tests {
    use super::client;
    #[test]
    fn it_works() {
        client::connect();
    }

    #[test]
    fn it_must_work() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}
