use std::future::Future;

enum Operation {
    Has(),
    HasLabel(),
}

struct Has {

}

struct Traversal {
    ops: Vec<Operation>, 
}

impl Traversal {
    fn has(self, key: &str, value: &str) -> Self {
        todo!()
    }

    fn has_label() -> Self {
        todo!()
    }
    
    fn faz_has(self) -> Self {
        todo!()
    }

    fn build(self) -> impl Future {
        std::future::ready(1)
    }
}

use cpp::cpp;

cpp!{{
    #include <iostream>
}}

fn main() {
    let name = std::ffi::CString::new("World").unwrap();
    let name_ptr = name.as_ptr();
    let r = unsafe {
        cpp!([name_ptr as "const char *"] -> u32 as "int32_t" {
            std::cout << "Hello, " << name_ptr << std::endl;
            return 42;
        })
    };
    assert_eq!(r, 42)
}
