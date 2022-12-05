use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

const OP_ROCK: &str = "A";
const OP_PAPER: &str = "B";
const OP_SCISSOR: &str = "C";

const MY_ROCK: &str = "X";
const MY_PAPER: &str = "Y";
const MY_SCISSOR: &str = "Z";

const LOSS: &str = "X";
const DRAW: &str = "Y";
const WIN: &str = "Z";

const ROCK_SCORE: u32 = 1;
const PAPER_SCORE: u32 = 2;
const SCISSOR_SCORE: u32 = 3;

const WIN_SCORE: u32 = 6;
const DRAW_SCORE: u32 = 3;
const LOSS_SCORE: u32 = 0;

enum MatchOutcome {
    Loss,
    Draw,
    Win,
}

fn main() {
    let mut score: u32 = 0;
    let path = Path::new("input");
    let file = File::open(&path).expect("Could not open file");

    let buf_reader = BufReader::new(file);

    for line in buf_reader.lines() {
        let turn = line.unwrap();
        let op_move:&str = &turn[..1];
        let outcome:&str = &turn[2..];
        let my_move = choose_move(op_move, outcome); 
        score += get_score(op_move, my_move);
    }
    println!("Score: {}", score);
}

fn get_score(op_move: &str, my_move: &str) -> u32 {
    let mut score: u32 = 0;

    match my_move {
        MY_ROCK => score += ROCK_SCORE,
        MY_PAPER => score += PAPER_SCORE,
        MY_SCISSOR => score += SCISSOR_SCORE,
        &_ => panic!("Unknown move {}", my_move),
    };

    match get_outcome(op_move, my_move) {
        MatchOutcome::Loss => score += LOSS_SCORE,
        MatchOutcome::Draw => score += DRAW_SCORE,
        MatchOutcome::Win => score += WIN_SCORE,
    };

    return score;
}

fn get_outcome(op_move: &str, my_move: &str) -> MatchOutcome {

    match op_move {
        OP_ROCK => {
            if my_move == MY_ROCK {
                return MatchOutcome::Draw;
            } else if my_move == MY_PAPER {
                return MatchOutcome::Win;
            } else if my_move == MY_SCISSOR {
                return MatchOutcome::Loss;
            } else {
                panic!("Unknown outcome!");
            }
        }
        OP_PAPER => {
            if my_move == MY_PAPER {
                return MatchOutcome::Draw;
            } else if my_move == MY_SCISSOR {
                return MatchOutcome::Win;
            } else if my_move == MY_ROCK {
                return MatchOutcome::Loss;
            } else {
                panic!("Unknown outcome!");
            }
        }
        OP_SCISSOR => {
            if my_move == MY_SCISSOR {
                return MatchOutcome::Draw;
            } else if my_move == MY_ROCK {
                return MatchOutcome::Win;
            } else if my_move == MY_PAPER {
                return MatchOutcome::Loss;
            } else {
                panic!("Unknown outcome!");
            }
        }
        &_ => panic!("Unknown move!")
    }
}

fn choose_move(op_move: &str, outcome: &str) -> &'static str {
    match outcome{
        LOSS => {
            match op_move{
                OP_ROCK => return MY_SCISSOR,
                OP_PAPER => return MY_ROCK,
                OP_SCISSOR => return MY_PAPER,
                &_ => panic!("unable to choose move"),
            } 
        }
        DRAW => {
            match op_move{
                OP_ROCK => return MY_ROCK,
                OP_PAPER => return MY_PAPER,
                OP_SCISSOR => return MY_SCISSOR,
                &_ => panic!("unable to choose move"),
            } 
        }
        WIN => {
            match op_move{
                OP_ROCK => return MY_PAPER,
                OP_PAPER => return MY_SCISSOR,
                OP_SCISSOR => return MY_ROCK,
                &_ => panic!("unable to choose move"),
            }
        }
        &_=> panic!("Unable to choose move"),
    }
}

