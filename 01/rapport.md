---
title: IDATT2101 Øving 2
author: Jakob Grønhaug (jakobkg@stud.ntnu.no)
---

# IDATT2021 Øving 1
Jakob Grønhaug (jakobkg@stud.ntnu.no)

## Algoritme

Algoritmen som er implementert for denne oppgaven er basert på å analysere differanse mellom relative aksje-priser, og kan beskrives som følger:

    Gitt en liste med kursendringer per dag
    For hver dag (i fra 1 til n)
        Iterer gjennom alle påfølgende dager (j fra i+1 til n)
            Beregn pris-differansen mellom dag i og dag j
            Hvis dette er en ny største differanse,
                noter dagene for kjøp (i) og salg (j), og differansen

I denne algoritmen vil den ytterste løkken utføres n ganger, og den innerste løkken vil utføres mellom n-1 og 1 ganger basert på hvor mange dager det er igjen i listen over kursendringer (n - i). Dette gir en teoretisk kompleksitet på O(n\^2).

## Implementasjon

En implementasjon av algoritmen i Rust medfølger i filen "aksjer.rs", og et skript for testing av dette i filen "benchmark". Skriptet "benchmark" er basert på bash, og er kun testet i Linux-miljø. Kompilering av implementasjonen krever en installert Rust-kompilator (rustc). En forhånds-kompilert kjørbar programfil medfølger for enkelhets skyld, kalt "aksjer". Denne er kompilert for Linux-miljø, og ikke testet i andre miljøer.

Implementasjonen i "aksjer.rs" inneholder også generering av tilfeldige kursendringer i intervallet \[-10, 10\] som brukes som test-data for algoritmen, tidtaking av algoritmen og enkel presentasjon av resultatet av algoritmen og tidtakingen til bruker. Den overordnede strukturen til implementasjonen er

    Generer n tilfeldige kursendringer
    Klargjør tidtaking
    Utfør algoritmen på de genererte kursendringene
    Avslutt tidtaking
    Presenter resultat

Mengden test-data som genereres angis av brukeren ved kjøring av programmet ved å oppgi antallet datapunkter som argument. For eksempel kan 1000 datapunkter genereres ved å kjøre `./aksjer 1000`, som vil gi resultat som ligner følgende:

    $ ./aksjer 1000
    Optimal trade: Buy on day 689, then sell on day 803.
    Gain: 158, time spent [µs]: 268

Om det er ønskelig å se test-dataen som er generert for å manuelt verifisere at algoritmen har ført til ønsket resultat kan man tilføye argumentet `-s`. Om man for eksempel vil ha et lite datasett på 10 dager for å verifisere algoritmen kan man bruke `./aksjer 10 -s`. Dette vil gi utskrift som ligner følgende:

    $ ./aksjer 10 -s
    [3, -9, -6, 8, 4, -5, 1, -6, 4, -7]
    Optimal trade: Buy on day 3, then sell on day 5.
    Gain: 12, time spent [µs]: 0

## Testing

For tidtaking av programmet er Rusts innebygde tid-modul brukt. Tidtaking startes etter at alle nødvendige variable er deklarert men før selve algoritmen startes, og avsluttes etter at algoritmen er ferdig men før resultatet presenteres. På denne måten sikres det at programmet ikke tar allokering av minne eller skriving til stdout med i tidtakingen.

Testing har vist at mikrosekunder \[µs\] er en passende tidsenhet for måling av kjøretid på egen maskin. For enkelhets skyld medfølger et lite bash-skript, "benchmark", som kjører programmet 100 ganger med n = 1000 og deretter 100 ganger med n = 10000 og beregner gjennomsnittlig kjøretid for testene. Et eksempel på utskrift av dette:

    $ ./benchmark
    No compiled binary found, compiling...
    Compilation succeeded!

    Running 100 tests with 1000 data points
    Average time: 131 µs

    Running 100 tests with 10000 data points
    Average time: 11551 µs

Dette underbygger tidligere analyse som tilsa at algoritmen har en kompleksitet på O(n\^2), da en ti-dobling av datamengde fører til omtrent en hundre-dobling av kjøretid.
