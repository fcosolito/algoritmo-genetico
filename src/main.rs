use std::process;

use crate::model::{Cromosoma, Simulacion};
use csv::Writer;
use std::error::Error;

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

            escribir_solucion(&mejor)

        }
    }
}

fn escribir_solucion(cromosoma: &Cromosoma)  {
    let mut wtr = Writer::from_path("solucion.csv").unwrap();

    // Escribir encabezado
    wtr.write_record(&["id", "color"]).unwrap();

    for (i, color) in cromosoma.genes.iter().enumerate() {
        wtr.write_record(&[&(i+1).to_string(), &color.to_string()]).unwrap();
    }

    wtr.flush().unwrap();
}