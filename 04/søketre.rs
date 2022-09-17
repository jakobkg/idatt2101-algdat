use std::{cmp::Ordering, collections::VecDeque, fmt::Display};

// En node er trivielt kopierbar
#[derive(Clone)]
/// Enum som representerer en nodes to tilstander, fylt eller tom
enum Node<T: Ord + Display + Clone> {
    /// En faktisk node med en verdi og to under-noder
    Node {
        verdi: T,
        /// Venstre under-node, vil alltid inneholde en verdi som er mindre enn denne nodens verdi
        venstre: Box<Node<T>>,
        /// Høyre under-node, vil alltid inneholde en verdi som er større enn denne nodens verdi
        høyre: Box<Node<T>>,
    },
    /// En tom node
    Tom,
}

impl<T: Ord + Display + Clone> Node<T> {
    /// Oppretter en ny, tom node
    pub fn ny_tom() -> Self {
        Self::Tom
    }

    /// Oppretter en ny node med gitt verdi, og tomme under-noder
    pub fn ny_node(verdi: T) -> Self {
        Self::Node {
            verdi,
            venstre: Box::new(Self::Tom),
            høyre: Box::new(Self::Tom),
        }
    }

    /// Setter inn en ny node med angitt verdi i treet under den angitte noden
    pub fn sett_inn(&mut self, ny_verdi: T) -> () {
        match self {
            Self::Node {
                verdi,
                venstre,
                høyre,
            } => match ny_verdi.cmp(verdi) {
                Ordering::Less => venstre.sett_inn(ny_verdi),
                Ordering::Greater => høyre.sett_inn(ny_verdi),
                Ordering::Equal => return,
            },
            Self::Tom => *self = Node::ny_node(ny_verdi),
        }
    }

    /// Returnerer en Vec<T> med verdiene fra treet ferdig sortert
    pub fn som_liste(&self) -> Vec<T> {
        let mut buffer = Vec::<T>::new();

        match self {
            Self::Tom => {}
            Self::Node {
                verdi,
                venstre,
                høyre,
            } => {
                buffer.append(&mut venstre.som_liste());
                buffer.push(verdi.clone());
                buffer.append(&mut høyre.som_liste());
            }
        }

        buffer
    }

    /// Genererer en riktig fjong utskrift av treet, med total bredde som angitt
    pub fn vis(self, bredde: usize) -> () {
        let mut kø = VecDeque::<Node<T>>::new();
        let mut node: Node<T>;

        let mut antall_noder: usize;
        let mut print_bredde = bredde;

        kø.push_back(self.clone());

        while !kø.is_empty() {
            antall_noder = kø.len();

            while antall_noder > 0 {
                node = kø.pop_front().unwrap();

                match node {
                    Node::Tom => {print!("{:^print_bredde$}", "");}
                    Node::Node {
                        verdi,
                        ref venstre,
                        ref høyre,
                    } => {
                        print!("{:^print_bredde$}", verdi);
                        kø.push_back(*venstre.clone());
                        kø.push_back(*høyre.clone());
                    }
                }

                antall_noder -= 1;
            }

            println!();
            print_bredde = print_bredde / 2;
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let mut tre = Node::<String>::ny_tom();

    for arg in args {
        tre.sett_inn(arg.to_string().to_lowercase());
    }

    println!("Sortert: {:?}", tre.som_liste());
    println!("Treet:");
    tre.vis(64);
}
