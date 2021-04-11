use proconio::input;
fn main() {
    input! {x: String}
    if x.contains(".") {
        let str: &str = x.split(".").next().unwrap();
        println!("{}", str);
    } else {
        println!("{}", x);
    }

}
