
fn main() { 
   let a = [1,2,3,4,5];
   
   let slice = &a[2..4];

   println!("the sliced array has the following: {:?}", slice); //normal print for array
   println!("the sliced array has the following: {:#?}", slice); //pretty print
}

