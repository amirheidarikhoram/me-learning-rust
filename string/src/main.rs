fn main() {
    // it is the most wtf thing i've ever seen in a language
    let mut a = "Hel";
    print!("{}", a);
    a = "lo";
    println!("{}", a);

    // but you will like this
    let mut b: String = "name".to_string();
    println!("{}",b);
    b.remove(0);
    b.insert(0, 'N');
    b.push('x');
    println!("{}", b);

    // #######################################
    // string conctenation
    let words = ["Hello", "my", "name", "is", "Amir Hosein"];
    let mut join = String::new();
    for w in words{
        join = format!("{} {}", join, w);
    }
    println!("Result: {}", join);
}
