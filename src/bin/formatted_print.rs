use std::fmt;

struct Rectangle(i32, i32);

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} +{}i", self.real, self.imag)
    }
}

struct List(Vec<i32>);
impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", count, v)?;
        }
        write!(f, "]")
    }
}

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
    println!("{:.3}", pi);

    println!("{}", Rectangle(3, 4));

    println!("{:?}", Complex{ real: 3.3, imag: 7.2 });
    println!("{}", Complex{ real: 3.3, imag: 7.2 });

    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}