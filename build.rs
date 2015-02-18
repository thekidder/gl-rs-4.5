#![feature(env)]
#![feature(io)]
#![feature(path)]

extern crate gl_generator;    // <-- this is your build dependency
extern crate khronos_api;    // included by gl_generator

use std::env;
use std::old_io::File;

fn main() {
    let dest = Path::new(env::var("OUT_DIR").unwrap());

    let mut file = File::create(&dest.join("gl_bindings.rs")).unwrap();

    gl_generator::generate_bindings(
        gl_generator::GlobalGenerator,
        gl_generator::registry::Ns::Gl,
        khronos_api::GL_XML,
        vec![],
        "4.5", "core", &mut file).unwrap();
}
