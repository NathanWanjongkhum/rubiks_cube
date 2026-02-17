use crate::pruning_table::PruningTables;

mod cubie_cube;
mod pruning_table;
mod turn;

fn main() {
    let tables = PruningTables::new();
    println!("{:#?}", tables.twist_move.len());
    println!("{:#?}", tables.flip_move.len());
    println!("{:#?}", tables.slice_move.len());
}
