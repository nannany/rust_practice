
fn main() {
    input! {a: i32, b: i32}
    let c = a + b;
    if 15 <= c && 8 <= b {
        println!("{}", 1);
    } else if 10 <= c && 3 <= b {
        println!("{}", 2);
    } else if 3 <= c {
        println!("{}", 3);
    } else {
        println!("{}", 4);
    }
}
