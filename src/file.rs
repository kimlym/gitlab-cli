use std::fs::{self, File};
use std::io::{Read, Write};

pub fn read_file(path: &str) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    return Ok(contents);
}

pub fn write_file(path: &str, data: &str) -> std::io::Result<()> {
    let mut split_path: Vec<&str> = path.split("/").collect();
    split_path.pop();
    let directory = split_path.join("/");
    fs::create_dir_all(directory).expect("Unable to create folder");
    let mut file = File::create(path).expect("Unable to create file");
    file.write_all(data.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_to_test_txt_should_succeed() {
        let path = "./resources/test.txt";
        let data = "abcdefg";
        write_file(path, data).unwrap();
    }

    #[test]
    fn read_test_txt_should_return_correct_content() {
        let path = "./resources/test.txt";
        let contents = read_file(path).unwrap();
        assert_eq!("abcdefg", contents);
    }

    #[test]
    fn read_not_existed_should_error() {
        let path = "./resources/not_existed.txt";
        match read_file(path) {
            Err(_) => assert!(true),
            _ => println!("ignroed"),
        };
    }
}
