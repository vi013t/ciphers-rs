use std::borrow::Borrow;

use crate::alphabet::{Alphabet, AlphabetIndex};

pub fn tabula_recta<T: Borrow<Alphabet>>(alphabet: T) -> std::collections::HashMap<char, std::collections::HashMap<char, char>> {
    let mut rows = std::collections::HashMap::new();
    let alphabet = alphabet.borrow();
    for row in 1..=26 {
        let shifted = alphabet.shift(row - 1);
        rows.insert(
            *alphabet.letter_at(AlphabetIndex::new(row).unwrap()),
            alphabet
                .characters()
                .iter()
                .map(|alphabet_char| (*alphabet_char, *shifted.letter_at(alphabet.index_of(*alphabet_char).unwrap())))
                .collect(),
        );
    }
    rows
}

pub trait TabulaRecta {
    fn at(&self, row: &char, column: &char) -> Option<&char>;
}

impl TabulaRecta for std::collections::HashMap<char, std::collections::HashMap<char, char>> {
    fn at(&self, row_letter: &char, column_letter: &char) -> Option<&char> {
        self.get(row_letter).and_then(|column| column.get(column_letter))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        alphabet::{Alphabet, AlphabetIndex},
        tabula_recta::{tabula_recta, TabulaRecta as _},
    };

    /// Run with `-- --nocapture` to avoid Rust suppressing the output.
    #[test]
    fn print_tabula_recta() {
        let alphabet = Alphabet::default();
        let tabula_recta = tabula_recta(&alphabet);

        for row in 1..=26 {
            for column in 1..=26 {
                let row_character = alphabet.letter_at(AlphabetIndex::new(row).unwrap());
                let column_character = alphabet.letter_at(AlphabetIndex::new(column).unwrap());
                print!("{}", tabula_recta.at(row_character, column_character).unwrap());
            }
            println!()
        }
    }
}
