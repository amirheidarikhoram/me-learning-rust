#![allow(dead_code)]

fn main() {
    // // this is how we decalre and use modules
    // let name = "Amir";
    // utils::say_hi(name);
    // utils::say_bye(name);

    // lets create a struct and add and impl for it
    let amir = utils::lib::Person::new("amir".to_string(), "heidari".to_string(), 29u32);
    utils::say_hi(&amir.get_name()[..]); // we can convert String to &str by slicing it | &variable_name[..]
    // lets see how super keyword works
    amir.say_hello();
}

mod utils {
    pub fn say_hi(name: &str) {
        println!("Hi {}", name)
    }

    pub fn say_bye(name: &str) {
        println!("Bye Bye {}", name)
    }

    pub mod lib {
        pub struct Person {
            first_name: String,
            last_name: String,
            pub age: u32,
        }

        impl Person {
            pub fn new(first_name: String, last_name: String, age: u32) -> Self {
                Self {
                    first_name,
                    last_name,
                    age,
                }
            }

            pub fn set_age(&mut self, new_age: u32) {
                self.age = new_age;
            }

            pub fn set_name(&mut self, new_name: String) {
                self.first_name = new_name;
            }

            pub fn set_last_name(&mut self, new_last_name: String) {
                self.last_name = new_last_name;
            }

            pub fn get_name(&self) -> String {
                format!("{} {}", self.first_name, self.last_name)
            }

            pub fn say_hello (&self) {
                super::say_hi(&self.first_name[..])
            }
        }
    }
}
