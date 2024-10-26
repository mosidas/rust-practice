#[path = "module_b.rs"]
pub mod module_b;

pub mod module_c; // search for module_c.rs from module_a directory if not path specified
pub mod module_d;
pub mod module_e;

pub fn hello_a() {
    println!("Hello, module_a!");
}
