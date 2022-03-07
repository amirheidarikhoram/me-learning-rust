fn main() {
    // // lets just get some arguments and print 'em
    // let command_line = std::env::args();
    // for cmd in command_line {
    //     print!("[{}]", cmd)
    // }

    // // oh i dont know why but i have to exit with a return (exit) code
    // std::process::exit(107);

    // // lets print env vars
    // for _env in std::env::vars() {
    //     println!("{}: {}", _env.0, _env.1);
    // }
    // // and define a new env var -- note that this env var is not permanent
    // let env_key = "NEW_VAR_TEST";
    // std::env::set_var(env_key, "hahaha");
    // println!("{:?}", std::env::var(env_key));

    // // but what if we need to read from console after the app runs?
    // let mut line = String::from("Initial Data");
    // println!("{:?}", std::io::stdin().read_line(&mut line));
    // println!("[{}]", &line);
    // // note that it appends read data to intial data of line variable

    // // unwrap returns the data inside the Ok enum option which is count of data read in bytes
    // let mut another_line = format!("");
    // println!("{}", std::io::stdin().read_line(&mut another_line).unwrap());
}
