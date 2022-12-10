//Lowercase item types a through z have priorities 1 through 26.
//Uppercase item types A through Z have priorities 27 through 52.
//1.read line to String to []char
//2. part to 2 arrays
// if even [1,2,3,4] [0,len/2) [len/2,len)
// if odd arr[1,2,3] => [0,(len-1)/2) [(len-1)/2,len-1)
// 3. find same char give the priority

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::BufReader,
};

fn main() -> Result<(), std::io::Error> {
    // Table Method
    let mut priority_map: HashMap<char, u32> = HashMap::new();
    ('a'..='z').into_iter().enumerate().for_each(|(i, c)| {
        priority_map.insert(c, i as u32 + 1);
    });

    // let mut uppercase_map: HashMap<char, u32> = HashMap::new();
    ('A'..='Z').into_iter().enumerate().for_each(|(i, c)| {
        priority_map.insert(c, i as u32 + 27);
    });

    //read
    // let f = File::open("./input")?;
    // let reader = BufReader::new(f);
    let mut sum: u32 = 0;

    //line -> string->[]char
    // include_str!("../input").lines().for_each(|l| {
    // println!("{:?}", l.chars());
    // let length = l.chars().count();
    // let mut counts: HashMap<char, u32> = HashMap::new();
    // l.chars().enumerate().for_each(|(i, c)| {
    //     if i < length / 2 {
    //         counts.entry(c).or_insert(1);
    //     } else {
    //         counts.entry(c).and_modify(|e| *e += 1).or_insert(1);
    //     }
    // });

    // sum += counts
    //     .into_iter()
    //     .filter(|(_, v)| *v > 1)
    //     .fold(0, |acc, (_, v)| acc + v);
    // });

    include_str!("../input")
        .lines()
        .map(|l| l.split_at(l.len() / 2)) //learn split_at
        .for_each(|(f, s)| {
            // let mut counts: HashMap<char, u32> = HashMap::new();
            let mut visit = false;
            f.chars().into_iter().for_each(|c| {
                if s.contains(c) && !visit {
                    sum += priority_map[&c];
                    visit = true;
                }
            })
        });
    // println!("{:?}", m.collect::Vec<String>());
    println!("Part 1:{:?}", sum);

    let mut sum_part2 = 0;
    let v: Vec<&str> = include_str!("../input").lines().collect();
    v.chunks(3).into_iter().for_each(|cs| {
        sum_part2 += prioritys(cs[0], cs[1], cs[2])
            .into_iter()
            .fold(0, |acc, x| acc + priority_map[&x]);
    });
    println!("Part 2:{:?}", sum_part2);
    Ok(())
}

fn prioritys(first: &str, second: &str, third: &str) -> Vec<char> {
    let f: HashSet<_> = first.chars().into_iter().collect();
    let s: HashSet<_> = second.chars().into_iter().collect();
    let t: HashSet<_> = third.chars().into_iter().collect();
    //f s intersection
    let mut res: Vec<char> = Vec::new();
    f.intersection(&s).for_each(|x| {
        if t.contains(x) {
            res.push(x.clone());
        }
    });
    res
}
