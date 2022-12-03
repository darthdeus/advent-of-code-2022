#![allow(dead_code)]

#[derive(Copy, Clone, Debug)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    pub fn from(text: &str) -> Choice {
        match text {
            "A" | "X" => Choice::Rock,
            "B" | "Y" => Choice::Paper,
            "C" | "Z" => Choice::Scissors,
            _ => unreachable!(),
        }
    }

    pub fn part2(enemy: Choice, text: &str) -> Choice {
        match text {
            // lose
            "X" => match enemy {
                Choice::Rock => Choice::Scissors,
                Choice::Paper => Choice::Rock,
                Choice::Scissors => Choice::Paper,
            },
            // draw
            "Y" => enemy,
            // win
            "Z" => match enemy {
                Choice::Rock => Choice::Paper,
                Choice::Paper => Choice::Scissors,
                Choice::Scissors => Choice::Rock,
            },
            _ => unreachable!(),
        }
    }

    pub fn score(&self) -> i32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    pub fn resolve(&self, other: Choice) -> i32 {
        let won = 6;
        let draw = 3;
        let lost = 0;

        match self {
            Choice::Rock => match other {
                Choice::Rock => draw,
                Choice::Paper => lost,
                Choice::Scissors => won,
            },
            Choice::Paper => match other {
                Choice::Rock => won,
                Choice::Paper => draw,
                Choice::Scissors => lost,
            },
            Choice::Scissors => match other {
                Choice::Rock => lost,
                Choice::Paper => won,
                Choice::Scissors => draw,
            },
        }
    }
}

fn part1() -> Vec<(Choice, Choice)> {
    let tournament = include_str!("../../input1")
        .lines()
        .map(|x| {
            if let [a, b] = x.split(" ").collect::<Vec<_>>()[..] {
                (Choice::from(a), Choice::from(b))
            } else {
                unreachable!("bad input");
            }
        })
        .collect::<Vec<_>>();

    tournament
}

fn part2() -> Vec<(Choice, Choice)> {
    let tournament = include_str!("../../input1")
        .lines()
        .map(|x| {
            if let [a, b] = x.split(" ").collect::<Vec<_>>()[..] {
                let enemy_choice = Choice::from(a);
                (enemy_choice, Choice::part2(enemy_choice, b))
            } else {
                unreachable!("bad input");
            }
        })
        .collect::<Vec<_>>();

    tournament
}

fn main() {
    let tournament = part2();
    let mut total = 0;

    for (a, b) in tournament.into_iter() {
        total += b.score() + b.resolve(a);
    }

    println!("{total}");
}
