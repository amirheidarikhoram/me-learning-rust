#![allow(dead_code)]
fn main() {
    // // as you don't know in rust there is not try/catch thing
    // // this is how we can catch errors:
    // let num = -10;
    // let result = not_negative(&num);
    // println!("{:?}", result);
    // // now we can unrap it if we are sure that it is Ok (but it is not so it will panic)
    // // we can handle err and ok cases with match
    // match &result {
    //     Ok(val) => println!("Ok with value {}", val),
    //     Err(err) => println!("Some error happened: {}", err)
    // }
    // // we know that it is not negative, so wee get and error then we can unwrap error safely
    // let error = result.unwrap_err();
    // println!("This should be displayed if we have error, error: {}", error)

    // let use ? with Result
    match composed(&3) {
        Ok(val) => {println!("{}", &val)},
        Err(error) => {println!("An error occured: {}", error)}
    }
}

fn not_negative(number: &i32) -> Result<&i32, String> {
    if *number >= 0 {
        Ok(number)
    } else {
        Err("Number is not negative".to_string())
    }
}

fn not_odd (number: &i32) -> Result<&i32, String> {
    if *number % 2 == 0 {
        Ok(number)
    } else {
        Err("Number is not even".to_string())
    }
}

fn composed (num: &i32) -> Result<i32, String> {
    let res = not_negative(not_odd(num)?)?;
    Ok(*res)
}