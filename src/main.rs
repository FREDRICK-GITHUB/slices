/*In this example, we changed the function parameter from String reference (s: $String) 
to using the slice for the string (s: *str) which is the data type for string literals as well*/
fn main() { // this is a simple exampe to split a string and return first word in string
    let my_string = String::from("hello world!");

    let word = first_word(&my_string[..]);
    println!("the word is: {}",word);

    let my_string_literal = "hello world!";
    let word = first_word(&my_string_literal[..]);
    println!("the word is: {}",word);

    let word = first_word(my_string_literal);
    println!("the word is: {}",word);

    println!("the second word is: {}", second_word(my_string_literal));
    
}

fn first_word(s: &str) -> &str {  // this function takes a reference and returns a reference (borrower)
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() { //iterate through the bytes and add them into a tuple
        if item == b' ' { //if item at the index reference is a space
            return &s[0..i];
        }
    }

    &s[..]
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i..];
        }
    }

    &s[..]
}