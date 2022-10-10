
fn main() { // this is a simple exampe to split a string and return first word in string
    let ss = String::from("hello world!");

    let first_word = first_word(&ss);

    println!("the first word in the above string is: {}",first_word);
    
}

fn first_word(s: &String) -> &str {  // this function takes a reference and returns a reference (borrower)
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() { //iterate through the bytes and add them into a tuple
        if item == b' ' { //if item at the index reference is a space
            return &s[0..i];
        }
    }

    &s[..]
}