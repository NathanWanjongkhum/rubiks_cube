use num_integer::binomial;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Corner {
    URF,
    UFL,
    ULB,
    UBR,
    DFR,
    DLF,
    DBL,
    DRB,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Edge {
    UR,
    UF,
    UL,
    UB,
    DR,
    DF,
    DL,
    DB,
    FR,
    FL,
    BL,
    BR,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CubieCube {
    // Permutation of the 8 corners (0..7)
    pub cp: [u8; 8],
    // Orientation of the 8 corners (0..2)
    pub co: [u8; 8],
    // Permutation of the 12 edges (0..11)
    pub ep: [u8; 12],
    // Orientation of the 12 edges (0..1)
    pub eo: [u8; 12],
}

impl CubieCube {
    // Identity (Solved State)
    pub const SOLVED: CubieCube = CubieCube {
        cp: [0, 1, 2, 3, 4, 5, 6, 7],
        co: [0; 8],
        ep: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
        eo: [0; 12],
    };
    
    pub const fn new() -> Self {
        Self::SOLVED
    }

    // Up Move
    pub const U: CubieCube = CubieCube {
        cp: [3, 0, 1, 2, 4, 5, 6, 7],
        co: [0, 0, 0, 0, 0, 0, 0, 0],
        ep: [3, 0, 1, 2, 4, 5, 6, 7, 8, 9, 10, 11],
        eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    };

    // Right Move
    pub const R: CubieCube = CubieCube {
        cp: [4, 1, 2, 0, 7, 5, 6, 3], // Corner Permutation
        co: [2, 0, 0, 1, 1, 0, 0, 2], // Corner Orientation changes (+1/-1)
        ep: [0, 1, 2, 3, 11, 5, 6, 7, 4, 9, 10, 8], // Edge Permutation
        eo: [0; 12], // Edges on L/R don't flip in this specific axis definition
    };

    // Front Move
    pub const F: CubieCube = CubieCube {
        cp: [1, 5, 2, 3, 0, 4, 6, 7],
        co: [1, 2, 0, 0, 2, 1, 0, 0],
        ep: [0, 9, 2, 3, 4, 8, 6, 7, 1, 5, 10, 11],
        eo: [0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0], // F/B moves flip edges
    };

    // Down Move
    pub const D: CubieCube = CubieCube {
        cp: [0, 1, 2, 3, 5, 6, 7, 4],
        co: [0; 8],
        ep: [0, 1, 2, 3, 5, 6, 7, 4, 8, 9, 10, 11],
        eo: [0; 12],
    };

    // Left Move
    pub const L: CubieCube = CubieCube {
        cp: [0, 2, 6, 3, 4, 1, 5, 7],
        co: [0, 1, 2, 0, 0, 2, 1, 0],
        ep: [0, 1, 6, 3, 4, 5, 10, 7, 8, 2, 9, 11],
        eo: [0; 12],
    };

    // Back Move
    pub const B: CubieCube = CubieCube {
        cp: [0, 1, 3, 7, 4, 5, 2, 6],
        co: [0, 0, 1, 2, 0, 0, 2, 1],
        ep: [0, 1, 2, 7, 4, 5, 6, 11, 8, 9, 3, 10],
        eo: [0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1],
    };
}

impl CubieCube {
    /// Group multiplication: Returns a new cube representing "self * other".
    /// This applies the transformation 'other' to 'self'.
    pub fn multiply(&self, other: &CubieCube) -> Self {
        let mut result = CubieCube::SOLVED;

        // Handle Corners
        for i in 0..8 {
            // Apply permutation
            result.cp[i] = self.cp[other.cp[i] as usize];

            // Apply orientation (sum modulo 3)
            let ori_a = self.co[other.cp[i] as usize];
            let ori_b = other.co[i];
            result.co[i] = (ori_a + ori_b) % 3;
        }

        // Handle Edges
        for i in 0..12 {
            // Apply permutation
            result.ep[i] = self.ep[other.ep[i] as usize];

            // Apply orientation (sum modulo 2)
            let ori_a = self.eo[other.ep[i] as usize];
            let ori_b = other.eo[i];
            result.eo[i] = (ori_a + ori_b) % 2;
        }

        result
    }

    // Helper to calculate the inverse of a cube state
    pub fn inverse(&self) -> Self {
        let mut result = CubieCube::SOLVED;

        for i in 0..8 {
            let item = self.cp[i] as usize;
            result.cp[item] = i as u8;
            let ori = self.co[i];
            // Inverse orientation is (3 - ori) % 3
            result.co[item] = (3 - ori) % 3;
        }

        for i in 0..12 {
            let item = self.ep[i] as usize;
            result.ep[item] = i as u8;
            let ori = self.eo[i];
            result.eo[item] = (2 - ori) % 2;
        }

        result
    }
}

impl CubieCube {
    /// Calculate Corner Orientation Coordinate (0..2186)
    /// Uses the first 7 corners (0..6). The 8th is determined by parity.
    pub fn get_twist(&self) -> u16 {
        let mut twist = 0;
        for i in 0..7 {
            twist = 3 * twist + self.co[i] as u16;
        }
        twist
    }

    /// Calculate Edge Orientation Coordinate (0..2048)
    /// Uses the first 11 edges (0..10). The 12th is determined by parity.
    pub fn get_flip(&self) -> u16 {
        let mut flip = 0;
        for i in 0..11 {
            flip = 2 * flip + self.eo[i] as u16;
        }
        flip
    }

    /// Calculate UD Slice Coordinate (0..495)
    /// Represents the position of the 4 slice edges (FR, FL, BL, BR)
    /// among the 12 edge positions.
    pub fn get_slice_sorted(&self) -> u16 {
        let mut idx = 0;
        let mut k = 3; // We are looking for 4 edges (indices 8,9,10,11 in standard notation)
        let mut n = 11;

        // Scan edges from right to left (11 down to 0)
        while k >= 0 && n > 0 {
            // n=0 case handled by loop termination
            // Check if the edge at position n is a "slice edge".
            // In standard notation, slice edges are indices 8, 9, 10, 11.
            if self.ep[n] >= 8 {
                // If we found a slice edge, we add C(n, k) to the index
                // and look for the next slice edge (k-1)
                idx += C_NK[n][k as usize];
                k -= 1;
            }
            n -= 1;
        }
        idx
    }
}

impl CubieCube {
    /// Inverse Twist: Creates a cube with the specified Corner Orientation (0..2186)
    pub fn set_twist(mut twist: u16) -> Self {
        let mut cc = CubieCube::SOLVED;
        let mut sum_twist = 0;
        
        // We set corners URF(0) through DBL(6)
        // Iterate in reverse (6 down to 0) to match the "get_twist" base-3 logic
        for i in (0..7).rev() {
            let val = (twist % 3) as u8;
            twist /= 3;
            cc.co[i] = val;
            sum_twist += val;
        }
        
        // The last corner (DRB/7) is determined by parity: sum must be divisible by 3
        cc.co[7] = (3 - (sum_twist % 3)) % 3;
        cc
    }

    /// Inverse Flip: Creates a cube with the specified Edge Orientation (0..2047)
    pub fn set_flip(mut flip: u16) -> Self {
        let mut cc = CubieCube::SOLVED;
        let mut sum_flip = 0;

        // Set edges 0..10. The 11th is determined by parity.
        for i in (0..11).rev() {
            let val = (flip % 2) as u8;
            flip /= 2;
            cc.eo[i] = val;
            sum_flip += val;
        }

        // The last edge (BR/11) must make total orientation even
        cc.eo[11] = (2 - (sum_flip % 2)) % 2;
        cc
    }

    /// Inverse Slice: Places the 4 slice edges (FR,FL,BL,BR) based on the index (0..494)
    pub fn set_slice_sorted(mut idx: u16) -> Self {
        let mut cc = CubieCube::SOLVED;
        
        // Slice edges (indices 8,9,10,11) and Non-slice edges (0..7)
        // We use arrays to pop available pieces into the positions.
        let mut slice_edges = [8, 9, 10, 11];
        let mut other_edges = [0, 1, 2, 3, 4, 5, 6, 7];
        let mut k = 4; // Number of slice edges to place

        // Scan positions from 11 down to 0
        for n in (0..12).rev() {
            if idx >= C_NK[n][k] {
                // Case: Position n IS a slice edge
                cc.ep[n] = slice_edges[k - 1]; // Place a slice edge here
                idx -= C_NK[n][k];
                k -= 1;
            } else {
                // Case: Position n is NOT a slice edge
                // We use the (n - k) index to pick from remaining others
                cc.ep[n] = other_edges[n - k];
            }
        }
        cc
    }
}

/// Represents the 18 possible moves in Half-Turn Metric
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Move {
    U,
    U2,
    U3,
    R,
    R2,
    R3,
    F,
    F2,
    F3,
    D,
    D2,
    D3,
    L,
    L2,
    L3,
    B,
    B2,
    B3,
}

impl Move {
    /// Returns the CubieCube representation for ANY move.
    /// This generalizes the logic: Base moves are looked up,
    /// Derived moves (X2, X') are calculated via group multiplication.
    pub fn to_cubie(&self) -> CubieCube {
        let base = match self {
            Move::U | Move::U2 | Move::U3 => CubieCube::U,
            Move::R | Move::R2 | Move::R3 => CubieCube::R,
            Move::F | Move::F2 | Move::F3 => CubieCube::F,
            Move::D | Move::D2 | Move::D3 => CubieCube::D,
            Move::L | Move::L2 | Move::L3 => CubieCube::L,
            Move::B | Move::B2 | Move::B3 => CubieCube::B,
        };

        match self {
            // Base moves (Quarter turns)
            Move::U | Move::R | Move::F | Move::D | Move::L | Move::B => base,
            
            // Double turns (Half-turn metric)
            Move::U2 | Move::R2 | Move::F2 | Move::D2 | Move::L2 | Move::B2 => {
                base.multiply(&base)
            },

            // Inverse turns (Prime moves)
            Move::U3 | Move::R3 | Move::F3 | Move::D3 | Move::L3 | Move::B3 => {
                base.multiply(&base.multiply(&base))
            }
        }
    }
}

// Precomputed Binomial Coefficients (n choose k) for the Slice coordinate
const C_NK: [[u16; 5]; 12] = [
    [1, 0, 0, 0, 0],
    [1, 1, 0, 0, 0],
    [1, 2, 1, 0, 0],
    [1, 3, 3, 1, 0],
    [1, 4, 6, 4, 1],
    [1, 5, 10, 10, 5],
    [1, 6, 15, 20, 15],
    [1, 7, 21, 35, 35],
    [1, 8, 28, 56, 70],
    [1, 9, 36, 84, 126],
    [1, 10, 45, 120, 210],
    [1, 11, 55, 165, 330]
];

pub struct PruningTables {
    pub twist_move: Vec<Vec<u16>>, // [2187][18]
    pub flip_move: Vec<Vec<u16>>,  // [2048][18]
    pub slice_move: Vec<Vec<u16>>, // [495][18]
}

impl PruningTables {
    pub fn new() -> Self {
        // Initialize basic CubieCube and moves
        let mut twist_move = vec![vec![0; 18]; 2187];
        let mut flip_move = vec![vec![0; 18]; 2048];
        let mut slice_move = vec![vec![0; 18]; 495];

        // This part requires iterating every valid coordinate,
        // converting it back to a CubieCube (inverse coord),
        // applying the move, and converting back to coord.
        // This is done ONLY ONCE at startup.

        Self {
            twist_move,
            flip_move: vec![],
            slice_move: vec![],
        }
    }
}

fn main() {
    println!("asd");
}
