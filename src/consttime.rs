// constant time equality comparison for arrays
pub fn ct_eq<A: AsRef<[u8]>, B: AsRef<[u8]>>(a: A, b: B) -> u8 {
    let a = a.as_ref();
    let b = b.as_ref();
    
    if a.len() != b.len() {
        return 0;
    }

    let mut result = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }

    (result == 0) as u8
}

// constant time selection between two values
pub fn ct_select<T: Copy>(a: T, b: T, choice: u8) -> T {
    let mask = (choice as i8).wrapping_neg() as u8;
    
    unsafe {
        let a_bytes = std::slice::from_raw_parts(
            &a as *const T as *const u8,
            std::mem::size_of::<T>()
        );
        let b_bytes = std::slice::from_raw_parts(
            &b as *const T as *const u8,
            std::mem::size_of::<T>()
        );
        
        let mut out = std::mem::MaybeUninit::<T>::uninit();
        let out_bytes = std::slice::from_raw_parts_mut(
            out.as_mut_ptr() as *mut u8,
            std::mem::size_of::<T>()
        );
        
        for (out_byte, (&a_byte, &b_byte)) in 
            out_bytes.iter_mut().zip(a_bytes.iter().zip(b_bytes.iter())) 
        {
            *out_byte = (a_byte & mask) | (b_byte & !mask);
        }
        
        out.assume_init()
    }
}


// constant time conditional copy

pub fn ct_cond_copy<T: Copy>(dst: &mut T, src: &T, choice: u8) {
    let mask = (choice as i8).wrapping_neg() as u8;
    let dst_bytes = unsafe {
        std::slice::from_raw_parts_mut(
            dst as *mut T as *mut u8,
            std::mem::size_of::<T>()
        )
    };
    let src_bytes = unsafe {
        std::slice::from_raw_parts(
            src as *const T as *const u8,
            std::mem::size_of::<T>()
        )
    };
    
    for (d, &s) in dst_bytes.iter_mut().zip(src_bytes.iter()) {
        *d = (*d & !mask) | (s & mask);
    }
}

