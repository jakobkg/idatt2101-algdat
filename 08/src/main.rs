use std::{env::args, process::exit};

mod lempel_ziv {
    use std::{
        fs::File,
        io::{Read, Write},
    };

    /// Denne funksjonen forsøker å åpne en angitt fil, komprimere den med LZ77-algoritmen,
    /// og skrive resultatet til samme fil-sti med filending .lz
    ///
    /// # Errors
    ///
    /// Funksjonen vil feile dersom åpning, lesing eller skriving av de nødvendige filene ikke er mulig.
    /// Den underliggende feilmeldingen fra operativsystemet propageres opp til kalleren gjennom en
    /// Err(String), som så kan håndteres videre
    pub fn komprimer_fil(inn: &str, ut: &str) -> Result<(), String> {
        // Les fil som Vec av bytes
        let mut f = match File::open(inn) {
            Ok(handle) => handle,
            Err(e) => {
                return Err(format!("Kunne ikke åpne filen {inn}\nFeilmelding: \"{e}\""));
            }
        };

        let mut data: Vec<u8> = Vec::new();

        match f.read_to_end(&mut data) {
            Ok(n) => {
                println!("Leste {n} bytes fra {inn}")
            }
            Err(e) => {
                return Err(format!(
                    "Kunne ikke lese fra filen {inn}\nFeilmelding: \"{e}\""
                ));
            }
        }

        // Komprimer data med LZ77-algoritmen
        let resultat = lz77(&data);

        // Skriv resultatet av komprimeringen ut til en ny fil
        let mut f = match File::create(ut) {
            Ok(handle) => handle,
            Err(e) => {
                return Err(format!(
                    "Kunne ikke åpne {ut} for å skrive\nFeilmelding: \"{e}\""
                ));
            }
        };

        match f.write_all(resultat) {
            Ok(_) => {
                println!("Skrev {} bytes til {ut}", resultat.len());
            }
            Err(e) => {
                return Err(format!("Kunne ikke skrive til {ut}\nFeilmelding: \"{e}\""));
            }
        }

        Ok(())
    }

    pub fn lz77(data: &[u8]) -> &[u8] {
        // TODO
        return data;
    }

    fn finn_bakoverreferanse(data: &[u8]) -> (u16, u8) {
        let mut hopp = 0u16;
        let mut lengde = 0u8;

        (0, 0)
    }
}

fn main() {
    let args: Vec<String> = args().collect();

    match args.len() {
        2 => match lempel_ziv::komprimer_fil(&args[1], &format!("{}.lz", &args[1])) {
            Ok(_) => {}
            Err(melding) => {
                println!("{melding}");
                exit(1);
            }
        },

        4 => {
            if &args[2] == "-o" {
                match lempel_ziv::komprimer_fil(&args[1], &args[3]) {
                    Ok(_) => {}
                    Err(melding) => {
                        println!("{melding}");
                        exit(1);
                    }
                }
            } else {
                println!("Kunne ikke tolke argumentene!");
                println!("Forventet kjøring: {} input [-o output]", args[0]);
            }
        }

        _ => {
            if args.len() < 2 {
                println!("Mangler argument! Vennligst angi filen som skal komprimeres.");
                println!("Forventet kjøring: {} input [-o output]", args[0]);
                exit(0);
            } else {
                println!("Kunne ikke tolke argumentene!");
                println!("Forventet kjøring: {} input [-o output]", args[0]);
            }
        }
    }
}
