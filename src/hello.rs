use std::io::BufRead;

#[link(name = "hello")]
extern {
    fn sayhello(x: u32) -> u32;
}

fn main() {
    unsafe {
       let i = sayhello(0);
       println!("hello world! {:?}", i);
    }
    
    let mut line = String::new();
    let stdin = std::io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    println!("{}", line)

}
