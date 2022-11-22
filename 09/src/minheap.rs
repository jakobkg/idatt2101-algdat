pub trait Vektet {
    fn vekt(&self) -> usize;

    fn sett_vekt(&mut self, vekt: usize);
}

#[derive(Debug)]
pub struct MinHeap<T: Ord + Copy + Vektet> {
    data: Vec<T>,
}

impl<T: Ord + Copy + Vektet> MinHeap<T> {
    pub fn opprett() -> Self {
        Self {
            data: Vec::new()
        }
    }

    pub fn fra_liste(data: &[T]) -> Self {
        let mut heap = Self { data: data.to_vec() };

        heap.lag_heap();

        heap
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn er_tom(&self) -> bool {
        self.len() == 0
    }

    fn over(indeks: usize) -> Option<usize> {
        if indeks == 0 {
            None
        } else {
            Some((indeks - 1) >> 1)
        }
    }

    fn venstre(indeks: usize) -> usize {
        (indeks << 1) + 1
    }

    fn høyre(indeks: usize) -> usize {
        (indeks + 1) << 1
    }

    fn fiks_heap(&mut self, indeks: usize) {
        let mut m = Self::venstre(indeks);

        if m < self.data.len() {
            let h = m + 1;

            if h < self.data.len() && self.data[h] < self.data[m] {
                m = h;
            }

            if self.data[m] < self.data[indeks] {
                self.data.swap(indeks, m);
                self.fiks_heap(m);
            }
        }
    }

    fn lag_heap(&mut self) {
        let mut i = self.data.len() / 2;
        while i > 0 {
            i -= 1;
            self.fiks_heap(i);
        }
    }

    pub fn push(&mut self, node: T) {
        self.data.push(node);

        let mut indeks = self.data.len() - 1;
        let mut forelder = Self::over(indeks);

        while forelder.is_some() && self.data[indeks].vekt() < self.data[forelder.unwrap()].vekt() {
            self.data.swap(indeks, forelder.unwrap());
            indeks = forelder.unwrap();
            forelder = Self::over(indeks);
        }

        self.fiks_heap(indeks);
    }

    pub fn pop(&mut self) -> Option<T> {
        if !self.data.is_empty() {
            let retval = self.data[0];

            self.data[0] = self.data[self.data.len() - 1];

            self.fiks_heap(0);

            self.data.remove(self.data.len() - 1);

            Some(retval)
        } else {
            None
        }
    }

    pub fn endre_vekt(&mut self, indeks: usize, vekt: usize) {
        if indeks < self.data.len() {
            if vekt > self.data[indeks].vekt() {
                self.vekt_opp(indeks, vekt);
            } else {
                self.vekt_ned(indeks, vekt);
            }
        }
    }

    pub fn finn_element(&self, element: &T) -> Option<usize>{
        if !self.data.contains(element) {
            return None
        }

        self.data.iter().position(|e| e == element)
    }

    fn vekt_opp(&mut self, indeks: usize, vekt: usize) {
        self.data[indeks].sett_vekt(vekt);

        self.fiks_heap(indeks);
    }

    fn vekt_ned(&mut self, indeks: usize, vekt: usize) {
        self.data[indeks].sett_vekt(vekt);

        let mut indeks = indeks;
        let mut forelder = Self::over(indeks);

        while forelder.is_some() && self.data[indeks].vekt() < self.data[forelder.unwrap()].vekt() {
            self.data.swap(indeks, forelder.unwrap());
            indeks = forelder.unwrap();
            forelder = Self::over(indeks);
        }
    }
}
