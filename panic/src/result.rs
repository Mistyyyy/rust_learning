use std::{fs, fs::File};
use std::io::{Read, ErrorKind};

pub fn try_open_file()-> std::fs::File {
    let f = File::open("./hello.txt");

    // let f = match f {
    //     Ok(content) => content,
    //     Err(err) => {
    //         panic!("Problem opening the file: {:?}", err)
    //     }
    // };

    let f = match f {
        Ok(content) => content,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("./hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        }
    };

    f
}

pub fn try_another_open() {
    let f = File::open("./hellos.txt").unwrap_or_else(|err| {
        if ErrorKind::NotFound == err.kind() {
            File::create("./hellos.txt").unwrap_or_else(|err| {
                panic!("Problem creating the file: {:?}", err)
            })
        } else {
            panic!("Problem opening the file: {:?}", err)
        }
    });
}

pub fn try_unwrap_expect()-> std::fs::File {
    File::open("./a.txt").expect("Failed to open the file");
    File::open("./a.txt").unwrap()
}

pub fn read_username_from_file() -> Result<String, std::io::Error> {
    let f = File::open("./user.txt");

    let mut f = match f {
        Ok(fc) => fc,
        Err(err) => return Err(err),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(err) => Err(err),
    }
}

// pub fn read_username_from_file_with_from_trait() -> Result<String, std::io::Error> {
//     let mut f = File::open("./user.txt")?;
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }

// pub fn read_username_from_file_with_from_trait() -> Result<String, std::io::Error> {
//     let mut s = String::new();
//     File::open("./user.txt")?.read_to_string(&mut s)?;
//     Ok(s)
// }

pub fn read_username_from_file_with_from_trait() -> Result<String, std::io::Error> {
    // let mut s = String::new();
    // File::open("./user.txt")?.read_to_string(&mut s)?;
    // Ok(s)
    fs::read_to_string("./user.txt")
}