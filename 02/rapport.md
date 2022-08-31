---
title: IDATT2101 Øving 2
author: Jakob Grønhaug (jakobkg@stud.ntnu.no)
---

# Algoritmene

I denne øvingen er to rekursive algoritmer for å regne ut potenser av tall analysert og implementert. Jeg vil gjennom denne besvarelsen referere til dem som algoritme 1 og algoritme 2.

## Algoritme 1

$$
    x^n =
    \begin{cases}
        x \cdot x^{n-1} & \text{når } n>0 \\
        1 & \text{når } n = 0
    \end{cases}
$$

Algoritme 1, fra oppgave 2.1-1 i læreboka, er en relativt enkel algoritme der hver iterasjon av algoritmen fører til en rekursjon der $n$ er redusert med 1, en rekursiv kompleksitet som uttrykkes som $T(n)=T(n-1)+1$. Dette fører til at rekursjonsdybden øker lineært med $n$, som tyder på at denne algoritmen har kompleksitet $\Theta(n)$.

## Algoritme 2

$$
    x^n =
    \begin{cases}
        1 & \text{når } n = 0 \\
        x \cdot (x^2)^\frac{n-1}{2} & \text{når $n$ er et oddetall} \\
        (x^2)^\frac{n}{2} & \text{når $n$ er et partall}
    \end{cases}
$$

Algoritme 2, fra bok-oppgave 2.2-3, er en mer kompleks algoritme der hver nye rekursjon kjøres med $\frac{n}{2}$, en rekursiv kompleksitet $T(n)=T(\frac{n}{2})+1$. Dette er ikke like lett å analysere som algoritme 1, men siden den rekursive kompleksiteten til algoritmen tilfredsstiller formen $T(n) = a \cdot T(\frac{n}{b}) + c \cdot n^k$ kan master-metoden brukes. Kompleksiteten $T(n)=T(\frac{n}{2})+1$ har koeffisienter $a = 1, b = 2, c = 1$ og $k = 0$. I dette tilfellet er $b^k = 2^0 = 1$ som gir $b^k = a$, og master-metoden forteller at dette gir en kompleksitet $T(n) \in \Theta(n^k \cdot \log n) \Rightarrow \Theta(\log n)$.

# Implementasjon

## Algoritme 1

Denne algoritmen kan implementeres med en enkel if-setning, da neste steg i algoritmen til enhver tid kun bestemmes av hvorvidt $n > 0$ eller $n = 0$. Pseudokode av denne algoritmen kan skrives som

```
metode potens(x, n)
    hvis n = 0
        returner 1
    hvis ikke
        returner x * potens(x, n - 1)
```

I den vedlagte kodefilen `potenser.rs` har jeg implementert algoritmen slik:

```rust
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
```

Som beskrevet tidligere har denne funksjonen kun ett rekursivt kall per iterasjon slik at $T(n) = T(n-1)+1$, og rekursjonen har et basistilfelle når $n=0$. Dersom man utfører denne algoritmen med $n<0$ vil rekursjonen bli uendelig, for å unngå dette har jeg valgt å legge til en ekstra sjekk i if-setningen for dette der funksjonen avbryter tidlig uten retur-verdi om $n$ er mindre enn 0.

## Algoritme 2

Denne algoritmen kan også implementeres ved å bruke en if-setning, denne gangen med tre muligheter heller enn to.

```
metode potens(x, n)
    hvis n = 0
        returner 1
    hvis n er et partall
        returner potens(x^2, n / 2)
    hvis n er et oddetall
        returner x * potens(x^2, (n - 1) / 2)
```

Dette har jeg implementert slik:

```rust
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
```

Som i algoritme 1 er ikke tilfellet der $n < 1$ tatt høyde for og vil føre til uendelig rekursjon, og jeg har valgt å håndtere dette tilfellet ved å returnere en tom verdi (`None`) dersom $n < 1$. Denne algoritmen ville også ført til uendelig rekursjon om utregningen av $x^2$ inne i det rekursive funksjonskallet også ble beregnet rekursivt, dette er derfor implementert som `x * x` per hintet i oppgaveteksten.

# Regner vi rett?

For å sjekke om potensene som beregnes faktisk stemmer har jeg implementert to hjelpefunksjoner, en for å runde av et flyttall til et gitt antall desimaler og en for å sammenligne algoritme 1 eller algoritme 2 sine utregninger med Rusts innebygde potensfunksjons utregninger til et gitt antall desimaler. Avrundingen er nødvendig på grunn av uunngåelige unøyaktigheter når man regner med flyttall.

Enkel testing av kjente verdier gir følgende utskrift:

```
Bekrefter at algoritmen fra 2.1-1 er korrekt
2^12 = 4096: OK
3^14 = 4782969: OK
1.1^2 = 1.21: OK

Bekrefter at algoritmen fra 2.2-3 er korrekt
2^12 = 4096: OK
3^14 = 4782969: OK
1.1^2 = 1.21: OK
```

# Tidtaking

Tidtaking av begge algoritmene er gjort ved hjelp av Rusts innebygde `time`-modul. Siden utregning av potenser går veldig raskt gjør jeg tidtaking ved å kjøre en løkke med gjentatte potens-utregninger i et sekund, telle antallet kjøringer som blir utført i løpet av dette sekundet og finne gjennomsnittlig kjøretid ved å regne $\frac{1\text{ sekund}}{\text{antall kjøringer}}$.

Denne strategien har jeg implementert slik

```rust
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

    println!("{antall:?} kjøringer på ett sekund med n = {n}, \
        {varighet:?} per kjøring");
}
```

All deklarasjon av variabler foregår utenfor hoved-løkken, men sjekk for om et sekund har gått og inkrementering av telleren foregår sammen med kjøring av algoritmene som evalueres. Heldigvis er begge disse et konstant arbeid som foregår uavhengig av algoritmen og dens $x$ eller $n$ og påvirker derfor kun de absolutte tidsmålingene og ikke de relative endringene i kjøretid når $n$ øker, slik at målingene som gjøres fortsatt kan brukes for å teste kompleksitetsanalysen av algoritmene.

## Algoritme 1

Tidtaking av algoritme 1 med metoden beskrevet over gir følgende utskrift 

```
Tidtaking av algoritme 1:
336817 kjøringer på ett sekund med n = 1000, 2.969µs per kjøring
33055 kjøringer på ett sekund med n = 10000, 30.253µs per kjøring
2817 kjøringer på ett sekund med n = 100000, 354.988µs per kjøring
```

Dette ser ut til å stemme svært godt overens med kompleksitetsanalysen, kjøretiden tidobles når $n$ tidobles som tilsier at kompleksitet $T(n) \in \Theta(n)$ er riktig for denne algoritmen.

## Algoritme 2

Tidtaking av algoritme 2 gir utskrift

```
Tidtaking av algoritme 2:
30575996 kjøringer på ett sekund med n = 10, 33ns per kjøring
27055929 kjøringer på ett sekund med n = 100, 37ns per kjøring
22839942 kjøringer på ett sekund med n = 1000, 44ns per kjøring
20981447 kjøringer på ett sekund med n = 10000, 48ns per kjøring
18895910 kjøringer på ett sekund med n = 100000, 53ns per kjøring
```

Her er det tydelig at hver tidobling av $n$ fører til en konstant økning i kjøretid, som tilsier en logaritmisk vekst. Dette stemmer overens med den asymptotiske analysen av algoritmen som konkluderte med kompleksitet $T(n) \in \Theta(\log n)$.

## Rusts innebygde potens-funksjon

Å putte Rusts innebygde funksjon for å oppheve et flyttall med en heltallspotens, `f32::powi(self, n)` inn i samme tidtaking gir denne utskriften:

```
Tidtaking av innebygd potens-metode:
34626443 kjøringer på ett sekund med n = 10, 29ns per kjøring
31408784 kjøringer på ett sekund med n = 100, 32ns per kjøring
27140357 kjøringer på ett sekund med n = 1000, 37ns per kjøring
24094568 kjøringer på ett sekund med n = 10000, 42ns per kjøring
22001562 kjøringer på ett sekund med n = 100000, 45ns per kjøring
```

Algoritmen denne funksjonen implementerer ser også ut til å ha logaritmisk kompleksitet, men er enda raskere enn algoritme 2!

Merk! Denne funksjonen har signatur `Fn(f32, i32) -> f32` og stemmer dermed ikke overens med implementasjonene av algoritme 1 og algoritme 2. For å kunne bruke denne funksjonen i den samme tidtakingen som de to andre har jeg derfor pakket den inn i en funksjon med ønsket signatur `Fn(f32, i32) -> Option<f32>`. Enkel testing tilsier at denne innpakningen ikke har noen målbar effekt på kjøretiden, og den optimaliseres trolig bort av kompilatoren siden armen der `None` kan returneres aldri blir brukt.
