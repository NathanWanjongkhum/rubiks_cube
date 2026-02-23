mod cubie_cube;
mod pruning_table;
mod solver;
mod turn;

use crate::{ cubie_cube::CubieCube, pruning_table::PruningTables, solver::Solver };

fn main() {
    let tables = PruningTables::new();
    let mut solver = Solver::new(&tables);

    let mut cube = CubieCube::new();

    // let scramble_str = "D2 R2 F2 D2 F2 U2";
    // let scramble_str = "D2 R2 F2 D2 F2 U2 R2 F2 U R2 D2 R B' U' L' F' L2 R' B' F2";

    // cube.apply_sequence(scramble_str).expect("Invalid scramble sequence");

    let scramble_moves = cube.scramble(1);

    let scramble_str: String = scramble_moves
        .iter()
        .map(|m| m.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    println!("Scramble: {}", scramble_str);

    if let Some(solution) = solver.solve(&cube) {
        println!("Solve Order: {}", solution);

        let mut verify_cube = CubieCube::SOLVED;
        verify_cube.apply_sequence(&scramble_str).unwrap();
        verify_cube.apply_sequence(&solution).unwrap();

        if verify_cube == CubieCube::SOLVED {
            println!("Verification: SUCCESS! Solution is 100% physically valid.");
        } else {
            println!("Verification: FAILED! State machine mismatch.");
        }
    }
}
