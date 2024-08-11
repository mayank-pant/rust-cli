use std::io::{self,IsTerminal};
use std::fs;

fn main() {
    let argument = std::env::args().nth(1).expect("no argument provided");

    match argument.as_str() {
        "-c" => {
            match bytes_in_file() {
                Some(result) => println!("Bytes in file - {:?}",result),
                None => {},
            }
        },
        // "-l" => {
        //     lines_in_file();
        // }
        _ => println!("Not a valid argument")
    }
}

fn bytes_in_file() -> Option<usize>{
    match find_content() {
       Some(value) =>{
            Some(value.len())
       },
       None => {
            println!("Invalid or non existent path or content");
            None
       },
    }
}

// fn lines_in_file() {
//     let content = find_content();
// }

fn find_content() -> Option<String>{
    let stdin = io::stdin();
    let mut piped: String = String::from("");
    
    if !stdin.is_terminal() { // v1.70.0 later
        piped = io::read_to_string(stdin).expect("no use");
    }
    

    let path = std::env::args().nth(2);
    
    if path.is_some() {
        read_content(path.unwrap())
    } else {
        Some(piped)
    }
}

fn read_content(path: String) -> Option<String> {
    // validate_path
    // return file content
    
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");
    if contents.is_empty() {
        None
    } else {
        Some(contents)
    }
}