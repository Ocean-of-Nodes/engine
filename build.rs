extern crate cpp_build;

fn main() {
    let include_path = "/usr/include/myexternallib";
    let lib_path = "/usr/lib/myexternallib";
    cpp_build::Config::new().include(include_path).build("src/main.rs");
}