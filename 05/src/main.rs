use std::collections::LinkedList;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::exit;

fn fil_til_vektor(filnavn: &str) -> Result<Vec<String>, ()> {
    let fil = match fs::File::open(filnavn) {
        Ok(fil) => fil,
        Err(_) => {
            println!("Kunne ikke åpne fil \"{filnavn}\"");
            return Err(());
        }
    };

    let leser = BufReader::new(fil);

    let mut vektor: Vec<String> = Vec::new();

    for (linjetall, linje) in leser.lines().enumerate() {
        match linje {
            Ok(linje) => vektor.push(linje),
            Err(_) => {
                println!("Feil ved lesing av linje {linjetall} i navnefil")
            }
        }
    }

    Ok(vektor)
}

struct HashTabell {
    tabell: Vec<LinkedList<String>>,
    kapasitet: usize,
    antall_verdier: usize,
    kollisjoner: usize
}

impl HashTabell {
    pub fn new(størrelse: usize) -> Self {
        let mut tabell: Vec<LinkedList<String>> = Vec::new();

        tabell.resize(størrelse, LinkedList::new());

        Self {
            tabell,
            kapasitet: størrelse,
            antall_verdier: 0,
            kollisjoner: 0
        }
    }

    fn hash(&self, nøkkel: &String) -> usize {
        let mut hash: usize = 0;

        for (i, bokstav) in nøkkel.chars().enumerate() {
            hash = hash + (i + 1) * 7 * bokstav as usize;
        }

        hash % self.kapasitet
    }

    pub fn sett_inn(&mut self, streng: String) -> () {
        let hash = Self::hash(&self, &streng);

        if self.tabell[hash].len() > 0 {
            self.kollisjoner = self.kollisjoner + 1;
            print!("{} kolliderte med", streng);
            for navn in &self.tabell[hash] {
                print!(" {navn},");
            }
            println!();
        }

        self.tabell[hash].push_front(streng);
        self.antall_verdier = self.antall_verdier + 1;
    }

    pub fn inneholder(&self, streng: String) -> bool {
        self.tabell[self.hash(&streng)].contains(&streng)
    }

    pub fn lasttall(&self) -> f32 {
        self.antall_verdier as f32 / self.kapasitet as f32
    }
}

fn main() {
    let navneliste = match fil_til_vektor("navn") {
        Ok(liste) => liste,
        Err(_) => exit(1),
    };

    let mut hashtabell: HashTabell = HashTabell::new(256);

    for navn in navneliste {
        hashtabell.sett_inn(navn);
    }

    println!("{}", hashtabell.inneholder("Jakob Karevold Grønhaug".to_string()));
    println!("{} kollisjoner på {} innsettinger", hashtabell.kollisjoner, hashtabell.antall_verdier);
    println!("Kollisjoner per person: {}", hashtabell.kollisjoner as f32 / hashtabell.antall_verdier as f32);
}
