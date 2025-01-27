use std::io;

use anyhow::{Context, Result};
use names::Generator;
use rand::prelude::*;

fn main() -> Result<()> {
    let generator = Generator::default();
    let people_size = ask_number_of_participants()?;
    let participants = populate_participants(generator, people_size);
    let rng = rand::rng();
    pair_participants(participants, rng);
    Ok(())
}

fn ask_number_of_participants() -> Result<usize, anyhow::Error> {
    let people_size = loop {
        println!("Enter the number of participants:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .context("Failed to read line")?;

        match input.trim().parse::<usize>() {
            Ok(size) if size % 2 == 0 => break size,
            Ok(_) => println!("Please enter an even number."),
            Err(_) => println!("Please enter a valid number."),
        }
    };
    Ok(people_size)
}

fn pair_participants(mut participants: Vec<String>, mut rng: ThreadRng) {
    while participants.len() > 1 {
        let one = participants.remove(0);
        let random_index = rng.random_range(0..participants.len());

        if participants.len() > 0 {
            let another = participants.remove(random_index);
            println!("Participant {} will swap gifts with {}", one, another)
        }
    }
}

fn populate_participants(mut generator: Generator<'_>, people_size: usize) -> Vec<String> {
    let mut participants = Vec::with_capacity(people_size);
    for _ in 0..people_size {
        if let Some(name) = generator.next() {
            participants.push(name);
        }
    }
    participants
}
