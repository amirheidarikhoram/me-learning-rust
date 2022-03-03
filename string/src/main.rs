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

    // this approach is not that efficient beacause everytime line 23 gets evaluated there is a pair of deallocating and allocating happening.
    let words = ["Hello", "my", "name", "is", "Amir Hosein"];
    let mut join = String::new();
    for w in words{
        join = format!("{} {}", join, w);
    }
    println!("Result: {}", join);

    // this is much better
    let mut better_join = String::new();
    for w in words{
        better_join += w;
    }

    println!("The more efficient result: {}", better_join);

    // in previos cases length of the array <words> was 5 and total characters count was 23 (occupying 23 bytes in UTF-8) so there happens 4 buffer allocations and we can make it better with createing a string with capacity of 24

    let mut best_join = String::with_capacity(24);
    for w in words {
        best_join += w;
    }
    println!("The most efficient result: {} with capacity, {}", best_join, best_join.capacity());
}
