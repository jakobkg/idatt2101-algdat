use std::{
    collections::VecDeque,
    env::args,
    fs::File,
    io::{BufRead, BufReader},
    process::exit,
};

struct Graf {
    noder: Vec<Vec<usize>>,
}

struct DfsStatus {
    besøkt: Vec<bool>,
}

impl DfsStatus {
    pub fn opprett(kapasitet: usize) -> Self {
        Self {
            besøkt: vec![false; kapasitet],
        }
    }

    pub fn besøk(&mut self, pos: usize) -> () {
        self.besøkt[pos] = true;
    }

    pub fn har_besøkt(&self, pos: usize) -> bool {
        self.besøkt[pos]
    }
}

impl Graf {
    pub fn opprett(kapasitet: usize) -> Self {
        Self {
            noder: vec![Vec::new(); kapasitet],
        }
    }

    pub fn legg_til_kant(&mut self, fra: usize, til: usize) -> Result<(), ()> {
        if fra >= self.antall_noder() || til >= self.antall_noder() {
            Err(())
        } else {
            self.noder[fra].push(til);
            Ok(())
        }
    }

    pub fn fra_fil(fil: File) -> Result<Self, String> {
        let mut leser = BufReader::new(fil);
        let mut stringbuf: String = "".into();

        let forventet_antall_kanter: usize;
        let mut lest_antall_kanter: usize = 0;

        let mut graf: Graf = match leser.read_line(&mut stringbuf) {
            Ok(_) => {
                let strbuf: Vec<&str> = stringbuf.split_whitespace().collect();
                let kapasitet = match strbuf[0].parse::<usize>() {
                    Ok(num) => num,
                    Err(_) => {
                        return Err("Kunne ikke lese antall noder fra starten av filen".into());
                    }
                };

                forventet_antall_kanter = match strbuf[1].parse::<usize>() {
                    Ok(num) => num,
                    Err(_) => {
                        return Err("Kunne ikke lese antall kanter fra starten av filen".into());
                    }
                };

                Graf::opprett(kapasitet)
            }
            Err(_) => return Err("Kunne ikke lese fra filen".into()),
        };

        for (linjenummer, linje) in leser.lines().enumerate() {
            match linje {
                Ok(linje) => {
                    let buf: Vec<&str> = linje.split_whitespace().collect();

                    let fra = match buf[0].parse::<usize>() {
                        Ok(fra) => fra,
                        Err(_) => {
                            return Err(format!(
                                "Kunne ikke lese node-tall fra linje {linjenummer}: {linje}"
                            ))
                        }
                    };

                    let til = match buf[1].parse::<usize>() {
                        Ok(til) => til,
                        Err(_) => {
                            return Err(format!(
                                "Kunne ikke lese node-tall fra linje {linjenummer}: {linje}"
                            ))
                        }
                    };

                    match graf.legg_til_kant(fra, til) {
                        Ok(_) => {
                            lest_antall_kanter += 1;
                        }
                        Err(_) => return Err(format!("Kunne ikke legge til kant {fra} -> {til}, da node {fra} ikke eksisterer")),
                    }
                }
                Err(_) => {}
            }
        }

        if lest_antall_kanter != forventet_antall_kanter {
            println!("ADVARSEL:\n>\tFilen inneholdt et annet antall kanter enn angitt, og er kanskje ugyldig.\n>\t({} forventet, {} lest)",
                forventet_antall_kanter, lest_antall_kanter);
        }

        Ok(graf)
    }

    pub fn antall_noder(&self) -> usize {
        self.noder.len()
    }

    pub fn invertert(&self) -> Result<Self, String> {
        let mut invertert = Self::opprett(self.antall_noder());

        for (til, kanter) in self.noder.iter().enumerate() {
            for fra in kanter {
                match invertert.legg_til_kant(*fra, til) {
                    Ok(_) => {}
                    Err(_) => {
                        return Err(format!(
                            "Kunne ikke legge til kant fra {fra} til {til}, da node {fra} ikke eksisterer."
                        ))
                    }
                }
            }
        }

        Ok(invertert)
    }

    pub fn dfs(&self) -> Vec<usize> {
        let mut status = DfsStatus::opprett(self.antall_noder());
        let mut rekkefølge: VecDeque<usize> = VecDeque::new();

        for node in 0..self.antall_noder() {
            if !status.har_besøkt(node) {
                status.besøk(node);
                self.besøk(node, &mut status, &mut rekkefølge);
                rekkefølge.push_front(node);
            }
        }

        rekkefølge.into()
    }

    fn besøk(&self, pos: usize, status: &mut DfsStatus, rekkefølge: &mut VecDeque<usize>) {
        for nabo in &self.noder[pos] {
            if !status.har_besøkt(*nabo) {
                status.besøk(*nabo);
                self.besøk(*nabo, status, rekkefølge);
                rekkefølge.push_front(*nabo);
            }
        }
    }

    fn traverser_og_print(&self, fra: usize, status: &mut DfsStatus) {
        for nabo in &self.noder[fra] {
            if !status.har_besøkt(*nabo) {
                print!("{nabo} ");
                status.besøk(*nabo);
                self.traverser_og_print(*nabo, status);
            }
        }
    }

    pub fn print_komponenter(&self) {
        let invers = match self.invertert() {
            Ok(graf) => {graf},
            Err(_) => {return},
        };

        let mut status = DfsStatus::opprett(self.antall_noder());
        let rekkefølge = self.dfs();
        let mut n = 1;

        for node in rekkefølge {
            if !status.har_besøkt(node) {
                print!("Komponent {n}: {node} ");

                status.besøk(node);
                invers.traverser_og_print(node, &mut status);

                println!();
                n += 1;
            }
        }
    }
}

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
        println!("DFS-rekkefølge: {:?}", graf.dfs());
    }
    graf.print_komponenter();
}
