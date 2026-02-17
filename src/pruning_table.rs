use crate::cubie_cube::CubieCube;
use crate::turn::Turn;

use std::collections::VecDeque;

pub struct PruningTables {
    pub twist_move: Vec<Vec<u16>>, // [2187][18]
    pub flip_move: Vec<Vec<u16>>,  // [2048][18]
    pub slice_move: Vec<Vec<u16>>, // [495][18]

    pub twist_slice_pruning: NibbleArray,
    pub flip_slice_pruning: NibbleArray,
}

impl PruningTables {
    pub fn new() -> Self {
        // Start by creating transistion tables for the pruning tables
        let mut twist_move = vec![vec![0; 18]; 2187];
        let mut flip_move = vec![vec![0; 18]; 2048];
        let mut slice_move = vec![vec![0; 18]; 495];

        // Precompute the 18 Turn CubieCubes
        // We map the Enum 0..17 to actual CubieCube structs to avoid re-generating them in loops
        let moves: Vec<CubieCube> = Turn::ALL.iter().map(|m| m.to_cubie()).collect();

        // Generate Twist Turn Table (Size 2187 * 18)
        // The orientation is invarient so by the closure principle the last corner is entailed (3^7=2187).
        println!("Generating Twist Tables...");
        for i in 0..2187 {
            let state = CubieCube::set_twist(i as u16);
            for (m_idx, m_cubie) in moves.iter().enumerate() {
                let result = state.multiply(m_cubie);
                twist_move[i][m_idx] = result.get_twist();
            }
        }

        // Generate Flip Turn Table (Size 2048 * 18)
        // The orientation is invarient so by the closure principle the last corner is entailed (2^11=2048).
        println!("Generating Flip Tables...");
        for i in 0..2048 {
            let state = CubieCube::set_flip(i as u16);
            for (m_idx, m_cubie) in moves.iter().enumerate() {
                let result = state.multiply(m_cubie);
                flip_move[i][m_idx] = result.get_flip();
            }
        }

        // Generate Slice Sorted Turn Table (Size 495 * 18)
        // FR, FL, BL, BR are the 4 middle-layer slices we get the combination of 4 spots out of 12.
        println!("Generating Slice Tables...");
        for i in 0..495 {
            let state = CubieCube::set_slice_sorted(i as u16);
            for (m_idx, m_cubie) in moves.iter().enumerate() {
                let result = state.multiply(m_cubie);
                slice_move[i][m_idx] = result.get_slice_sorted();
            }
        }

        println!("Generating Twist-Slice Pruning...");
        let twist_slice_pruning = Self::generate_pruning_table(
            &twist_move,
            &slice_move,
            2187,
            495,
            CubieCube::SOLVED.get_twist() as usize,
            CubieCube::SOLVED.get_slice_sorted() as usize,
        );

        println!("Generating Flip-Slice Pruning...");
        let flip_slice_pruning = Self::generate_pruning_table(
            &flip_move,
            &slice_move,
            2048,
            495,
            CubieCube::SOLVED.get_flip() as usize,
            CubieCube::SOLVED.get_slice_sorted() as usize,
        );

        Self {
            twist_move,
            flip_move,
            slice_move,
            twist_slice_pruning,
            flip_slice_pruning,
        }
    }

    fn generate_pruning_table(
        move_table_1: &[Vec<u16>],
        move_table_2: &[Vec<u16>],
        num_states_1: usize,
        num_states_2: usize,
        start_idx_1: usize,
        start_idx_2: usize,
    ) -> NibbleArray {
        let size = num_states_1 * num_states_2;
        // Initialize with 0xF (15), which represents "unvisited"
        let mut table = NibbleArray::new(size, 0xF);
        let mut queue = VecDeque::new();

        // Initialize solved state (distance 0)
        let start_combined = start_idx_1 * num_states_2 + start_idx_2;
        table.set(start_combined, 0);
        queue.push_back(start_combined);

        // BFS
        while let Some(current_combined) = queue.pop_front() {
            // We get the distance of the current node
            let dist = table.get(current_combined);

            // Phase 1 max depth is ~12, so we won't overflow 15.
            if dist >= 14 {
                continue;
            }

            let idx_1 = current_combined / num_states_2;
            let idx_2 = current_combined % num_states_2;

            for move_idx in 0..18 {
                let next_1 = move_table_1[idx_1][move_idx] as usize;
                let next_2 = move_table_2[idx_2][move_idx] as usize;
                let next_combined = next_1 * num_states_2 + next_2;

                // Check if unvisited (0xF)
                if table.get(next_combined) == 0xF {
                    table.set(next_combined, dist + 1);
                    queue.push_back(next_combined);
                }
            }
        }
        table
    }
}

#[derive(Debug, Clone)]
pub struct NibbleArray {
    pub data: Vec<u8>,
    pub length: usize,
}

impl NibbleArray {
    pub fn new(size: usize, default: u8) -> Self {
        // We need ceil(size / 2) bytes
        let num_bytes = (size + 1) / 2;

        // Pack the default value (e.g., 0xFF) into both nibbles
        let packed = (default << 4) | (default & 0x0F);

        Self {
            data: vec![packed; num_bytes],
            length: size,
        }
    }

    /// Get value at index
    #[inline(always)]
    pub fn get(&self, index: usize) -> u8 {
        let byte = self.data[index / 2];
        if index % 2 == 0 {
            // Lower nibble
            byte & 0x0F
        } else {
            // Upper nibble
            (byte >> 4) & 0x0F
        }
    }

    /// Set value at index
    #[inline(always)]
    pub fn set(&mut self, index: usize, value: u8) {
        let byte_idx = index / 2;
        let current_byte = self.data[byte_idx];

        if index % 2 == 0 {
            // Set lower nibble: Clear lower 4 bits, then OR in new value
            self.data[byte_idx] = (current_byte & 0xF0) | (value & 0x0F);
        } else {
            // Set upper nibble: Clear upper 4 bits, then OR in new value shifted
            self.data[byte_idx] = (current_byte & 0x0F) | (value << 4);
        }
    }
}
