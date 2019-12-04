use crate::{NUM_DIGITS, get_feedback};
use std::convert::TryInto;

pub struct MastermindSolver {
    possibilites: Vec<bool>
}

impl MastermindSolver {
    pub fn new() -> Self {
        MastermindSolver {
            possibilites: vec![true; 10usize.pow(NUM_DIGITS)],
        }
    }

    pub fn make_guess(&self) -> u32 {
        for (i, possible) in self.possibilites.iter().enumerate() {
            if *possible {
                return i.try_into().unwrap()
            } else {
                continue
            }
        }
        0
    }

    pub fn use_feedback(&mut self, guess: u32, correct_place: u8, incorrect_place: u8) {
        self.possibilites.iter_mut().enumerate().for_each(|(possible_code, still_possible)| {
            if !*still_possible {
                return
            }
            let [cp, icp] = get_feedback(guess, possible_code.try_into().unwrap());
            *still_possible = cp == correct_place && icp == incorrect_place;
        });
    }
}
