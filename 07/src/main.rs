#![feature(binary_heap_retain)]
mod graf;

use std::process::exit;

use graf::Graf;

pub fn main() {
    let graf = match Graf::fra_fil("vg4") {
        Ok(graf) => graf,
        Err(feil) => {
            println!("En uventet feil oppsto ved lesing av filen:\n{feil}");
            exit(1);
        }
    };
    
    println!("Dijkstras algoritme på vg4");
    graf.dijkstra(1);
}
