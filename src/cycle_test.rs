#[cfg(test)]

use crate::{cubie_cube::CubieCube, turn::Turn};


#[test]
fn test_all_move_cycles() {
    for m in Turn::ALL {
        let m_cubie = m.to_cubie();
        
        let m2 = m_cubie.multiply(&m_cubie);
        
        match m {
            // Half-turns should equal to identity after exactly 2 actions
            Turn::U2 | Turn::R2 | Turn::F2 | Turn::D2 | Turn::L2 | Turn::B2 => {
                assert_eq!(
                    m2, 
                    CubieCube::SOLVED, 
                    "Half-turn {:?} failed the order-2 identity cycle test!", m
                );
            }
            // Quarter and Prime turns should equal to identity after 4 actions
            _ => {
                let m4 = m2.multiply(&m2);
                assert_eq!(
                    m4, 
                    CubieCube::SOLVED, 
                    "Quarter/Prime turn {:?} failed the order-4 identity cycle test!", m
                );
            }
        }
    }
}