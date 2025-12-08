use std::{cmp, collections::HashSet, fs};

fn main() {
    let input: Vec<Coordinate> = fs::read_to_string("../input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let coords: Vec<usize> = line.split(',').map(|s| s.parse().unwrap()).collect();
            Coordinate {
                x: coords[0],
                y: coords[1],
                z: coords[2],
            }
        })
        .collect();

    let mut pairs: Vec<(Coordinate, Coordinate, f64)> = input
        .iter()
        .enumerate()
        .flat_map(|(i, &coord1)| {
            input[(i + 1)..]
                .iter()
                .map(move |&coord2| (coord1, coord2, coord1.distance_to(&coord2)))
        })
        .collect();

    pairs.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    let mut circuits: Vec<HashSet<Coordinate>> = Vec::new();

    for i in 0..pairs.len() {
        let (jb1, jb2, _) = &pairs[i];

        let jb1_circuit_idx = circuits.iter().position(|circuit| circuit.contains(jb1));
        let jb2_circuit_idx = circuits.iter().position(|circuit| circuit.contains(jb2));

        let mut merge_into = |circuit_idx: usize, new: &Coordinate| {
            if circuits[circuit_idx].insert(*new) {
                if let Some(circuit_idx_to_merge) =
                    circuits.iter().enumerate().find_map(|(idx, circuit)| {
                        (idx != circuit_idx && circuit.contains(new)).then_some(idx)
                    })
                {
                    let removed_circuit = circuits
                        .remove(circuit_idx_to_merge)
                        .into_iter()
                        .filter(|coord| coord != new);

                    circuits[circuit_idx - (circuit_idx_to_merge < circuit_idx) as usize]
                        .extend(removed_circuit);
                }
            }
        };

        if let Some(jb1_circuit_idx) = jb1_circuit_idx {
            merge_into(jb1_circuit_idx, jb2);
        } else if let Some(jb2_circuit_idx) = jb2_circuit_idx {
            merge_into(jb2_circuit_idx, jb1);
        } else {
            circuits.push(HashSet::from([*jb1, *jb2]));
        }

        if i == 1000 {
            circuits.sort_unstable_by_key(|circuit| cmp::Reverse(circuit.len()));
            println!(
                "{}",
                circuits
                    .iter()
                    .take(3)
                    .map(|coords| coords.len())
                    .product::<usize>()
            );
        }

        if let [circuit] = circuits.as_slice() {
            if circuit.len() == input.len() {
                println!("{}", jb1.x * jb2.x);
                break;
            }
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinate {
    x: usize,
    y: usize,
    z: usize,
}

impl Coordinate {
    fn distance_to(&self, other: &Coordinate) -> f64 {
        let x_diff = self.x.abs_diff(other.x) as f64;
        let y_diff = self.y.abs_diff(other.y) as f64;
        let z_diff = self.z.abs_diff(other.z) as f64;

        (x_diff.powi(2) + y_diff.powi(2) + z_diff.powi(2)).sqrt()
    }
}
