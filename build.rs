extern crate gl_generator;

use gl_generator::{Api, Fallbacks, 
    StaticGenerator, 
    //GlobalGenerator,
    Profile, Registry};
use std::env;
use std::fs::File;
use std::path::Path;


// This is required for OpenGL on macOS.
fn main() {
    let dest = env::var("OUT_DIR").unwrap();
    let mut file = File::create(&Path::new(&dest).join("bindings.rs")).unwrap();

    Registry::new(Api::Gl, (2, 1), Profile::Core, Fallbacks::All, [])
        .write_bindings(StaticGenerator, &mut file)
        .unwrap();
}

