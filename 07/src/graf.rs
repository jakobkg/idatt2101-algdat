use std::{
    cmp::Ordering,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, AddAssign},
};

pub struct Graf {
    noder: Vec<Node>,
    kanter: Vec<Kant>,
}

pub struct Node {
    id: usize,
}

#[derive(Clone, Copy)]
pub struct Kant {
    fra: usize,
    til: usize,
    vekt: Avstand,
}

pub struct MinHeap {
    noder: Vec<Køelement>,
}

impl MinHeap {
    pub fn opprett() -> Self {
        Self { noder: Vec::new() }
    }

    pub fn er_tom(&self) -> bool {
        self.noder.is_empty()
    }

    fn til_venstre_for(indeks: usize) -> usize {
        (indeks << 1) + 1
    }

    fn sorter(&mut self, fra: usize) {
        let mut m = MinHeap::til_venstre_for(fra);

        if m < self.noder.len() {
            let h = m + 1;

            if h < self.noder.len() && self.noder[h] > self.noder[m] {
                m = h;
            }

            if self.noder[m] > self.noder[fra] {
                self.noder.swap(fra, m);
                self.sorter(m);
            }
        }
    }

    fn pop(&mut self) -> Option<Køelement> {
        if self.er_tom() {
            return None;
        } else if self.noder.len() == 1 {
            return Some(self.noder.remove(0));
        } else {
            let topp = self.noder[0];

            self.noder[0] = self.noder.remove(self.noder.len() - 1);

            self.sorter(0);

            Some(topp)
        }
    }

    fn sett_inn(&mut self, ny: Køelement) {
        if self.er_tom() {
            self.noder.push(ny);
            return;
        }

        let mut inneholder = false;

        self.noder = self
            .noder
            .iter()
            .map(|e| {
                if e.node == ny.node {
                    inneholder = true;
                    ny
                } else {
                    *e
                }
            })
            .collect();

        if !inneholder {
            self.noder.push(ny);
        }

        self.sorter(0);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Avstand {
    Verdi(usize),
    Uendelig,
}

impl Add for Avstand {
    type Output = Avstand;

    fn add(self, rhs: Self) -> Self::Output {
        // Hvis begge de to avstandene har tall-verdier
        if let Avstand::Verdi(venstre) = self {
            if let Avstand::Verdi(høyre) = rhs {
                // Summer dem
                return Avstand::Verdi(venstre + høyre);
            }
        }

        // Om en eller to av avstandene er uendelige er resultatet også uendelig
        return Avstand::Uendelig;
    }
}

impl AddAssign for Avstand {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs);
    }
}

impl Display for Avstand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Self::Verdi(verdi) = self {
            write!(f, "{}", verdi)
        } else {
            write!(f, "∞")
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Køelement {
    pub node: usize,
    pub avstand: Avstand,
}

impl PartialEq for Køelement {
    fn eq(&self, other: &Self) -> bool {
        self.avstand == other.avstand
    }
}

impl Eq for Køelement {}

impl PartialOrd for Køelement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Køelement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.avstand == other.avstand {
            Ordering::Equal
        } else if self.avstand > other.avstand {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl Node {
    pub fn opprett(id: usize) -> Self {
        Self { id }
    }
}

impl Kant {
    pub fn opprett(fra: usize, til: usize, vekt: usize) -> Self {
        Self {
            fra,
            til,
            vekt: Avstand::Verdi(vekt),
        }
    }
}

impl Graf {
    pub fn opprett(kapasitet: usize) -> Self {
        let mut noder = Vec::new();
        let kanter = Vec::new();

        for id in 0..kapasitet {
            noder.push(Node::opprett(id));
        }

        Self { noder, kanter }
    }

    pub fn ny_kant(&mut self, fra: usize, til: usize, vekt: usize) -> Result<(), String> {
        // Sjekker ikke om verdiene er negative siden typen allerede er unsigned
        if fra >= self.noder.len() || til >= self.noder.len() {
            Err(format!(
                "Ugyldig kant fra {fra} til {til}, men grafen har kun noder 0 til {}",
                self.noder.len() - 1
            ))
        } else {
            self.kanter.push(Kant::opprett(fra, til, vekt));
            Ok(())
        }
    }

    /// Konstruerer en graf med data fra en fil som følger formatet angitt i oppgaveteksten
    /// Returnerer Ok(Graf) om grafen ble konstruert, eller Err(String) med en feilmelding om
    /// noe gikk galt under konstruering av grafen (f.eks. pga uventede verdier eller feil format i filen)
    pub fn fra_fil(filnavn: &str) -> Result<Self, String> {
        let fil = match File::open(filnavn) {
            Ok(handle) => handle,
            Err(_) => return Err(format!("Kunne ikke åpne filen {filnavn}")),
        };

        let mut leser = BufReader::new(fil);
        let mut stringbuf: String = "".into();

        let forventet_antall_kanter: usize;
        let mut lest_antall_kanter: usize = 0;

        // Les første linje fra filen, som skal inneholde antallet noder og kanter
        let mut graf: Graf = match leser.read_line(&mut stringbuf) {
            Ok(_) => {
                let strbuf: Vec<&str> = stringbuf.split_whitespace().collect();

                // Les antallet noder og lagre verdien
                let kapasitet = match strbuf[0].parse::<usize>() {
                    Ok(num) => num,
                    Err(_) => {
                        return Err(format!(
                            "Kunne ikke lese antall noder fra starten av filen {filnavn}"
                        ));
                    }
                };

                // Les antallet kanter og lagre verdien
                forventet_antall_kanter = match strbuf[1].parse::<usize>() {
                    Ok(num) => num,
                    Err(_) => {
                        return Err(format!(
                            "Kunne ikke lese antall kanter fra starten av filen {filnavn}"
                        ));
                    }
                };

                // Opprett en graf med det angitte antall noder
                Graf::opprett(kapasitet)
            }
            Err(_) => return Err(format!("Kunne ikke lese første linje fra filen {filnavn}")),
        };

        // Les resten av filen linje for linje, der hver linje representerer en kant
        for (linjenummer, linje) in leser.lines().enumerate() {
            match linje {
                Ok(linje) => {
                    let buf: Vec<&str> = linje.split_whitespace().collect();

                    // Les noden kanten skal gå fra
                    let fra = match buf[0].parse::<usize>() {
                        Ok(fra) => fra,
                        Err(_) => {
                            return Err(format!(
                                "Kunne ikke lese node-tall fra linje {linjenummer}: {linje}"
                            ))
                        }
                    };

                    // Les noden kanten skal gå til
                    let til = match buf[1].parse::<usize>() {
                        Ok(til) => til,
                        Err(_) => {
                            return Err(format!(
                                "Kunne ikke lese node-tall fra linje {linjenummer}: {linje}"
                            ))
                        }
                    };

                    // Les vekten til kanten
                    let vekt = match buf[2].parse::<usize>() {
                        Ok(vekt) => vekt,
                        Err(_) => {
                            return Err(format!(
                                "Kunne ikke lese vekt fra linje {linjenummer}: {linje}"
                            ))
                        }
                    };

                    // Forsøk å legge til den nye kanten i grafen
                    match graf.ny_kant(fra, til, vekt) {
                        Ok(_) => {
                            lest_antall_kanter += 1;
                        }
                        Err(feil) => return Err(feil),
                    }
                }
                Err(_) => {}
            }
        }

        // Om et annet antall kanter enn forventet ble lest fra filen er den ugyldig
        if lest_antall_kanter != forventet_antall_kanter {
            return Err(format!("Forventet {forventet_antall_kanter} kanter, men filen inneholdt {lest_antall_kanter}"));
        }

        Ok(graf)
    }

    pub fn dijkstra(&self, start: usize) {
        let mut avstander: Vec<Avstand> = vec![Avstand::Uendelig; self.noder.len()];
        avstander[start] = Avstand::Verdi(0);

        let mut forløpere: Vec<Option<usize>> = vec![None; self.noder.len()];

        let mut kø: MinHeap = MinHeap::opprett();

        for node_id in 0..self.noder.len() {
            kø.sett_inn(Køelement {
                node: node_id,
                avstand: avstander[node_id],
            })
        }

        while !kø.er_tom() {
            let element = kø.pop().unwrap(); // Trygg unwrap siden vi allerede har sjekket om køen er tom

            let avstand_hit = avstander[element.node];

            let kanter: Vec<Kant> = self
                .kanter
                .iter()
                .map(|kant| *kant)
                .filter(|kant| kant.fra == element.node)
                .collect();

            let naboer: Vec<usize> = kanter.iter().map(|kant| kant.til).collect();

            for (index, &nabo) in naboer.iter().enumerate() {
                let ny_avstand = avstand_hit + kanter[index].vekt;

                if ny_avstand < avstander[nabo] {
                    avstander[nabo] = ny_avstand;
                    forløpere[nabo] = Some(element.node);

                    kø.sett_inn(Køelement {
                        node: nabo,
                        avstand: ny_avstand,
                    });
                }
            }
        }

        println!("|Node |Forgjenger|Avstand|");
        println!("|=====+==========+=======|");

        for node in &self.noder {
            println!(
                "|{:<5}|{:<10}|{:<7}|",
                node.id,
                match forløpere[node.id] {
                    Some(node) => {
                        node.to_string()
                    }
                    None => {
                        if node.id == start {
                            "start".to_string()
                        } else {
                            "nåes ikke".to_string()
                        }
                    }
                },
                avstander[node.id].to_string()
            );
        }
    }
}
