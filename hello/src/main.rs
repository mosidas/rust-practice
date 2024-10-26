fn ex_div_some(a: i32, b: i32) -> Option<i32> {
    let ans = if b == 0 {
        None
    } else {
        Some(a / b)
    };
    ans
}

fn func_ex_div_result(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("Division by zero")
    } else {
        Ok(a / b)
    }
}

fn func_ex_print_some_if<T: std::fmt::Display>(ans: Option<T>) {
    if let Some(x) = ans {
        println!("{}", x);
    } else {
        println!("None");
    }
}

fn func_ex_print_some_match<T: std::fmt::Display>(x: Option<T>) {
    match x {
        Some(s) => println!("{}", s),
        None => println!("None"),
    }
}

fn func_ex_print_result_match<T: std::fmt::Display, E: std::fmt::Display>(ans: Result<T, E>) {
    match ans {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{}", e),
    }
}

fn main() {
    func_ex_print_some_if(ex_div_some(10, 5));
    func_ex_print_some_if(ex_div_some(10, 0));
    func_ex_print_some_match(ex_div_some(10, 5));
    func_ex_print_some_match(ex_div_some(10, 0));
    func_ex_print_result_match(func_ex_div_result(10, 5));
    func_ex_print_result_match(func_ex_div_result(10, 0));
}
