use rand::Rng;

// my function
use crate::zero_one::game;

pub fn random() {
    let goal = rand::thread_rng().gen_range(2..=180);
    game(goal);
}