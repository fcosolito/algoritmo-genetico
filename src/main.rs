use std::process;

use crate::model::{MatrizAdyacencia, Simulacion};

mod model;
fn main() {
    let result = model::MatrizAdyacencia::new("adyacencias.csv");
    
    match result {
        Err(err) => {
            println!("{:?}", err);
            process::exit(1);
        },
        Ok(matriz) => {
            let simulacion = Simulacion::new(300, 200, 0.9, 1.0, &matriz);
            simulacion.simular();
        }
    }
}

