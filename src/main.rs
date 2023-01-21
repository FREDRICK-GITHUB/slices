
fn main() { 
   //This is a string. Stored on heap because size can expand or shrink during runtime
   let string_one = String::from("127.0.0.1:8080");

   //unofficial way of showing the slice from the first bit, return three bits
   let string_slice = &string_one[0..3];

   //borrowing a String as a string slice
   let borrowed_string: &str = &string_one;
   
   //string literal is treated as a string slice since size is known at compile time
   let string_literal = "1234";

   //find() returns the string slice at a given bit
   let indexer = &string_literal.find("2");

   dbg!(&string_one);
   dbg!(string_slice);
   dbg!(borrowed_string);
   dbg!(string_literal);
   dbg!(indexer);
   println!("slice values: {} from string: {}", string_slice, string_one);
}

