fn finn_lengste_treff(data: &[u8], posisjon: usize) -> (i16, u8) {
    let mut beste_hopp = 0i16;
    let mut beste_lengde = 0u8;

    let start = if posisjon > 255 { posisjon - 255 } else { 0 };

    for hopp in start..posisjon {
        let len = matcher(data, hopp, posisjon);

        if len > beste_lengde {
            beste_hopp = (posisjon - (hopp as usize)) as i16;
            beste_lengde = len;
        }
    }

    (-beste_hopp, beste_lengde)
}

fn matcher(data: &[u8], hopp: usize, slutt: usize) -> u8 {
    let mut hopp = hopp;
    let mut posisjon = slutt;
    let mut lengde = 0u8;

    while hopp < posisjon
        && posisjon < data.len()
        && data[hopp] == data[posisjon]
        && lengde < 255
        && hopp < slutt
    {
        hopp += 1;
        posisjon += 1;
        lengde += 1;
    }

    lengde
}

pub fn komprimer(data: &[u8]) -> Vec<u8> {
    let mut komprimert = Vec::new();
    let mut posisjon = 0;

    let mut ukomprimert: Vec<u8> = Vec::new();

    while posisjon < data.len() {
        let (mut hopp, mut lengde) = finn_lengste_treff(data, posisjon);

        // Implementasjonen bruker tre byte med header for en komprimert blokk, så om et mønster ikke er
        // større enn dette er det ikke vits i å sette inn en header, og en ukomprimert blokk bygges
        if lengde < 4 {
            ukomprimert.push(data[posisjon]);
            posisjon += 1;

            (hopp, lengde) = finn_lengste_treff(data, posisjon);

            while lengde < 4 && posisjon < data.len() {
                ukomprimert.push(data[posisjon]);
                posisjon += 1;

                (hopp, lengde) = finn_lengste_treff(data, posisjon);
            }

            // Når den ukomprimerte blokken er ferdig, sett inn lengden på den på begynnelsen
            (ukomprimert.len() as i16)
                .to_be_bytes()
                .iter()
                .for_each(|&byte| komprimert.push(byte));

            komprimert.append(&mut ukomprimert);
        }

        if lengde > 3 {
            hopp.to_be_bytes()
                .iter()
                .for_each(|&byte| komprimert.push(byte));

            komprimert.push(lengde);
        }

        posisjon += lengde as usize;
    }

    komprimert
}

pub fn dekomprimer(data: &[u8]) -> Result<Vec<u8>, String> {
    let mut dekomprimert: Vec<u8> = Vec::new();

    let mut posisjon: usize = 0;
    let mut header;
    let mut hopp;
    let mut headerbytes: [u8; 2] = [0, 0];
    let mut byte: u8;

    loop {
        // Sjekk om det fremdeles er data igjen i input, bryt løkken om det ikke er det
        match data.get(posisjon) {
            Some(_) => {}
            None => break,
        }

        for i in 0..2 {
            byte = get_og_iterer!(data, posisjon);
            headerbytes[i] = byte;
        }

        header = i16::from_be_bytes(headerbytes);

        if header > 0 {
            // Kopier ukomprimert blokk direkte til output
            for _ in 0..header {
                dekomprimert.push(get_og_iterer!(data, posisjon));
            }
        } else {
            // Vi er i en komprimert blokk!
            let blokklengde = get_og_iterer!(data, posisjon) as usize;

            hopp = header.abs() as usize;

            for _ in 0..blokklengde {
                dekomprimert.push(dekomprimert[dekomprimert.len() - hopp]);
            }
        }
    }

    Ok(dekomprimert)
}
