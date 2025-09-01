use std::process;

use crate::model::Simulacion;

mod model;
fn main() {
    let result = model::MatrizAdyacencia::new("adyacencias.csv");
    
    match result {
        Err(err) => {
            println!("{:?}", err);
            process::exit(1);
        },
        Ok(matriz) => {
            let simulacion = Simulacion::new(20000, 200, 0.7, 0.7, &matriz);
            let mejor = simulacion.simular();

            println!("El mejor fue: {:?}", mejor);
            // Esto tiro hace un rato:
            // El mejor fue: Cromosoma { genes: [11, 17, 17, 25, 4, 3, 4, 11, 11, 25, 25, 3, 25, 11, 4, 25, 11, 17, 3, 25, 11, 11, 3], fitness: 5 }


        }
    }
}

