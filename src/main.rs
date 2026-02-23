// Tests
mod cycle_test;
// Crates
mod cubie_cube;
mod pruning_table;
mod solver;
mod turn;

use crate::{cubie_cube::CubieCube, pruning_table::PruningTables, solver::Solver};

fn main() {
    let tables = PruningTables::new();
    let mut solver = Solver::new(&tables);

    let mut cube = CubieCube::new();
    let scramble_moves = cube.scramble(2);

    let scramble_str: String = scramble_moves
        .iter()
        .map(|m| m.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    println!("Scramble: {}", scramble_str);

    if let Some(solution) = solver.solve(&cube) {
        println!("Solve Order: {:#?}", solution);
    } else {
        println!("No solution found within depth limit");
    }
}
