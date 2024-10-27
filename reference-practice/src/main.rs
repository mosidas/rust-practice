fn main() {
    copy_semantics();
}

fn copy_semantics() {
    let s = 1;
    let t = 2;
    println!("{}", s);
    println!("{}", t);

    let s = "hello".to_string();
    let t = s.clone(); // copy
    println!("{}", s); // コピーを渡したので、所有権はsにある
    println!("{}", t); // 所有権はtにある
}

fn move_semantics() {
    // let s = "hello".to_string();
    // let t = s;
    // println!("{}", s); // error[E0382]: borrow of moved value: `s` → 所有権がtに移動したので、sに所有権がない
    // println!("{}", t);
}
