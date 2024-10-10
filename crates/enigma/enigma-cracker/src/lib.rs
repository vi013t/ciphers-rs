use std::io::Write;

use enigma_simulator::{EnigmaBuilder as _, EnigmaMachine, EnigmaResult};

pub fn decrypt_enigma(ciphertext: &str) -> EnigmaResult<()> {
    let plugboard = "BY EW FZ GI MQ RV UX";
    let reflector = "B";

    let (rotors, offsets) = best_rotors(plugboard, reflector, ciphertext)?;
    println!("Best rotors: {}, {}, {}", rotors.0, rotors.1, rotors.2);
    println!("Best offsets: {}, {}, {}", offsets.0, offsets.1, offsets.2);

    let ring_settings = best_ring_settings(reflector, plugboard, rotors, offsets, ciphertext)?;
    println!("Best ring settings: {}, {}, {}", ring_settings.0, ring_settings.1, ring_settings.2);

    let plaintext = &EnigmaMachine::new()
        .reflector(reflector)
        .plugboard(plugboard)
        .rotors(rotors.0, rotors.1, rotors.2)
        .ring_positions(offsets.0, offsets.1, offsets.2)
        .ring_settings(ring_settings.0, ring_settings.1, ring_settings.2)?
        .decrypt(ciphertext);

    println!("Plaintext: {plaintext}");

    Ok(())
}

#[allow(clippy::type_complexity)]
fn best_rotors(plugboard: &str, reflector: &str, ciphertext: &str) -> EnigmaResult<((u8, u8, u8), (u8, u8, u8))> {
    let mut plaintexts = Vec::new();
    let total = 8 * 8 * 8 * 26 * 26 * 26;
    let mut iteration = 0;

    println!("\n");

    for rotor_1 in 1..=8 {
        for rotor_2 in 1..=8 {
            for rotor_3 in 1..=8 {
                for offset_1 in 1..=26 {
                    for offset_2 in 1..=26 {
                        for offset_3 in 1..=26 {
                            let machine = EnigmaMachine::new()
                                .plugboard(plugboard)
                                .reflector(reflector)
                                .rotors(rotor_1, rotor_2, rotor_3)
                                .ring_positions(offset_1, offset_2, offset_3)
                                .ring_settings(1, 1, 1)?;
                            let plaintext = machine.decrypt(ciphertext);
                            let distance = (index_of_coincidence(&plaintext) - 0.0667).abs();
                            plaintexts.push((distance, ((rotor_1, rotor_2, rotor_3), (offset_1, offset_2, offset_3))));

                            iteration += 1;
                            let progress = 100f64 * (iteration as f64 / total as f64);
                            print!("\x1B[A");
                            println!("Finding best rotor settings... ({:.2}%)", progress);
                            std::io::stdout().flush().unwrap();
                        }
                    }
                }
            }
        }
    }

    Ok(plaintexts.iter().min_by(|first, second| first.0.total_cmp(&second.0)).unwrap().1)
}

fn best_ring_settings(reflector: &str, plugboard: &str, rotors: (u8, u8, u8), ring_positions: (u8, u8, u8), ciphertext: &str) -> EnigmaResult<(u8, u8, u8)> {
    let mut plaintexts = Vec::new();
    let mut iteration = 1;
    let total = 26 * 26 * 26;

    println!();

    for offset_1 in 1..=26 {
        for offset_2 in 1..=26 {
            for offset_3 in 1..=26 {
                let machine = EnigmaMachine::new()
                    .plugboard(plugboard)
                    .reflector(reflector)
                    .rotors(rotors.0, rotors.1, rotors.2)
                    .ring_positions(ring_positions.0, ring_positions.1, ring_positions.2)
                    .ring_settings(ring_positions.0, ring_positions.1, ring_positions.2)?;
                let plaintext = machine.decrypt(ciphertext);
                let distance = (index_of_coincidence(&plaintext) - 0.0667).abs();
                plaintexts.push((distance, (offset_1, offset_2, offset_3)));

                iteration += 1;
                let progress = 100f64 * (iteration as f64 / total as f64);
                print!("\x1B[A");
                println!("Finding best ring settings... ({:.2}%)", progress);
                std::io::stdout().flush().unwrap();
            }
        }
    }

    Ok(plaintexts.iter().min_by(|first, second| first.0.total_cmp(&second.0)).unwrap().1)
}

fn index_of_coincidence(text: &str) -> f64 {
    let mut frequency = [0u32; 26];
    let mut total_letters = 0;

    for c in text.chars() {
        if c.is_alphabetic() {
            let idx = c.to_ascii_lowercase() as usize - 'a' as usize;
            frequency[idx] += 1;
            total_letters += 1;
        }
    }

    if total_letters < 2 {
        return 0.0;
    }

    let mut numerator = 0u32;

    for &count in &frequency {
        numerator += count * (count - 1);
    }

    let denominator = total_letters * (total_letters - 1);

    numerator as f64 / denominator as f64
}

// fn best_plugboard(plugboard: &str, reflector: &str, ciphertext: &str) -> EnigmaResult<String> {
//     let mut plugboard = std::collections::HashMap::new();
// }

#[cfg(test)]
mod tests {
    use enigma_simulator::{EnigmaBuilder, EnigmaMachine, EnigmaResult};

    use crate::{best_ring_settings, best_rotors};

    #[test]
    #[ignore]
    fn rotors() -> EnigmaResult<()> {
        let plugboard = "BY EW FZ GI MQ RV UX";
        let reflector = "B";
        let ciphertext = include_str!("../tests/encrypted_letter.txt");

        let (rotors, offsets) = best_rotors(plugboard, reflector, ciphertext)?;

        assert_eq!(rotors, (5, 8, 3));
        assert_eq!(offsets, (5, 22, 3));

        Ok(())
    }

    #[test]
    #[ignore]
    fn ring_settings() -> EnigmaResult<()> {
        let plugboard = "BY EW FZ GI MQ RV UX";
        let reflector = "B";
        let ciphertext = include_str!("../tests/encrypted_letter.txt");

        let (rotors, offsets) = best_rotors(plugboard, reflector, ciphertext)?;
        println!("Best rotors: {}, {}, {}", rotors.0, rotors.1, rotors.2);
        println!("Best offsets: {}, {}, {}", offsets.0, offsets.1, offsets.2);

        let ring_settings = best_ring_settings(reflector, plugboard, rotors, offsets, ciphertext)?;
        println!("Best ring settings: {}, {}, {}", ring_settings.0, ring_settings.1, ring_settings.2);

        let plaintext = &EnigmaMachine::new()
            .reflector(reflector)
            .plugboard(plugboard)
            .rotors(rotors.0, rotors.1, rotors.2)
            .ring_positions(offsets.0, offsets.1, offsets.2)
            .ring_settings(ring_settings.0, ring_settings.1, ring_settings.2)?
            .decrypt(ciphertext);

        println!("Plaintext: {plaintext}");

        Ok(())
    }
}
