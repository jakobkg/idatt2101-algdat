use std::{env, fmt, ops::Add, ops::Sub, process::exit, ptr::NonNull};

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
    pub fn new(verdi: u32) -> Option<Self> {
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
    pub const fn new() -> Self {
        Self {
            første: None,
            siste: None,
            antall_siffer: 0,
        }
    }

    pub fn er_tomt(&self) -> bool {
        self.første.is_none()
    }

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

impl fmt::Display for Siffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.verdi)
    }
}

impl fmt::Display for Tall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.er_tomt() {
            write!(f, "")
        } else {
            unsafe {
                let mut buffer = String::new();
                let mut siffer = self.første.as_ref().map(|siffer| siffer.as_ref());
                loop {
                    match siffer {
                        Some(some) => {
                            buffer = format!("{}{}", buffer, some.verdi);
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
        let mut sum: Tall = Tall::new();

        let (lengste, korteste) = if self.antall_siffer > rhs.antall_siffer {
            (self, rhs)
        } else {
            (rhs, self)
        };

        let mut lengste_siffer = lengste.siste;
        let mut korteste_siffer = korteste.siste;
        let mut verdisum: u32;
        let mut mente: u32 = 0;

        unsafe {
            loop {
                match korteste_siffer {
                    Some(siffer) => {
                        verdisum = (*siffer.as_ptr()).verdi
                            + (*lengste_siffer.unwrap().as_ptr()).verdi
                            + mente;
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

            loop {
                match lengste_siffer {
                    Some(siffer) => {
                        verdisum = (*siffer.as_ptr()).verdi + mente;
                        sum.sett_inn_foran(verdisum % 10);
                        mente = verdisum / 10;

                        lengste_siffer = (*siffer.as_ptr()).forrige;
                    }
                    None => {
                        if mente > 0 {
                            sum.sett_inn_foran(mente);
                        }
                        break;
                    }
                }
            }
        }

        sum
    }
}

impl std::ops::Sub<Tall> for Tall {
    type Output = Tall;

    fn sub(self, rhs: Tall) -> Self::Output {
        let mut diff = Tall::new();

        let mut rhs_kopi = rhs;

        while rhs_kopi.antall_siffer < self.antall_siffer {
            rhs_kopi.sett_inn_foran(0);
        }

        let mut verdidiff: i32;
        let mut lån = 0;
        let mut minuend_siffer = self.siste;
        let mut subtrahend_siffer = rhs_kopi.siste;

        unsafe {
            loop {
                match minuend_siffer {
                    Some(minuend) => {
                        match subtrahend_siffer {
                            Some(subtrahend) => {
                                verdidiff = (*minuend.as_ptr()).verdi as i32
                                    - (*subtrahend.as_ptr()).verdi as i32
                                    - lån;

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
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Ingen tall oppgitt!");
        println!("Forventet bruk:\n{} [tall] [+/-] [tall]", args[0]);
        return;
    }

    let tall1 = match args.get(1) {
        Some(tall) => Tall::fra_streng(tall.clone()),
        None => {
            println!("uffda");
            return
        }
    };

    let symbol = match args.get(2) {
        Some(symbol) => symbol,
        None => {
            println!("Ingen operator funnet");
            return
        }
    };

    let operasjon = match args.get(2) {
        Some(symbol) => match symbol.as_str() {
            "+" => Tall::add,
            "-" => Tall::sub,
            _ => {
                println!("Ukjent operator \"{symbol}\"");
                return
            }
        },
        None => {
            println!("Fant ingen operator");
            return
        }
    };

    let tall2 = match args.get(3) {
        Some(tall) => Tall::fra_streng(tall.clone()),
        None => {
            println!("uffda");
            return
        }
    };

    let resultat = operasjon(tall1, tall2);

    let lengde = usize::max(usize::max(tall1.antall_siffer, tall2.antall_siffer), resultat.antall_siffer);

    println!("  {:>lengde$}", format!("{}", tall1));
    println!("{symbol} {:>lengde$}", format!("{}", tall2));
    println!("= {:>lengde$}", format!("{}", resultat));
}
