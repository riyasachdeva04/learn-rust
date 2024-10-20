fn main() {
    let name = String::from("Riya");
    let len = get_str_length(name);
    println!("{}", len);
}
fn get_str_length(str: String) -> usize{
    str.chars().count()
}