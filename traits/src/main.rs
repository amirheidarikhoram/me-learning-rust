fn main() {
    // // lets create a trait and implement ot for i32
    // // this is not good to put traits inside unction scopes but it is fot test and learn so dont "panic"
    // trait Runs {
    //     fn count_to(self, to: i32);
    // }
    // impl Runs for i32 {
    //     fn count_to(self, to: i32) {
    //         for i in self..=to {
    //             println!("{}", i)
    //         }
    //     }
    // }
    // println!("Hello, world!");
    // let num = 3;
    // num.count_to(5);

    // // using traits with multiple types
    // let a = 2;
    // println!("{}", a.sqrt()); // not that it will print 1 instead of 1.41 because we cast the result in f64 for to i32 and it looses it floating point value
    // trait HasSqrt {
    //     fn sqrt(self) -> Self;
    // }
    // impl HasSqrt for i32 {
    //     // we need to return something with the type of "Self" to be able to chaing these functions
    //     fn sqrt(self) -> Self {
    //         (self as f64).sqrt() as i32
    //     }
    // }

    // // using multiple traits and mixing them in (with trait bind)
    // trait HasSquareRoot {
    //     fn sqrt(self) -> Self;s
    // }
    // impl HasSquareRoot for f32 {
    //     fn sqrt(self) -> Self {
    //         self.sqrt()
    //     }
    // }
    // impl HasSquareRoot for f64 {
    //     fn sqrt(self) -> Self {
    //         self.sqrt()
    //     }
    // }
    // trait HasAbsoluteValue {
    //     fn abs(self) -> Self;
    // }
    // impl HasAbsoluteValue for f32 {
    //     fn abs(self) -> Self {
    //         self.abs()
    //     }
    // }
    // impl HasAbsoluteValue for f64 {
    //     fn abs(self) -> Self {
    //         self.abs()
    //     }
    // }
    // fn abs_quartic_root<Number>(x: Number) -> Number
    // where
    //     Number: HasSquareRoot + HasAbsoluteValue, // it is called trait bind
    // {
    //     x.abs().sqrt().sqrt()
    // }
    // let numf64 = -100f64;
    // let numf32 = -100f32;
    // println!("f64 call result: {}", abs_quartic_root(numf64));
    // println!("f32 call result: {}", abs_quartic_root(numf32));
    // // lets see how trait inheretence works in rust
    // trait HasSqrtAndAbs: HasSquareRoot + HasAbsoluteValue {}
    // impl HasSqrtAndAbs for f32 {}
    // impl HasSqrtAndAbs for f64 {}
    // fn another_abs_quartic_root<Number>(x: Number) -> Number where Number: HasSqrtAndAbs {
    //     x.abs().sqrt().sqrt()
    // }
    // println!("f64 call result with inheretence: {}", another_abs_quartic_root(numf64));
    // println!("f32 call result with inheretence: {}", another_abs_quartic_root(numf32));

    // // a much harder example to get with generic traits
    // // let's assume that we want to implement a exponentiate function for f32 an f64 to exponentiate a number to a given power
    // trait HasLnAndExp {
    //     fn ln (self) -> Self;
    //     fn exp (self) -> Self;
    // }
    // impl HasLnAndExp for f32 {
    //     fn ln (self) -> Self {
    //         self.ln()
    //     }
    //     fn exp (self) -> Self {
    //         self.exp()
    //     }
    // }
    // impl HasLnAndExp for f64 {
    //     fn ln (self) -> Self {
    //         self.ln()
    //     }
    //     fn exp (self) -> Self {
    //         self.exp()
    //     }
    // }
    // trait HasMultiply<Rhs> {
    //     fn multiply (self, other: Rhs) -> Self;
    // }

    // impl<Rhs> HasMultiply<Rhs> for f64 where Rhs: Into<Self>{
    //     fn multiply (self, other: Rhs) -> Self {
    //         self * other.into()
    //     }
    // }
    // impl<Rhs> HasMultiply<Rhs> for f32 where Rhs: Into<Self>{
    //     fn multiply (self, other: Rhs) -> Self {
    //         self * other.into()
    //     }
    // }
    // // now we can define our exponentiate function
    // fn exponentiate<Base, Exponent> (base: Base, exponent: Exponent) -> Base where Base: HasLnAndExp + HasMultiply<Exponent> {
    //     base.ln().multiply(exponent).exp()
    // }
    // println!("2 ^ 3 is {}", exponentiate(2., 3)); // it will not display an accurate result because of ln and exp

    // // generic traits can be a pain in the arse then lets use associated types ith traits
    // trait Dictionary {
    //     type Key; // an associated type
    //     type Item; // another associated type

    //     fn get(&self, key: Self::Key) -> Option<Self::Item>;
    // }

    // struct Record {
    //     id: u32,
    //     name: String
    // }

    // struct RecordSet {
    //     data: Vec<Record>
    // }

    // impl Dictionary for RecordSet {
    //     type Key = u32;
    //     type Item = String;
    //     fn get(&self, key: Self::Key) -> Option<Self::Item> {

    //         for _record in self.data.iter() {
    //             if _record.id == key {
    //                 return Some(String::from(&_record.name));
    //             }
    //         }

    //         None
    //     }
    // }
    // fn get_name<D>(dict: &D, id: <D as Dictionary>::Key) -> Option<<D as Dictionary>::Item> where D: Dictionary {
    //     dict.get(id)
    // }
    // let names = RecordSet {
    //     data: vec![
    //         Record {id: 1, name: format!("amir")},
    //         Record {id: 2, name: format!("hosein")},
    //         Record {id: 3, name: format!("heidar")},
    //         Record {id: 4, name: format!("khoram")},
    //     ]
    // };
    // println!("{}", get_name(&names, 3).unwrap());
    // // the benefit of associated types is you dont have to define every type in functions like get_name and you can refer to associated types of the trait you are working with

    // implementing function overloading with generic traits

    struct Human {
        position: Coordinate,
    }

    struct Coordinate {
        x: f64,
        y: f64,
    }

    trait CanMove<T> {
        fn move_to(&mut self, dest: T);
    }

    impl std::fmt::Display for Coordinate {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({},{})", &self.x, &self.y)
        }
    }

    impl CanMove<Coordinate> for Human {
        fn move_to(&mut self, dest: Coordinate) {
            println!("Moving to {}", dest);
            self.position = dest
        }
    }
    impl CanMove<Vec<Coordinate>> for Human {
        fn move_to(&mut self, dest: Vec<Coordinate>) {
            for _coordinate in dest.into_iter() {
                println!("Moving to {}", _coordinate);
                self.position = _coordinate;
            }
        }
    }

    let mut amir = Human {
        position: Coordinate { x: 2., y: 2. },
    };

    amir.move_to(Coordinate { x: 5., y: 5. });
    println!("Amir is at: {}", &amir.position);

    amir.move_to(vec![
        Coordinate { x: 6., y: 7. },
        Coordinate { x: 10., y: 13. },
        Coordinate { x: 2., y: 6. },
        Coordinate { x: 7., y: 20. },
    ]);

    println!("Amir is at: {}", &amir.position);
}
