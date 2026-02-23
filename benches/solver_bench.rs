use divan::Bencher;
use rubiks_cube::cubie_cube::CubieCube;
use rubiks_cube::pruning_table::PruningTables;
use rubiks_cube::solver::Solver;

fn main() {
    divan::main();
}

#[divan::bench]
fn bench_hard_20_move_scramble(bencher: Bencher) {
    let tables = PruningTables::new();

    bencher
        .with_inputs(|| {
            let mut cube = CubieCube::SOLVED;
            let scramble = "L' U' R' U D2 F' B L2 B2 R F' D2 R' D B2 R U' L D' R2";
            cube.apply_sequence(scramble).expect("Invalid scramble string");
            cube
        })
        .bench_values(|cube| {
            let mut solver = Solver::new(&tables);
            solver.solve(&cube)
        });
}

#[divan::bench]
fn bench_simple_5_move_scramble(bencher: Bencher) {
    let tables = PruningTables::new();

    bencher
        .with_inputs(|| {
            let mut cube = CubieCube::SOLVED;
            cube.apply_sequence("R U R' U' R").unwrap();
            cube
        })
        .bench_values(|cube| {
            let mut solver = Solver::new(&tables);
            solver.solve(&cube)
        });
}
