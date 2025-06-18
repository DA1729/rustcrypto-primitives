use rustcrypto_primitives::consttime::*;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ct_eq() {
        assert_eq!(ct_eq([0u8; 4], [0u8; 4]), 1);
        assert_eq!(ct_eq([1u8, 2, 3, 4], [1u8, 2, 3, 4]), 1);
        assert_eq!(ct_eq([1u8, 2, 3, 4], [1u8, 2, 3, 5]), 0);
        assert_eq!(ct_eq([1u8], [1u8, 2]), 0);
    }

    #[test]
    fn test_ct_select() {
        let a = 0x1234u16;
        let b = 0x5678u16;
        
        assert_eq!(ct_select(a, b, 1), a);
        assert_eq!(ct_select(a, b, 0), b);
        
        // Test with arrays
        let arr1 = [1u8, 2, 3];
        let arr2 = [4u8, 5, 6];
        assert_eq!(ct_select(arr1, arr2, 1), arr1);
        assert_eq!(ct_select(arr1, arr2, 0), arr2);
    }

    #[test]
    fn test_ct_cond_copy() {
        let mut val = 0x1234u16;
        let new_val = 0x5678u16;
        
        ct_cond_copy(&mut val, &new_val, 1);
        assert_eq!(val, new_val);
        
        let original = 0x1234u16;
        ct_cond_copy(&mut val, &original, 0);
        assert_eq!(val, new_val); // Should remain unchanged
    }
}
