use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), std::io::Error> {
    let f = File::open("./input")?;
    let reader = BufReader::new(f);

    let mut nums = Vec::with_capacity(100);
    let mut sum: i32 = 0;
    reader.lines().into_iter().for_each(|x| match x {
        Ok(r) => {
            // let i: i32 = r.parse()?;
            if r.eq("") {
                nums.push(sum);
                sum = 0;
            } else {
                sum += r.parse::<i32>().unwrap();
            }
        }
        // Err(err) => println!("x is wrong"),
        _ => println!("x is wrong"),
    });
    nums.sort_by(|a, b| b.cmp(a));
    println!("Part one: {:?}", nums[0]);
    println!(
        "Part two:{:?}",
        // nums.iter().sorted().rev().take(3).sum::<i32>()
        nums.iter().take(3).sum::<i32>()
    );
    Ok(())
}
