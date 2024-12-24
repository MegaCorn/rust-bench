extern crate libc;

use libc::c_int;

#[repr(C)]
pub struct MyClass {
    pub x: c_int,
    pub y: c_int,
}

extern "C" {
    fn malloc_and_access(a: c_int, b: c_int) -> *mut MyClass;
    fn free_memory(ptr: *mut MyClass);
}

fn main() {
    unsafe {
        let ptr = malloc_and_access(1,2);

        if ptr.is_null() {
            println!("malloc fail");
            return;
        }

        println!("{} {}", (*ptr).x, (*ptr).y);

        free_memory(ptr);
    }
}
