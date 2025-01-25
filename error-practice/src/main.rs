fn main() {
    result();
    custom_error();
    custom_error2();
    handling_error();
    nest_custom_error2_main();
}

fn result() {
    let a = div_result(1, 0);
    println!("{:?}", a); // Err("Division by zero")
    match a {
        Ok(result) => println!("{}", result),
        Err(e) => println!("{}", e),
    }

    let b = div_result(1, 1);
    println!("{:?}", b); // Ok(1)
    match b {
        Ok(result) => println!("{}", result),
        Err(e) => println!("{}", e),
    }
}

fn div_result(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn custom_error() {
    let a = div_custom_error(1, 0);
    // println!("{:?}", a); // Err(DivisionByZero("Division by zero"))
    match a {
        Ok(result) => println!("{}", result),
        Err(CustomError::DivisionByZero(e)) => println!("division by zero: {}", e),
        Err(CustomError::Other(e)) => println!("{} is a negative number", e),
    }

    let b = div_custom_error(1, 1);
    // println!("{:?}", b); // Ok(1)
    match b {
        Ok(result) => println!("{}", result),
        Err(CustomError::DivisionByZero(e)) => println!("division by zero: {}", e),
        Err(CustomError::Other(e)) => println!("{} is a negative number", e),
    }

    let c = div_custom_error(-1, 1);
    // println!("{:?}", c); // Err(Other(-1))
    match c {
        Ok(result) => println!("{}", result),
        Err(CustomError::DivisionByZero(e)) => println!("division by zero: {}", e),
        Err(CustomError::Other(e)) => println!("{} is a negative number", e),
    }
}

fn div_custom_error(a: i32, b: i32) -> Result<i32, CustomError> {
    if b == 0 {
        Err(CustomError::DivisionByZero(b))
    } else if a < 0 {
        Err(CustomError::Other(a))
    } else {
        Ok(a / b)
    }
}

enum CustomError {
    DivisionByZero(i32),
    Other(i32),
}

fn custom_error2() {
    let a = div_custom_error2(1, 0);
    match a {
        Ok(result) => println!("{}", result),
        Err(e) => println!("{}", e),
    }

    let b = div_custom_error2(1, 1);
    match b {
        Ok(result) => println!("{}", result),
        Err(e) => println!("{}", e),
    }

    let c = div_custom_error2(-1, 1);
    match c {
        Ok(result) => println!("{}", result),
        Err(e) => println!("{}", e),
    }
}

fn div_custom_error2(a: i32, b: i32) -> Result<i32, CustomError2> {
    if b == 0 {
        Err(CustomError2::DivisionByZero(b))
    } else if a < 0 {
        Err(CustomError2::Other(a))
    } else {
        Ok(a / b)
    }
}

use thiserror::Error;
#[derive(Error, Debug)]
enum CustomError2 {
    #[error("Division by zero: {0}")]
    DivisionByZero(i32),
    #[error("{0} is a negative number")]
    Other(i32),
}

fn handling_error() {
    let a = div_custom_error2(1, 0);
    if a.is_ok() {
        println!("{}", a.unwrap());
    } else {
        println!("{}", a.err().unwrap());
    }
}

fn nest_custom_error2(a: i32, b: i32) -> Result<i32, CustomError2> {
    let c = div_custom_error2(a, b)?;
    Ok(c)
}

fn nest_custom_error2_main() {
    let a = nest_custom_error2(1, 0);
    match a {
        Ok(result) => println!("{}", result),
        Err(e) => println!("{}", e),
    }
}
