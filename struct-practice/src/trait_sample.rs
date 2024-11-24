// implement Debug trait
#[derive(Debug)]
pub struct DebugUser {
    pub username: String,
    pub age: u32,
}

pub fn print_debug_user() {
    let user = DebugUser {
        username: String::from("user5"),
        age: 20,
    };
    println!("{}", user.username);
    println!("{}", user.age);
    println!("{:?}", user);
}

// implement Clone and Copy trait
#[derive(Clone, Copy, Debug)]
pub struct CopyUser<'a> {
    pub username: &'a String,
    pub age: u32,
}

pub fn print_copy_user() {
    let user = CopyUser {
        username: &String::from("user6"),
        age: 20,
    };
    println!("{}", user.username);
    println!("{}", user.age);
    let user_copy = user;
    println!("{:?}", user_copy);
}
