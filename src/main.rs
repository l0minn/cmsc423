
use std::fs;
use std::collections::HashMap;

fn main() {
    
    //read input file contents into a string called contents
    let input = fs::read_to_string("input").expect("Unable to read file");
    let mut output_buffer = String::new();
    
    //Processing code here (use push or push_str to add to buffer contents)

    let processed_inpput:String = input.split_ascii_whitespace().collect();
    output_buffer.push_str(bwt_decrypt(processed_inpput.trim()).as_str());

    //compile buffer into a str and write in output file
    let output = output_buffer.as_str();
    fs::write("output", output).expect("Unable to write file");
}

fn bwt_decrypt(text:&str) -> String {
    
    let last_row = index_characters(text);
    let first_row = index_characters(sort_characters(text).as_str());

    let mut decrypted_text = String::new();
    let mut target = &String::from("$");

    loop {
        for i in 0..last_row.len() {
            if last_row[i] == *target {
                target = &first_row[i];
                break;
            }
        }

        if *target == String::from("$") {
            decrypted_text.push_str("$");
            break;
        }

        decrypted_text.push_str(target.get(0..1).unwrap());
    }

    decrypted_text
}

fn index_characters(text:&str) -> Vec<String> {

    let mut indexed_characters = Vec::new();
    let mut char_counts = HashMap::new();

    for c in text.chars() {
        if c == '$' {
            indexed_characters.push(format!("$"));
            continue;
        }

        if char_counts.contains_key(&c) {
            let count_clone = char_counts.clone();
            let count = count_clone.get(&c).unwrap();
            indexed_characters.push(format!("{}{}", c, count));
            char_counts.insert(c, count+1);
        } else {
            indexed_characters.push(format!("{}0", c));
            char_counts.insert(c, 1);
        }
    }
    
    indexed_characters
}

fn sort_characters(text:&str) -> String {

    let mut sorted_characters = String::new();
    let mut char_counts = HashMap::new();

    for c in text.chars() {
        if c == '$' {
            continue;
        }

        if char_counts.contains_key(&c) {
            char_counts.insert(c, char_counts.get(&c).unwrap()+1);
        } else {
            char_counts.insert(c, 1);
        }
    }

    sorted_characters.push_str("$");

    while !char_counts.is_empty() {
        let count_clone = char_counts.clone();
        let keys:Vec<&char> = count_clone.keys().collect();
        let mut min_char = keys[0];

        for k in keys {
            if k < &min_char {
                min_char = k;
            }
        }

        for _i in 0..*char_counts.get(min_char).unwrap() {
            sorted_characters.push_str(format!("{}", min_char).as_str());
        }

        char_counts.remove(min_char);
    }
    
    sorted_characters
}
