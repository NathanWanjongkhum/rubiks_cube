use crate::cubie_cube::CubieCube;

/// Represents the 18 possible moves in Half-Turn Metric
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Turn {
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

impl Turn {
    pub const ALL: [Turn; 18] = [
            Turn::U, Turn::U2, Turn::U3,
            Turn::R, Turn::R2, Turn::R3,
            Turn::F, Turn::F2, Turn::F3,
            Turn::D, Turn::D2, Turn::D3,
            Turn::L, Turn::L2, Turn::L3,
            Turn::B, Turn::B2, Turn::B3,
    ];
    
    /// Returns the CubieCube representation for ANY move.
    /// This generalizes the logic: Base moves are looked up,
    /// Derived moves (X2, X') are calculated via group multiplication.
    pub fn to_cubie(&self) -> CubieCube {
        let base = match self {
            Turn::U | Turn::U2 | Turn::U3 => CubieCube::U,
            Turn::R | Turn::R2 | Turn::R3 => CubieCube::R,
            Turn::F | Turn::F2 | Turn::F3 => CubieCube::F,
            Turn::D | Turn::D2 | Turn::D3 => CubieCube::D,
            Turn::L | Turn::L2 | Turn::L3 => CubieCube::L,
            Turn::B | Turn::B2 | Turn::B3 => CubieCube::B,
        };

        match self {
            // Base moves (Quarter turns)
            Turn::U | Turn::R | Turn::F | Turn::D | Turn::L | Turn::B => base,
            
            // Double turns (Half-turn metric)
            Turn::U2 | Turn::R2 | Turn::F2 | Turn::D2 | Turn::L2 | Turn::B2 => {
                base.multiply(&base)
            },

            // Inverse turns (Prime moves)
            Turn::U3 | Turn::R3 | Turn::F3 | Turn::D3 | Turn::L3 | Turn::B3 => {
                base.multiply(&base.multiply(&base))
            }
        }
    }
}
