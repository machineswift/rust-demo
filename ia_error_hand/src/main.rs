use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("target/hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("target/hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    //let f_01 = File::open("hello.txt").unwrap();

    //let f_02 = File::open("hello.txt").expect("Failed to open hello.txt");


    let f_03 = File::open("target/hello.txt");

    let mut f_03 = match f_03 {
        Ok(file) => file,
        Err(e) => return Err(e),
    };


//    let mut buffer = String::new();
//
//    f_03.read_to_string(&mut buffer)?;
}
