fn main() {
    println!("{}", fibonacci(4));
}

fn fibonacci(num: i32) -> i32 {
    if num == 0 {
        return 0;
    } else if num == 1 {
        return 1;
    }
    
    let mut a = 0;
    let mut b = 1;
    
    for _ in 2..=num {
        let c = a + b;
        a = b;
        b = c;
    }
    
    return b;
}
