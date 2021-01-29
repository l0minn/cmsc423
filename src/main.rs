
use std::fs;

fn main() {
    
    //read input file contents into a string called contents
    let contents = fs::read_to_string("input").expect("Unable to read file");

    //count letters (ignores if not one of the 4 match options)
    let mut a_count = 0;
    let mut c_count = 0;
    let mut g_count = 0;
    let mut t_count = 0;
    for c in contents.chars() {
        match c {
            'A' => a_count = a_count + 1,
            'C' => c_count = c_count + 1,
            'G' => g_count = g_count + 1,
            'T' => t_count = t_count + 1,
            _ => (),
        }
    }

    //write count results to output file
    let output_data = format!("{} {} {} {}", a_count, c_count, g_count, t_count);
    fs::write("output", output_data).expect("Unable to write file");
}
