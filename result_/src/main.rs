use std::fs;

fn main(){
    let greeting_file_result = fs::read_to_string("file1.txt");
    match greeting_file_result {
        Ok(file_content) => {
            println!("{}", file_content);
        },
        Err(err) => {
            println!("{}", err);
        }
    }
}