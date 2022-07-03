pub mod basisparser;

use std::{fs::File, io::Write, path::Path};

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

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_parse_basis() {
        let bas = basisparser::get_basis("cc-pvdz");

        assert_eq!(bas[2].0, "Li");
    }
}
