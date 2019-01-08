use std::{thread, time};
use std::i32::MAX;
use std::os::raw::c_int;
extern {
    fn sayhello(x: c_int);
}

fn main() {
    
    println!("{}", "Running rust-usdt-helloworld");
    let mut number = 0;
    loop {
        
        let secs = time::Duration::from_millis(3000);
        thread::sleep(secs);
        if number + 1 >= MAX {
            number = 0;
        }
        number += 1;

        unsafe {
            sayhello(number);
        }
    }
}
