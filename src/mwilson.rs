// Efficient Integer to String Conversions, by Matthew Wilson.

use std::mem::MaybeUninit;
use std::slice;

static DIGITS: [u8; 19] = *b"9876543210123456789";

pub fn u64toa_mwilson(value: u64, f: &dyn Fn(&str)) {
    let mut buf = [MaybeUninit::<u8>::uninit(); 20];

    let mut i = value;
    let mut p = buf.as_mut_ptr().cast::<u8>();

    while {
        let lsd = (i % 10) as i32;
        i /= 10;
        unsafe {
            *p = *DIGITS.as_ptr().add((9 + lsd) as usize);
            p = p.add(1);
        }
        i != 0
    } {}

    let slice = unsafe {
        slice::from_raw_parts_mut(
            buf.as_mut_ptr().cast::<u8>(),
            p.offset_from_unsigned(buf.as_ptr().cast::<u8>()),
        )
    };
    slice.reverse();
    f(unsafe { str::from_utf8_unchecked(slice) });
}
