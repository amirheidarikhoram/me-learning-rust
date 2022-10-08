
fn main() {
    let mut x: u8 = 5;
    let y: &mut u8 = &mut x;

    *y += 1;

    println!("x = {}", x);
}
