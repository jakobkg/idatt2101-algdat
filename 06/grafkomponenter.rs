use std::{
    collections::VecDeque,
    env::args,
    fs::File,
    io::{BufRead, BufReader},
    process::exit,
};

/// Datatype som representerer en graf
///
/// Grafen inneholder en liste noder, som igjen inneholder en liste med naboer,
/// der "naboer" er andre noder som denne noden har kanter til
struct Graf {
    noder: Vec<Vec<usize>>,
}

/// Datatype som brukes for å ha kontroll på hvorvidt en node har blitt besøkt
/// som del av en dybde-først-traversering eller ikke
struct DfsStatus {
    besøkt: Vec<bool>,
}

impl DfsStatus {
    pub fn opprett(kapasitet: usize) -> Self {
        Self {
            besøkt: vec![false; kapasitet],
        }
    }

    /// Markerer en node som besøkt
    pub fn besøk(&mut self, node: usize) -> () {
        self.besøkt[node] = true;
    }

    /// Sjekker om en node er blitt besøkt eller ikke
    pub fn har_besøkt(&self, node: usize) -> bool {
        self.besøkt[node]
    }
}

impl Graf {
    /// Oppretter en ny graf, med angitt antall noder og ingen kanter
    pub fn opprett(kapasitet: usize) -> Self {
        Self {
            noder: vec![Vec::new(); kapasitet],
        }
    }

    /// Legger til en kant i grafen mellom de to angitte nodene
    ///
    /// Returnerer Ok() om kanten ble lagt til som forventet,
    /// og Err() om en av nodene kanten skulle være mellom ikke eksisterer i grafen
    pub fn legg_til_kant(&mut self, fra: usize, til: usize) -> Result<(), ()> {
        /*
         * En node er ugyldig om den har plass lik eller større enn antallet noder i grafen
         * (f.eks.: en graf med 5 noder har bare noder 0 til og med 4, pga 0-indeksering)
         */
        if fra >= self.antall_noder() || til >= self.antall_noder() {
            Err(())
        } else {
            self.noder[fra].push(til);
            Ok(())
        }
    }

    /// Konstruerer en graf med data fra en fil som følger formatet angitt i oppgaveteksten
    /// Returnerer Ok(Graf) om grafen ble konstruert, eller Err(String) med en feilmelding om
    /// noe gikk galt under konstruering av grafen (f.eks. pga uventede verdier eller feil format i filen)
    pub fn fra_fil(fil: File) -> Result<Self, String> {
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
                        return Err("Kunne ikke lese antall noder fra starten av filen".into());
                    }
                };

                // Les antallet kanter og lagre verdien
                forventet_antall_kanter = match strbuf[1].parse::<usize>() {
                    Ok(num) => num,
                    Err(_) => {
                        return Err("Kunne ikke lese antall kanter fra starten av filen".into());
                    }
                };

                // Opprett en graf med det angitte antall noder
                Graf::opprett(kapasitet)
            }
            Err(_) => return Err("Kunne ikke lese fra filen".into()),
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

                    // LEs noden kanten skal gå til
                    let til = match buf[1].parse::<usize>() {
                        Ok(til) => til,
                        Err(_) => {
                            return Err(format!(
                                "Kunne ikke lese node-tall fra linje {linjenummer}: {linje}"
                            ))
                        }
                    };

                    // Forsøk å legge til den nye kanten i grafen
                    match graf.legg_til_kant(fra, til) {
                        Ok(_) => {
                            lest_antall_kanter += 1;
                        }
                        Err(_) => return Err(format!("Kunne ikke legge til kant {fra} -> {til}, grafen inneholder kun noder mellom 0 og {}", graf.antall_noder())),
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

    /// Returnerer antallet noder i grafen
    pub fn antall_noder(&self) -> usize {
        self.noder.len()
    }

    /// Inverterer grafen om mulig, og returnerer den
    ///
    /// En graf er potensielt ikke inverterbar om den inneholder
    /// kanter mellom noder som ikke eksisterer,
    pub fn invertert(&self) -> Result<Self, String> {
        let mut invertert = Self::opprett(self.antall_noder());

        for (til, kanter) in self.noder.iter().enumerate() {
            for fra in kanter {
                match invertert.legg_til_kant(*fra, til) {
                    Err(_) => {
                        return Err(format!(
                            "Kunne ikke legge til kant fra {fra} til {til}, da en av disse ikke eksisterer."
                        ))
                    }
                    _ => {}
                }
            }
        }

        Ok(invertert)
    }

    /// Utfører en dybde-først-traversering av alle nodene i grafen og returnerer en Vec<usize> med rekkefølgen nodene ble besøkt i
    pub fn dfs(&self) -> Vec<usize> {
        let mut status = DfsStatus::opprett(self.antall_noder());
        let mut rekkefølge: VecDeque<usize> = VecDeque::new();

        for node in 0..self.antall_noder() {
            if !status.har_besøkt(node) {
                status.besøk(node);
                self.dybdebesøk(node, &mut status, &mut rekkefølge);
                rekkefølge.push_front(node);
            }
        }

        rekkefølge.into()
    }

    /// Hjelpemetode for dybde-først-traversering
    fn dybdebesøk(&self, node: usize, status: &mut DfsStatus, rekkefølge: &mut VecDeque<usize>) {
        for nabo in &self.noder[node] {
            if !status.har_besøkt(*nabo) {
                status.besøk(*nabo);
                self.dybdebesøk(*nabo, status, rekkefølge);
                rekkefølge.push_front(*nabo);
            }
        }
    }

    /// Finner sterkt koblede komponenter i grafen og skriver dem ut
    pub fn print_komponenter(&self) {
        let invers = match self.invertert() {
            Ok(graf) => graf,
            Err(_) => return,
        };

        let mut status = DfsStatus::opprett(self.antall_noder());
        let rekkefølge = self.dfs();
        let mut n = 0;

        let mut komponenter: Vec<VecDeque<usize>> = Vec::new();

        for node in rekkefølge {
            if !status.har_besøkt(node) {
                komponenter.push(VecDeque::new());

                status.besøk(node);
                komponenter[n].push_front(node);
                invers.dybdebesøk(node, &mut status, &mut komponenter[n]);
                n += 1;
            }
        }

        println!("Grafen har {} sterkt koblede komponenter", n);

        n = 1;
        for komponent in komponenter {
            print!("Komponent {n}: ");

            for node in komponent {
                print!("{node} ");
            }

            println!();
            n += 1;
        }
    }
}

/// Implementerer traitet Display for Graf typen, så den enkelt kan brukes i print!() og lignende makroer
impl std::fmt::Display for Graf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (nummer, node) in self.noder.iter().enumerate() {
            write!(f, "{} ->", nummer)?;

            for kant in node {
                write!(f, " {}", kant)?;
            }

            write!(f, "\n")?;
        }

        Ok(())
    }
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        println!("Ingen graf-fil angitt!");
        println!("Forventet argument: {} [filnavn]", args[0]);
        println!("For eksempel: {} ø6g1", args[0]);
        exit(1);
    }

    let fil = match File::open(&args[1]) {
        Ok(fil) => fil,
        Err(_) => {
            println!("Kunne ikke åpne fil {}", &args[1]);
            exit(1);
        }
    };

    let graf = match Graf::fra_fil(fil) {
        Ok(graf) => graf,
        Err(err) => {
            println!("Kunne ikke opprette graf fra filen \"{}\". Er du sikker på at dette er en gyldig graf-fil?", args[1]);
            println!("{err}");
            exit(1);
        }
    };

    if graf.antall_noder() < 100 {
        println!("Graf:\n{graf}");
    }

    graf.print_komponenter();
}
