fn main() {
    let words = ["Hello", "kid", "how", "are", "you", "?"];
    println!("{:?}", words);
    // we need to slice with references because slice size is not infered in compile time
    let how_are_you = &words[2..words.len()];
    println!("{:?}", how_are_you);

    // note that slicing with ranges of type other than usize causes a compiletime panic the line below will throw a compilation error
    // let wrong_how_are_you = &words[2i32..words.len()];

    // note that if slicing extremes are out of arr range you will face a runtime panic. try compiling and running line below
    // let other_wrong_how_are_you = &words[2..words.len()+1];
    // println!("{:?}", other_wrong_how_are_you);
}
