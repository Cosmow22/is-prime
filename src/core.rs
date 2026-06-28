use std::fs::File;
use std::io::{BufReader, Read};


pub fn is_prime(number: u32) -> bool {
    if  number == 0 || number == 1 { 
        return false
    }

    let file = File::open("../primes.bin")
        .expect("Le fichier n'a pas été trouvé");
    let mut reader = BufReader::new(file);
    let mut buffer = [0u8; 4];
    while reader.read_exact(&mut buffer).is_ok() {
        let prime = u32::from_le_bytes(buffer);
        if prime == number { 
            return true 
        }
        if number % prime == 0 {
            return false
        }
    }
    return true
}