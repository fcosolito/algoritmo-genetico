use std::collections::HashMap;

use rand::thread_rng;
use rand::Rng;

use super::MatrizAdyacencia;
use super::Cromosoma;

pub struct Simulacion<'a> {
    ma: &'a MatrizAdyacencia,
    generaciones: usize,
    individuos: usize,
    tasa_cruza: f64,
    tasa_mutacion: f64
}

impl<'a> Simulacion<'a> {
    pub fn new(generaciones: usize, individuos: usize, tasa_cruza: f64, tasa_mutacion: f64, ma: &'a MatrizAdyacencia) -> Simulacion<'a> {
        Simulacion {
            ma,
            generaciones,
            individuos,
            tasa_cruza,
            tasa_mutacion,
        }
    }

    pub fn simular(&self) {
        let mut generacion: Vec<Cromosoma> = Vec::with_capacity(self.individuos);
        let mut mejor = Cromosoma::new();
        mejor.fitness = 1000;

        // Generacion inicial
        for _ in 0..self.individuos {
            let mut cromosoma = Cromosoma::new();
            self.fit(&mut cromosoma);
            generacion.push(cromosoma);
        }

        // Avance de generaciones
        for _ in 0..self.generaciones {
            // ordenar por fitness
            generacion.sort_by_key(|g: &Cromosoma| g.fitness as i32); // Orden ascendente
            if generacion.get(0).unwrap().fitness < mejor.fitness {
                mejor = generacion.get(0).unwrap().clone();
            }

            // seleccion 
            let mut seleccion: Vec<Cromosoma> = Vec::with_capacity(self.individuos);
            //      elitismo
            seleccion.extend_from_slice(generacion.drain(..10).as_slice());

            //      ventanas
            let mut sel_ventana = self.seleccion_ventana(&generacion, generacion.len());
            seleccion.append(&mut sel_ventana);

            // Crear una nueva generacion
            generacion.clear();
            let mut rng = thread_rng();

            // Tomar pares, cruzarlos y agregarlos a la nueva generacion
            for _ in 0..(seleccion.len()/2) {
                let c1 = seleccion.remove(rng.gen_range(0..(seleccion.len())));
                let c2 = seleccion.remove(rng.gen_range(0..(seleccion.len())));

                let (c1, c2) = if rng.gen_bool(self.tasa_cruza) {
                    // cruzar
                    //let (mut h1, mut h2) = self.cruza_1p(&c1, &c2, 10);
                    let mascara: Vec<bool> = [0,1,1,1,0,1,0,1,1,0,1,1,1,1,0,1,1,1,1,0,1,1,1].iter()
                            .map(|n|{
                                if *n == 0 {
                                    false
                                } else {
                                    true
                                }
                            }).collect();
                    let (mut h1, mut h2) = self.cruza_mascara(&c1, &c2, &mascara[..]);
                    // mutar
                    let mutacion = rng.gen_bool(self.tasa_mutacion);
                    let cantidad_colores = 25;
                    if mutacion {
                        let indice = rng.gen_range(0..23);
                        let color = rng.gen_range(1..=cantidad_colores);
                        let indice2 = rng.gen_range(0..23);
                        let indice3 = rng.gen_range(0..23);
                        let color2 = rng.gen_range(1..=cantidad_colores);

                        h1.genes[indice] = color;
                        h1.genes[indice2] = color;
                        h1.genes[indice3] = color;
                        h2.genes[indice] = color2;
                        h2.genes[indice2] = color2;
                        h2.genes[indice3] = color2;
                    }
                    (h1, h2)
                } else {
                    (c1, c2)
                };
                generacion.push(c1);
                generacion.push(c2);
            }

            // Calcular fitness
            for c in &mut generacion {
                self.fit(c);
            }
        }

        // Resultado
        println!("Resultado");
        generacion.sort_by_key(|g: &Cromosoma| g.fitness as i32); // Orden ascendente
        for c in &generacion[..10] {
            let colores = c.genes.iter()
                .copied()
                .fold(HashMap::new(), |mut acc, color|{
                    acc.insert(color, 0);
                    acc
                });
            println!("{:?}, colores: {}", c, colores.len());
        }
        println!("El mejor fue: {:?}", mejor);
    }

    pub fn cruza_1p(&self, p1: &Cromosoma, p2: &Cromosoma, p: usize) -> (Cromosoma, Cromosoma) {
        let mut h1 = Cromosoma::new();
        let mut h2 = Cromosoma::new();
        
        h1.genes = [&p1.genes[..p], &p2.genes[p..]].concat().to_vec();
        h2.genes = [&p2.genes[..p], &p1.genes[p..]].concat().to_vec();

        (h1, h2)
    }

    pub fn cruza_mascara(&self, p1: &Cromosoma, p2: &Cromosoma, mascara: &[bool]) -> (Cromosoma, Cromosoma) {
        let mut h1 = Cromosoma::new();
        let mut h2 = Cromosoma::new();
        h1.genes.clear();
        h2.genes.clear();

        for (i,b) in mascara.iter().enumerate() {
            if *b {
                h1.genes.push(p1.genes[i].clone());
                h2.genes.push(p2.genes[i].clone());
            } else {
                h2.genes.push(p1.genes[i].clone());
                h1.genes.push(p2.genes[i].clone());
            }
        }

        (h1, h2)
    }


    pub fn seleccion_ventana(&self, generacion: &[Cromosoma], e: usize) -> Vec<Cromosoma> {
        // El argumento generacion tiene que estar ordenado 
        // de forma que los mejores individuos se ubiquen
        // al inicio del arreglo (orden ascendente).
        if generacion.is_empty() { return vec![] }

        let mut rng = thread_rng();
        let last = generacion.len();

        let mut v: Vec<Cromosoma> = Vec::with_capacity(e);

        for i in 1..=e {
            let end = if i < last { i } else { last };
            let index = rng.gen_range(0..end);
            
            v.push(generacion[index].clone());
        }
        v
    }

    pub fn fit(&self, cromosoma: &mut Cromosoma) -> usize {
            let mut result = 0;
            let colores_vistos = cromosoma.genes.iter()
                .copied()
                .fold(HashMap::new(), |mut acc, color|{
                    acc.insert(color, 0);
                    acc
                });
            result += colores_vistos.len();
            result += self.errores(cromosoma) * 100;

            cromosoma.fitness = result;
            result
        }

    fn errores(&self, cromosoma: &Cromosoma) -> usize {
        let mut errores = 0;
        for (i, row) in (self.ma.0).iter().enumerate() {
            for provincia in row {
                let index = provincia -1;
                if cromosoma.genes.get(i).unwrap() == cromosoma.genes.get(index).unwrap() {
                    errores += 1;
                }
            }
        }
        errores

    }
}
