
use std::fs;

fn main() {
    
    //read input file contents into a string called contents
    let input = fs::read_to_string("input").expect("Unable to read file");
    let mut output_buffer = String::new();
    
    //Processing code here (use push or push_str to add to buffer contents)
    let lines:Vec<&str> = input.split_ascii_whitespace().collect();

    let k:usize = lines[0].parse().unwrap();
    let t:usize = lines[1].parse().unwrap();
    let dna:Vec<&str> = lines[2..].to_vec();
    
    let best_motifs = greedy_motif_search(&dna, k, t);

    for s in best_motifs {
        output_buffer.push_str(format!("{}\n", s).as_str());
    }

    //compile buffer into a str and write in output file
    let output = output_buffer.as_str();
    fs::write("output", output).expect("Unable to write file");
}

fn greedy_motif_search<'a>(dna:&'a Vec<&str>, k:usize, t:usize) -> Vec<&'a str>{
    //BestMotifs ← motif matrix formed by first k-mers in each string from Dna
    let mut best_motifs = Vec::new();
    for s in dna {
        best_motifs.push(&s[0..k]);
    }
    // println!("Best motifs:"); 
    // for i in 0..best_motifs.len() {
    //     print!(" {} ", best_motifs[i]);
    // }
    // println!("Score is: {}\n", score(&best_motifs));

    //for each k-mer Motif in the first string from Dna
    for start_index in 0..=(dna[0].len() - k) {
        //Motif1 ← Motif
        let mut motifs = Vec::new();
        motifs.push(&dna[0][start_index..(start_index+k)]);
        //for i = 2 to t
        for i in 1..t {
            //form Profile from motifs Motif1, …, Motifi - 1
            let profile = create_profile(&motifs);
            //Motifi ← Profile-most probable k-mer in the i-th string in Dna
            let motifi = find_profile_most_probable_k_mer(&profile, dna[i]);
            //Motifs ← (Motif1, …, Motift)
            &motifs.push(motifi);
        }

        
        // println!("motifs to be tested:"); 
        // for i in 0..motifs.len() {
        //     print!(" {} ", motifs[i]);
        // }
        // println!("\n");

        //if Score(Motifs) < Score(BestMotifs)
        if score(&motifs) < score(&best_motifs) {
            //BestMotifs ← Motifs
            best_motifs = motifs;

            // println!("New best motifs:"); 
            // for i in 0..best_motifs.len() {
            //     print!(" {} ", best_motifs[i]);
            // }
            // println!("Score is: {}\n", score(&best_motifs));
        }
    }

    //output BestMotifs
    best_motifs
}

fn create_profile(motifs:&Vec<&str>) -> Vec<Vec<f64>>{
    let mut profile = Vec::new();

    for j in 0..motifs[0].len() {
        let mut profile_col = Vec::new();

        let mut total_count = 4.0;
        let mut a_count = 1.0;
        let mut c_count = 1.0;
        let mut g_count = 1.0;
        let mut t_count = 1.0;

        for i in 0..motifs.len() {
            total_count += 1.0;
            if &motifs[i][j..j+1] == "A" {
                a_count += 1.0;
            } else if &motifs[i][j..j+1] == "C" {
                c_count += 1.0;
            } else if &motifs[i][j..j+1] == "G" {
                g_count += 1.0;
            } else if &motifs[i][j..j+1] == "T" {
                t_count += 1.0;
            }
        }

        profile_col.push(a_count/total_count);
        profile_col.push(c_count/total_count);
        profile_col.push(g_count/total_count);
        profile_col.push(t_count/total_count);

        profile.push(profile_col);
    }

    profile
}

fn find_profile_most_probable_k_mer<'a,'b>(profile:&'a Vec<Vec<f64>>, dna:&'b str) -> &'b str {
    let mut max_probability = -1.0;
    let mut most_probable_k_mer = "";

    for i in 0..=(dna.len() - profile.len()) {
        let k_mer = &dna[i..(i + profile.len())];
        let probability = k_mer_probability(profile, k_mer);

        if probability > max_probability {
            max_probability = probability;
            most_probable_k_mer = k_mer;
        }
    }

    most_probable_k_mer
}

fn k_mer_probability(profile:&Vec<Vec<f64>>, k_mer:&str) -> f64 {
    let mut probability = 1.0;

    for i in 0..k_mer.len() {
        if &k_mer[i..i+1] == "A" {
            probability *= profile[i][0];
        } else if &k_mer[i..i+1] == "C" {
            probability *= profile[i][1];
        } else if &k_mer[i..i+1] == "G" {
            probability *= profile[i][2];
        } else if &k_mer[i..i+1] == "T" {
            probability *= profile[i][3];
        }
    }

    probability
}

fn score(motifs:&Vec<&str>) -> usize{
    let mut score = 0;
    let ideal = score_ideal_finder(motifs);

    // println!("ideal: {}", ideal);

    for i in 0..motifs[0].len() {
        for s in motifs {
            if s[i..i+1] != ideal[i..i+1] {
                score += 1;
            }
        }
    }

    score
}

fn score_ideal_finder<'a>(motifs:&'a Vec<&str>) -> String {
    let mut ideal = String::new();
    let profile = create_profile(motifs);

    for col in profile {
        let mut max_index = 0;

        for i in 0..col.len() {
            if col[i] > col[max_index] {
                max_index = i;
            }
        }

        match max_index {
            0 => ideal.push('A'),
            1 => ideal.push('C'),
            2 => ideal.push('G'),
            3 => ideal.push('T'),
            _ => panic!("this shouldn't be possible")
        }
    }

    ideal
}
