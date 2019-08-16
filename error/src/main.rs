use std::fs;
use std::io;
use std::io::Read;

fn main() {
    // let f = open_or_panic("hello");
    // println!("{:?}", f);
    let _ = match read_file("hello.go") {
        Err(e) => panic!("{:?}", e),
        Ok(c) => println!("{}", c),
    };
    // let _ = match fs::File::open("./hello.go") {
    //     Ok(file) => println!("{:?}", file),
    //     Err(e) => match e.kind() {
    //         ErrorKind::NotFound => match fs::File::create("hello.go") {
    //             Ok(_) => println!("created new file: {}", "hello.go"),
    //             Err(e) => panic!("tried to create file: hello.go still getting error: {}", e),
    //         },
    //         other_error => panic!("got trouble with error: {:?}", other_error),
    //     },
    // };
}

// fn open_or_panic(filepath: &str) -> fs::File {
//     fs::File::open(String::from(filepath)).unwrap()
// }

fn read_file(filepath: &str) -> Result<String, io::Error> {
    // let f = fs::File::open(String::from(filepath));
    // let mut f = match f {
    //     Ok(f) => f,
    //     Err(e) => return Err(e),
    // };

    // let mut content = String::new();
    // match f.read_to_string(&mut content) {
    //     Ok(_) => return Ok(content),
    //     Err(e) => return Err(e),
    // };
    let mut s = String::new();
    fs::File::open(String::from(filepath))?.read_to_string(&mut s)?;
    Ok(s)
}
