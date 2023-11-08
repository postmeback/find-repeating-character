use std::{io::stdin, collections::HashMap};

fn main() {

    let mut repeated_string: String = "No repeated string".to_string();
    let mut my_string = String::new();

    println!("Enter the string");

    stdin().read_line(&mut my_string).expect("Invalid Input");

    let mut char_count: HashMap<char, i32> = HashMap::new();

    for c in my_string.chars() {
         
         if char_count.contains_key(&c) {
              repeated_string = c.to_string();
         }
         else {
            char_count.entry(c).or_default();
         }     
        

    }
    println!("The answer is {}", repeated_string);
}
