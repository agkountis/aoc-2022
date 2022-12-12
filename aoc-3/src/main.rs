use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::Read;

fn generate_priorities_table() -> HashMap<char, i8> {
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    alphabet.chars()
        .enumerate()
        .map(|(index, char)| (char, index as i8 + 1i8))
        .collect()
}

fn part1(content: &str) {
    let priorities = generate_priorities_table();
    let sum = content.lines()
        .map(|rucksack| {
            rucksack.split_at(rucksack.len() / 2)
        })
        .map(|(comp_1, comp_2)| {
            let h1 = comp_1.chars().collect::<HashSet<_>>();
            let h2 = comp_2.chars().collect::<HashSet<_>>();

            h1.intersection(&h2)
                .next()
                .map_or(0, |c| priorities[c] as i32)
        })
        .fold(0, |acc, priority| acc + priority);
    println!("Wrong item priority sum: {}", sum);
}

fn part2(content: &str) {
    let priorities = generate_priorities_table();
    let lines = content.lines().collect::<Vec<&str>>();
    let sum = (&lines)
        .chunks(3)
        .map(|slice| {
            let unique_items1 = slice[0].chars().collect::<HashSet<_>>();
            let unique_items2 = slice[1].chars().collect::<HashSet<_>>();
            let unique_items3 = slice[2].chars().collect::<HashSet<_>>();

            unique_items1
                .intersection(&unique_items2)
                .copied()
                .collect::<HashSet<_>>()
                .intersection(&unique_items3)
                .next()
                .map_or(0, |c| priorities[c] as i32)
        })
        .fold(0, |acc, priority| acc + priority);
    println!("Badge item priority sum: {}", sum);
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("aoc-3/data/rucksacks.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    part1(&content);
    part2(&content);

    Ok(())
}
