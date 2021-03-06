use crate::AtomBasis;

use std::fs::{read_dir, read_to_string};

use once_cell::sync::Lazy;
use regex::Regex;

use ndarray::Array2;

fn load_basis_file(basis_name: &str) -> String {
    let path = format!("{}/ao_basis/{basis_name}.nw", env!("OUT_DIR"));
    read_to_string(path).unwrap()
}

pub fn basis_names() -> impl Iterator<Item = String> {
    read_dir(format!("{}/ao_basis", env!("OUT_DIR")))
        .unwrap()
        .map(|entry| {
            let entry = entry.unwrap();
            let filename = entry.file_name();
            let filename = filename.to_str().unwrap();
            filename.split('.').next().unwrap().to_owned()
        })
}

fn get_angular_momentum(c: char) -> i32 {
    match c {
        'S' => 0,
        'P' => 1,
        'D' => 2,
        'F' => 3,
        'G' => 4,
        'H' => 5,
        'I' => 6,
        _ => unimplemented!(),
    }
}

fn parse_basis(basis_file: &str) -> Vec<(String, AtomBasis)> {
    static REG1: Lazy<Regex> = Lazy::new(|| {
        Regex::new(
            r"BASIS SET: \((.+?)\) -> \[(.+?)\]\n((:?.|\n)+?)(:?#|(:?END))",
        )
        .unwrap()
    });

    static REG2: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"([A-Z][a-z]?) +([A-Z])((?:\n(?: +-?\d+\.\d+(?:E(?:\+|-)\d+)?)+ *)+)").unwrap()
    });

    static REG3: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"-?\d+\.\d+(?:E(?:\+|-)\d+)?").unwrap());

    REG1.captures_iter(basis_file)
        .map(|c| {
            let mut atom = None;
            let v = REG2
                .captures_iter(&c[3])
                .map(|c| {
                    let mut h = 0;
                    let mut buf: Vec<f64> = Vec::new();

                    for line in c[3].split('\n').skip(1) {
                        h += 1;
                        for c in REG3.captures_iter(line) {
                            buf.push(c[0].parse().unwrap());
                        }
                    }

                    let w = buf.len() / h;

                    if atom.is_none() {
                        atom = Some(c[1].to_owned());
                    }

                    (
                        get_angular_momentum(c[2].chars().next().unwrap()),
                        Array2::from_shape_vec((h, w), buf).unwrap(),
                    )
                })
                .collect();

            (atom.unwrap(), AtomBasis { basis: v })
        })
        .collect()
}

pub fn get_basis(basis_name: &str) -> Vec<(String, AtomBasis)> {
    parse_basis(&load_basis_file(basis_name))
}
