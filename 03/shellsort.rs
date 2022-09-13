use std::{fs::File, io::Read};
use std::time::{Duration, Instant};

trait IsSortedExt<T: Ord> {
    fn is_sorted(&self) -> bool;
}

impl IsSortedExt<u32> for Vec<u32> {
    fn is_sorted(&self) -> bool {
        let mut retval = true;

        for i in 1..self.len() {
            if self[i] < self[i - 1] {
                retval = false;
                break
            }
        }

        retval
    }
}

fn generate_random(antall: usize, max: u32) -> Vec<u32> {
    // Opprett to vektorer, en som buffer for lesing fra /dev/urandom
    // og en for å oppbevare prisendringer
    let mut filebuf = vec![0u8; 4 * antall];
    let mut numbers =  vec![0u32; antall];

    // Gjør klar for å lese fra /dev/urandom
    let mut f = File::open("/dev/urandom")
    .expect("/dev/urandom not available");

    // Les bytes derfra (heltall i intervallet [0, 255])
    f.read_exact(&mut filebuf)
    .expect("/dev/urandom not available");

    // Kombiner disse med bit-manipulasjon til u32
    for i in 0..antall {
        numbers[i] = ((filebuf[4 * i] as u32) << 24 |
            (filebuf[4 * i + 1] as u32) << 16 |
            (filebuf[4 * i + 2] as u32) << 8 |
            (filebuf[4 * i + 3] as u32)) % max;
    }

    numbers
}

fn shellsort<T: Ord + Clone>(liste: &mut [T], sprik_faktor: f32) -> () {
    let mut sprik_liste = vec![liste.len() / 2];

    while *sprik_liste.last().unwrap() >= sprik_faktor.ceil() as usize {
        sprik_liste.push((*sprik_liste.last().unwrap() as f32 / sprik_faktor) as usize);
    }

    // Garanter at tallene for sprik slutter med [1, 0]
    if *sprik_liste.last().unwrap() != 1 {
        sprik_liste.push(1);
    }

    sprik_liste.push(0);

    for sprik in sprik_liste.iter() {
        for i in *sprik..liste.len() {
            let mut j = i;
            let temp = liste[i].clone();

            while j >= *sprik && liste[j - sprik] > temp {
                liste.swap(j, j - sprik);
                j -= *sprik;
            }

            liste[j] = temp;
        }
    }
}

fn main() {
    let mut tilfeldig: Vec<u32>;
    let mut start: Instant;
    let mut slutt: Instant;
    let mut tid;

    const ANTALL_KJØRINGER: u32 = 5;

    let mut sprikverdier = vec![5.550];
    while sprikverdier[sprikverdier.len() - 1] < 5.560 {
        sprikverdier.push(sprikverdier[sprikverdier.len() - 1] + 0.001);
    }

    for i in sprikverdier {
        tid = Duration::from_micros(0);
        for _ in 0..ANTALL_KJØRINGER {
            tilfeldig = generate_random(5_000_000, u32::max_value());
        
            start = Instant::now();
            shellsort(&mut tilfeldig, i);
            slutt = Instant::now();
        
            tid += slutt - start;
        }
        tid = tid / ANTALL_KJØRINGER;
        println!("({i:.3}, {:#?}),", tid.as_millis());
    }
}
