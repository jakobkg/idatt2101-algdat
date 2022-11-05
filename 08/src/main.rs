use std::{
    env::args,
    process::exit,
};

mod fil {
    use std::{
        fs::File,
        io::{Read, Write},
    };

    pub fn les_bytes(filsti: &str) -> Result<Vec<u8>, String> {
        // Les fil som Vec av bytes
        let mut f = match File::open(filsti) {
            Ok(handle) => handle,
            Err(e) => {
                return Err(format!(
                    "Kunne ikke åpne filen {filsti}\nFeilmelding: \"{e}\""
                ));
            }
        };

        let mut data: Vec<u8> = Vec::new();

        match f.read_to_end(&mut data) {
            Ok(n) => {
                println!("Leste {n} bytes fra {filsti}");
            }
            Err(e) => {
                return Err(format!(
                    "Kunne ikke lese fra filen {filsti}\nFeilmelding: \"{e}\""
                ));
            }
        }

        Ok(data)
    }

    pub fn skriv_bytes(filsti: &str, data: &[u8]) -> Result<(), String> {
        // Skriv resultatet av komprimeringen ut til en ny fil
        let mut f = match File::create(filsti) {
            Ok(handle) => handle,
            Err(e) => {
                return Err(format!(
                    "Kunne ikke åpne {filsti} for å skrive\nFeilmelding: \"{e}\""
                ));
            }
        };

        match f.write_all(&data) {
            Ok(_) => {
                println!("Skrev {} bytes til {filsti}", data.len());
            }
            Err(e) => {
                return Err(format!(
                    "Kunne ikke skrive til {filsti}\nFeilmelding: \"{e}\""
                ));
            }
        }

        Ok(())
    }
}

mod lz77 {
    fn finn_lengste_treff(data: &[u8], posisjon: usize) -> (i16, u8) {
        let mut beste_hopp = 0i16;
        let mut beste_lengde = 0u8;

        let start = if posisjon > 255 { posisjon - 255 } else { 0 };

        for hopp in start..posisjon {
            let len = matcher(data, hopp, posisjon);

            if len > beste_lengde {
                beste_hopp = (posisjon - (hopp as usize)) as i16;
                beste_lengde = len;
            }
        }

        (-beste_hopp, beste_lengde)
    }

    fn matcher(data: &[u8], hopp: usize, slutt: usize) -> u8 {
        let mut hopp = hopp;
        let mut posisjon = slutt;
        let mut lengde = 0u8;

        while hopp < posisjon
            && posisjon < data.len()
            && data[hopp] == data[posisjon]
            && lengde < 255
            && hopp < slutt
        {
            hopp += 1;
            posisjon += 1;
            lengde += 1;
        }

        lengde
    }

    pub fn komprimer(data: &[u8]) -> Vec<u8> {
        let mut komprimert = Vec::new();
        let mut posisjon = 0;

        let mut ukomprimert: Vec<u8> = Vec::new();

        while posisjon < data.len() {
            let (mut hopp, mut lengde) = finn_lengste_treff(data, posisjon);

            // Implementasjonen bruker tre byte med header for en komprimert blokk, så om et mønster ikke er
            // større enn dette er det ikke vits i å sette inn en header, og en ukomprimert blokk bygges
            if lengde < 4 {
                ukomprimert.push(data[posisjon]);
                posisjon += 1;

                (hopp, lengde) = finn_lengste_treff(data, posisjon);

                while lengde < 4 && posisjon < data.len() {
                    ukomprimert.push(data[posisjon]);
                    posisjon += 1;

                    (hopp, lengde) = finn_lengste_treff(data, posisjon);
                }

                // Når den ukomprimerte blokken er ferdig, sett inn lengden på den på begynnelsen
                (ukomprimert.len() as i16)
                    .to_be_bytes()
                    .iter()
                    .for_each(|&byte| komprimert.push(byte));

                komprimert.append(&mut ukomprimert);
            }

            if lengde > 3 {
                hopp.to_be_bytes()
                    .iter()
                    .for_each(|&byte| komprimert.push(byte));

                komprimert.push(lengde);
            }

            posisjon += lengde as usize;
        }

        komprimert
    }

    pub fn dekomprimer(data: &[u8]) -> Result<Vec<u8>, String> {
        #[macro_export]
        macro_rules! get_og_iterer {
            ( $arr:ident, $idx:ident ) => {
                match $arr.get($idx) {
                    Some(&byte) => {
                        $idx += 1;
                        byte
                    }
                    None => return Err("En uventet feil oppsto ved dekomprimering".to_string()),
                }
            };
        }

        let mut dekomprimert: Vec<u8> = Vec::new();

        let mut posisjon: usize = 0;
        let mut header;
        let mut hopp;
        let mut headerbytes: [u8; 2] = [0, 0];
        let mut byte: u8;

        loop {
            // Sjekk om det fremdeles er data igjen i input, bryt løkken om det ikke er det
            match data.get(posisjon) {
                Some(_) => {}
                None => break,
            }

            for i in 0..2 {
                byte = get_og_iterer!(data, posisjon);
                headerbytes[i] = byte;
            }

            header = i16::from_be_bytes(headerbytes);

            if header > 0 {
                // Kopier ukomprimert blokk direkte til output
                for _ in 0..header {
                    dekomprimert.push(get_og_iterer!(data, posisjon));
                }
            } else {
                // Vi er i en komprimert blokk!
                let blokklengde = get_og_iterer!(data, posisjon) as usize;

                hopp = header.abs() as usize;

                for _ in 0..blokklengde {
                    dekomprimert.push(dekomprimert[dekomprimert.len() - hopp]);
                }
            }
        }

        Ok(dekomprimert)
    }
}

mod huffman {
    use std::collections::BinaryHeap;
    use typer::ByteFrekvens;

    mod typer {
        use std::cmp::Ordering;

        #[derive(Debug)]
        pub struct ByteFrekvens {
            byte: u8,
            frekvens: u32
        }
    
        impl ByteFrekvens {
            pub fn ny(byte: u8) -> Self {
                Self {
                    byte,
                    frekvens: 0
                }
            }
    
            pub fn øk(&mut self) -> () {
                self.frekvens += 1;
            }
        }
    
        impl Ord for ByteFrekvens {
            fn cmp(&self, other: &Self) -> Ordering {
                let natural = self.frekvens.cmp(&other.frekvens);
    
                match natural {
                    std::cmp::Ordering::Less => Ordering::Greater,
                    std::cmp::Ordering::Equal => Ordering::Equal,
                    std::cmp::Ordering::Greater => Ordering::Less,
                }
            }
        }
        
        impl Eq for ByteFrekvens {}
    
        impl PartialOrd for ByteFrekvens {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
    
        impl PartialEq for ByteFrekvens {
            fn eq(&self, other: &Self) -> bool {
                self.frekvens == other.frekvens
            }
        }
    
    }
    
    pub fn komprimer(data: &[u8]) -> Vec<u8> {
        let mut komprimert: Vec<u8> = Vec::new();

        let mut frekvenser = frekvenser(data);

        let mut kø = BinaryHeap::from(frekvenser);

        println!("{kø:?}");

        komprimert
    }

    fn frekvenser(data: &[u8]) -> Vec<ByteFrekvens> {
        let mut frekvenser: Vec<ByteFrekvens> = Vec::new();

        let mut data = Vec::from(data);

        data.sort();

        let mut forrige_byte = data[0];
        let mut frekvens = ByteFrekvens::ny(forrige_byte);

        for byte in data {
            if byte == forrige_byte {
                frekvens.øk();
            } else {
                frekvenser.push(frekvens);
                frekvens = ByteFrekvens::ny(byte);
                frekvens.øk();
                forrige_byte = byte;
            }
        }

        frekvenser
    }
}

fn print_hjelp(args: Vec<String>) {
    println!("Kunne ikke tolke argumentene!");
    println!("Forventet kjøring: {} -k/-d input [-o output]", args[0]);
    println!("\t-k, --komprimer:   Komprimer den angitte filen");
    println!("\t-d, --dekomprimer: Dekomprimer den angitte filen");
    println!("\t-o, --output:      Valgfritt filnavn å skrive resultatet til.");
    println!("\t\tDefault er å bruke samme filnavn som input, og legge til eller fjerne \".lz\"");
}

enum Handling {
    Komprimer,
    Dekomprimer,
}

fn main() {
    let mut data = fil::les_bytes("testfile").unwrap();

    huffman::komprimer(&data);
}

/*
fn main() {
    let args: Vec<String> = args().collect();

    let handling: Handling;
    let input: String;
    let output: String;

    let data;
    let resultat: Vec<u8>;

    match args.len() {
        3 => {
            input = args[2].clone();

            if args[1].contains("-k") {
                handling = Handling::Komprimer;
            } else if args[1].contains("-d") {
                handling = Handling::Dekomprimer;
            } else {
                print_hjelp(args);
                exit(1);
            }

            match handling {
                Handling::Dekomprimer => {
                    if !input.ends_with(".lz") {
                        println!(
                            "{} virker ikke å være komprimert med dette programmet",
                            args[2]
                        );
                        exit(1);
                    }

                    output = args[2][0..args[2].len() - 3].to_string();
                }
                Handling::Komprimer => {
                    output = format!("{}.lz", input);
                }
            }
        }

        5 => {
            input = args[2].clone();

            if ["-k", "--komprimer"].contains(&args[1].as_str()) {
                handling = Handling::Komprimer;
            } else if ["-d", "--dekomprimer"].contains(&args[1].as_str()) {
                handling = Handling::Dekomprimer;
            } else {
                print_hjelp(args);
                exit(1);
            }

            if ["-o", "--output"].contains(&args[3].as_str()) {
                output = args[4].clone()
            } else {
                print_hjelp(args);
                exit(1);
            }
        }
        _ => {
            print_hjelp(args);
            exit(1);
        }
    }

    data = match fil::les_bytes(&input) {
        Ok(data) => data,
        Err(melding) => {
            println!("En feil oppsto ved lesing av {input}:\n{melding}");
            exit(1);
        }
    };

    match handling {
        Handling::Komprimer => {
            resultat = lz77::komprimer(&data);
        }
        Handling::Dekomprimer => {
            resultat = match lz77::dekomprimer(&data) {
                Ok(data) => data,
                Err(melding) => {
                    println!("{melding}");
                    exit(1);
                }
            };
        }
    };

    match fil::skriv_bytes(&output, &resultat) {
        Ok(_) => {}
        Err(melding) => {
            println!("Noe gikk galt ved skriving til {output}:\n{melding}");
            exit(1);
        }
    }
}
*/