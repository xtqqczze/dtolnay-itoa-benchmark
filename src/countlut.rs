use crate::countdecimaldigit::count_decimal_digit_64;
use crate::digitslut::DIGITS_LUT;
use std::mem::MaybeUninit;
use std::slice;

pub fn u64toa_countlut(mut value: u64, f: &dyn Fn(&str)) {
    let digit = count_decimal_digit_64(value);

    let mut buffer = [MaybeUninit::<u8>::uninit(); 20];
    let mut out = unsafe { buffer.as_mut_ptr().add(digit as usize).cast::<u8>() };

    while value >= 100000000 {
        let a = (value % 100000000) as u32;
        value /= 100000000;

        let b = a / 10000;
        let c = a % 10000;

        let b1 = (b / 100) << 1;
        let b2 = (b % 100) << 1;
        let c1 = (c / 100) << 1;
        let c2 = (c % 100) << 1;

        unsafe {
            out = out.sub(8);

            *out = *DIGITS_LUT.get_unchecked(b1 as usize);
            *out.add(1) = *DIGITS_LUT.get_unchecked(b1 as usize + 1);
            *out.add(2) = *DIGITS_LUT.get_unchecked(b2 as usize);
            *out.add(3) = *DIGITS_LUT.get_unchecked(b2 as usize + 1);
            *out.add(4) = *DIGITS_LUT.get_unchecked(c1 as usize);
            *out.add(5) = *DIGITS_LUT.get_unchecked(c1 as usize + 1);
            *out.add(6) = *DIGITS_LUT.get_unchecked(c2 as usize);
            *out.add(7) = *DIGITS_LUT.get_unchecked(c2 as usize + 1);
        }
    }

    let mut value32 = value as u32;
    while value32 >= 100 {
        let i = ((value32 % 100) as usize) << 1;
        value32 /= 100;
        unsafe {
            out = out.sub(1);
            *out = *DIGITS_LUT.get_unchecked(i + 1);
            out = out.sub(1);
            *out = *DIGITS_LUT.get_unchecked(i);
        }
    }

    if value32 < 10 {
        unsafe {
            out = out.sub(1);
            *out = value32 as u8 + b'0';
        }
    } else {
        let i = (value32 as usize) << 1;
        unsafe {
            out = out.sub(1);
            *out = *DIGITS_LUT.get_unchecked(i + 1);
            out = out.sub(1);
            *out = *DIGITS_LUT.get_unchecked(i);
        }
    }

    f(unsafe { str::from_utf8_unchecked(slice::from_raw_parts(out, digit as usize)) });
}
