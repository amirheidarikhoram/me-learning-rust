#![allow(dead_code)]

fn main() {
    // let message = "Hello, world!";
    // print_nth_char(message, 3);
    // print_chars_with_utf8(message);
    // print_chars_with_utf8_syntactic_sugar(message);
    // print_bytes(message);

    // let vector_items = vec!["my", "name", "is", "amir"];
    // print_vector_items(&vector_items);

    // // working with slice iterators
    // let slicable = [1,2,3,4,5,6,7];
    // let sliced = &slicable[..5];
    // for item in sliced.iter() {
    //     println!("item of slice: {}", item);
    // }

    // // working mutating iterators
    // let mut numbers_to_be_increased = [1,2,3,4,5,6];
    // increase_all(&mut numbers_to_be_increased, 2);
    // println!("{:?}", numbers_to_be_increased);

    // // lets do some things with iterator adapters
    // let mut numbers_to_be_adapted = [1,2,3,4,5,6,7,8];
    // print_even_numbers(&numbers_to_be_adapted);
    // print_mul_all(&numbers_to_be_adapted, 2);
    // mul_all_in_place(&mut numbers_to_be_adapted, 5);
    // println!("{:?}", numbers_to_be_adapted);

    // // lets enumerate over iterators
    // let mut numbers_to_be_enumerated = [1,2,3,4,5,6,7,8];
    // mul_in_place_by_index(&mut numbers_to_be_enumerated);
    // println!("{:?}", numbers_to_be_enumerated);

    // // working with iterator consumers
    // let name = "Amir Hosein Heidari Khoram";
    // let search_char = 'K';
    // if name.chars().any(|ch| ch == search_char) {
    //     println!("Yay it has {} in it", search_char);
    // }

    // // working with iterator chain
    // let mut numbers = vec![-3, -2, -1, 0, 1, 2, 3, 4, 5, 6];
    // numbers = numbers
    //     .into_iter()
    //     .filter(|x| *x >= 0)
    //     .map(|x| x * 2)
    //     .collect();
    // println!("{:?}", numbers);
}

fn print_nth_char(txt: &str, mut n: u32) {
    let mut iter = txt.chars();
    loop {
        let item = iter.next();
        match item {
            Some(val) => {
                if n == 0 {
                    println!("{}", val);
                    break;
                }
            }
            _ => {
                break;
            }
        }
        n -= 1
    }
}

fn print_chars_with_utf8(txt: &str) {
    let mut chars = txt.chars();
    loop {
        let item = chars.next();
        match item {
            Some(val) => {
                println!("{}: {}", val, val as u32)
            }
            None => {
                break;
            }
        }
    }
}

fn print_chars_with_utf8_syntactic_sugar(txt: &str) {
    for _char in txt.chars() {
        println!("{}: {}", _char, _char as u32)
    }
}

fn print_bytes(txt: &str) {
    for byte in txt.bytes() {
        println!("{}", byte)
    }
}

fn print_vector_items(items: &Vec<&str>) {
    // for item in items.into_iter() {
    //     println!("{}", item);
    // }

    // no need to explicitly call into_iter method

    for item in items {
        println!("{}", item);
    }
}

fn increase_all(items: &mut [i32], by: i32) {
    // for item in items.iter_mut() {
    //     *item += by;
    // }

    // no need to explicitly call iter.mut method if items is type of &mut <T>

    for item in items {
        *item += by;
    }
}

fn print_even_numbers(numbers: &[i32]) {
    for _num in numbers.into_iter().filter(|x| **x % 2 == 0) {
        println!("{}", _num)
    }
}

fn print_mul_all(items: &[i32], by: i32) {
    for _num in items.into_iter().map(|x| *x * by) {
        println!("{}", _num)
    }
}

fn mul_all_in_place(items: &mut [i32], by: i32) {
    for _num in items {
        *_num = *_num * by;
    }
}

fn mul_in_place_by_index(items: &mut [i32]) {
    for (_index, _num) in items.iter_mut().enumerate() {
        *_num = *_num * _index as i32;
    }
}
