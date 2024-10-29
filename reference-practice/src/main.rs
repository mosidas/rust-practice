fn main() {
    println!("--copy_semantics--");
    copy_semantics();
    println!("--move_semantics--");
    move_semantics();
    println!("--borrow_reference--");
    borrow_reference();
    println!("--borrow_reference_mut--");
    borrow_reference_mut();
    println!("--lifetime_parameter--");
    lifetime_parameter();
    println!("--clone_reference--");
    clone_reference();
    println!("--clone_reference2--");
    clone_reference2();
}

fn copy_semantics() {
    let s = 1;
    let t = 2;
    println!("{}", s);
    println!("{}", t);

    let s = "hello world".to_string();
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

fn borrow_reference() {
    let s = "hello".to_string();
    my_print(&s); // borrow the reference of s
    my_print(&s); // borrow the reference of s

    let t = "world".to_string();
    let ref_t = &t;
    my_print(ref_t); // borrow the reference of t
    my_print(ref_t); // borrow the reference of t
}

fn my_print<T: std::fmt::Display>(s: &T) {
    println!("{}", *s);
}

fn borrow_reference_mut() {
    let mut s = "hello".to_string();
    let ref_s1 = &mut s; // borrow the reference of s
    *ref_s1 = "world".to_string(); // mutate the reference of s

    // let ref_s2 = &mut s; // error: cannot borrow `s` as mutable more than once at a time
    // *ref_s2 = "world".to_string();
    println!("{}", ref_s1); // ref_s1 scope is over

    let ref_s2 = &mut s; // ref_s2 can borrow the reference of s because ref_s scope is over
    *ref_s2 = "world".to_string();
    println!("{}", ref_s2);
}

// can not return the reference of local scope
// fn return_local_scope_reference() -> &String {
//     let s = "hello".to_string();
//     &s // cannot return reference to local variable `s` returns a reference to data owned by the current function
// }

// fn local_scope_reference() {
//     let x;
//     {
//         let y = 1;
//         x = &y; // `y` does not live long enough borrowed value does not live long enough
//     }
//     println!("{}", x);
// }

fn lifetime_parameter() {
    let v1 = [1, 2, 3, 4, 5];
    let p1 = pick1(&v1, 3);
    println!("{:?}", p1);

    let v2 = [10, 20, 30, 40, 50];
    let (p1, p2) = pick2(&v1, &v2, 3);
    println!("{:?}, {:?}", p1, p2);
}

fn pick1(x: &[i32], end: usize) -> &[i32] {
    &x[..end]
}

fn pick2<'a, 'b>(x: &'a [i32], y: &'b [i32], end: usize) -> (&'a [i32], &'b [i32]) {
    let end_x = if x.len() < end { x.len() } else { end };
    let end_y = if y.len() < end { y.len() } else { end };

    (&x[..end_x], &y[..end_y])
}

// fn pick3<'a, 'b>(x: &'a [i32], y: &'b [i32], end: usize) -> (&'b [i32], &'a [i32]) {
//     (&x[..end], &y[..end]) // error: lifetime may not live long enough
// }

fn clone_reference() {
    #[derive(Clone)]
    struct ChildStruct {
        d: i64,
    }

    #[derive(Clone)]
    struct MyStruct {
        a: i64,
        c: ChildStruct,
    }

    let mut obj = MyStruct {
        a: 0,
        c: ChildStruct { d: 10 },
    };
    let copy = obj.clone();

    obj.a = 5;
    obj.c.d = 100;

    println!("{}", copy.a); // => 0
    println!("{}", copy.c.d); // => 10
}

fn clone_reference2() {
    use std::cell::Cell;
    #[derive(Clone)]
    struct ChildStruct {
        d: Cell<i64>,
    }

    #[derive(Clone)]
    struct MyStruct<'a> {
        a: i64,
        c: &'a ChildStruct,
    }

    let child = ChildStruct { d: Cell::new(10) };
    let mut obj = MyStruct { a: 0, c: &child };
    let copy = obj.clone();

    obj.a = 5;
    obj.c.d.set(100);

    println!("{}", copy.a); // => 0
    println!("{}", copy.c.d.get()); // => 100
}
