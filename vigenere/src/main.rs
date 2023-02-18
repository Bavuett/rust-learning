use std::fs;

fn main() {
    println!("Vigenere Bruteforce.");

    let string_from_file: String = fs::read_to_string("message.txt").expect("Unable to read file");
    let mut shifted_strings: Vec<String> = Vec::new();
    let string_array: Vec<char> = string_from_file.chars().collect();
    
    println!("Message: {}. Length: {}.\n", string_from_file, string_from_file.len());

    for i in 0..string_array.len() {
        let mut shifted_string: String = String::new();

        for j in 0..string_array.len() - i {
            shifted_string.push(string_array[j]);
        }

        shifted_strings.push(shifted_string);
    }

    for i in 0..shifted_strings.len() {
        let number_of_spaces: usize = string_array.len() - shifted_strings[i].len();
        let shifted_string_to_vec: Vec<char> = shifted_strings[i].chars().collect();

        let mut string_vector_to_compare: Vec<char> = Vec::new();

        for _j in 0..number_of_spaces {
            string_vector_to_compare.push(' ');
        }

        for j in 0..shifted_string_to_vec.len() {
            string_vector_to_compare.push(shifted_string_to_vec[j]);
        }

        println!("Number of spaces: {}. \nString vector to compare: {:?}\n", number_of_spaces, string_vector_to_compare);
    }
}
