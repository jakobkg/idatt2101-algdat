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

pub struct Node {
    id: usize,
    lengdegrad: f64,
    breddegrad: f64,
    kanter: Vec<Kant>,
}

#[derive(Clone, Copy)]
pub struct Kant {
    fra: usize,
    til: usize,
    vekt: Avstand,
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
    pub avstand: Avstand,
    pub node: usize,
}

impl Vektet for Køelement {
    fn vekt(&self) -> usize {
        if let Avstand::Verdi(vekt) = self.avstand {
            vekt
        } else {
            usize::MAX
        }
    }

    fn sett_vekt(&mut self, vekt: usize) {
        self.avstand = Avstand::Verdi(vekt)
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
        self.avstand.partial_cmp(&other.avstand)
    }
}

impl Ord for Køelement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.avstand.partial_cmp(&other.avstand).unwrap()
    }
}

impl Node {
    pub fn opprett(id: usize, lengdegrad: f64, breddegrad: f64) -> Self {
        Self {
            id,
            lengdegrad,
            breddegrad,
            kanter: Vec::new(),
        }
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
            Err(_) => return Err(format!("Kunne ikke åpne node-filen")),
        };

        let mut leser = BufReader::new(nodefil_handle);
        let mut buffer = String::new();

        let forventet_antall_noder;
        let mut lest_antall_noder: usize = 0;

        // Opprett en tom graf
        let mut graf = Graf::opprett();

        // Les første linje fra filen, som skal inneholde antallet noder
        match leser.read_line(&mut buffer) {
            Ok(_) => {
                let strbuf: Vec<&str> = buffer.split_whitespace().collect();

                // Les antallet noder og lagre verdien
                forventet_antall_noder = match strbuf[0].parse::<usize>() {
                    Ok(num) => num,
                    Err(_) => {
                        return Err(format!("Kunne ikke lese antall noder fra starten av filen"));
                    }
                };
            }
            Err(_) => return Err(format!("Kunne ikke lese første linje fra filen")),
        };

        // Les resten av filen linje for linje, der hver linje representerer en node
        for (linjenummer, linje) in leser.lines().enumerate() {
            match linje {
                Ok(linje) => {
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

                    // Les nodens lengdegrad
                    let lengdegrad = match buf[1].parse::<f64>() {
                        Ok(lengdegrad) => lengdegrad,
                        Err(_) => {
                            return Err(format!(
                                "Kunne ikke lese lengdegrad fra linje {linjenummer}: {linje}"
                            ))
                        }
                    };

                    // Les nodens breddegrad
                    let breddegrad = match buf[2].parse::<f64>() {
                        Ok(breddegrad) => breddegrad,
                        Err(_) => {
                            return Err(format!(
                                "Kunne ikke lese vekt fra linje {linjenummer}: {linje}"
                            ))
                        }
                    };

                    // Legg til den nye noden i grafen
                    graf.push_node(Node::opprett(id, lengdegrad, breddegrad));
                    lest_antall_noder += 1;
                }
                Err(_) => {}
            }
        }

        if forventet_antall_noder != lest_antall_noder {
            return Err(format!("Filen skulle inneholdt {forventet_antall_noder} noder, men {lest_antall_noder} ble lest"));
        }

        // SAMME GREIA MEN FOR KANTER, ORKET IKKE LAGE EN GENERISK METODE
        let kantfil_handle = match File::open(kantfil) {
            Ok(handle) => handle,
            Err(_) => return Err(format!("Kunne ikke åpne kant-filen")),
        };

        leser = BufReader::new(kantfil_handle);
        buffer = String::new();

        let forventet_antall_kanter;
        let mut lest_antall_kanter: usize = 0;

        // Les første linje fra filen, som skal inneholde antallet kanter
        match leser.read_line(&mut buffer) {
            Ok(_) => {
                let strbuf: Vec<&str> = buffer.split_whitespace().collect();

                // Les antallet noder og lagre verdien
                forventet_antall_kanter = match strbuf[0].parse::<usize>() {
                    Ok(num) => num,
                    Err(_) => {
                        return Err(format!(
                            "Kunne ikke lese antall kanter fra starten av filen"
                        ));
                    }
                };
            }
            Err(_) => return Err(format!("Kunne ikke lese første linje fra filen")),
        };

        // Les resten av filen linje for linje, der hver linje representerer en kant
        for (linjenummer, linje) in leser.lines().enumerate() {
            match linje {
                Ok(linje) => {
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

                    // Les kantens vekt
                    let vekt = match buf[2].parse::<usize>() {
                        Ok(vekt) => vekt,
                        Err(_) => {
                            return Err(format!(
                                "Kunne ikke lese vekt fra linje {linjenummer}: {linje}"
                            ))
                        }
                    };

                    // Legg til den nye noden i grafen
                    graf.push_kant(Kant::opprett(fra, til, vekt));
                    lest_antall_kanter += 1;
                }
                Err(_) => {}
            }
        }

        if forventet_antall_kanter != lest_antall_kanter {
            return Err(format!("Filen skulle inneholdt {forventet_antall_kanter} kanter, men {lest_antall_kanter} ble lest"));
        }

        Ok(graf)
    }

    pub fn dijkstra_veivalg(&self, fra: usize, til: usize) -> Result<Vec<Node>, String> {
        let start: &Node = match self.noder.get(fra) {
            Some(node) => node,
            None => return Err(format!("Node {fra} finnes ikke i grafen")),
        };

        let mut node: &Node;
        let mut korteste: Køelement;

        let mut avstander: Vec<Avstand> = vec![Avstand::Uendelig; self.noder.len()];
        avstander[fra] = Avstand::Verdi(0);

        let mut forløpere: Vec<Option<usize>> = vec![None; self.noder.len()];

        let mut kø: MinHeap<Køelement> = MinHeap::opprett();
        let mut besøkt: Vec<usize> = Vec::new();
        besøkt.push(fra);

        for nodeid in 0..self.noder.len() {
            if nodeid != fra {
                kø.push(Køelement { avstand: Avstand::Uendelig, node: nodeid });
            } else {
                kø.push(Køelement { avstand: Avstand::Verdi(0), node: fra });
            }
        }

        while !kø.er_tom() {
            korteste = kø.pop().unwrap();
            node = &self.noder[korteste.node];

            println!("{}, {}", node.lengdegrad, node.breddegrad);
            
            if node.id == til {
                break
            }

            for kant in node.kanter.iter() {
                let ny_avstand = kant.vekt + avstander[node.id];

                if ny_avstand < avstander[kant.til] {
                    avstander[kant.til] = ny_avstand;
                    forløpere[kant.til] = Some(kant.fra);

                    if let Some(idx) = kø.finn_element(&Køelement { avstand: Avstand::Uendelig, node: kant.til }) {
                        if let Avstand::Verdi(vekt) = ny_avstand {
                            kø.endre_vekt(idx, vekt);
                        }
                    }
                }
            }
        }

        Err(format!("lol"))
    }
}
