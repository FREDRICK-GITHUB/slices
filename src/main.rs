
fn main() {
    let s = String::from("hello world!");

    let hello = &s[0..5]; //slice string up to the fourth index
    let world = &s[6..12]; //slice string from the fifth to the eleventh index
    let all = &world[..];  //slice the string from the first to the last index

    println!("{}", hello);
    println!("{}", world);
    println!("{}", all);
}
