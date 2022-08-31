use std::time::{Instant, Duration};

// Implementasjon av algoritmen fra 2.1-1
fn algoritme1(x: f32, n: i32) -> Option<f32> {
    if n < 0 {
        None
    } else if n == 0 {
        Some(1.)
    } else {
        Some(x * algoritme1(x, n - 1).unwrap())
    }
}

// Implementasjon av algoritmen fra 2.2-3
fn algoritme2(x: f32, n: i32) -> Option<f32> {
    if n < 0 {
        None
    } else if n == 0 {
        Some(1.)
    } else if n % 2 == 0 {
        algoritme2(x * x, n / 2)
    } else {
        Some(x * algoritme2(x * x, (n - 1) / 2).unwrap())
    }
}

fn innebygd(x: f32, n: i32) -> Option<f32> {
    if n < 0 {
        None
    } else {
        Some(x.powi(n))
    }
}

// Hjelpefunksjon for avrunding av flyttall til et gitt antall desimaler
fn rund(x: f32, desimaler: i32) -> Option<f32> {
    if desimaler < 0 {
        None
    } else {
        Some(((x * 10f32.powi(desimaler)).round()) / 10f32.powi(desimaler))
    }
}

// Hjelpefunksjon for å teste om en potens-beregning er korrekt med et gitt antall desimalers presisjon
fn bekreft(f: &dyn Fn(f32, i32) -> Option<f32>, x: f32, n: i32, desimaler: i32) {
    let utregnet = rund(f(x, n).unwrap(), desimaler);
    print!("{x}^{n} = {}: ", utregnet.unwrap());
    println!("{}", match utregnet == rund(x.powi(n), desimaler) {
        true => "✔️",
        false => "❌"
    });
}

// Hjelpefunksjon for å beregne tidsbruk av en annen funksjon ved å gjenta den i et sekund og telle kjøringer
fn stoppeklokke(f: &dyn Fn(f32, i32) -> Option<f32>, x: f32, n: i32) {
    let start: Instant;
    let mut slutt: Instant;
    let mut antall = 0;

    let sekund = Duration::from_secs(1);

    start = Instant::now();

    loop {
        f(x, n);
        antall += 1;

        slutt = Instant::now();

        if slutt - start > sekund {
            break;
        }
    }

    let varighet = Duration::from_secs_f32(1./(antall as f32));

    println!("{antall:?} kjøringer på ett sekund med n = {n}, {varighet:?} per kjøring");
}

fn main() {
    println!("Bekrefter at algoritmen fra 2.1-1 er korrekt");
    bekreft(&algoritme1, 2., 12, 0);
    bekreft(&algoritme1, 3., 14, 0);
    bekreft(&algoritme1, 1.1, 2, 2);
    println!();

    println!("Bekrefter at algoritmen fra 2.2-3 er korrekt");
    bekreft(&algoritme2, 2., 12, 0);
    bekreft(&algoritme2, 3., 14, 0);
    bekreft(&algoritme2, 1.1, 2, 2);
    println!();

    println!("Tidtaking av algoritme 1:");
    stoppeklokke(&algoritme1, 1.1, 1000);
    stoppeklokke(&algoritme1, 1.1, 10000);
    stoppeklokke(&algoritme1, 1.1, 100000);
    println!();

    println!("Tidtaking av algoritme 2:");
    stoppeklokke(&algoritme2, 1.1, 10);
    stoppeklokke(&algoritme2, 1.1, 100);
    stoppeklokke(&algoritme2, 1.1, 1000);
    stoppeklokke(&algoritme2, 1.1, 10000);
    stoppeklokke(&algoritme2, 1.1, 100000);
    println!();

    println!("Tidtaking av innebygd potens-metode:");
    stoppeklokke(&innebygd, 1.1, 10);
    stoppeklokke(&innebygd, 1.1, 100);
    stoppeklokke(&innebygd, 1.1, 1000);
    stoppeklokke(&innebygd, 1.1, 10000);
    stoppeklokke(&innebygd, 1.1, 100000);
}
