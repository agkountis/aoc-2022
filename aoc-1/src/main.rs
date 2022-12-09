use std::error::Error;
use std::fs::File;
use std::io::Read;
use itertools::Itertools;


fn main() -> Result<(), Box<dyn Error>> {

    let mut file = File::open("aoc-1/data/inventories.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    // Fancy way with itertools lib (for group_by) ----------------------------
    let sums = content.lines()
        .group_by(|&s| s != "")
        .into_iter()
        .filter_map(|(_, group)| {
            let v = group.collect_vec();
            let s = v.iter().next().unwrap_or(&"");
            (!s.is_empty()).then_some(v)
        })
        .map(|v| {
            v.into_iter()
                .fold(0, |acc, s| acc + s.parse::<i32>().unwrap_or(0))
        })
        .sorted()
        .rev()
        .take(3)
        .collect_vec();

    println!("Max calories: {}", sums[0]);
    println!("Total top3 calories: {}", sums.iter().sum::<i32>());


    // Traditional way ------------------------------------
    let mut calories_per_elf = vec![];
    let mut calories = 0;
    for line in content.lines() {
        if line == "" {
            calories_per_elf.push(calories);
            calories = 0;
            continue
        }
        calories += line.parse::<i32>()?;
    }

    calories_per_elf.sort();
    calories_per_elf.reverse();

    println!("Max calories: {}", calories_per_elf[0]);
    println!("Total top3 calories: {}", calories_per_elf.iter().take(3).sum::<i32>());

    Ok(())
}
