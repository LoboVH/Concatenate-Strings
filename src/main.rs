fn concatenate_strings(x: &str, y: &str) -> String {
    let mut result = String::new();
    result.push_str(x);
    result.push_str(y);
    result
}

fn main() {
    let string1 = "Hello";
    let string2 = "World";
    
    let concatenated_string = concatenate_strings(&string1, &string2);

    println!("Concatenated string is: {:?}", concatenated_string); // Output: "HelloWorld"

    let formatted_string = format!("{},{}", &string1, &string2);
    
    println!("formatted string is: {:?}", formatted_string); // Output: "Hello,World"
}
