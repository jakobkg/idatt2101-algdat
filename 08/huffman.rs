use std::collections::BinaryHeap;

use self::typer::{ByteFrekvens, Hopp, Node};

mod typer {
    use std::cmp::Ordering;

    #[derive(Debug, Clone, Copy)]
    pub struct ByteFrekvens {
        pub byte: u8,
        pub frekvens: u32,
    }

    impl ByteFrekvens {
        pub fn ny(byte: u8) -> Self {
            Self { byte, frekvens: 0 }
        }

        pub fn øk(&mut self) -> () {
            self.frekvens += 1;
        }

        pub fn til_bytes(&self) -> [u8; 5] {
            let mut resultat: [u8; 5] = [0, 0, 0, 0, 0];
            let frekvensbytes = self.frekvens.to_be_bytes();

            for i in 0..frekvensbytes.len() {
                resultat[i] = frekvensbytes[i];
            }

            resultat[4] = self.byte;

            resultat
        }

        pub fn fra_bytes(bytes: [u8; 5]) -> Self {
            let mut frekvensbytes: [u8; 4] = [0, 0, 0, 0];

            for i in 0..4 {
                frekvensbytes[i] = bytes[i];
            }

            Self {
                byte: bytes[4],
                frekvens: u32::from_be_bytes(frekvensbytes),
            }
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub enum Hopp {
        Venstre,
        Høyre,
    }

    #[derive(Debug, Clone)]
    pub struct Node {
        pub frekvens: u32,
        pub verdi: Option<u8>,
        pub venstre: Option<Box<Node>>,
        pub høyre: Option<Box<Node>>,
    }

    impl Node {
        pub fn ny(frekvens: u32, verdi: Option<u8>, venstre: Box<Node>, høyre: Box<Node>) -> Self {
            Self {
                frekvens,
                verdi: verdi,
                venstre: Some(venstre),
                høyre: Some(høyre),
            }
        }

        pub fn fra_frekvens(bytefrekvens: &ByteFrekvens) -> Self {
            Self {
                frekvens: bytefrekvens.frekvens,
                verdi: Some(bytefrekvens.byte),
                venstre: None,
                høyre: None,
            }
        }

        pub fn finn_node(&self, verdi: u8, sti: &mut Vec<Hopp>) -> bool {
            let mut res: bool = false;

            if Some(verdi) == self.verdi {
                res = true;
            }

            match &self.venstre {
                Some(venstre) => {
                    sti.push(Hopp::Venstre);

                    if venstre.finn_node(verdi, sti) {
                        res = true;
                    } else {
                        sti.pop();
                    }
                }
                None => {}
            };

            match &self.høyre {
                Some(høyre) => {
                    sti.push(Hopp::Høyre);

                    if høyre.finn_node(verdi, sti) {
                        res = true;
                    } else {
                        sti.pop();
                    }
                }
                None => {}
            };

            return res;
        }
    }

    impl Ord for Node {
        fn cmp(&self, other: &Self) -> Ordering {
            let naturlig = self.frekvens.cmp(&other.frekvens);

            match naturlig {
                std::cmp::Ordering::Less => Ordering::Greater,
                std::cmp::Ordering::Equal => Ordering::Equal,
                std::cmp::Ordering::Greater => Ordering::Less,
            }
        }
    }

    impl Eq for Node {}

    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl PartialEq for Node {
        fn eq(&self, other: &Self) -> bool {
            self.frekvens == other.frekvens
        }
    }
}

pub fn komprimer(data: &[u8]) -> Vec<u8> {
    let mut komprimert: Vec<u8> = Vec::new();

    let frekvenstabell = finn_frekvenser(data);

    // Skriv lengden på frekvenstabellen til output
    for byte in frekvenstabell.len().to_be_bytes() {
        komprimert.push(byte);
    }

    // Skriv selve frekvenstabellen til output
    for frekvens in frekvenstabell.iter() {
        for byte in frekvens.til_bytes() {
            komprimert.push(byte);
        }
    }

    let tre = konstruer_tre(frekvenstabell);
    let mut sti: Vec<Hopp> = Vec::new();
    let mut totalsti: Vec<Hopp> = Vec::new();

    // For hver byte i input-dataen, finn den korresponderende stien i Huffman-treet
    for &byte in data {
        if !tre.finn_node(byte, &mut sti) {
            panic!("Verdi ikke i treet");
        }

        // Sett sammen en "totalsti" for all dataen i treet
        totalsti.append(&mut sti);
    }

    // Skriv lengden på totalstien til output
    for byte in totalsti.len().to_be_bytes() {
        komprimert.push(byte);
    }

    let mut bytebuffer = 0b0000_0000u8;
    let mut bit_teller = 0;

    // Iterer gjennom den ovennevnte totalstien og bygg bytes til output av dem
    for hopp in totalsti.iter() {
        // Forskyv alle bits i byten som bygges en plass til venstre
        bytebuffer = bytebuffer.rotate_left(1);

        match hopp {
            // Å gå til venstre i treet representeres ved bit 0
            Hopp::Venstre => {}
            // Å gå til høyre i treet representeres ved bit 1
            Hopp::Høyre => {
                bytebuffer += 1;
            }
        };

        bit_teller += 1;

        // For hver åttende bit som er behandlet er en byte ferdig og klar for output
        if bit_teller == 8 {
            // Skriv ferdig byte
            komprimert.push(bytebuffer);

            // Reset teller og buffer
            bit_teller = 0;
            bytebuffer = 0;
        }
    }

    // Om løkken ikke hadde flaks og stoppet på kanten av en byte
    if bit_teller != 0 {
        // Plasser bits korrekt i siste byte og legg den til i komprimert data
        bytebuffer = bytebuffer.rotate_left(bit_teller);
        komprimert.push(bytebuffer);
    }

    komprimert
}

pub fn dekomprimer(data: &[u8]) -> Result<Vec<u8>, String> {
    let mut resultat: Vec<u8> = Vec::new();
    let mut posisjon: usize = 0;

    // Les lengden på frekvenstabellen fra starten av dataen
    let mut tabell_lengde_bytes: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];

    for i in 0..8 {
        tabell_lengde_bytes[i] = get_og_iterer!(data, posisjon);
    }

    let tabell_lengde = usize::from_be_bytes(tabell_lengde_bytes);

    // Les de neste 5*n bytes som frekvenstabell
    let mut frekvenstabell: Vec<ByteFrekvens> = Vec::new();
    let mut frekvens_bytes: [u8; 5] = [0, 0, 0, 0, 0];

    for _ in 0..tabell_lengde {
        for i in 0..5 {
            frekvens_bytes[i] = get_og_iterer!(data, posisjon);
        }

        frekvenstabell.push(ByteFrekvens::fra_bytes(frekvens_bytes));
    }

    // Konstruer et Huffman-tre fra den elste frekvenstabellen
    let rotnode = konstruer_tre(frekvenstabell);

    // Les antallet bits den gjenstående dataen skal bestå av
    let mut data_lengde_bytes: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];

    for i in 0..8 {
        data_lengde_bytes[i] = get_og_iterer!(data, posisjon);
    }

    let data_lengde = usize::from_be_bytes(data_lengde_bytes);

    let mut bytebuffer: u8 = get_og_iterer!(data, posisjon);
    let mut bit_teller = 0;
    let mut node: Node = rotnode.clone();

    while bit_teller < data_lengde {
        if bytebuffer > 127 {
            node = *node.høyre.unwrap();
        } else {
            node = *node.venstre.unwrap();
        }

        bit_teller += 1;
        bytebuffer = bytebuffer.rotate_left(1);

        if let Some(verdi) = node.verdi {
            resultat.push(verdi);
            node = rotnode.clone();
        }

        if bit_teller % 8 == 0 && bit_teller < data_lengde {
            bytebuffer = get_og_iterer!(data, posisjon);
        }
    }

    Ok(resultat)
}

fn finn_frekvenser(data: &[u8]) -> Vec<ByteFrekvens> {
    let mut frekvenser: Vec<ByteFrekvens> = Vec::new();

    let mut alle_frekvenser: Vec<ByteFrekvens> = Vec::new();

    for byte in 0u8..=u8::MAX {
        alle_frekvenser.push(ByteFrekvens::ny(byte));
    }

    for &byte in data {
        alle_frekvenser[byte as usize].øk();
    }

    for frekvens in alle_frekvenser {
        if frekvens.frekvens > 0 {
            frekvenser.push(frekvens);
        }
    }

    frekvenser
}

fn lag_noder(frekvenser: &Vec<ByteFrekvens>) -> Vec<Node> {
    let mut resultat: Vec<Node> = Vec::new();

    for frekvens in frekvenser {
        resultat.push(Node::fra_frekvens(frekvens));
    }

    resultat
}

fn konstruer_tre(frekvenstabell: Vec<ByteFrekvens>) -> Node {
    let noder = lag_noder(&frekvenstabell);
    let mut kø = BinaryHeap::from(noder);
    let mut ny_node;
    let mut venstre;
    let mut høyre;

    // Bygg Huffman-tre ved å kombinere de to minst vanlige verdiene i køen til en forelder-node
    // og putte denne tilbake i køen til det kun er en enkelt node igjen som er rota i treet
    while kø.len() > 1 {
        venstre = kø.pop().unwrap(); // Unwrap er trygt her, while-kondisjonen garanterer at køen ikke er tom
        høyre = kø.pop().unwrap();

        ny_node = Node::ny(
            venstre.frekvens + høyre.frekvens,
            None,
            Box::new(venstre),
            Box::new(høyre),
        );

        kø.push(ny_node)
    }

    // Følgende er en trygg unwrap siden løkken over garanterer at køen har nøyaktig ett element igjen
    // Dette elementet er rot-noden i Huffman-treet
    kø.pop().unwrap()
} 