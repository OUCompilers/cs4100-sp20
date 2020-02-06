use std::str::FromStr;

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    //An "inherent" method of the Point type
    fn point_to_string(&self) -> String {
        format!("(x: {}, y: {})", self.x, self.y)
    }
}

//A trait (interface) describing types with a 'to_string' operation
trait ToString {
    fn to_string(&self) -> String;
}

//Point implements ToString via point_to_string
impl ToString for Point {
    fn to_string(&self) -> String {
        self.point_to_string()
    }
}

//Generically print any value with a ToString implementation
fn generic_print<T: ToString>(t: &T) {
    println!("Generically printing: {}", t.to_string())
}

#[derive(Debug, PartialEq)]
enum Instr {
    Add,
    Sub
} 

use Instr::{Add, Sub};

impl FromStr for Instr {
    type Err = ();
    fn from_str(s: &str) -> Result<Instr, ()> {
        match s {
            "add" => Ok(Add),
            "sub" => Ok(Sub),
            _ => Err(()),
        }
    }
}

//Do TopHat question

impl ToString for Instr {
    fn to_string(&self) -> String {
        match self {
            Add => format!("add"),
            Sub => format!("sub"),
        }
    }
}

#[test]
fn tofromstr_instr() {
    assert_eq!(Instr::from_str(&Instr::to_string(&Add)).unwrap(), Add);
    assert_eq!(Instr::from_str(&Instr::to_string(&Sub)).unwrap(), Sub);
}

fn main() {
    let p = Point {x: 0.5, y: 1.0};
    generic_print(&p);
    let i = Instr::from_str("sub").unwrap();
    generic_print(&i);
}
