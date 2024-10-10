use std::io::Write;

use enigma_simulator::{EnigmaBuilder, EnigmaMachine, UncheckedEnigmaBuilder};
use rand::seq::SliceRandom;
use rand::Rng;

fn random_in<R: rand::distributions::uniform::SampleRange<u8>>(range: R) -> u8 {
    rand::thread_rng().gen_range(range)
}

fn random_char() -> char {
    random_in(65..=90) as char
}

fn random_string(length: u8) -> String {
    (0..length).map(|_| random_char()).collect()
}

fn random_plugboard(pairs: u8) -> String {
    let mut numbers: Vec<u8> = (65..=90).collect();
    numbers.shuffle(&mut rand::thread_rng());
    numbers
        .iter()
        .take(pairs as usize * 2)
        .map(|code| (*code as char).to_string())
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|pair| pair.join(""))
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
fn random_enigmas_benchmark() -> anyhow::Result<()> {
    let mut unchecked_times = Vec::new();
    let mut checked_times = Vec::new();

    println!();

    loop {
        // We use different settings for the two machines to make sure that rust caching
        // optimization doesn't make the second run through faster.

        // Safe API
        let plugboard = random_plugboard(10);
        let (rotor_a, rotor_b, rotor_c) = (random_in(1..=8), random_in(1..=8), random_in(1..=8));
        let (ring_a, ring_b, ring_c) = (random_in(1..=26), random_in(1..=26), random_in(1..=26));
        let (position_a, position_b, position_c) = (random_in(1..=26), random_in(1..=26), random_in(1..=26));
        let cipher = random_string(100);
        let start = std::time::Instant::now();
        let machine = EnigmaMachine::new()
            .reflector("B")
            .plugboard(&plugboard)
            .rotors(rotor_a, rotor_b, rotor_c)
            .ring_settings(ring_a, ring_b, ring_c)
            .ring_positions(position_a, position_b, position_c)?;
        machine.decrypt(&cipher);
        let elapsed = start.elapsed().as_nanos();
        checked_times.push(elapsed);

        // Unchecked
        let plugboard = random_plugboard(10);
        let (rotor_a, rotor_b, rotor_c) = (random_in(1..=8), random_in(1..=8), random_in(1..=8));
        let (ring_a, ring_b, ring_c) = (random_in(1..=26), random_in(1..=26), random_in(1..=26));
        let (position_a, position_b, position_c) = (random_in(1..=26), random_in(1..=26), random_in(1..=26));
        let cipher = random_string(100);
        let start = std::time::Instant::now();
        let machine = EnigmaMachine::unchecked()
            .reflector("B")
            .plugboard(&plugboard)
            .rotors(rotor_a, rotor_b, rotor_c)
            .ring_settings(ring_a, ring_b, ring_c)
            .ring_positions(position_a, position_b, position_c)
            .build();
        unsafe { machine.decrypt_unchecked(&cipher) };
        let elapsed = start.elapsed().as_nanos();
        unchecked_times.push(elapsed);

        // Print results
        std::io::stdout().flush().unwrap();
        let average = (unchecked_times.len() as f64 / (unchecked_times.iter().sum::<u128>() as f64 / 1e9)) as u32;
        println!("Current unsafe API rate: {average} decodes per second");
        let average = (checked_times.len() as f64 / (checked_times.iter().sum::<u128>() as f64 / 1e9)) as u32;
        println!("Current safe API rate:   {average} decodes per second");
        print!("\x1B[2A\x1B[1G");
    }
}
