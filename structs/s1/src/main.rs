struct User{
    first_name: String,
    last_name: String,
    age: i32
}

fn main(){
    let user = User{
        first_name: String::from("Riya"),
        last_name: String::from("Sachdeva"),
        age: 32
    };
    println!("Age of user {} {} is {}", user.first_name, user.last_name, user.age);
}