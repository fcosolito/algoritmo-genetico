use rand::prelude::*;

pub struct BolsaColores {
    colores: Vec<u8>,
    rng: ThreadRng,
}

impl BolsaColores {
    pub fn new(colores: u8) -> BolsaColores {
        BolsaColores { 
            colores: (1..=colores).collect(), 
            rng: thread_rng(),
        }
    }

    pub fn take(&mut self) -> Option<u8> {
        self.colores.shuffle(&mut self.rng);
        self.colores.pop()
    }
}