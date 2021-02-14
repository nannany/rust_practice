fn main() {
    let mut vec: Vec<usize> = Vec::<usize>::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(5);
    let mut iter = vec.iter();
    while let Some(ele) = iter.next() {
        println!("{}", ele);
    }
}
