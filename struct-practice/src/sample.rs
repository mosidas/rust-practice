struct User {
    username: String,
    age: u32,
}

impl User {
    // &self is a immutable reference to the instance of the struct
    fn print_username(&self) {
        println!("{}", self.username);
    }

    // &mut self is a mutable reference to the instance of the struct
    fn add_age(&mut self, n: u32) {
        self.age += n;
    }
}

pub fn print_user() {
    let mut user = User {
        username: String::from("user1"),
        age: 20,
    };
    user.print_username();
    user.add_age(1);
    println!("{}", user.age);
}

pub struct PublicUser {
    username: String,
    age: u32,
}

impl PublicUser {
    pub fn print_username(&self) {
        println!("{}", self.username);
    }

    pub fn add_age(&mut self, n: u32) {
        self.age += n;
    }
}

pub fn print_public_user() {
    let mut user = PublicUser {
        username: String::from("user2"),
        age: 20,
    };
    user.print_username();
    user.add_age(2);
    println!("{}", user.age);
}

pub struct SuperPublicUser {
    pub username: String,
    pub age: u32,
}

impl SuperPublicUser {
    pub fn print_username(&self) {
        println!("{}", self.username);
    }

    pub fn add_age(&mut self, n: u32) {
        self.age += n;
    }
}

pub fn print_super_public_user() {
    let mut user = SuperPublicUser {
        username: String::from("user3"),
        age: 20,
    };
    user.print_username();
    user.add_age(3);
    println!("{}", user.age);
}
