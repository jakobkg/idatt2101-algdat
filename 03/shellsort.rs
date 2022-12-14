use std::collections::VecDeque;
use std::{fs::File, io::Read};
use std::time::{Duration, Instant};
use std::thread::{self, JoinHandle};

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

fn tilfeldige_heltall(antall: usize, max: u32) -> Vec<u32> {
    // Opprett to vektorer, en som buffer for lesing fra /dev/urandom
    // og en for å oppbevare prisendringer
    let mut filbuffer = vec![0u8; 4 * antall];
    let mut tall =  vec![0u32; antall];

    // Gjør klar for å lese fra /dev/urandom
    let mut f = File::open("/dev/urandom")
    .expect("/dev/urandom ikke tilgjengelig");

    // Les bytes derfra (heltall i intervallet [0, 255])
    f.read_exact(&mut filbuffer)
    .expect("/dev/urandom ikke tilgjengelig");

    // Kombiner disse med bit-manipulasjon til u32
    for i in 0..antall {
        tall[i] = ((filbuffer[4 * i] as u32) << 24 |
            (filbuffer[4 * i + 1] as u32) << 16 |
            (filbuffer[4 * i + 2] as u32) << 8 |
            (filbuffer[4 * i + 3] as u32)) % max;
    }

    tall
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

fn tidtaking(n: usize, delingstall: f32, samples: u32) -> Duration {
    let mut thread: thread::JoinHandle<Duration>;
    let mut threads = VecDeque::<JoinHandle<Duration>>::new();
    let mut totaltid = Duration::from_secs(0);

    for _ in 0..samples {
        threads.push_back(thread::spawn(move || {
            let start: Instant;
            let slutt: Instant;

            let mut data = tilfeldige_heltall(n, u32::max_value());

            start = Instant::now();
            shellsort(&mut data, delingstall);
            slutt = Instant::now();

            slutt - start
        }))
    }

    while !threads.is_empty() {
        thread = threads.pop_front().unwrap();

        match thread.join() {
            Ok(tid) => {totaltid = totaltid + tid},
            Err(_) => {},
        }
    }

    totaltid / samples
}

fn main() {   
    let delingstall = 5.556;
    let n_list = [1_000_000, 2_000_000, 4_000_000, 8_000_000, 16_000_000, 32_000_000];
    let mut tid: Duration;

    println!("Delingstall: {delingstall}");

    for n in n_list {
        tid = tidtaking(n, delingstall, 5);// OBS: Antallet samples må være mindre
                                                    // enn tilgjengelig antall CPU-kjerner.
                                                    // Om det er for mange vil CPU scheduling
                                                    // påvirke tidtakingen, da alle samples 
                                                    // samles i parallelle tråder.
        println!("Tid med n = {n}: {} ms", tid.as_millis());
    }

}
