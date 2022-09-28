use std::collections::{HashMap, LinkedList};
use std::env::args;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Read};
use std::process::exit;
use std::time::{Duration, Instant};

fn fil_til_vektor<T: Into<String> + Copy>(filnavn: T) -> Result<Vec<String>, ()> {
    let fil = match fs::File::open(filnavn.into()) {
        Ok(fil) => fil,
        Err(_) => {
            println!("Kunne ikke åpne fil \"{}\"", filnavn.into());
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

fn tilfeldige_heltall(antall: usize, max: usize) -> Vec<usize> {
    // Sjekk om vi er på 64, 32 eller 16 bits plattform,
    // og sett bredden i bytes til returverdien etter dette
    let bredde = if cfg!(target_pointer_width = "64") {
        8
    } else if cfg!(target_pointer_width = "32") {
        4
    } else {
        2
    };

    let mut filbuffer = vec![0_u8; bredde * antall];
    let mut tall = vec![0_usize; antall];

    // Gjør klar for å lese fra /dev/urandom
    let mut f = File::open("/dev/urandom").expect("/dev/urandom ikke tilgjengelig");

    // Les bytes derfra (heltall i intervallet [0, 255])
    f.read_exact(&mut filbuffer)
        .expect("/dev/urandom ikke tilgjengelig");

    // Bruker bit-forskyvning til å generere verdier av korrekt størrelse,
    // basert på antall bits vi har plass til i verdiene som skal returneres
    for i in 0..antall {
        tall[i] = (filbuffer[bredde * i] as usize)
            | (filbuffer[bredde * i + 1] as usize) << 8
            | if bredde > 2 {
                (filbuffer[bredde * i + 2] as usize) << 16
                    | (filbuffer[bredde * i + 3] as usize) << 24
                    | if bredde > 4 {
                        (filbuffer[bredde * i + 4] as usize) << 32
                            | (filbuffer[bredde * i + 5] as usize) << 40
                            | (filbuffer[bredde * i + 6] as usize) << 48
                            | (filbuffer[bredde * i + 7] as usize) << 56
                    } else {
                        0
                    }
            } else {
                0
            } % max;
    }

    tall
}

struct HashTabell {
    tabell: Vec<LinkedList<String>>,
    kapasitet: usize,
    antall_verdier: usize,
    kollisjoner: usize,
}

impl HashTabell {
    pub fn new(kapasitet: usize) -> Self {
        let mut tabell: Vec<LinkedList<String>> = Vec::new();
        tabell.resize(kapasitet, LinkedList::new());

        Self {
            tabell,
            kapasitet,
            antall_verdier: 0,
            kollisjoner: 0,
        }
    }

    fn hash(&self, nøkkel: &String) -> usize {
        let mut hash: usize = 0;

        // Genererer hash ved å iterere gjennom bokstavene i strengen, med ulik vekting for hver bokstav (5^i)
        for bokstav in nøkkel.chars() {
            hash = (5 * hash) + bokstav as usize;
            hash = hash % self.kapasitet;
        }

        hash
    }

    pub fn sett_inn(&mut self, streng: String) -> () {
        // Beregn hash
        let hash = Self::hash(&self, &streng);

        // Hvis listen på den angitte plassen allerede har innhold er det en kollisjon
        if !self.tabell[hash].is_empty() {
            // Tell kollisjonen
            self.kollisjoner = self.kollisjoner + 1;

            // Og print den
            print!("{} kolliderte med", streng);
            for navn in &self.tabell[hash] {
                print!(" {navn},");
            }
            println!();
        }

        // Kollisjonen håndteres av den koblede listen, så innsetting kan gjøres uavhengig av om en kollisjon oppsto
        self.tabell[hash].push_front(streng);
        self.antall_verdier = self.antall_verdier + 1;
    }

    pub fn inneholder(&self, streng: String) -> bool {
        // Sjekker om listen på den angitte plassen inneholder elementet vi ser etter
        self.tabell[self.hash(&streng)].contains(&streng)
    }

    pub fn lasttall(&self) -> f32 {
        self.antall_verdier as f32 / self.kapasitet as f32
    }
}

struct HashTabell2 {
    tabell: Vec<Option<usize>>,
    kapasitet: usize,
    antall_verdier: usize,
    kollisjoner: usize,
}

impl HashTabell2 {
    pub fn new(kapasitet: usize) -> Self {
        let mut tabell: Vec<Option<usize>> = Vec::new();
        tabell.resize(kapasitet, None);

        Self {
            tabell,
            kapasitet,
            antall_verdier: 0,
            kollisjoner: 0,
        }
    }

    // Obs, antar her at kapasiteten til tabellen er et primtall
    fn hash(&self, nøkkel: usize) -> usize {
        nøkkel % self.kapasitet
    }

    fn hash2(&self, nøkkel: usize) -> usize {
        (nøkkel % (self.kapasitet - 1)) + 1
    }

    pub fn sett_inn(&mut self, nøkkel: usize) -> () {
        // Beregn hash
        let mut hash = self.hash(nøkkel);

        // Hvis en kollisjon inntreffer (noen andre er allerede på den ønskede plassen)
        if let Some(_) = self.tabell[hash] {
            // Regn ut sekundær hash
            let hash2 = self.hash2(nøkkel);

            loop {
                self.kollisjoner = self.kollisjoner + 1;
                // Hopp fremover i tabellen med hopplengde hash2
                hash = (hash + hash2) % self.kapasitet;

                // Hvis vi hoppet til en ledig plass, bryt løkken
                if let None = self.tabell[hash] {
                    break;
                }
            }
        }

        // Etter rutinen ovenfor er vi garantert å ha funnet en ledig plass, og setter inn den nye verdien
        self.tabell[hash] = Some(nøkkel);
        self.antall_verdier = self.antall_verdier + 1;
    }
}

fn main() {
    let args: Vec<String> = args().collect();

    let filnavn: String = match args.get(1) {
        Some(arg) => arg.clone(),
        None => "navn".to_string(),
    };

    let navneliste = match fil_til_vektor(&filnavn) {
        Ok(liste) => liste,
        Err(_) => exit(1),
    };

    println!("Oppgave 1:");

    let mut hashtabell: HashTabell = HashTabell::new(127);

    for navn in navneliste {
        hashtabell.sett_inn(navn);
    }

    println!();

    println!(
        "Jeg (Jakob Karevold Grønhaug) er i hashtabellen: {}",
        hashtabell.inneholder("Jakob Karevold Grønhaug".to_string())
    );

    println!(
        "{} kollisjoner på {} innsettinger",
        hashtabell.kollisjoner, hashtabell.antall_verdier
    );
    println!(
        "Kollisjoner per person: {}",
        hashtabell.kollisjoner as f32 / hashtabell.antall_verdier as f32
    );
    println!("Lasttall: {}", hashtabell.lasttall());

    drop(hashtabell);

    println!("\nOppgave 2:");

    let mut hashtabell: HashTabell2 = HashTabell2::new(13_000_027); // Neste primtall etter 13 000 000

    // Ti millioner tall med største mulige spredning for 64 bits
    let tall_liste = tilfeldige_heltall(10_000_000, usize::MAX);

    let mut start: Instant = Instant::now();
    for tall in &tall_liste {
        hashtabell.sett_inn(*tall);
    }

    let mut tid: Duration = Instant::now() - start;

    println!(
        "Satte inn på {} sekunder, med {} kollisjoner",
        tid.as_secs_f32(),
        hashtabell.kollisjoner
    );
    println!(
        "({} kollisjoner per innsetting)",
        hashtabell.kollisjoner as f64 / hashtabell.antall_verdier as f64
    );
    println!(
        "Lasttall: {}",
        hashtabell.antall_verdier as f64 / hashtabell.kapasitet as f64
    );

    let mut innebygd: HashMap<usize, usize> = HashMap::new();

    start = Instant::now();
    for tall in &tall_liste {
        innebygd.insert(*tall, *tall);
    }

    tid = Instant::now() - start;

    println!(
        "Innebygd hashtabell gjorde samme innsettinger på {} sekunder",
        tid.as_secs_f32()
    );
}
