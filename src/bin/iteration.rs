fn main() {
    let v = vec!( ("Herman".to_string(), 5));
    let names: Vec<String> = v.into_iter().map(|(name, _score)| name).collect();
    assert_eq!(names, ["Herman"]);
    // into_iter() moves value so v is droped
    //println!("{:?}", v);
}
