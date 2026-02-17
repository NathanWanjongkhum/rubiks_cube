// TODO:: Verify all 6 generating moves
// TODO:: Let this be its own crate
// https://kociemba.org/math/coordlevel.htm

#[cfg(test)]
mod tests {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct CubieCube {
        pub cp: [u8; 8],   // Corner Permutation
        pub co: [u8; 8],   // Corner Orientation
        pub ep: [u8; 12],  // Edge Permutation
        pub eo: [u8; 12],  // Edge Orientation
    }

    impl CubieCube {
        pub const R: CubieCube = CubieCube {
            cp: [4, 1, 2, 0, 7, 5, 6, 3],
            co: [2, 0, 0, 1, 1, 0, 0, 2],
            ep: [0, 1, 2, 3, 11, 5, 6, 7, 4, 9, 10, 8],
            eo: [0; 12],
        };

        // Pascal Translations
        ///   s:=0;
        ///   for co:= URF to Pred(DRB) do s:= 3*s + PCorn^[co].o;
        pub fn corn_ori_coord(&self) -> u16 {
            let mut s: u16 = 0;
            // Loop 0..7 (URF to Pred(DRB) means 0 to 6)
            for i in 0..7 {
                s = 3 * s + self.co[i] as u16;
            }
            s
        }

        ///   x:= 0;
        ///   for i:= DRB downto Succ(URF) do begin
        ///     s:=0;
        ///     for j:= Pred(i) downto URF do
        ///       if PCorn^[j].c > PCorn^[i].c then Inc(s);
        ///     x:= (x+s)*Ord(i);
        ///   end;
        pub fn corn_perm_coord(&self) -> usize {
            let mut x: usize = 0;
            // DRB(7) down to Succ(URF)(1)
            for i in (1..=7).rev() {
                let mut s = 0;
                // Pred(i)(i-1) down to URF(0)
                for j in (0..i).rev() {
                    if self.cp[j] > self.cp[i] {
                        s += 1;
                    }
                }
                x = (x + s) * i;
            }
            x
        }
    }

    #[test]
    fn test_verify_r_move_against_paper() {
        let r_move = CubieCube::R;

        // Verify Corner Orientation
        let co_coord = r_move.corn_ori_coord();
        println!("Corner Orientation Coordinate: {}", co_coord);
        assert_eq!(co_coord, 1494, "Corner Orientation must match source value 1494");

        // Verify Corner Permutation
        let cp_coord = r_move.corn_perm_coord();
        println!("Corner Permutation Coordinate: {}", cp_coord);
        assert_eq!(cp_coord, 21021, "Corner Permutation must match source value 21021");
    }
}