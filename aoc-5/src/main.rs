use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::ops::{Index, IndexMut};
use itertools::Itertools;

#[derive(Default, Debug, Copy, Clone)]
pub struct Command {
    pub amount: usize,
    pub from: usize,
    pub to: usize,
}

impl Command {
    pub fn generate_commands(content: &str) -> Vec<Command> {
        content.lines()
            .skip_while(|l| !l.is_empty())
            .collect::<Vec<_>>()
            .into_iter()
            .skip(1)
            .map(|line| {
                let moves = line.split_whitespace()
                    .filter_map(|s| s.to_string().parse::<usize>().ok())
                    .collect_vec();

                let amount = moves[0];
                let from = moves[1] - 1;
                let to = moves[2] - 1;
                Command {
                    amount,
                    from,
                    to,
                }
            })
            .collect()
    }
}

pub enum Crane {
    CrateMover9000,
    CrateMover9001,
}

impl Crane {
    pub fn move_crates(&self, commands: Vec<Command>, stacks: &mut Stacks) {
        match self {
            Crane::CrateMover9000 => Self::move_behavior_9000(commands, stacks),
            Crane::CrateMover9001 => Self::move_behavior_9001(commands, stacks),
        }
    }

    fn move_behavior_9000(commands: Vec<Command>, stacks: &mut Stacks) {
        for command in commands.into_iter() {
            let Command {
                amount,
                from,
                to
            } = command;

            for _ in 0..amount {
                if let Some(c) = stacks[from].pop() {
                    stacks[to].push(c)
                }
            }
        }
    }

    fn move_behavior_9001(commands: Vec<Command>, stacks: &mut Stacks) {
        for command in commands.into_iter() {
            let Command {
                amount,
                from,
                to
            } = command;

            let mut from_crates = Vec::with_capacity(amount);
            for _ in 0..amount {
                if let Some(c) = stacks[from].pop() {
                    from_crates.push(c)
                }
            }
            from_crates.reverse();
            stacks[to].append(&mut from_crates)
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Stacks {
    stacks: Vec<Vec<char>>
}

impl Stacks {
    pub fn new(content: &str) -> Self {
    let stack_input = content.lines().take_while(|l| !l.is_empty()).collect::<Vec<_>>();

    let stacks_size = stack_input.iter()
            .last()
            .unwrap()
            .chars()
            .filter(|&c| c != ' ')
            .count();

        let stacks = Self {
            stacks: (0..stacks_size).map(|_| vec![]).collect(),
        };

        stack_input.iter()
            .rev()
            .skip(1)
            .fold(stacks, |mut stacks, &line| {
                let _ = &line.chars()
                    .chunks(4)
                    .into_iter()
                    .enumerate()
                    .fold(&mut stacks, |stacks, (index, chunk)| {
                        let c = chunk.filter(|&c| c != '[').next().unwrap();
                        if c != ' ' {
                            stacks.stacks[index].push(c);
                        }
                        stacks
                    });
                stacks
            })
    }

    pub fn print_top_of_stacks(&self) {
        for stack in self.stacks.iter() {
            let c = if stack.is_empty() {
                '-'
            } else {
                stack[stack.len() - 1]
            };

            print!("{c}")
        }

        println!()
    }

    pub fn print_stacks(&self) {
        for (i, stack) in self.stacks.iter().enumerate() {
            println!("{i}: {:?}", stack)
        }
    }
}

impl Index<usize> for Stacks {
    type Output = Vec<char>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.stacks[index]
    }
}

impl IndexMut<usize> for Stacks {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.stacks[index]
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("aoc-5/data/stacks_and_commands.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;


    let commands = Command::generate_commands(&content);
    let commands2 = commands.clone();
    let mut stacks = Stacks::new(&content);
    let mut stacks2 = stacks.clone();

    // Part1
    let mut crane = Crane::CrateMover9000;
    crane.move_crates(commands, &mut stacks);
    stacks.print_top_of_stacks();

    // Part 2
    crane = Crane::CrateMover9001;
    crane.move_crates(commands2, &mut stacks2);
    stacks2.print_top_of_stacks();

    Ok(())
}

