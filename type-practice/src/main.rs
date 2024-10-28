fn main() {
    type_unit();
    type_integer();
    type_float();
    type_bool();
    type_char();
    type_array();
    type_tuple();
    type_slice();
    type_vector();
    type_string();
    type_struct();
}

fn type_unit() {
    //  値がないことを表す
    let a: () = ();
    println!("{:?}", a); // ()
}

fn type_integer() {
    // signed integer
    let a: i8 = 1;
    println!("{}", a); // 1
    let b: i16 = 2;
    println!("{}", b); // 2
    let c: i32 = 3;
    println!("{}", c); // 3
    let d: i64 = 4;
    println!("{}", d); // 4
    let e: i128 = 5;
    println!("{}", e); // 5
    let f: isize = 6;
    println!("{}", f); // 6

    let g = 42; // default i32
    println!("{}", g); // 42

    // unsigned integer
    let a: u8 = 1;
    println!("{}", a); // 1
    let b: u16 = 2;
    println!("{}", b); // 2
    let c: u32 = 3;
    println!("{}", c); // 3
    let d: u64 = 4;
    println!("{}", d); // 4
    let e: u128 = 5;
    println!("{}", e); // 5
    let f: usize = 6;
    println!("{}", f); // 6
    let g = 42; // default u32
    println!("{}", g); // 42
}

fn type_float() {
    let a: f32 = 1.0;
    println!("{}", a); // 1.0
    let b: f64 = 2.0;
    println!("{}", b); // 2.0

    let c = 42.0; // default f64
    println!("{}", c); // 42.0
}

fn type_bool() {
    let a: bool = true;
    println!("{}", a); // true
    let b: bool = false;
    println!("{}", b); // false
}

fn type_char() {
    let a: char = 'a';
    println!("{}", a); // a
    let b: char = 'あ';
    println!("{}", b); // あ
    let c: char = '😻';
    println!("{}", c); // 😻
}

fn type_array() {
    let a: [i32; 3] = [1, 2, 3];
    println!("{:?}", a); // [1, 2, 3]
    let b: [i32; 10] = [0; 10];
    println!("{:?}", b); // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

    let c = [1, 2, 3];
    println!("{:?}", c); // [1, 2, 3]
    let d = [0; 10];
    println!("{:?}", d); // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
}

fn type_tuple() {
    let a: (i32, f64, char) = (1, 2.0, 'a');
    println!("{:?}", a); // (1, 2.0, a)
}

fn type_slice() {
    let ary = [1, 2, 3, 4, 5];
    println!("{:?}", ary); // [1, 2, 3, 4, 5]
    let slc = &ary[1..3];
    for i in slc {
        println!("{}", i);
    }
    // 1
    // 2
}

fn type_vector() {
    let mut vec = vec![1, 2, 3, 4, 5];

    println!("{:?}", &vec); // [1, 2, 3, 4, 5]
    vec.push(6);
    vec[0] += 1;
    println!("{:?}", &vec); // [2, 2, 3, 4, 5, 6]

    // empty vector
    let vec2: Vec<i32> = Vec::new();
    println!("{:?}", vec2); // []
    let vec3 = Vec::<i32>::new();
    println!("{:?}", vec3); // []
}

fn type_string() {
    let s1 = "hello"; // &'static str
    println!("{}", s1); // hello
    println!("{}", &s1[0..2]); // he
    let s2 = "あいうえお";
    println!("{}", &s2[0..6]); // あい
    println!("{}", &s2[0..5]); // panic: 'byte index 5 is a char boundary; it is inside 'い' (bytes 3..6) of `あいうえお`'

    for c in s2.chars() {
        // character iterator
        println!("{}", c);
    }

    for b in s2.bytes() {
        // byte iterator
        println!("{}", b);
    }
}

fn type_struct() {
    struct User {
        username: String,
        email: String,
        age: u32,
    }

    let user1 = User {
        username: String::from("user1"),
        email: String::from("foo@example.com"),
        age: 20,
    };
    println!("{}", user1.username); // user1
    println!("{}", user1.email); // foo@example.com
    println!("{}", user1.age); // 20

    let user2 = User {
        username: String::from("user2"),
        ..user1 // copy other fields
    };
    println!("{}", user2.username); // user2
    println!("{}", user2.email); // foo@example.com
    println!("{}", user2.age); // 20

    // tuple struct
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
    println!("{} {} {}", black.0, black.1, black.2); // 0 0 0
}
