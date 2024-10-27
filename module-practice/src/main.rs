mod module_a;
use module_a::module_d; // import module with use
use module_a::module_e::hello_e; // import function with use
use module_a::module_e::{hello_e2, hello_e3}; // import multiple functions with use

fn main() {
    module_a::hello_a();
    module_a::module_b::hello_b();
    module_a::module_c::hello_c();
    module_d::hello_d();
    hello_e();
    hello_e2();
    hello_e3();
}
