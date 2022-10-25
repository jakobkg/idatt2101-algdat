mod graf;

use std::{env::args, process::exit};

use graf::Graf;

pub fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 3 {
        println!(
            "Manglende argumenter!\nForventet kjøring: {} filnavn startnode",
            args.get(0).unwrap()
        );
        exit(1);
    }

    let filnavn = args.get(1).unwrap();
    let startnode: usize = match args.get(2).unwrap().parse() {
        Ok(verdi) => verdi,
        Err(_) => {
            println!("Kunne ikke lese argumentet \"{}\" som startnode. Pass på at dette er et positivt heltall!", args[2]);
            exit(1)
        }
    };

    let graf = match Graf::fra_fil(filnavn) {
        Ok(graf) => graf,
        Err(feil) => {
            println!("En uventet feil oppsto ved lesing av filen:\n{feil}");
            exit(1);
        }
    };

    graf.dijkstra(startnode);
}
