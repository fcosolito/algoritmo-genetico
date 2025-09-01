use super::BolsaColores;

#[derive(Debug)]
#[derive(Clone)]
pub struct Cromosoma {
    pub genes: Vec<u8>,
    pub fitness: usize,
}

impl Cromosoma {
    pub fn new() -> Cromosoma {
        let mut genes: Vec<u8> = vec![];
        let mut colores = BolsaColores::new(25);
        for _ in 0..23 {
            genes.push(colores.take().unwrap())
        }
        let cromosoma = Cromosoma {
            genes,
            fitness: 0,
        };
        cromosoma
    }
}