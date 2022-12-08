//  "The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors.
// The second column, you reason, must be what you should play in response: X for Rock, Y for Paper, and Z for Scissors. Winning every time would be suspicious, so the responses must have been carefully chosen.

//select shape had 1 for rack 2 for paper and 3 for scissors
//lost 0, draw 3 , win 6

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), std::io::Error> {
    let battle_map: HashMap<&str, HashMap<&str, i32>> = HashMap::from([
        ("A", HashMap::from([("X", 3), ("Y", 6), ("Z", 0)])),
        ("B", HashMap::from([("X", 0), ("Y", 3), ("Z", 6)])),
        ("C", HashMap::from([("X", 6), ("Y", 0), ("Z", 3)])),
    ]);
    let shape_socre: HashMap<&str, i32> = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);

    let mut score: i32 = 0;

    //读取文件
    let f = File::open("./input")?;

    let reader = BufReader::new(f);

    reader.lines().into_iter().for_each(|x| match x {
        Ok(s) => {
            let arr: Vec<&str> = s.split(" ").into_iter().collect();
            score += battle_map[arr[0]][arr[1]] + shape_socre[arr[1]]
        }
        _ => println!("error"),
    });

    println!("Score: {:?}", score);
    Ok(())
}
