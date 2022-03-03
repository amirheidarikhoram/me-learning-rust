use std::cell::RefCell;

fn main() {
    println!("Testing closures");
    let factor = RefCell::new(2);
    let multiply = |num: i32| -> i32 {
        num * (*factor.borrow())
    };

    println!("{}", multiply(2));
    *factor.borrow_mut() += 5;
    println!("{}", multiply(2));
}
