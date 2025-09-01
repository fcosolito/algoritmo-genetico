use std::process;

use crate::model::{Cromosoma, MatrizAdyacencia, Simulacion};

mod model;
fn main() {
    let result = model::MatrizAdyacencia::new("adyacencias.csv");
    
    match result {
        Err(err) => {
            println!("{:?}", err);
            process::exit(1);
        },
        Ok(matriz) => {
            println!("{:?}", matriz);
            let simulacion = Simulacion::new(300, 200, 0.9, 1.0, &matriz);
            //let mut cromosoma = Cromosoma::new();
            //let fit = simulacion.fit(&mut cromosoma);
            //println!("Fit de {:?}: {}", &cromosoma, fit);
            simulacion.simular();
        }
    }
}

