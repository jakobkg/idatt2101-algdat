use std::{env, fmt, ops::Add, ops::Sub, process::exit, ptr::NonNull};

// Fortell kompilatoren at denne typen er trivielt kopierbar
#[derive(Clone, Copy)]
struct Tall {
    første: Option<NonNull<Siffer>>,
    siste: Option<NonNull<Siffer>>,
    antall_siffer: usize,
}

struct Siffer {
    verdi: u32,
    neste: Option<NonNull<Siffer>>,
    forrige: Option<NonNull<Siffer>>,
}

impl Siffer {
    /// Konstruktør, tar inn en verdi og genererer tilsvarende Siffer-objekt
    /// Kan returnere None om verdien som gis ikke er et gyldig siffer
    /// (mindre enn 0 eller større enn 9)
    pub fn new(verdi: u32) -> Option<Self> {
        // Pass på at et siffer bare har ett siffer
        if verdi > 9 {
            None
        } else {
            Some(Self {
                verdi,
                neste: None,
                forrige: None,
            })
        }
    }
}

impl Tall {
    /// Konstruktør
    pub const fn new() -> Self {
        Self {
            første: None,
            siste: None,
            antall_siffer: 0,
        }
    }

    /// Hjelpemetode, sjekk om et Tall inneholder noen Siffer eller ikke
    pub fn er_tomt(&self) -> bool {
        self.første.is_none()
    }

    /// Sett inn et nytt Siffer på begynnelsen av et Tall (venstre)
    pub fn sett_inn_foran(&mut self, verdi: u32) {
        let mut siffer = Box::new(match Siffer::new(verdi) {
            Some(siffer) => siffer,
            None => return,
        });

        unsafe {
            siffer.neste = self.første;
            siffer.forrige = None;
            let siffer: Option<NonNull<Siffer>> = Some(Box::leak(siffer).into());

            match self.første {
                None => self.siste = siffer,
                Some(første) => (*første.as_ptr()).forrige = siffer,
            }

            self.første = siffer;
            self.antall_siffer += 1;
        }
    }

    /// Sett inn et nytt Siffer på slutten av et Tall (høyre)
    pub fn sett_inn_bak(&mut self, verdi: u32) {
        let mut siffer = Box::new(match Siffer::new(verdi) {
            Some(siffer) => siffer,
            None => return,
        });

        unsafe {
            siffer.forrige = self.siste;
            siffer.neste = None;
            let siffer: Option<NonNull<Siffer>> = Some(Box::leak(siffer).into());

            match self.siste {
                Some(siste) => (*siste.as_ptr()).neste = siffer,
                None => self.første = siffer,
            }

            self.siste = siffer;
            self.antall_siffer += 1;
        }
    }

    /// Opprett et Tall fra en String ved å iterere over bokstavene i String og konvertere dem til Siffer
    pub fn fra_streng(streng: String) -> Self {
        let mut tall = Tall::new();

        for bokstav in streng.chars() {
            tall.sett_inn_bak(match bokstav.to_digit(10) {
                Some(verdi) => verdi,
                None => {
                    println!("Ugyldig tegn \"{bokstav}\" i tallet \"{streng}\"");
                    exit(1);
                }
            })
        }

        tall
    }
}

// Gjør det mulig å printe et Siffer med print!() og println!()-makroene
impl fmt::Display for Siffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.verdi)
    }
}

// Gjør det mulig å printe et Tall med print!() og println!()-makroene
impl fmt::Display for Tall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Om et Tall ikke har noen Siffer, svar med en tom string
        if self.er_tomt() {
            write!(f, "")
        } else {
            unsafe {
                // Opprett en buffer
                let mut buffer = String::new();
                let mut siffer = self.første.as_ref().map(|siffer| siffer.as_ref());

                // Iterer gjennom alle Siffer i Tallet
                loop {
                    match siffer {
                        Some(some) => {
                            // Legg til et Siffer i bufferen (ved hjelp av fmt::Display for Siffer fra ovenfor)
                            buffer = buffer + format!("{}", some).as_str();
                            siffer = some.neste.as_ref().map(|neste| neste.as_ref());
                        }
                        None => {
                            break;
                        }
                    }
                }

                write!(f, "{}", buffer)
            }
        }
    }
}

// Implementasjon av sum-operasjonen for Tall
impl std::ops::Add<Tall> for Tall {
    type Output = Tall;

    fn add(self, rhs: Tall) -> Self::Output {
        // Opprett et nytt Tall for å oppbevare resultatet av summeringen
        let mut sum: Tall = Tall::new();

        // Finn ut hvilket tall som er lengst
        let (lengste, mut korteste) = if self.antall_siffer > rhs.antall_siffer {
            (self, rhs)
        } else {
            (rhs, self)
        };

        // Fyll det korteste tallet med 0 til de to tallene har like mange siffer (f.eks. blir 100 + 50 til 100 + 050)
        while korteste.antall_siffer < lengste.antall_siffer {
            korteste.sett_inn_foran(0);
        }

        let mut lengste_siffer = lengste.siste;
        let mut korteste_siffer = korteste.siste;
        let mut verdisum: u32;
        let mut mente: u32 = 0;

        unsafe {
            // Iterer gjennom alle sifrene
            loop {
                match korteste_siffer {
                    Some(siffer) => {
                        // Summer verdiene til sifrene
                        verdisum = (*siffer.as_ptr()).verdi
                            + (*lengste_siffer.unwrap().as_ptr()).verdi
                            + mente;

                        // Om summen av to siffer blir over 10, skriv kun ener-plassen til summen og sett tier-plassen i mente
                        sum.sett_inn_foran(verdisum % 10);
                        mente = verdisum / 10;

                        korteste_siffer = (*siffer.as_ptr()).forrige;
                        lengste_siffer = (*lengste_siffer.unwrap().as_ptr()).forrige;
                    }
                    None => {
                        break;
                    }
                }
            }
        }

        sum
    }
}

// Implementasjon av subtraksjon-operasjonen for Tall
impl std::ops::Sub<Tall> for Tall {
    type Output = Tall;

    fn sub(self, rhs: Tall) -> Self::Output {
        // Opprett et nytt Tall å oppbevare resultatet i
        let mut diff = Tall::new();

        // Ta en kopi av høyre side (subtrahenden)
        let mut rhs_kopi = rhs;

        // Om nødvendig, fyll subtrahenden med 0 til de to numrene er like lange (100 - 50 bilr 100 - 050)
        while rhs_kopi.antall_siffer < self.antall_siffer {
            rhs_kopi.sett_inn_foran(0);
        }

        let mut verdidiff: i32;
        let mut lån = 0;
        let mut minuend_siffer = self.siste;
        let mut subtrahend_siffer = rhs_kopi.siste;

        unsafe {
            // Iterer gjennom alle sifrene
            loop {
                match minuend_siffer {
                    Some(minuend) => {
                        match subtrahend_siffer {
                            Some(subtrahend) => {
                                // Trekk de to sifrene på samme plass fra hverandre
                                // (her må det omformes litt da sifrenes verdi er unsigned men differansen kan bli negativ)
                                verdidiff = (*minuend.as_ptr()).verdi as i32
                                    - (*subtrahend.as_ptr()).verdi as i32
                                    - lån;

                                // Hvis differansen mellom to sifre er negativ, lån fra neste siffer
                                if verdidiff < 0 {
                                    lån = 1;
                                    diff.sett_inn_foran((10 + verdidiff) as u32);
                                } else {
                                    lån = 0;
                                    diff.sett_inn_foran(verdidiff as u32);
                                }

                                subtrahend_siffer = (*subtrahend.as_ptr()).forrige;
                            }
                            None => {
                                break;
                            }
                        }

                        minuend_siffer = (*minuend.as_ptr()).forrige;
                    }
                    None => {
                        break;
                    }
                }
            }
        }

        diff
    }
}

fn main() {
    // Les argumenter fra terminalen
    let args: Vec<String> = env::args().collect();

    // Lag en string med info til utskrift om antatt brukerfeil oppdages
    let hjelp = format!("Forventet bruk: {} [tall] [+/-] [tall]", args[0]);

    // Sjekk antall argumenter, om ingen er oppgitt så skriv ut en enkel forklaring
    if args.len() <= 1 {
        println!("Ingen tall oppgitt!");
        println!("{}", hjelp);
        return;
    }

    // Les første tall fra terminalen og konstruer et Tall-objekt
    let tall1 = match args.get(1) {
        Some(tall) => Tall::fra_streng(tall.to_string()),
        None => {
            println!("uffda");
            return;
        }
    };

    // Les operatoren fra terminalen
    let operator = match args.get(2) {
        Some(symbol) => symbol,
        None => {
            println!("Ingen operator angitt");
            println!("{}", hjelp);
            return;
        }
    };

    // Avgjør hvilken matematisk operasjon som skal utføres ut fra operatoren
    let operasjon = match operator.as_str() {
        "+" => Tall::add,
        "-" => Tall::sub,
        _ => {
            println!("Ukjent operator \"{operator}\"");
            println!("{}", hjelp);
            return;
        }
    };

    // Les andre tall fra terminalen og konstruer dets Tall-objekt
    let tall2 = match args.get(3) {
        Some(tall) => Tall::fra_streng(tall.clone()),
        None => {
            println!("Tall mangler!");
            println!("{}", hjelp);
            return;
        }
    };

    // Utfør utregning
    let resultat = operasjon(tall1, tall2);

    // Finn ut hvor bred utskriften må være for å romme alle tallene i bredden
    let lengde = usize::max(
        usize::max(tall1.antall_siffer, tall2.antall_siffer),
        resultat.antall_siffer,
    );

    // Skriv ut resultatet av utregningen
    println!("  {:>lengde$}", format!("{}", tall1));
    println!("{operator} {:>lengde$}", format!("{}", tall2));
    println!("= {:>lengde$}", format!("{}", resultat));
}
