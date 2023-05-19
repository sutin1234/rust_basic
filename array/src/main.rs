use core::num;

fn main() {
    let arr = [1, 2, 3, 4, 5];
    for i in 0..arr.len() {
        println!("index: {}  value: {}", i, arr[i]);
    }

    for n in arr.iter() {
        print!("{}: ", n);
    }

    let lang = ["Rust", "Java", "Go", "Python"];
    for i in 0..lang.len() {
        println!("{}", lang[i]);
    }

    let number = [2; 100];
    for num in 0..number.len() {
        println!("{}", num);
    }
}
