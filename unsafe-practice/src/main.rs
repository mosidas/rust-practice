fn main() {
    unsafe_example();
    box_example();
    recursive_enum();
    // recursive_enum_error();
    static_example();
    rc_example();
    refcall_example();
}

fn unsafe_example() {
    let x = 1;
    // create a raw pointer to x
    let x_ptr: *const i32 = &x;

    println!("*x_ptr: {}", unsafe { *x_ptr });

    let mut y = 1;
    // create a mutable raw pointer to y
    let y_ptr = &mut y as *mut i32;

    // dereference the raw pointer and increment the value
    unsafe {
        *y_ptr = *x_ptr + 1;
    }

    println!("*y_ptr: {}", unsafe { *y_ptr });
    println!("y: {}", y);
}

fn box_example() {
    // allocate memory on the heap
    let x = Box::new(1);
    let val = *x;

    println!("val: {}", val);
}

fn recursive_enum() {
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));

    println!("list: {:?}", list);
}

// fn recursive_enum_error() {
//     error[E0072]: recursive type `List` has infinite size
//     enum List {
//         Cons(i32, List),
//         Nil,
//     }

//     let list = List::Cons(1, List::Cons(2, List::Nil));
// }

const BUFFER_SIZE: usize = 1024;
static OFFSET: usize = 2;

fn add_static() {
    const INCREMENT: usize = 2;
    static mut COUNTER: usize = 1;

    unsafe {
        COUNTER += INCREMENT;
        println!("COUNTER: {}", COUNTER);
    }
}

fn static_example() {
    let offset_ref = &OFFSET;

    // const is not allowed in a reference
    println!("BUFFER_SIZE: {}", BUFFER_SIZE);
    // static is allowed in a reference
    println!("OFFSET: {}", *offset_ref);

    add_static(); // COUNTER: 3
    add_static(); // COUNTER: 5
    add_static(); // COUNTER: 7
}

use std::rc::Rc;
struct DataA {
    number_a: Option<Rc<i32>>,
}

struct DataB {
    number_b: Option<Rc<i32>>,
}

fn set_data(data_a: &mut DataA, data_b: &mut DataB, value: i32) {
    let number = Rc::new(value + 10);
    data_a.number_a = Some(Rc::clone(&number)); // let y = x.clone() ≒ let y = Rc::clone(&x). but latter is faster.
    data_b.number_b = Some(Rc::clone(&number));
}

fn rc_example() {
    let mut data_a_1 = DataA { number_a: None };
    let mut data_b_1 = DataB { number_b: None };
    let mut data_a_2 = DataA { number_a: None };
    let mut data_b_2 = DataB { number_b: None };

    set_data(&mut data_a_1, &mut data_b_1, 1);
    set_data(&mut data_a_2, &mut data_b_2, 2);

    println!("data_a_1: {}", data_a_1.number_a.unwrap());
    println!("data_b_1: {}", data_b_1.number_b.unwrap());
    println!("data_a_2: {}", data_a_2.number_a.unwrap());
    println!("data_b_2: {}", data_b_2.number_b.unwrap());
}

use std::cell::RefCell;
struct Node {
    value: i32,
    child: Option<Rc<RefCell<Node>>>,
}

fn print_link(start_node: Rc<RefCell<Node>>) {
    let mut current_node = start_node;
    loop {
        println!("node: {}", current_node.borrow().value);

        if current_node.borrow().child.is_none() {
            break;
        }

        let tmp = Rc::clone(current_node.borrow().child.as_ref().unwrap());
        current_node = tmp;
    }
}

fn refcall_example() {
    let node_1 = Rc::new(RefCell::new(Node {
        value: 1,
        child: None,
    }));

    let node_2 = Rc::new(RefCell::new(Node {
        value: 2,
        child: None,
    }));

    let node_3 = Rc::new(RefCell::new(Node {
        value: 3,
        child: None,
    }));
    let node_4 = Rc::new(RefCell::new(Node {
        value: 4,
        child: None,
    }));

    node_1.borrow_mut().child = Some(Rc::clone(&node_2));
    node_2.borrow_mut().child = Some(Rc::clone(&node_3));
    node_3.borrow_mut().child = Some(Rc::clone(&node_4));

    print_link(Rc::clone(&node_1));
    print_link(Rc::clone(&node_2));
}
