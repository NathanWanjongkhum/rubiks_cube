mod cubie_cube;
mod pruning_table;
mod turn;

use crate::pruning_table::PruningTables;

fn main() {
    let tables = PruningTables::new();

    println!("Twist States: {:#?}", tables.twist_move.len());
    println!("Flip States: {:#?}", tables.flip_move.len());
    println!("Slice States: {:#?}", tables.slice_move.len());
    println!(
        "Twist-Slice Logical States: {}",
        tables.twist_slice_pruning.length
    );
    println!(
        "Twist-Slice Physical Bytes: {}",
        tables.twist_slice_pruning.data.len()
    );

    println!(
        "Flip-Slice Logical States:  {}",
        tables.flip_slice_pruning.length
    );
    println!(
        "Flip-Slice Physical Bytes:  {}",
        tables.flip_slice_pruning.data.len()
    );
}
