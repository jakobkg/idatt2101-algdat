use std::env;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut should_print: bool = false;

    if args.len() < 2 {
        println!("Expected a non-negative integer as an argument");
        return
    }

    // Sjekk om brukeren ønsker å få hele test-dataen presentert
    if args.contains(&"-s".to_string()) {
        should_print = true;
    }

    // Les antall dager med prisendring som skal genereres fra args
    let antall: usize = args[1].parse()
    .expect("Argument must be a valid non-negative integer");
    
    // Opprett to vektorer, en som buffer for lesing fra /dev/urandom
    // og en for å oppbevare prisendringer
    let mut filebuf = vec![0u8; antall];
    let mut deltabuf =  vec![0; antall];

    // Gjør klar for å lese fra /dev/urandom
    let mut f = File::open("/dev/urandom")
    .expect("/dev/urandom not available");

    // Les bytes derfra (heltall i intervallet [0, 255])
    f.read_exact(&mut filebuf)
    .expect("/dev/urandom not available");

    // Konverter disse til heltall i intervallet [-10, 10]
    for (i, byte) in filebuf.iter().enumerate() {
        deltabuf[i] = (*byte % 21) as i32 - 10;
    }

    // Vis prisendringene til brukeren, om ønsket
    if should_print { println!("{:?}", deltabuf); }

    // Deklarer nødvendige variabler
    let mut diff;
    let mut diffmax = 0;
    let mut buyday = 0;
    let mut sellday = 0;

    // Start tidtaking
    let start: Instant = Instant::now();

    // Iterer gjennom prisendringene
    for i in 0..deltabuf.len() {
        diff = 0;

        // Iterer gjennom dagene etter dag i
        for j in i+1..deltabuf.len() {
            // Beregn prisdifferansen mellom dag i og dag j
            diff += deltabuf[j];

            // Om prisdifferansen er den største observert så langt,
            // lagre differansen og dagene det kjøpes/selges på for å oppnå denne
            if diff > diffmax {
                diffmax = diff;
                buyday = i+1;
                sellday = j+1;
            }
        }
    }

    // Sjekk løpt tid etter fullført arbeid
    let time = start.elapsed();

    // Presenter resultat
    print!("Optimal trade: Buy on day {}, ", buyday);
    print!("then sell on day {}. ", sellday);
    print!("Gain: {}, ", diffmax);
    println!("time spent [µs]: {:?}", time.as_micros());
}
