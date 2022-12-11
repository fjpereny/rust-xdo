
mod rust_xdo;

fn main() {

    let x = 50;
    let x: String = x.to_string();
    println!("{}", x);

    // println!("Pressing 1 key");
    // rust_xdo::press("1\n"); 
    // rust_xdo::typewrite("Hello, world!".to_string(), 37);
    // rust_xdo::get_active_window();
}
