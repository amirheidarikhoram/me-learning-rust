use std::{fmt::{Display, Formatter, Result}};

fn main() {
    let com1 = Complex::new(1,2);
    let com2 = Complex::new(4,3);
    let com3 = com1 + com2;
    println!("{} + {} = {}", com1, com2, com3);
}

#[derive(Copy, Clone)]
struct Complex<Number>{
    re: Number,
    im: Number
}

impl<Number> Complex<Number>{
    fn new (re: Number, im: Number) -> Self {
        Complex {
            re: re,
            im: im
        }
    }
}

impl<Number: Display> Display for Complex<Number> {
    fn fmt (&self,f: &mut Formatter) -> Result {
        write!(f, "{}+{}i", self.re, self.im)
    }
}

impl<Number> std::ops::Add for Complex<Number> where Number: std::ops::Add<Output = Number>{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im 
        }
    }
}

// impl<Number> Drop for Complex<Number> where Number: Display {
//     fn drop (&mut self) {
//         println!("Dropping Complex number {}", &self);
//     }
// } // this causes error