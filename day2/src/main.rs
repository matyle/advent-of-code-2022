//  "The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors.
// The second column, you reason, must be what you should play in response: X for Rock, Y for Paper, and Z for Scissors. Winning every time would be suspicious, so the responses must have been carefully chosen.
//
// part2 X:loss ,Y:Draw, Z:Win

//select shape had 1 for rack 2 for paper and 3 for scissors
//lost 0, draw 3 , win 6

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

fn main() -> Result<(), std::io::Error> {
    let battle_map: HashMap<&str, HashMap<&str, i32>> = HashMap::from([
        ("A", HashMap::from([("X", 3), ("Y", 6), ("Z", 0)])),
        ("B", HashMap::from([("X", 0), ("Y", 3), ("Z", 6)])),
        ("C", HashMap::from([("X", 6), ("Y", 0), ("Z", 3)])),
    ]);
    let shape_socre: HashMap<&str, i32> = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);

    let mut score: i32 = 0;

    let battle_outcome_map: HashMap<&str, HashMap<&str, i32>> = HashMap::from([
        ("A", HashMap::from([("X", 3), ("Y", 1), ("Z", 2)])),
        ("B", HashMap::from([("X", 1), ("Y", 2), ("Z", 3)])),
        ("C", HashMap::from([("X", 2), ("Y", 3), ("Z", 1)])),
    ]);

    let res_socre: HashMap<&str, i32> = HashMap::from([("X", 0), ("Y", 3), ("Z", 6)]);

    let mut score_part2: i32 = 0;

    //读取文件
    let f = File::open("./input")?;

    let reader = BufReader::new(f);

    reader.lines().into_iter().for_each(|x| match x {
        Ok(s) => {
            let arr: Vec<&str> = s.split(" ").into_iter().collect();
            score += battle_map[arr[0]][arr[1]] + shape_socre[arr[1]];
            score_part2 += battle_outcome_map[arr[0]][arr[1]] + res_socre[arr[1]];
        }
        _ => println!("error"),
    });
    println!("Part 1 Score: {:?}", score);
    println!("Part 2 Score: {:?}", score_part2);
    Ok(())
}

enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

//实现move
impl Move {
    fn play1(&self, other: Move) -> Outcome {
        todo!()
    }

    fn score1(&self, other: Move) -> i32 {
        todo!()
    }

    fn play2(&self, outcome: Outcome) -> Move {
        todo!()
    }
    fn score(&self, outcome: Outcome) -> i32 {
        todo!()
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl FromStr for Outcome {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
