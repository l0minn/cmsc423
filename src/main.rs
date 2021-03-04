
use std::fs;

fn main() {
    
    //read input file contents into a string called contents
    let input = fs::read_to_string("input").expect("Unable to read file");
    let mut output_buffer = String::new();
    
    //Processing code here (use push or push_str to add to buffer contents)
    let values:Vec<&str> = input.split_ascii_whitespace().collect();
    let mut spectrum = Vec::new();
    for s in values {
        spectrum.push(s.parse::<usize>().unwrap());
    }

    // for i in &spectrum {
    //     print!("{} ", i);
    // }
    // println!();
    
    let cyclopeptides = cyclopeptide_sequencing(spectrum);
    for peptide in &cyclopeptides {
        for amino_acid in peptide {
            output_buffer.push_str(format!("{}-", amino_acid).as_str());
        }
        output_buffer.pop();
        output_buffer.push_str(" ");
    }

    //compile buffer into a str and write in output file
    let output = output_buffer.as_str();
    fs::write("output", output).expect("Unable to write file");
}

fn cyclopeptide_sequencing(spectrum:Vec<usize>) -> Vec<Vec<usize>>{
    //Find the masses used by the peptide
    let building_blocks = get_compiant_amino_acids(spectrum.clone());

    //Build the candidate peptide table
    let mut running_table = Vec::new();
    for i in building_blocks.clone() {
        running_table.push(vec!(i));
    }

    println!("{}", running_table.len());

    let mut candidate_table = Vec::new();

    //continue to build and trim table until its empty
    while !running_table.is_empty() {
        for peptide in running_table.clone() {
            if matches_spectrum(peptide.clone(), spectrum.clone()) {
                candidate_table.push(peptide);
            }
        }
        running_table = trim_table(expand_peptide_table(running_table.clone(), building_blocks.clone()), spectrum.clone());
        
        println!("{}", running_table.len());

    }

    candidate_table
}

fn get_compiant_amino_acids(spectrum:Vec<usize>) -> Vec<usize> {
    //Parse all unit building blocks
    let block_strs:Vec<&str> = "57 71 87 97 99 101 103 113 114 115 128 129 131 137 147 156 163 186".split_ascii_whitespace().collect();
    let mut building_blocks = Vec::new();
    for s in block_strs {
        building_blocks.push(s.parse::<usize>().unwrap());
    }

    //build vector of compiant amino acids
    let mut compliant_amino_acids = Vec::new();
    let mut i = 0;
    let mut j = 0;
    while i < spectrum.len() && j < building_blocks.len(){
        if building_blocks[j] < spectrum[i] {
            j += 1;
        } else if building_blocks[j] == spectrum[i] {
            compliant_amino_acids.push(building_blocks[j]);
            i += 1;
            j += 1;
        } else {
            i += 1;
        }
    }
    compliant_amino_acids
}

fn expand_peptide_table(table: Vec<Vec<usize>>, building_blocks: Vec<usize>) -> Vec<Vec<usize>> {
    let mut expanded_table = Vec::new();
    for peptide in table {
        for new_block in building_blocks.clone() {
            let mut new_peptide = Vec::new();
            for old_block in peptide.clone() {
                new_peptide.push(old_block);
            }
            new_peptide.push(new_block);
            expanded_table.push(new_peptide.clone());
        }
    }
    expanded_table
}

fn trim_table(table:Vec<Vec<usize>>, spectrum:Vec<usize>) -> Vec<Vec<usize>> {
    let mut trimmed_table = Vec::new();
    for peptide in table {
        if contained_in_spectrum(peptide.clone(), spectrum.clone()) {
            trimmed_table.push(peptide.clone());
        }
    }
    trimmed_table
}

fn contained_in_spectrum(peptide:Vec<usize>, spectrum:Vec<usize>) -> bool {
    let p_spectrum = generate_spectrum(peptide);
    let mut i = 0;
    let mut j = 0;
    while i < spectrum.len() && j < p_spectrum.len(){
        if p_spectrum[j] < spectrum[i] {
            return false
        } else if p_spectrum[j] == spectrum[i] {
            i += 1;
            j += 1;
        } else {
            i += 1;
        }
    }
    true
}

fn generate_spectrum(peptide:Vec<usize>) -> Vec<usize>{
    let mut spectrum = Vec::new();
    //add zero break
    spectrum.push(0);
    spectrum.push(get_mass(peptide.clone()));
    //loop through other breaking possibilities
    for length in 1..peptide.len() {
        for i in 0..peptide.len() {
            let mut mass = 0;
            for j in i..(i+length) {
                mass += peptide[j%peptide.len()];
            }
            spectrum.push(mass);
        }
    }

    spectrum.sort();
    spectrum
}

fn get_mass(peptide:Vec<usize>) -> usize {
    let mut mass = 0;
    for i in peptide {
        mass += i;
    }
    mass
}

fn matches_spectrum(peptide:Vec<usize>, spectrum:Vec<usize>) -> bool {
    let p_spectrum = generate_spectrum(peptide);
    if p_spectrum.len() != spectrum.len() {
        return false
    }
    for i in 0..spectrum.len() {
        if p_spectrum[i] != spectrum[i] {
            return false
        }
    }
    true
}
