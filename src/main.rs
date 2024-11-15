use std::fs::File;
use std::io::{self, Read, Write};

fn write_data_to_file(data: &[u8], file_path: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(data)?;
    return Ok(());
}

fn read_from_file(file_path: &str) -> io::Result<Vec<u8>> {
    let mut file_content: Vec<u8> = Vec::new();
    let mut file = File::open(file_path)?;

    file.read_to_end(&mut file_content)?;

    return Ok(file_content);
}

fn convert_ascii_vec_to_string(data: &Vec<u8>) -> String {
    let mut string_buffer = String::new();
    for i in 0..data.len() {
        string_buffer.push(data[i] as char);
    }
    return string_buffer;
}

fn main() {
    let file_path = "./hello.txt";
    let file_content = String::from("Hello iam a rust developer");

    if let Err(err) = write_data_to_file(file_content.as_bytes(), file_path) {
        println!("Error occurred  -> {:?}", err);
        return;
    }

    println!("file write operation successfull");

    match read_from_file(file_path) {
        Ok(data) => {
            println!("file read operation successfull");
            let file_content = convert_ascii_vec_to_string(&data);
            println!("file content -> {:?}", file_content);
        }
        Err(err) => {
            println!("Error occurred -> {:?}", err)
        }
    }

    return ();
}
