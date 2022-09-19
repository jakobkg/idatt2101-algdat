---
title: IDATT2101, Øving 3
author: Jakob Grønhaug (jakobkg@stud.ntnu.no)
---

# Shellsort

I denne øvingen var det mulig å gjøre oppgaver om forskjellige sorteringsalgoritmer, enten quicksort med enkel og dobbel pivot eller shellsort. Jeg har valgt å gjøre oppgaven om shellsort. Shellsort er en forbedring av en enkel insertion sort, der man sammenligner tall som er lengre unna hverandre og bytter plass på dem om nødvendig, og så ser på tall nærmere og nærmere hverandre i listen. På denne måten blir listen først grovt sortert, og så sortert finere og finere til det til sist gjøres et enkelt pass med det som i praksis er insertion sort når nabotall sammenlignes.

# Implementasjon

Min implementasjon av shellsort i Rust medfølger i filen `shellsort.rs`. Denne implementasjonen avviker fra eksempel-implemetasjonen fra fagboken i at jeg har valgt å på forhånd beregne delingstallene før selve sorteringen starter, heller enn å beregne delingstallene underveis i sorteringen. Dette er implementert ved å opprette en vektor $S$ som kun inneholder en verdi $\frac{n}{2}$, hvor $n$ er antall elementer i dataen som skal sorteres. Denne vektoren fylles så opp med verdier som igjen og igjen deles på delingstallet $s$, slik at $S_i = \frac{n}{2 \cdot s^{i - 1}}$. Dette gjentas til $S_i < s$, og de to siste elementene som trengs for at shellsort skal fungere, [1, 0], legges til på slutten av $S$.

```rust
    let mut S = vec![liste.len() / 2];

    while *S.last().unwrap() >= s.ceil() as usize {
        S.push((*S.last().unwrap() as f32 / s) as usize);
    }

    if *S.last().unwrap() != 1 {
        S.push(1);
    }

    S.push(0);
```

For å oppdatere $s$ gjennom sorteringen kan jeg dermed iterere gjennom $S$ heller enn å gjøre en ny utregning for hver iterasjon av sorteringen. Merk at siden et heltall av typen `usize` kreves for å indeksere en liste i Rust og $s$ er et flyttall av type `f32` må verdier omformes mellom disse to typene i løpet av beregningen.

Når delingstallene er ferdig beregnet kan de brukes i selve sorteringsløkken

```rust
    for s in S.iter() {
        for i in *s..liste.len() {
            let mut j = i;
            let temp = liste[i].clone();

            while j >= *s && liste[j - s] > temp {
                liste.swap(j, j - s);
                j -= *s;
            }

            liste[j] = temp;
        }
    }
```

For å bekrefte at algoritmen gjør som den skal, kan den kjøres gjennom noen enkle tester for om sum av listen er bevart gjennom sortering, og om listen faktisk blir sortert eller ikke.

```rust
    let mut liste = tilfeldige_heltall(15, 100);
   
    println!("Listen før sortering:\n{liste:?}");
    println!("Sum av listen før sortering: {}",
        liste.iter().sum::<u32>()
    );
    println!("Er listen sortert? {}",
        if liste.is_sorted() {"Ja"} else {"Nei"}
    );
    
    shellsort(&mut liste, 2.2);
    
    println!("Listen etter sortering:\n{liste:?}");
    println!("Sum av listen etter sortering: {}",
        liste.iter().sum::<u32>()
    );
    println!("Er listen sortert? {}",
        if liste.is_sorted() {"Ja"} else {"Nei"}
    );
```

Kjøring av denne koden gir utskrift tilsvarende følgende

```
    > ./shellsort
    Listen før sortering:
    [48, 79, 93, 30, 12, 94, 11, 32, 44, 83, 67, 63, 60, 25, 66]
    Sum av listen før sortering: 807
    Er listen sortert? Nei
    Listen etter sortering:
    [11, 12, 25, 30, 32, 44, 48, 60, 63, 66, 67, 79, 83, 93, 94]
    Sum av listen etter sortering: 807
    Er listen sortert? Ja
```

I denne testen gjøres sum-sjekk ved å bruke Rusts innebygde metoder for å summere en liste med verdier, mens metoden `liste.is_sorted()` er implementert med en enkel iterasjon over listen:

```rust
    impl IsSortedExt<u32> for Vec<u32> {
        fn is_sorted(&self) -> bool {
            let mut retval = true;

            for i in 1..self.len() {
                if self[i] < self[i - 1] {
                    retval = false;
                    break
                }
            }

            retval
        }
    }
```

# Optimalisering av $s$

For å ta tiden på sorteringer med ulike verdier for $s$ valgte jeg å generere en liste med kandidat-verdier for $s$, og for hver $s$ generere fem lister med fem millioner tilfeldige heltall. Så sorterer jeg disse fem listene og tar den totale tiden for disse fem sorteringene, og tar snittet av dem for å få en sikrere måling. Rusts innebygde `std::time` er brukt for tidtaking i alle tester.

Ved en måling av verdier for $s$ fra og med 1.1 med steg på 0.1 til og med 10.0, blir resultatene som vist i figur \ref{fig1}.

![Plott av tidsmålinger for $s$ i intervallet [1.1, 10] \label{fig1}](illustrasjoner/plot_1-10_full.png){height=30%}

For å unngå de store hoppene ved heltall for $s$ endret jeg intervallet til å ha steg på 0.2, slik at heltallsverdier for $s$ ikke blir brukt da disse virker å føre til ekstremt dårlig ytelse. Disse resultatene uten heltall kan sees i figur \ref{fig2}.

![Plott av tidsmålinger for $s$ i intervallet [1.1, 10]\label{fig2}](illustrasjoner/plot_1-10_halvt.png){height=30%}

Her ser det ut til at kjøretidene har en bunn i intervallet $[5, 7]$, og testing i dette intervallet med steg på 0.1 gir resultatene i figur \ref{fig3}

![Plott av tidsmålinger for $s$ i intervallet [5, 7]\label{fig3}](illustrasjoner/plot_5-7.png){height=30%}

Her ser det ut til at laveste kjøretid finnes mellom $s = 5.5$ og $s = 6$, og etter et par videre innsnevringer ender jeg på intervallet $s \in [5.55, 5.56]$. Resultatet av denne siste testen kan sees i figur \ref{fig4}, og viser at minste kjøretid *oppnås* med $s = 5.556$

![Plott av tidsmålinger for $s$ i intervallet [5.55, 5.561]\label{fig4}](illustrasjoner/plot_5.55-5.56.png){height=30%}

# Kompleksitetsmåling

Nå som en tilsynelatende optimal $s = 5.556$ er funnet kan kompleksiteten av shellsort med denne delingsverdien estimeres ved noen tidtakinger av varierende størrelse $n$ på datasettet som sorteres.

Ved å implementere en enkel hjelpemetode for tidtaking (se `shellsort.rs`) kan følgende kode brukes for å måle tidbruk med økende $n$. Da sortering går relativt raskt valgte jeg å tidoble $n$ for hver tidtaking, men å starte på $n = 1 000 000$ og doble en rekke ganger ville også vært en god metodikk.

Kode og utskrift fra tidtaking er som følger:

```rust
    let delingstall = 5.556;

    tidtaking(1_000_000, delingstall);
    tidtaking(10_000_000, delingstall);
    tidtaking(100_000_000, delingstall);
```
 
```
    Kjøretid med n = 1000000: 87 ms
    Kjøretid med n = 10000000: 1096 ms
    Kjøretid med n = 100000000: 13706 ms
```

Siden det antas en kompleksitet på formen $O(n^x)$ for shellsort kan målingene brukes til å estimere $x$. Når $n$ tidobles mellom kjøringer gir det følgende ligning for $x$:

$$10^x = \frac{t_2}{t_1}$$
$$10^x = \frac{1096}{87}$$
$$10^x = 12.598$$
$$x \cdot \log{10} = \log{12.598}$$
$$x = \frac{\log{12.598}}{\log{10}}$$
$$x = 1.1$$

Dette kan bekreftes ved å bruke et annet par av kjøretider til samme utregning.

$$10^x = \frac{t_2}{t_1}$$
$$10^x = \frac{13706}{1096}$$
$$10^x = 12.505$$
$$x \cdot \log{10} = \log{12.505}$$
$$x = \frac{\log{12.505}}{\log{10}}$$
$$x = 1.097$$

De to utregningene er ikke helt like, men en grad av manglende presisjon er å forvente med så mange ukontrollerte faktorer som det er med tidtaking av en algoritme på en personlig PC. Temperatur på laptopen jeg har testet på, andre bakgrunnsprosesser, hvilken kjerne av CPU prosessen ble utført på og mer til vil alt ha innvirkning på kjøretiden til en prosess og bidra til unøyaktige målinger.

Likevel kan vi med grei sikkerhet konstantere at shellsort med delingstall $s = 5.556$ har en kjøretidskompleksitet tilnærmet $O(n^{1.1})$.

Slett ikke verst!
