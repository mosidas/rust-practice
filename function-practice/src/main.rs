fn main() {
    let add_2 = func_of_func(2);
    println!("{}", add_2(4));
    println!("{}", add_2(10));
}

fn func_of_func(b: i32) -> impl Fn(i32) -> i32 {
    move |x| x + b
}
