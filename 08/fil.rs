use std::{
    fs::File,
    io::{Read, Write},
};

pub fn les_bytes(filsti: &str) -> Result<Vec<u8>, String> {
    // Les fil som Vec av bytes
    let mut f = match File::open(filsti) {
        Ok(handle) => handle,
        Err(e) => {
            return Err(format!(
                "Kunne ikke åpne filen {filsti}\nFeilmelding: \"{e}\""
            ));
        }
    };

    let mut data: Vec<u8> = Vec::new();

    match f.read_to_end(&mut data) {
        Ok(n) => {
            println!("Leste {n} bytes fra {filsti}");
        }
        Err(e) => {
            return Err(format!(
                "Kunne ikke lese fra filen {filsti}\nFeilmelding: \"{e}\""
            ));
        }
    }

    Ok(data)
}

pub fn skriv_bytes(filsti: &str, data: &[u8]) -> Result<(), String> {
    // Skriv resultatet av komprimeringen ut til en ny fil
    let mut f = match File::create(filsti) {
        Ok(handle) => handle,
        Err(e) => {
            return Err(format!(
                "Kunne ikke åpne {filsti} for å skrive\nFeilmelding: \"{e}\""
            ));
        }
    };

    match f.write_all(&data) {
        Ok(_) => {
            println!("Skrev {} bytes til {filsti}", data.len());
        }
        Err(e) => {
            return Err(format!(
                "Kunne ikke skrive til {filsti}\nFeilmelding: \"{e}\""
            ));
        }
    }

    Ok(())
}
