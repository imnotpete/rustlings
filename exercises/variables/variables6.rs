// variables6.rs
// Execute `rustlings hint variables6` or use the `hint` watch subcommand for a hint.


static NUMBER:i32 = 3;
fn main() {
    static NUMBER: &str = "three";
    println!("Number {}", NUMBER);
}
