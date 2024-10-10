use enigma_simulator::{EnigmaBuilder as _, EnigmaMachine, EnigmaResult, UncheckedEnigmaBuilder};

#[test]
fn encrypt_and_decrypt() -> EnigmaResult<()> {
    let ciphertext = "KDZVKMNTYQJPHFXI";
    let plaintext = "TOPSECRETMESSAGE";

    let machine = EnigmaMachine::new()
        .rotors(1, 2, 3)
        .reflector("B")
        .ring_settings(10, 12, 14)
        .ring_positions(5, 22, 3)
        .plugboard("BY EW FZ GI QM RV UX")?;

    let unchecked_machine = EnigmaMachine::unchecked()
        .rotors(1, 2, 3)
        .reflector("B")
        .ring_settings(10, 12, 14)
        .ring_positions(5, 22, 3)
        .plugboard("BY EW FZ GI QM RV UX")
        .build();

    assert_eq!(plaintext, machine.decrypt(ciphertext));
    assert_eq!(plaintext, unsafe { unchecked_machine.decrypt_unchecked(ciphertext) });

    assert_eq!(ciphertext, machine.encrypt(plaintext));
    assert_eq!(ciphertext, unsafe { unchecked_machine.encrypt_unchecked(plaintext) });

    Ok(())
}

#[test]
fn debug_information() -> EnigmaResult<()> {
    let ciphertext = "HI";

    let machine = EnigmaMachine::new()
        .rotors(1, 2, 3)
        .reflector("B")
        .ring_settings(10, 12, 14)
        .ring_positions(5, 22, 3)
        .plugboard("BY EW FZ GI QM RV UX")
        .debug()?;

    machine.decrypt(ciphertext);

    Ok(())
}
