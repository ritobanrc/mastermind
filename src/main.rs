pub mod solver;

use crate::solver::MastermindSolver;
use rand::prelude::*;
use std::io::{stdin, stdout, Write};
use std::error::Error;
use std::convert::TryInto;

const NUM_DIGITS: usize = 6;
const USER: bool = false;

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let code = rng.gen_range(0, 10u32.pow(NUM_DIGITS.try_into()?));

    let mut solver = MastermindSolver::new()?;

    loop {
        let guess = if USER {
            print!("Guess a 6 digit number: ");
            let _ = stdout().flush();
            let mut s = String::new();
            stdin().read_line(&mut s)?;

            s.trim().parse()?
        } else {
            solver.make_guess()
        };

        println!("Made Guess: {:?}", guess);

        let [correct_place, incorrect_place] = get_feedback(guess, code);

        println!("{:?} {:?}", correct_place, incorrect_place);

        if !USER {
            solver.use_feedback(guess, correct_place, incorrect_place);
        }


        if guess == code {
            println!("I guessed it!");
            break Ok(());
        }
    }
}

pub fn get_feedback(guess: u32, code: u32) -> [u8; 2] {
    let guess = get_digits(guess);
    let code = get_digits(code); // OPT: Cache this

    let mut guess_unchecked = [true; NUM_DIGITS];
    let mut code_unchecked = [true; NUM_DIGITS];

    //let mut code_store = code.clone();

    let mut correct_place = 0;
    let mut incorrect_place = 0;

    for (i, (gd, cd)) in guess.iter().zip(&code).enumerate() {
        if gd == cd {
            correct_place += 1;
            guess_unchecked[i] = false;
            code_unchecked[i] = false;
        }
    }

    for (i, gd) in guess.iter().enumerate() {
        for (j, cd) in code.iter().enumerate() {
            if gd == cd && code_unchecked[j] && guess_unchecked[i] && i != j {
                incorrect_place += 1;
                guess_unchecked[i] = false;
                code_unchecked[j] = false;
            }
        }
    }


    [correct_place, incorrect_place]
}



fn get_digits(n: u32) -> Vec<u32> {
    fn next_digit(n: u32, digits: &mut Vec<u32>, position: usize) {
        if n >= 10 {
            next_digit(n / 10, digits, position - 1);
        }
        digits[position] = n % 10;
    }

    let mut digits = vec![0; 6];
    next_digit(n, &mut digits, 5);
    digits
}

#[cfg(test)]
mod tests {
    use super::*;

    // NOTE: This test currently does not work. This is because get_feedback does not handle
    // repetition properly.
    #[test]
    fn test_feedback() {
        let [correct_place, incorrect_place] = get_feedback(125577, 123456);
        assert_eq!(correct_place, 2);
        assert_eq!(incorrect_place, 1);
    }
}

