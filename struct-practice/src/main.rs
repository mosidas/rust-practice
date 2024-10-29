mod sample;
mod trait_sample;

fn main() {
    sample();
    trait_sample::print_debug_user();
    trait_sample::print_copy_user();
}

fn sample() {
    sample::print_user();
    sample::print_public_user();
    sample::print_super_public_user();

    // プライベートな構造体のインスタンスは外部モジュールから作成できない
    // // struct `User` is private
    // let mut user = sample::User {
    //     username: String::from("user"),
    //     age: 20,
    // };
    // user.print_username(); // method `print_username` is private
    // user.add_age(1); // method `add_age` is private

    // プライベートなフィールドを持つ構造体のインスタンスは外部モジュールから作成できない
    // let mut user = sample::PublicUser {
    //     //  field `username` of struct `PublicUser` is private
    //     username: String::from("user"),
    //     // field `age` of struct `PublicUser` is private
    //     age: 20,
    // };
    // user.print_username();
    // user.add_age(1);

    // パブリックな構造体、かつパブリックなフィールドを持つ構造体のインスタンスは外部モジュールから作成できる
    let mut user = sample::SuperPublicUser {
        username: String::from("user4"),
        age: 20,
    };
    user.print_username();
    user.add_age(4);
    println!("{}", user.age);
}
