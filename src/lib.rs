pub mod basisparser;

use std::{
    collections::HashMap,
    fs::{self, read_dir, File},
    io::Write,
    path::Path,
};

use ndarray::Array2;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AtomBasis {
    pub basis: Vec<(i32, Array2<f64>)>,
}

pub fn save_basis(path: &str, basis: &[(String, AtomBasis)]) {
    for (atom, basis) in basis {
        let serialized = bincode::serialize(basis).unwrap();

        let path = Path::new(path).join(atom);

        let mut file = File::create(path).unwrap();
        file.write_all(&serialized).unwrap();
    }
}

fn deserialize_basis<P: AsRef<Path>>(filepath: P) -> AtomBasis {
    let serialized = fs::read(filepath).unwrap();

    bincode::deserialize(&serialized).unwrap()
}

pub struct LazyAtomBasis {
    basis: Option<AtomBasis>,
    filepath: String,
}

impl LazyAtomBasis {
    pub fn get(&mut self) -> &[(i32, Array2<f64>)] {
        if self.basis.is_none() {
            self.basis = Some(deserialize_basis(&self.filepath));
        }

        &self.basis.as_ref().unwrap().basis
    }
}

pub struct LazyBasis {
    basis: HashMap<String, LazyAtomBasis>,
}

impl LazyBasis {
    pub fn get(&mut self, atom: &str) -> &[(i32, Array2<f64>)] {
        if let Some(basis) = self.basis.get_mut(atom) {
            basis.get()
        } else {
            panic!("Atom \"{atom}\" is not available in basis!");
        }
    }
}

pub fn load_basis(path: &str) -> LazyBasis {
    LazyBasis {
        basis: read_dir(path)
            .unwrap()
            .map(|atom| {
                let entry = atom.unwrap();
                let atom = entry.file_name().into_string().unwrap();
                let filepath = entry.path().to_str().unwrap().to_owned();
                let basis = LazyAtomBasis {
                    basis: None,
                    filepath,
                };

                (atom, basis)
            })
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_parse_basis() {
        let bas = basisparser::get_basis("cc-pvdz");

        assert_eq!(bas[2].0, "Li");
    }

    #[test]
    fn test_basis_names() {
        for name in basisparser::basis_names() {
            println!("basis: {}", name);
        }
    }
}
