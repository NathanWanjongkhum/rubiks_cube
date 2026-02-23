use crate::cubie_cube::CubieCube;
use crate::turn::Turn;

use std::{ collections::VecDeque, fs::File, io::Read };
use std::io::Write;

use rkyv::{ Archive, Deserialize, Serialize };
use rkyv::rancor::Error;

#[derive(Archive, Serialize, Deserialize)]
pub struct PruningTables {
    // Phase 1 Move Tables
    pub twist_move: Vec<Vec<u16>>, // [2187][18]
    pub flip_move: Vec<Vec<u16>>, // [2048][18]
    pub slice_move: Vec<Vec<u16>>, // [495][18]

    pub twist_slice_pruning: NibbleArray,
    pub flip_slice_pruning: NibbleArray,

    // Phase 2 Move Tables
    // We use u16 because 8! = 40320, which fits in u16.
    pub cp_move: Vec<Vec<u16>>, // [40320][18] Corner Permutation Move Table
    pub ud_edge_move: Vec<Vec<u16>>, // [40320][18] U/D Edge Permutation Move Table
    pub ep_slice_move: Vec<Vec<u16>>, // [24][18]    Slice Permutation Move Table (Small: 24)

    // Phase 2 Pruning Tables (Distance)
    // CP (40320) * Slice (24) = 967,680 entries (~483KB with NibbleArray)
    pub corner_slice_pruning: NibbleArray,
    // UD (40320) * Slice (24) = 967,680 entries (~483KB with NibbleArray)
    pub ud_edge_slice_pruning: NibbleArray,
}

impl PruningTables {
    pub fn new() -> Self {
        let cache_path = "pruning_tables.rkyv";

        if let Ok(mut file) = File::open(cache_path) {
            println!("Loading pruning tables from cache...");
            let mut buffer = Vec::new();
            if file.read_to_end(&mut buffer).is_ok() {
                if let Ok(tables) = rkyv::from_bytes::<PruningTables, Error>(&buffer) {
                    println!("Successfully loaded tables.");
                    return tables;
                }
            }
            println!("Cache corrupted or outdated. Regenerating...");
        }

        println!("Generating pruning tables from scratch...");
        let tables = Self::generate();

        println!("Saving pruning tables to disk...");
        let bytes = rkyv::to_bytes::<Error>(&tables).expect("Failed to serialize tables");
        if let Ok(mut file) = File::create(cache_path) {
            let _ = file.write_all(&bytes);
            println!("Saved tables to {}.", cache_path);
        }

        tables
    }

    fn generate() -> Self {
        // Start by creating transistion tables for the pruning tables
        let mut twist_move = vec![vec![0; 18]; 2187];
        let mut flip_move = vec![vec![0; 18]; 2048];
        let mut slice_move = vec![vec![0; 18]; 495];

        // Precompute the 18 Turn CubieCubes
        // We map the Enum 0..17 to actual CubieCube structs to avoid re-generating them in loops
        let moves: Vec<CubieCube> = Turn::ALL.iter()
            .map(|m| m.to_cubie())
            .collect();

        // Generate Twist Turn Table (Size 2187 * 18)
        // The orientation is invarient so by the closure principle the last corner is entailed (3^7=2187).
        for i in 0..2187 {
            let state = CubieCube::set_twist(i as u16);
            for (m_idx, m_cubie) in moves.iter().enumerate() {
                let result = state.multiply(m_cubie);
                twist_move[i][m_idx] = result.get_twist();
            }
        }
        println!("Twist States: {:#?}", twist_move.len());

        // Generate Flip Turn Table (Size 2048 * 18)
        // The orientation is invarient so by the closure principle the last corner is entailed (2^11=2048).
        for i in 0..2048 {
            let state = CubieCube::set_flip(i as u16);
            for (m_idx, m_cubie) in moves.iter().enumerate() {
                let result = state.multiply(m_cubie);
                flip_move[i][m_idx] = result.get_flip();
            }
        }
        println!("Flip States: {:#?}", flip_move.len());

        // Generate Slice Sorted Turn Table (Size 495 * 18)
        // FR, FL, BL, BR are the 4 middle-layer slices we get the combination of 4 spots out of 12.
        for i in 0..495 {
            let state = CubieCube::set_slice_sorted(i as u16);
            for (m_idx, m_cubie) in moves.iter().enumerate() {
                let result = state.multiply(m_cubie);
                slice_move[i][m_idx] = result.get_slice_sorted();
            }
        }
        println!("Slice States: {:#?}", slice_move.len());

        println!("Generating Phase 1 Pruning...");

        let twist_slice_pruning = Self::generate_pruning_table(
            &twist_move,
            &slice_move,
            2187,
            495,
            CubieCube::SOLVED.get_twist() as usize,
            CubieCube::SOLVED.get_slice_sorted() as usize
        );
        println!("Twist-Slice States: {}", twist_slice_pruning.length);
        println!("Twist-Slice Physical Bytes: {}", twist_slice_pruning.data.len());

        let flip_slice_pruning = Self::generate_pruning_table(
            &flip_move,
            &slice_move,
            2048,
            495,
            CubieCube::SOLVED.get_flip() as usize,
            CubieCube::SOLVED.get_slice_sorted() as usize
        );
        println!("Flip-Slice States:  {}", flip_slice_pruning.length);
        println!("Flip-Slice Physical Bytes:  {}", flip_slice_pruning.data.len());

        // Phase 2
        // For move tables, we calculate ALL 18 moves.
        let mut cp_move = vec![vec![0; 18]; 40320];
        let mut ud_edge_move = vec![vec![0; 18]; 40320];
        let mut ep_slice_move = vec![vec![0; 18]; 24];

        // Generate Corner Permutation Move Table
        // Iterate through all 8! permutations
        for i in 0..40320 {
            let state = CubieCube::set_corner_perm(i);
            for (m_idx, m_cubie) in moves.iter().enumerate() {
                let result = state.multiply(m_cubie);
                cp_move[i][m_idx] = result.get_corner_perm() as u16;
            }
        }
        println!("Corner-Permutation States: {:#?}", cp_move.len());

        // Generate U/D Edge Permutation Move Table
        // Iterate through all 8! permutations
        for i in 0..40320 {
            let state = CubieCube::set_ud_edges(i);
            for (m_idx, m_cubie) in moves.iter().enumerate() {
                let result = state.multiply(m_cubie);
                ud_edge_move[i][m_idx] = result.get_ud_edges() as u16;
            }
        }
        println!("U/D Edge Permutation States: {:#?}", ud_edge_move.len());

        // Generate Slice Permutation Move Table
        // Iterate through all 4! (24) permutations
        for i in 0..24 {
            let state = CubieCube::set_slice_perm(i);
            for (m_idx, m_cubie) in moves.iter().enumerate() {
                let result = state.multiply(m_cubie);
                ep_slice_move[i][m_idx] = result.get_slice_perm() as u16;
            }
        }
        println!("Slice Permutation States: {:#?}", ep_slice_move.len());

        // The Phase 2 Move Subset
        // Indices corresponding to U, U2, U3, D, D2, D3, R2, L2, F2, B2
        // U(0), U2(1), U3(2), D(9), D2(10), D3(11), R2(4), L2(13), F2(7), B2(16)
        let phase2_moves = vec![0, 1, 2, 9, 10, 11, 4, 13, 7, 16];

        println!("Generating Phase 2 Pruning...");

        let corner_slice_pruning = Self::generate_phase2_pruning(
            &cp_move,
            &ep_slice_move,
            40320,
            24,
            0,
            0,
            &phase2_moves
        );
        println!("Corner-Slice States:  {}", corner_slice_pruning.length);
        println!("Corner-Slice Physical Bytes:  {}", corner_slice_pruning.data.len());

        let ud_edge_slice_pruning = Self::generate_phase2_pruning(
            &ud_edge_move,
            &ep_slice_move,
            40320,
            24,
            0,
            0,
            &phase2_moves
        );
        println!("U/D Edge-Slice States:  {}", ud_edge_slice_pruning.length);
        println!("U/D Edge-Slice Physical Bytes:  {}", ud_edge_slice_pruning.data.len());

        Self {
            twist_move,
            flip_move,
            slice_move,
            twist_slice_pruning,
            flip_slice_pruning,
            cp_move,
            ud_edge_move,
            ep_slice_move,
            corner_slice_pruning,
            ud_edge_slice_pruning,
        }
    }

    fn generate_pruning_table(
        move_table_1: &[Vec<u16>],
        move_table_2: &[Vec<u16>],
        num_states_1: usize,
        num_states_2: usize,
        start_idx_1: usize,
        start_idx_2: usize
    ) -> NibbleArray {
        let size = num_states_1 * num_states_2;
        // Initialize with 0xF (15), which represents "unvisited"
        let mut table = NibbleArray::new(size, 0xf);
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
                if table.get(next_combined) == 0xf {
                    table.set(next_combined, dist + 1);
                    queue.push_back(next_combined);
                }
            }
        }
        table
    }

    fn generate_phase2_pruning(
        table1: &[Vec<u16>],
        table2: &[Vec<u16>],
        size1: usize,
        size2: usize,
        start1: usize,
        start2: usize,
        allowed_moves: &[usize]
    ) -> NibbleArray {
        let mut pruning = NibbleArray::new(size1 * size2, 0xf);
        let mut queue = std::collections::VecDeque::new();

        let start_node = start1 * size2 + start2;
        pruning.set(start_node, 0);
        queue.push_back(start_node);

        while let Some(curr) = queue.pop_front() {
            let dist = pruning.get(curr);
            if dist >= 14 {
                continue;
            } // Max Phase 2 depth is usually < 18

            let idx1 = curr / size2;
            let idx2 = curr % size2;

            // Only iterate allowed G1 moves
            for &m_idx in allowed_moves {
                let next1 = table1[idx1][m_idx] as usize;
                let next2 = table2[idx2][m_idx] as usize;
                let next_node = next1 * size2 + next2;

                if pruning.get(next_node) == 0xf {
                    pruning.set(next_node, dist + 1);
                    queue.push_back(next_node);
                }
            }
        }
        pruning
    }
}

#[derive(Clone, Archive, Serialize, Deserialize)]
pub struct NibbleArray {
    pub data: Vec<u8>,
    pub length: usize,
}

impl NibbleArray {
    pub fn new(size: usize, default: u8) -> Self {
        // We need ceil(size / 2) bytes
        let num_bytes = (size + 1) / 2;

        // Pack the default value (e.g., 0xFF) into both nibbles
        let packed = (default << 4) | (default & 0x0f);

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
            byte & 0x0f
        } else {
            // Upper nibble
            (byte >> 4) & 0x0f
        }
    }

    /// Set value at index
    #[inline(always)]
    pub fn set(&mut self, index: usize, value: u8) {
        let byte_idx = index / 2;
        let current_byte = self.data[byte_idx];

        if index % 2 == 0 {
            // Set lower nibble: Clear lower 4 bits, then OR in new value
            self.data[byte_idx] = (current_byte & 0xf0) | (value & 0x0f);
        } else {
            // Set upper nibble: Clear upper 4 bits, then OR in new value shifted
            self.data[byte_idx] = (current_byte & 0x0f) | (value << 4);
        }
    }
}
