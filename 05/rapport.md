---
title: IDAT2101, Øving 5
author: Jakob Grønhaug (jakobkg@stud.ntnu.no)
---

# Oppg 1

Oppgave 1 er implementert med en hash-algoritme som tar hver bokstav i en gitt streng og vekter den med $5^{n-i}$ der $i$ er plasseringen til bokstaven i strengen og $n$ er lengden på strengen. På denne måten sikres det at to strenger med samme bokstaver og lengde fortsatt får forskjellig hash.

For eksempel:
\begin{minipage}{.5\linewidth}
    $$H_1(\text{'ole'}) = \text{'o'} \cdot 5^2 + \text{'l'} \cdot 5^1 + \text{'e'} \cdot 5^0$$
    $$H_1(\text{'ole'}) = 111 \cdot 25 + 108 \cdot 5 + 101$$
    $$H_1(\text{'ole'}) = 3416$$
\end{minipage}
\begin{minipage}{.5\linewidth}
    $$H_1(\text{'leo'}) = \text{'l'} \cdot 5^2 + \text{'e'} \cdot 5^1 + \text{'o'} \cdot 5^0$$
    $$H_1(\text{'leo'}) = 108 \cdot 25 + 101 \cdot 5 + 111$$
    $$H_1(\text{'leo'}) = 3316$$
\end{minipage}

Denne algoritmen kan likevel ikke garantere at vi unngår kollisjoner, da hashene må begrenses til kapasiteten til hashtabellen for å kunne utføre innsetting. I en tabell med kapasitet på 100 elementer vil faktisk plassering av disse to være gitt ved

\begin{minipage}{.5\linewidth}
    $$Pos(\text{'ole'}) = H_1(\text{'ole'}) \bmod{100}$$
    $$Pos(\text{'ole'}) = 3416 \bmod{100}$$
    $$Pos(\text{'ole'}) = 16$$
\end{minipage}
\begin{minipage}{.5\linewidth}
    $$Pos(\text{'leo'}) = H_1(\text{'leo'}) \bmod{100}$$
    $$Pos(\text{'leo'}) = 3316 \bmod{100}$$
    $$Pos(\text{'leo'}) = 16$$
\end{minipage}

Som vi ser kan selv ulike hasher føre til kollisjoner, men ved lure valg av vekttall og kapasitet på hashtabellen kan antall kollisjoner holdes til et minimum. I min implementasjon av oppgaven har jeg valgt kapasitet 127 (første primtall etter 114, som er antallet studenter i navnelisten).

Den ferdige implementasjonen tar inn filnavnet til tekstfilen med student navn som et valgfritt argument, om ingen argumenter angis ser programmet etter en fil med filnavn `"navn"` (ingen fil-ending) i mappen programmet ble startet fra.

Kjøring av programmet skal gi følgende utskrift:

```
> ./hashtabell 
Oppgave 1:
Håkon Sørli kolliderte med Magnus Fikse Forbord,
Thomas Svendal kolliderte med Katarzyna Szlejter,
Gunnar Antoni Solli Olsen kolliderte med Sander Olin Arild Johansen,
Hadar Hayat kolliderte med Håkon Sørli, Magnus Fikse Forbord,
Nils William Ormestad Lie kolliderte med Kristoffer Svedal,
Ådne Tøftum Svendsrud kolliderte med Nicolai Hollup Brand,
Eric Bieszczad-Stie kolliderte med Daniel André Vestly Evensen,
Aurora Schmidt-Brekken Vollvik kolliderte med Pedro Pablo Cardona Arroyave,
Eirik Leer Talstad kolliderte med Camilla Kristiansen Birkelund,
[...]
Ingrid Flatland kolliderte med Edvard Sørby,
Steinar Nilsskog kolliderte med Jarand Jensen Romestrand,
Aleksander Halvorsen Holthe kolliderte med Håkon Henriksen Juell,

Jeg (Jakob Karevold Grønhaug) er i hashtabellen: true
35 kollisjoner på 114 innsettinger
Kollisjoner per person: 0.30701753
Lasttall: 0.8976378
```

# Oppg 2

I oppgave 2 har jeg valgt å generalisere implemetasjonen noe, og tillater andre kapasiteter enn spesifikt ti millioner tall. Ellers er implementasjonen i stor grad basert på forelesningsfoilene, og følger en antagelse om at kapasiteten er et primtall for å unngå kollisjoner i størst mulig grad.

For å implementere dobbel hashing med antagelsen om at kapasiteten $k$ er et primtall og dataen $x$ som skal settes inn har stor spredning er følgende hash-algoritmer valgt:

$$H_1(x) = x \bmod{k},\quad H_2(x) = (x \bmod{k - 1}) + 1$$

I den ferdige implementasjonen er dataen som skal settes inn tilfeldig genererte 64-bits heltall som sikrer god spredning, og kapasiteten $k$ er satt til 13000027, det første primtallet etter 13 millioner. Dette tallet er valgt for å sikre god overhead og redusere kollisjoner, men ikke overdrevent mye.

Tidtaking av innsetting er satt opp for å sikre at kun innsettingen måles:

```rust
    let mut hashtabell: HashTabell2 = HashTabell2::new(13_000_027);

    let tall_liste = tilfeldige_heltall(10_000_000, usize::MAX);

    let mut start: Instant = Instant::now();
    for tall in &tall_liste {
        hashtabell.sett_inn(*tall);
    }

    let mut tid: Duration = Instant::now() - start;
```

Rusts innebygde `HashMap<K, V>` er målt på samme måte.

Kjøring av denne implementasjonen gir følgende utskrift:

```
Oppgave 2:
Satte inn på 0.4367959 sekunder, med 9057839 kollisjoner
(0.9057839 kollisjoner per innsetting)
Lasttall: 0.7692291716009513
Innebygd hashtabell gjorde samme innsettinger på 0.6217706 sekunder
```
