fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let x = String::from("Glenn is a man");
    let y = "Joy is a woman";
    let z = longest(x.as_str(), y);
    println!("{}", z);
}
