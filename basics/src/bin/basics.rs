extern crate basics;

use basics::lib_if;
use basics::lib_bindings;
use basics::lib_functions;
use basics::lib_compounds;

fn main() {
    println!("main function called!");

    lib_bindings::basics_bindings();
    lib_if::basics_if();
    lib_functions::foo();

    println!("{}", lib_functions::add_one(32));
    println!("{}", lib_functions::add_five(32));

    lib_compounds::my_tuples();
    lib_compounds::my_structs();
    lib_compounds::my_enums();
}
