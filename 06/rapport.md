---
title: IDATT2101, Øving 6
author: Jakob Grønhaug (jakobkg@stud.ntnu.no)
---

# Utskrifter

```
> ./grafkomponenter ø6g1
Graf:
0 -> 2 3
1 -> 3 4 6
2 -> 0 5 3
3 -> 0 5 2 1 6
4 -> 1 6
5 -> 2 3
6 -> 3 1 4

Grafen har 1 sterkt koblede komponenter
Komponent 1: 2 3 5 1 4 6 0

> ./grafkomponenter ø6g2
Graf:
0 -> 43 41 43
1 -> 3 17 46 32
2 -> 29 47
3 -> 47

[forkortet]

48 ->
49 -> 4 40

Grafen har 24 sterkt koblede komponenter
Komponent 1: 23 
Komponent 2: 16 
Komponent 3: 10 
Komponent 4: 7 
Komponent 5: 25 
Komponent 6: 5 
Komponent 7: 0 
Komponent 8: 30 42 41 
Komponent 9: 1 
Komponent 10: 11 6 33 36 15 39 12 20 18 28 21 19 24 17
35 29 38 34 27 14 37 8 45 2 43 
Komponent 11: 31 
Komponent 12: 3 
Komponent 13: 22 
Komponent 14: 9 
Komponent 15: 44 
Komponent 16: 48 
Komponent 17: 32 
Komponent 18: 46 
Komponent 19: 26 
Komponent 20: 13 
Komponent 21: 47 
Komponent 22: 49 
Komponent 23: 40 
Komponent 24: 4

> ./grafkomponenter ø6g5
Graf:
0 -> 3 2
1 ->
2 -> 1 5 4
3 -> 5 2
4 -> 1
5 -> 1
6 -> 4 3

Grafen har 7 sterkt koblede komponenter
Komponent 1: 6 
Komponent 2: 0 
Komponent 3: 3 
Komponent 4: 2 
Komponent 5: 4 
Komponent 6: 5 
Komponent 7: 1 

> ./grafkomponenter ø6g6
Graf:
0 ->
1 -> 2 1
2 -> 7 3
3 -> 4 5
4 -> 3 5
5 ->
6 -> 3 4 5
7 -> 1 3

Grafen har 5 sterkt koblede komponenter
Komponent 1: 6 
Komponent 2: 7 2 1 
Komponent 3: 4 3 
Komponent 4: 5 
Komponent 5: 0 
```


# Implementasjonsnotater

## Selve grafen

For denne oppgaven har jeg valgt å representere grafene som lister av lister av tall, der indeksene i den første listen representerer nodene og verdiene i listen på en gitt indeks representerer kanter.

For eksempel kan grafen fra eksempelet på s.188 i boka representeres som 
```
[
    [],
    [1, 2],
    [3, 7],
    [4, 5],
    [3, 5],
    [],
    [3, 4, 5],
    [1, 3]
]
```

For enkelhets skyld har jeg pakket inn denne datastrukturen i en enkel struct:
```rust
struct Graf {
    noder: Vec<Vec<usize>>,
}
```
(Datatypen usize er her valgt da den er unsigned, og lar meg slippe å sjekke for negative verdier for noder eller kanter senere i koden)

Med denne strukturen er det veldig lett å gjøre grunnleggende operasjoner på en graf, som å legge til kanter eller å se hvilke andre noder en gitt node har kanter til.

```rust
fn legg_til_kant(&mut self, fra: usize, til: usize) {
    self.noder[fra].push(til);
}

fn finn_naboer(&self, node: usize) -> Vec<usize> {
    self.noder[node]
}
```

## Fil-lesing

Lesing av filer for å opprette grafer er stort sett veldig rett frem ved å følge prosedyren
```
les første linje av filen
    opprett en graf med korrekt antall noder

les resten av filen linje for linje
    hent ut de to tallene fra hver linje
    legg til en kant i grafen mellom de to angitte nodene
```

Det hele er pakket inn i en rekke tester for å sikre at den leste dataen er gyldig og en forsøksvis hjelpsom feilmelding kan vises om filen som leses på en eller annen måte ikke er gyldig (ikke to tall per linje, uventede node-verdier, osv.)

## Dybde først-traversering

Dybde først-traversering av grafen er implementert ved en ekstra struct som holder kontroll på hvorvidt en node er blitt besøkt enda som del av søket, og en rekursiv metode som drar nytte av denne til å gjennomføre det faktiske søket. Denne metoden gir korrekt traverseringsrekkefølge uten å faktisk bruke første/siste besøkstid som den foreslåtte algoritmen i fagboken, ved å bruke den ekstra status-structen for å holde styr på første besøk av en node og den ferdige rekkefølge-listen for å lagre en node etter at den er besøkt.

```rust
struct DfsStatus {
    besøkt: Vec<bool>,
}

impl DfsStatus {
    /// Markerer en node som besøkt
    pub fn besøk(&mut self, node: usize) -> () {
        self.besøkt[node] = true;
    }

    /// Sjekker om en node er blitt besøkt eller ikke
    pub fn har_besøkt(&self, node: usize) -> bool {
        self.besøkt[node]
    }
}

/// Hjelpemetode for dybde-først-traversering
fn dybdebesøk(&self,
    node: usize,
    status: &mut DfsStatus, 
    rekkefølge: &mut VecDeque<usize>) {
    // Se på alle naboene til noden vi har kalt funksjonen på
    for nabo in &self.noder[node] {
        // Om vi ikke allerede har besøkt en nabo
        if !status.har_besøkt(*nabo) {
            // Marker naboen som besøkt
            status.besøk(*nabo);

            // Kall denne funksjonen på naboen
            self.dybdebesøk(*nabo, status, rekkefølge);

            // Når vi er ferdige med traverseringen av naboen,
            // legg den til i fronten av rekkefølgen
            rekkefølge.push_front(*nabo);
        }
    }
}
```

