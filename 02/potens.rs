use std::time::{Instant, Duration};

// Implementasjon av algoritmen fra 2.1-1
fn del1(x: f32, n: i32) -> f32 {
    if n == 0 {
        1.
    } else {
        x * del1(x, n - 1)
    }
}

// Implementasjon av algoritmen fra 2.2-3
fn del2(x: f32, n: i32) -> f32 {
    if n == 0 {
        1.
    } else if n % 2 == 0 {
        del2(x * x, n / 2)
    } else {
        x * del2(x * x, (n - 1) / 2)
    }
}

// Hjelpefunksjon for avrunding av flyttall til et gitt antall desimaler
fn rund(x: f32, desimaler: i32) -> f32 {
    ((x * 10f32.powi(desimaler)).round()) / 10f32.powi(desimaler)
}

// Hjelpefunksjon for å teste om en potens-beregning er korrekt med et gitt antall desimalers presisjon
fn bekreft(f: &dyn Fn(f32, i32) -> f32, x: f32, n: i32, desimaler: i32) {
    print!("{x}^{n} = {}: ", rund(f(x, n), desimaler));
    println!("{}", match rund(f(x, n), desimaler) == rund(x.powi(n), desimaler) {
        true => "✔️",
        false => "❌"
    });
}

// Hjelpefunksjon for å beregne tidsbruk av en annen funksjon ved å gjenta den i et sekund og telle kjøringer
fn stoppeklokke(f: &dyn Fn(f32, i32) -> f32, x: f32, n: i32) {
    let start: Instant;
    let mut end: Instant;
    let mut antall = 0;

    let second = Duration::from_secs(1);

    start = Instant::now();

    loop {
        f(x, n);
        antall += 1;

        end = Instant::now();

        if end - start > second {
            break;
        }
    }

    let varighet = Duration::from_secs_f32(1./(antall as f32));

    println!("{antall:?} kjøringer på et sekund med n = {n}, {varighet:?} per kjøring");
}

fn main() {
    println!("Bekrefter at algoritmen fra 2.1-1 er korrekt");
    bekreft(&del1, 2., 12, 0);
    bekreft(&del1, 3., 14, 0);
    bekreft(&del1, 1.1, 2, 2);
    println!();

    println!("Bekrefter at algoritmen fra 2.2-3 er korrekt");
    bekreft(&del2, 2., 12, 0);
    bekreft(&del2, 3., 14, 0);
    bekreft(&del2, 1.1, 2, 2);
    println!();

    println!("Tidtaking av algoritme 1:");
    stoppeklokke(&del1, 1.1, 10000);
    stoppeklokke(&del1, 1.1, 100000);
    println!();

    println!("Tidtaking av algoritme 2:");
    stoppeklokke(&del2, 1.1, 10000);
    stoppeklokke(&del2, 1.1, 100000);
    println!();

    println!("Tidtaking av innebygd potens-metode:");
    stoppeklokke(&f32::powi, 1.1, 10000);
    stoppeklokke(&f32::powi, 1.1, 100000);
}
