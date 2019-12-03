use rand::prelude::*;
use std::io::{stdin, stdout, Write};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let code = rng.gen_range(0, 1_000_000);

    dbg!("Code: {:?}", code);

    loop {
        print!("Guess a 6 digit number: ");
        let _ = stdout().flush();
        let mut s = String::new();
        stdin().read_line(&mut s)?;

        let guess: u32 = s.trim().parse()?;

        let [correct_place, incorrect_place] = get_feedback(guess, code);

        println!("{:?} {:?}", correct_place, incorrect_place);

        if guess == code {
            println!("I guessed it!");
            break Ok(());
        }
    }

}

fn get_feedback(guess: u32, code: u32) -> [u8; 2] {
    let guess = get_digits(guess);
    let code = get_digits(code); // OPT: Cache this

    let mut code_store = code.clone();

    let mut correct_place = 0;
    let mut incorrect_place = 0;

    for (gd, cd) in guess.iter().zip(&code) {
        if gd == cd {
            correct_place += 1;
            code_store.retain(|x| x != gd);
        } else if code_store.contains(gd) {
            incorrect_place += 1;
            code_store.retain(|x| x != gd);
        }
    }

    [correct_place, incorrect_place]
}



fn get_digits(n: u32) -> Vec<u32> {
    fn next_digit(n: u32, digits: &mut Vec<u32>) {
        if n >= 10 {
            next_digit(n / 10, digits);
        }
        digits.push(n % 10);
    }

    let mut digits = Vec::new();
    next_digit(n, &mut digits);
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
