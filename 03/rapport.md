---
author: Jakob Grønhaug
title: IDATT2101, Øving 3
---

# Shellsort

I denne øvingen var det mulig å gjøre oppgaver om forskjellige sorteringsalgoritmer, enten quicksort med enkel og dobbel pivot eller shellsort. Jeg har valgt å gjøre oppgaven om shellsort. Shellsort er en forbedring av en enkel insertion sort, der man sammenligner tall som er lengre unna hverandre og bytter plass på dem om nødvendig, og så ser på tall nærmere og nærmere hverandre i listen. På denne måten blir listen først grovt sortert, og så sortert finere og finere til det til sist gjøres et enkelt pass med det som i praksis er insertion sort når nabotall sammenlignes.

# Implementasjon

Min implementasjon av shellsort i Rust medfølger i filen `shellsort.rs`. Denne implementasjonen avviker fra eksempel-implemetasjonen fra fagboken i at jeg har valgt å på forhånd beregne delingstallene før selve sorteringen starter, heller enn å beregne delingstallene underveis i sorteringen. Dette er implementert ved å opprette en vektor $S$ som kun inneholder en verdi $\frac{n}{2}$, hvor $n$ er antall elementer i dataen som skal sorteres. Denne vektoren fylles så opp med verdier som igjen og igjen deles på delingstallet $s$, slik at $S_i = \frac{n}{2 \cdot s^{i - 1}}$. Dette gjentas til $S_i < s$, og de to siste elementene som trengs for at shellsort skal fungere, $[1, 0]$, legges til på slutten av $S$.

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
