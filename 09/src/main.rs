mod graf;
mod minheap;
use std::process::exit;

use graf::Graf;
use minheap::{Vektet};

impl Vektet for usize {
    fn vekt(&self) -> usize {
        *self
    }

    fn sett_vekt(&mut self, vekt: usize) {
        *self = vekt
    }
}

fn main() {
    let mut graf = match Graf::fra_filer("noder.txt", "kanter.txt") {
        Ok(graf) => graf,
        Err(grunn) => {
            println!("{grunn}");
            exit(1);
        }
    };

    match graf.finn_vei(3430400, 7425499) {
        Ok(_) => {},
        Err(e) => {println!("{e}")},
    }
}
