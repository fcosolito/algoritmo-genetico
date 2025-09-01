use std::error::Error;
use csv::Reader;

#[derive(Debug)]
pub struct MatrizAdyacencia(pub Vec<Vec<usize>>);

impl MatrizAdyacencia {
    pub fn new(path: &str) -> Result<MatrizAdyacencia, Box<dyn Error>> {
        let mut v = vec![];
        let mut reader = Reader::from_path(path).unwrap();
        for result in reader.records() {
            match result {
                Err(err) => return Err(From::from(err)),
                Ok(record) => {
                    let mut row = vec![];
                    for i in 1..record.len() {
                        match record.get(i) {
                            Some(field) => {
                                match field.parse::<usize>() {
                                    Ok(adyacente) => row.push(adyacente),
                                    Err(_) => {},
                                }
                            },
                            None => {},
                        }

                    }
                    v.push(row);
                }
            }
        }
        Ok(MatrizAdyacencia(v))
    }
}