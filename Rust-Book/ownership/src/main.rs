// fn main() {
//     let first = String::from("Ferris");
//     let full = add_suffix(first);
//     println!("{full}");
// }

// fn add_suffix(mut name: String) -> String {
//     name.push_str(" Jr.");
//     name
// }

fn main() {
    let s = String::from("hello");
    let s2;
    let b = false;

    if b {
        s2 = s;
    }
    println!("{}", s);
}