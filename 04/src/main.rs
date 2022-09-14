struct Siffer {
    verdi: u32,
    forrige: Option<*mut Siffer>,
    neste: Option<*mut Siffer>,
}

impl Siffer {
    pub fn nytt(verdi: u32) -> Option<Self> {
        if verdi > 9 {
            None
        } else {
            Some(
                Self {
                    verdi,
                    forrige: None,
                    neste: None
                }
            )
        }
    }
}

fn main() {}
