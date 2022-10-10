
fn main() { // this is a simple exampe to split a string and return first word in string
    let ss = String::from("hello world!");

    let first_word = first_word(&ss);

    let second_word = second_word(&ss);

    println!("the first word in the above string is: {}",first_word);
    
    println!("the second word in the above string is: {}", second_word);
    
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

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i..];
        }
    }

    &s[..]
}