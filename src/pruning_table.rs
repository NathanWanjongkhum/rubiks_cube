use crate::cubie_cube::CubieCube;
use crate::turn::Turn;

pub struct PruningTables {
    pub twist_move: Vec<Vec<u16>>, // [2187][18]
    pub flip_move: Vec<Vec<u16>>,  // [2048][18]
    pub slice_move: Vec<Vec<u16>>, // [495][18]
}

impl PruningTables {
    pub fn new() -> Self {
        let mut twist_move = vec![vec![0; 18]; 2187];
        let mut flip_move = vec![vec![0; 18]; 2048];
        let mut slice_move = vec![vec![0; 18]; 495];

        // Precompute the 18 Turn CubieCubes
        // We map the Enum 0..17 to actual CubieCube structs to avoid re-generating them in loops
        let moves: Vec<CubieCube> = Turn::ALL.iter()
                .map(|m| m.to_cubie())
                .collect();

        // Generate Twist Turn Table (Size 2187 * 18)
        println!("Generating Twist Tables...");
        for i in 0..2187 {
            let state = CubieCube::set_twist(i as u16);
            for (m_idx, m_cubie) in moves.iter().enumerate() {
                let result = state.multiply(m_cubie);
                twist_move[i][m_idx] = result.get_twist();
            }
        }

        // Generate Flip Turn Table (Size 2048 * 18)
        println!("Generating Flip Tables...");
        for i in 0..2048 {
            let state = CubieCube::set_flip(i as u16);
            for (m_idx, m_cubie) in moves.iter().enumerate() {
                let result = state.multiply(m_cubie);
                flip_move[i][m_idx] = result.get_flip();
            }
        }

        // Generate Slice Sorted Turn Table (Size 495 * 18)
        println!("Generating Slice Tables...");
        for i in 0..495 {
            let state = CubieCube::set_slice_sorted(i as u16);
            for (m_idx, m_cubie) in moves.iter().enumerate() {
                let result = state.multiply(m_cubie);
                slice_move[i][m_idx] = result.get_slice_sorted();
            }
        }

        Self {
            twist_move,
            flip_move,
            slice_move,
        }
    }
}
