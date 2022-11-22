use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, AddAssign},
};

use crate::minheap::{MinHeap, Vektet};

pub struct Graf {
    noder: Vec<Node>,
}

#[derive(Clone)]
pub struct Node {
    id: usize,
    pub lengdegrad: f64,
    pub breddegrad: f64,
    kanter: Vec<Kant>,
    estimat: Option<Kjøretid>,
}

#[derive(Clone, Copy)]
pub struct Kant {
    fra: usize,
    til: usize,
    kjøretid: Kjøretid,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
pub enum Kjøretid {
    Verdi(usize),
    Uendelig,
}

impl Add for Kjøretid {
    type Output = Kjøretid;

    fn add(self, rhs: Self) -> Self::Output {
        // Hvis begge de to avstandene har tall-verdier
        if let Kjøretid::Verdi(venstre) = self {
            if let Kjøretid::Verdi(høyre) = rhs {
                // Summer dem
                return Kjøretid::Verdi(venstre + høyre);
            }
        }

        // Om en eller to av avstandene er uendelige er resultatet også uendelig
        Kjøretid::Uendelig
    }
}

impl AddAssign for Kjøretid {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs);
    }
}

impl Display for Kjøretid {
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
    pub kjøretid: Kjøretid,
    pub node: usize,
}

impl Vektet for Køelement {
    fn vekt(&self) -> usize {
        if let Kjøretid::Verdi(vekt) = self.kjøretid {
            vekt
        } else {
            usize::MAX
        }
    }

    fn sett_vekt(&mut self, vekt: usize) {
        self.kjøretid = Kjøretid::Verdi(vekt)
    }
}

impl PartialEq for Køelement {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node
    }
}

impl Eq for Køelement {}

impl PartialOrd for Køelement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.kjøretid.partial_cmp(&other.kjøretid)
    }
}

impl Ord for Køelement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.kjøretid.partial_cmp(&other.kjøretid).unwrap()
    }
}

impl Node {
    pub fn opprett(id: usize, lengdegrad: f64, breddegrad: f64) -> Self {
        Self {
            id,
            lengdegrad,
            breddegrad,
            kanter: Vec::new(),
            estimat: None,
        }
    }

    pub fn sett_estimat(&mut self, estimat: Kjøretid) {
        self.estimat = Some(estimat);
    }
}

impl Kant {
    pub fn opprett(fra: usize, til: usize, kjøretid: usize) -> Self {
        Self {
            fra,
            til,
            kjøretid: Kjøretid::Verdi(kjøretid),
        }
    }
}

impl Graf {
    pub fn opprett() -> Self {
        Self { noder: Vec::new() }
    }

    pub fn push_node(&mut self, node: Node) {
        self.noder.push(node);
    }

    pub fn push_kant(&mut self, kant: Kant) {
        self.noder[kant.fra].kanter.push(kant);
    }

    pub fn fra_filer(nodefil: &str, kantfil: &str) -> Result<Self, String> {
        let nodefil_handle = match File::open(nodefil) {
            Ok(handle) => handle,
            Err(_) => return Err("Kunne ikke åpne node-filen".into()),
        };

        let mut leser = BufReader::new(nodefil_handle);
        let mut buffer = String::new();

        let mut lest_antall_noder: usize = 0;

        // Opprett en tom graf
        let mut graf = Graf::opprett();

        // Les første linje fra filen, som skal inneholde antallet noder
        let forventet_antall_noder = match leser.read_line(&mut buffer) {
            Ok(_) => {
                let strbuf: Vec<&str> = buffer.split_whitespace().collect();

                // Les antallet noder og lagre verdien
                match strbuf[0].parse::<usize>() {
                    Ok(num) => num,
                    Err(_) => {
                        return Err("Kunne ikke lese antall noder fra starten av filen".into());
                    }
                }
            }
            Err(_) => return Err("Kunne ikke lese første linje fra filen".into()),
        };

        // Les resten av filen linje for linje, der hver linje representerer en node
        for (linjenummer, linje) in leser.lines().enumerate() {
            if let Ok(linje) = linje {
                let buf: Vec<&str> = linje.split_whitespace().collect();

                // Les ID til noden på linjen
                let id = match buf[0].parse::<usize>() {
                    Ok(id) => id,
                    Err(_) => {
                        return Err(format!(
                            "Kunne ikke lese node-id fra linje {linjenummer}: {linje}"
                        ))
                    }
                };

                // Les nodens breddegrad
                let breddegrad = match buf[1].parse::<f64>() {
                    Ok(breddegrad) => breddegrad,
                    Err(_) => {
                        return Err(format!(
                            "Kunne ikke lese vekt fra linje {linjenummer}: {linje}"
                        ))
                    }
                };

                // Les nodens lengdegrad
                let lengdegrad = match buf[2].parse::<f64>() {
                    Ok(lengdegrad) => lengdegrad,
                    Err(_) => {
                        return Err(format!(
                            "Kunne ikke lese lengdegrad fra linje {linjenummer}: {linje}"
                        ))
                    }
                };

                // Legg til den nye noden i grafen
                graf.push_node(Node::opprett(id, lengdegrad, breddegrad));
                lest_antall_noder += 1;
            }
        }

        if forventet_antall_noder != lest_antall_noder {
            return Err(format!("Filen skulle inneholdt {forventet_antall_noder} noder, men {lest_antall_noder} ble lest"));
        }

        // SAMME GREIA MEN FOR KANTER, ORKET IKKE LAGE EN GENERISK METODE
        let kantfil_handle = match File::open(kantfil) {
            Ok(handle) => handle,
            Err(_) => return Err("Kunne ikke åpne kant-filen".into()),
        };

        leser = BufReader::new(kantfil_handle);
        buffer = String::new();

        let mut lest_antall_kanter: usize = 0;

        // Les første linje fra filen, som skal inneholde antallet kanter
        let forventet_antall_kanter = match leser.read_line(&mut buffer) {
            Ok(_) => {
                let strbuf: Vec<&str> = buffer.split_whitespace().collect();

                // Les antallet noder og lagre verdien
                match strbuf[0].parse::<usize>() {
                    Ok(num) => num,
                    Err(_) => {
                        return Err("Kunne ikke lese antall kanter fra starten av filen".into());
                    }
                }
            }
            Err(_) => return Err("Kunne ikke lese første linje fra filen".into()),
        };

        // Les resten av filen linje for linje, der hver linje representerer en kant
        for (linjenummer, linje) in leser.lines().enumerate() {
            if let Ok(linje) = linje {
                let buf: Vec<&str> = linje.split_whitespace().collect();

                // Les hvilken node denne kanten går fra
                let fra = match buf[0].parse::<usize>() {
                        Ok(fra) => fra,
                        Err(_) => {
                            return Err(format!(
                                "Kunne ikke lese node-id kanten på linje {linjenummer} skal gå fra: {linje}"
                            ))
                        }
                    };

                // Les hvilken node denne kanten går til
                let til = match buf[1].parse::<usize>() {
                        Ok(til) => til,
                        Err(_) => {
                            return Err(format!(
                                "Kunne ikke lese node-id kanten på linje {linjenummer} skal gå fra: {linje}"
                            ))
                        }
                    };

                // Les kantens kjøretid
                let kjøretid = match buf[2].parse::<usize>() {
                    Ok(kjøretid) => kjøretid,
                    Err(_) => {
                        return Err(format!(
                            "Kunne ikke lese vekt fra linje {linjenummer}: {linje}"
                        ))
                    }
                };

                // Legg til den nye noden i grafen
                graf.push_kant(Kant::opprett(fra, til, kjøretid));
                lest_antall_kanter += 1;
            }
        }

        if forventet_antall_kanter != lest_antall_kanter {
            return Err(format!("Filen skulle inneholdt {forventet_antall_kanter} kanter, men {lest_antall_kanter} ble lest"));
        }

        Ok(graf)
    }

    pub fn finn_vei(&mut self, fra: usize, til: usize) -> Result<Vec<Node>, String> {
        let start = &self.noder[fra];
        let start = start.clone();
        let mål = &self.noder[til];
        let mål = mål.clone();

        let mut kjøretider: Vec<Kjøretid> = vec![Kjøretid::Uendelig; self.noder.len()];
        kjøretider[fra] = Kjøretid::Verdi(0);

        let mut forløpere: Vec<Option<usize>> = vec![None; self.noder.len()];

        let mut kø: MinHeap<Køelement> = MinHeap::opprett();

        kø.push(Køelement {
            kjøretid: Kjøretid::Verdi(0),
            node: start.id,
        });

        while !kø.er_tom() {
            let korteste = kø.pop().unwrap();
            let node = &self.noder[korteste.node];
            let node = node.clone();

            // println!("{}, {}", node.breddegrad, node.lengdegrad);

            if node.id == til {
                break;
            }

            for kant in node.kanter.iter() {
                let mut ny_kjøretid = kjøretider[node.id] + kant.kjøretid;

                match self.noder[kant.til].estimat {
                    Some(estimat) => ny_kjøretid += estimat,
                    None => {
                        // Haversine
                        let diameter = (2 * 6371) as f64;
                        let b1b2 = (self.noder[kant.til].breddegrad - mål.breddegrad).to_radians();
                        let cosb1cosb2 = self.noder[kant.til].breddegrad.to_radians().cos()
                            * mål.breddegrad.to_radians().cos();
                        let l1l2 = (self.noder[kant.til].lengdegrad - mål.lengdegrad).to_radians();
                        let estimat = Kjøretid::Verdi(
                            match ((diameter * f64::asin(f64::sqrt((b1b2 / 2.0).sin().powi(2) + cosb1cosb2 * (l1l2 / 2.0).sin().powi(2)))) as usize).checked_mul(0)
                            {
                                Some(tid) => {tid},
                                None => {
                                    return Err(
                                        "Estimert kjøretid fra node til mål fikk ikke plass i en 64-bits integer".into()
                                    )
                                }
                            },
                        );

                        self.noder[kant.til].sett_estimat(estimat);

                        ny_kjøretid += estimat;
                    }
                }

                if ny_kjøretid < kjøretider[kant.til] {
                    kjøretider[kant.til] = ny_kjøretid;
                    forløpere[kant.til] = Some(kant.fra);

                    if let Some(idx) = kø.finn_element(&Køelement {
                        kjøretid: Kjøretid::Uendelig,
                        node: kant.til,
                    }) {
                        if let Kjøretid::Verdi(ny_kjøretid) = ny_kjøretid {
                            kø.endre_vekt(idx, ny_kjøretid);
                        }
                    } else {
                        kø.push(Køelement {
                            kjøretid: ny_kjøretid,
                            node: kant.til,
                        });
                    }
                }
            }
        }

        let mut forrige = Some(til);

        let mut vei: Vec<Node> = Vec::new();

        while forrige.is_some() {
            vei.push(self.noder[forrige.unwrap()].clone());
            forrige = forløpere[forrige.unwrap()];
        }

        vei.reverse();

        Ok(vei)
    }
}
