use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let vals = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut output = 0;
    let mut output2 = 0;

    let mut group: Vec<String> = Vec::new();

    io::BufReader::new(File::open("input.txt").expect("Should have been able to read the file"))
        .lines()
        .for_each(|maybeLine| {
            let line = maybeLine
                .expect("Sould have been able to read file")
                .clone();
            let splitted = line.split_at(line.len() / 2);
            group.push(line.clone());

            let mut elem = '&';
            for left in splitted.0.chars() {
                for right in splitted.1.chars() {
                    if left == right {
                        elem = left;
                    }
                }
            }
            if (elem != '&') {
                output += vals.find(elem).expect("Should have found a value") + 1;
            }

            if group.len() == 3 {
                'outer: for left in group[0].chars() {
                    for center in group[1].chars() {
                        for right in group[2].chars() {
                            if left == center && center == right {
                                output2 += vals.find(left).expect("Should have found a value") + 1;
                                break 'outer;
                            }
                        }
                    }
                }
                group.pop();
                group.pop();
                group.pop();
            }
        });
    println!("VALEUR: {output}");
    println!("Valeur 2: {output2}");
}
