use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::num::ParseIntError;

#[derive(Debug, Copy, Clone)]
struct SectionRange {
    start: i32,
    end: i32,
}

impl SectionRange {
    pub fn new(start: i32, end: i32) -> Self {
        Self {
            start,
            end
        }
    }

    pub fn contains(&self, rhs: &SectionRange) -> bool {
        if self.start <= rhs.start && self.end >= rhs.end {
            return true
        }

        false
    }

    pub fn overlaps(&self, rhs: &SectionRange) -> bool {
        if self.start >= rhs.start && self.start <= rhs.end ||
            self.end >= rhs.start && self.end <= rhs.end {
            return true
        }

        false
    }
}

impl TryFrom<(&str, &str)> for SectionRange {
    type Error = ParseIntError;

    fn try_from(value: (&str, &str)) -> Result<Self, Self::Error> {
        Ok(Self {
            start: value.0.parse::<i32>()?,
            end: value.1.parse::<i32>()?,
        })
    }
}

fn part1(content: &str) {
    let count = content.lines()
        .filter_map(|line| {
            line.split_once(",")
        })
        .filter_map(|(a, b)| {
            a.split_once("-")
                .zip(b.split_once("-"))
        })
        .filter_map(|(range1, range2)| {
            SectionRange::try_from(range1).ok()
                .zip(SectionRange::try_from(range2).ok())
        })
        .filter_map(|(range1, range2)| {
            (range1.contains(&range2) || range2.contains(&range1)).then_some(())
        })
        .count();

    println!("Fully contained: {}", count)
}

fn part2(content: &str) {
    let count = content.lines()
        .filter_map(|line| {
            line.split_once(",")
        })
        .filter_map(|(a, b)| {
            a.split_once("-")
                .zip(b.split_once("-"))
        })
        .filter_map(|(range1, range2)| {
            SectionRange::try_from(range1).ok()
                .zip(SectionRange::try_from(range2).ok())
        })
        .filter_map(|(range1, range2)| {
            (range1.overlaps(&range2) || range2.overlaps(&range1)).then_some(())
        })
        .count();

    println!("Overlaps: {}", count)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("aoc-4/data/section_ids.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    part1(&content);
    part2(&content);

    Ok(())
}

