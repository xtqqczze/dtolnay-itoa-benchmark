use crate::digitslut::DIGITS_LUT;
use std::mem::MaybeUninit;
use std::slice;

pub fn u64toa_lut(mut value: u64, f: &dyn Fn(&str)) {
    let mut temp = [MaybeUninit::<u8>::uninit(); 20];
    let mut p = temp.as_mut_ptr().cast::<u8>();

    while value >= 100 {
        let i = ((value % 100) as usize) << 1;
        value /= 100;
        unsafe {
            *p = *DIGITS_LUT.get_unchecked(i + 1);
            p = p.add(1);
            *p = *DIGITS_LUT.get_unchecked(i);
            p = p.add(1);
        }
    }

    if value < 10 {
        unsafe {
            *p = value as u8 + b'0';
            p = p.add(1);
        }
    } else {
        let i = (value as usize) << 1;
        unsafe {
            *p = *DIGITS_LUT.get_unchecked(i + 1);
            p = p.add(1);
            *p = *DIGITS_LUT.get_unchecked(i);
            p = p.add(1);
        }
    }

    let mut buffer = [MaybeUninit::<u8>::uninit(); 20];
    let mut out = buffer.as_mut_ptr().cast::<u8>();
    while {
        unsafe {
            p = p.sub(1);
            *out = *p;
            out = out.add(1);
        }
        p.cast_const() != temp.as_ptr().cast::<u8>()
    } {}

    f(unsafe {
        str::from_utf8_unchecked(slice::from_raw_parts(
            buffer.as_ptr().cast::<u8>(),
            out.offset_from_unsigned(buffer.as_ptr().cast::<u8>()),
        ))
    });
}
