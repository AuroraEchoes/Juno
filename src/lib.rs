pub mod grid;
pub mod vector;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector::*;

    #[test]
    fn ivec_macro() {
        let vec = ivec!(2, 2);
        let equiv = IVec2::new(2, 2);
        assert_eq!(vec, equiv);
    }

    #[test]
    fn test_add() {
        assert_eq!(ivec!(2, 5), ivec!(1, 4) + ivec!(1, 1));
        let mut add_eq = ivec!(1, 1);
        add_eq += ivec!(2, 4);
        assert_eq!(ivec!(3, 5), add_eq);
        assert_eq!(ivec!(2, 2), ivec!(1, 1) + 1);
        let mut add_eq_i32 = ivec!(1, 1);
        add_eq_i32 += 1;
        assert_eq!(ivec!(2, 2), add_eq_i32);
    }

    #[test]
    fn test_divide() {
        assert_eq!(ivec!(3, 2), ivec!(9, 8) / ivec!(3, 4));
        let mut div_eq = ivec!(9, 8);
        div_eq /= ivec!(3, 4);
        assert_eq!(ivec!(3, 2), div_eq);
        assert_eq!(ivec!(2, 4), ivec!(4, 8) / 2);
        let mut div_eq_i32 = ivec!(2, 4);
        div_eq_i32 /= 2;
        assert_eq!(ivec!(1, 2), div_eq_i32);
    }
}
