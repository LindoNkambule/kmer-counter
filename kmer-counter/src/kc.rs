use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;

fn kmer_count(seq: String, k: i32, mut h: HashMap<String, i32>) -> HashMap<String, i32> {
    let _len = seq.len();

    // let x = _len as i32;

    let range_upper: i32 = (_len as i32) -k + 1;

    for i in 0..range_upper {

        // let start: usize = i as usize;
        // let stop: usize =  (i + k) as usize;
        // let string = String::from(seq);

        let _kmer = &seq[i as usize..(i + k) as usize];

        if h.contains_key(_kmer) {
            *h.get_mut(_kmer).unwrap() += 1;
        } else {
            h.insert(_kmer.to_string(), 1);
        }
    }

    return h;
}


pub(crate) fn count_fasta_seq(file_path: &str) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut records = 0;

    let mut seq_hash: HashMap<String, i32> = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if line.chars().nth(0).unwrap() == '>' {
            records +=1;
            continue;
        }
        else{
            // run kmer count on the sequence
            seq_hash = kmer_count(line, 4, seq_hash)            
        }
    }

    let mut k_total: i32 = 0;
    let mut k_uniq: i32 = 0;
    for (key_var, value_var) in seq_hash.iter() {
        k_total += value_var;
        k_uniq += 1;
        println!("{}: {}", key_var, value_var);
    }

    println!("There are {} records in the file", records);
    println!("There are {} unique 4-mers in the file", k_uniq);

    Ok(())
}