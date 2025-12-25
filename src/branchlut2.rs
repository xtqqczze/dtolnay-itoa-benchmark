use crate::digitslut::DIGITS_LUT;
use std::mem::MaybeUninit;
use std::slice;

pub fn u64toa_branchlut2(x: u64, f: &dyn Fn(&str)) {
    let mut buffer = [MaybeUninit::<u8>::uninit(); 20];
    let mut p = buffer.as_mut_ptr().cast::<u8>();

    macro_rules! begin2 {
        ($n:expr) => {
            let mut t = $n as usize;
            if t < 10 {
                unsafe {
                    *p = b'0' + t as u8;
                    p = p.add(1);
                }
            } else {
                t *= 2;
                unsafe {
                    *p = *DIGITS_LUT.get_unchecked(t);
                    p = p.add(1);
                    *p = *DIGITS_LUT.get_unchecked(t + 1);
                    p = p.add(1);
                }
            }
        };
    }
    macro_rules! middle2 {
        ($n:expr) => {
            let t = ($n * 2) as usize;
            unsafe {
                *p = *DIGITS_LUT.get_unchecked(t);
                p = p.add(1);
                *p = *DIGITS_LUT.get_unchecked(t + 1);
                p = p.add(1);
            }
        };
    }
    macro_rules! begin4 {
        ($n:expr) => {
            let t4 = $n;
            if t4 < 100 {
                begin2!(t4);
            } else {
                begin2!(t4 / 100);
                middle2!(t4 % 100);
            }
        };
    }
    macro_rules! middle4 {
        ($n:expr) => {
            let t4 = $n;
            middle2!(t4 / 100);
            middle2!(t4 % 100);
        };
    }
    macro_rules! begin8 {
        ($n:expr) => {
            let t8 = $n as u32;
            if t8 < 10000 {
                begin4!(t8);
            } else {
                begin4!(t8 / 10000);
                middle4!(t8 % 10000);
            }
        };
    }
    macro_rules! middle8 {
        ($n:expr) => {
            let t8 = $n as u32;
            middle4!(t8 / 10000);
            middle4!(t8 % 10000);
        };
    }
    macro_rules! middle16 {
        ($n:expr) => {
            let t16 = $n as u64;
            middle8!(t16 / 100000000);
            middle8!(t16 % 100000000);
        };
    }

    if x < 100000000 {
        begin8!(x);
    } else if x < 10000000000000000 {
        begin8!(x / 100000000);
        middle8!(x % 100000000);
    } else {
        begin4!(x / 10000000000000000);
        middle16!(x % 10000000000000000);
    }

    f(unsafe {
        str::from_utf8_unchecked(slice::from_raw_parts(
            buffer.as_ptr().cast::<u8>(),
            p.offset_from_unsigned(buffer.as_ptr().cast::<u8>()),
        ))
    });
}
