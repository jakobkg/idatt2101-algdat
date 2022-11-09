use std::{env::args, process::exit};


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

mod fil;
#[macro_use]
mod huffman;
#[macro_use]
mod lz77;

fn print_hjelp(args: Vec<String>) {
    println!("Kunne ikke tolke argumentene!");
    println!("Forventet kjøring: {} -k/-d input [-o output]", args[0]);
    println!("\t-k, --komprimer:   Komprimer den angitte filen");
    println!("\t-d, --dekomprimer: Dekomprimer den angitte filen");
    println!("\t-o, --output:      Valgfritt filnavn å skrive resultatet til.");
    println!("\t\tDefault er å bruke samme filnavn som input, og legge til eller fjerne \".lzh\"");
}

enum Handling {
    Komprimer,
    Dekomprimer,
}

fn main() {
    let args: Vec<String> = args().collect();

    let handling: Handling;
    let input: String;
    let output: String;

    let mut data;

    // Sjekk argumentene for å finne ut hva som skal gjøres
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
                    if !input.ends_with(".lzh") {
                        println!(
                            "{} virker ikke å være komprimert med dette programmet",
                            args[2]
                        );
                        exit(1);
                    }

                    output = args[2][0..args[2].len() - 4].to_string();
                }
                Handling::Komprimer => {
                    output = format!("{}.lzh", input);
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

    // Les data fra angitt fil
    data = match fil::les_bytes(&input) {
        Ok(data) => data,
        Err(melding) => {
            println!("En feil oppsto ved lesing av {input}:\n{melding}");
            exit(1);
        }
    };

    // Utfør aktuell handling
    match handling {
        Handling::Komprimer => {
            data = huffman::komprimer(&data);
            data = lz77::komprimer(&data);
        },
        
        Handling::Dekomprimer => {
            data = match lz77::dekomprimer(&data) {
                Ok(data) => data,
                Err(melding) => {
                    println!("{melding}");
                    exit(1);
                }
            };

            data = match huffman::dekomprimer(&data) {
                Ok(data) => data,
                Err(melding) => {
                    println!("{melding}");
                    exit(1);
                }
            }
        }
    };

    match fil::skriv_bytes(&output, &data) {
        Ok(_) => {}
        Err(melding) => {
            println!("Noe gikk galt ved skriving til {output}:\n{melding}");
            exit(1);
        }
    }
}