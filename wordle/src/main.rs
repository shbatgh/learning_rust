use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    io, thread::current, env::current_exe,
    collections::HashMap,
};
use rand::Rng;


fn main() {
    let mut guesses_left = 6;

    let answers = get_answers("C:/Users/grand/dev/learning_rust/wordle/words/answers.txt");
    let rand_number = rand::thread_rng().gen_range(1..=answers.len());
    let answer = &answers[rand_number];
    let answer = to_vec_char(answer);

    println!("{:?}", answer);

    let mut guess = String::new();

    let mut checker: Vec<u8> = Vec::new();

    //guess_checker(&guess, answer);

    while guesses_left > 0 {
        println!("Guess a five-letter word: ");
        
        if guesses_left <= 5 {
            guess.clear();
            checker.clear();
        }

        io::stdin()
            .read_line(&mut guess)
            .expect("Invalid input");

        let guess = guess.trim();
        let guess: Vec<char> = guess.chars().collect();

        println!("{:?}", guess);

        if guess == answer {
            break
        }

        for i in 0..5 {
            let current_letter = guess.get(i).unwrap();
            
            if current_letter == answer.get(i).unwrap() {
                checker.insert(i, 0);
                //println!("{current_letter}, {answer_letter}, {i}, {j}");
            }
            else if !answer.contains(current_letter) {
                checker.insert(i, 2)
            }
            else {
                for j in 0..5 {
                    let answer_letter = answer.get(j).unwrap();
                    
                    if current_letter == answer_letter {
                        checker.insert(i, 1);
                    }
                }
            }
        }

        println!("{:?}", checker);

        guesses_left -= 1;
    }
}

fn to_vec_char(t: &String) -> HashMap<char, usize> {
    let mut chars = HashMap::new();

    for i in 0..5 {
        chars.insert(t.as_bytes()[i] as char, i.eq(&0) as usize);
        //chars.push(t.as_bytes()[i] as char);
    }

    chars
}

fn get_answers(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
