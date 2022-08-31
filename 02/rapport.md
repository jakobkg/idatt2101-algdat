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

Algoritme 2, fra bok-oppgave 2.2-3, er en mer kompleks algoritme der hver nye rekursjon kjøres med $\frac{n}{2}$, en rekursiv kompleksitet $T(n)=T(\frac{n}{2})+1$. Dette er ikke like lett å analysere som algoritme 1, men siden den rekursive kompleksiteten til algoritmen tilfredsstiller formen $T(n) = a \cdot T(\frac{n}{b}) + c \cdot n^k$ kan master-metoden brukes. Kompleksiteten $T(n)=T(\frac{n}{2})+1$ har koeffisienter $a = 1, b = 2, c = 1$ og $k = 0$. I dette tilfellet er $b^k = 2^0 = 1$ som gir $b^k = a$, og master-metoden forteller at dette gir en kompleksitet $T(n) \in \Theta(\log n)$.

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
