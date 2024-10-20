fn find_first_a(s: String) -> Option<i32>{
    for (index, character) in s.chars().enumerate(){
        if character == 'a' {
            return Some(index as i32);
        }
    }
    return None;
}

fn main(){
    let my_str = String::from("Riya");
    match find_first_a(my_str){
        Some(index) => println!("first a present at {} index", index),
        None => println!("a not present in string")
    };
}