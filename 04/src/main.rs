struct Node {
    verdi: u8,
    forrige: Option<Box<Node>>,
    neste: Option<Box<Node>>,
}

impl Node {
    pub fn new(verdi: u8, forrige: Option<Box<Node>>, neste: Option<Box<Node>>) -> Self {
        Self {
            verdi,
            forrige,
            neste
        }
    }

    pub fn new_tail(verdi: u8, forrige: Option<Box<Node>>) -> Self {
        Self {
            verdi,
            forrige,
            neste: None
        }
    }

    pub fn new_head(verdi: u8, neste: Option<Box<Node>>) -> Self {
        Self {
            verdi, 
            forrige: None,
            neste
        }
    }
}

fn main() {
    println!("Hello, world!");
}
