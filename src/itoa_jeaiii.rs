/*
MIT License
Copyright (c) 2017 James Edward Anhalt III (jeaiii)
https://github.com/jeaiii/itoa
*/

use crate::digitslut::DIGITS_LUT;
use paste::paste;
use std::mem::MaybeUninit;
use std::slice;

pub fn u64toa_jeaiii(n: u64, f: &dyn Fn(&str)) {
    let mut buffer = [MaybeUninit::<u8>::uninit(); 20];
    let mut b = buffer.as_mut_ptr().cast::<u8>();
    let mut u: u32;
    let mut t: u64;

    macro_rules! A {
        ($N:expr) => {
            t = const { (1u64 << (32 + $N / 5 * $N * 53 / 16)) / 10u32.pow($N) as u64 + 1 - $N / 9 };
            t *= u64::from(u);
            t >>= const { $N / 5 * $N * 53 / 16 };
            t += const { $N / 5 * 4 };
        };
    }

    macro_rules! W {
        ($N:expr, $I:expr) => {
            unsafe {
                b.add($N)
                    .cast::<u16>()
                    .write_unaligned(*DIGITS_LUT.as_ptr().cast::<u16>().add($I as usize));
            }
        };
    }
    macro_rules! Q {
        ($N:expr) => {
            unsafe {
                *b.add($N) = ((10u64 * u64::from(t as u32)) >> 32) as u8 + b'0';
            }
        };
    }
    macro_rules! D {
        ($N:expr) => {
            W!($N, t >> 32);
        };
    }
    macro_rules! E {
        () => {
            t = 100u64 * u64::from(t as u32);
        };
    }

    macro_rules! L0 {
        () => {
            unsafe {
                *b = u as u8 + b'0';
            }
        };
    }
    macro_rules! L1 {
        () => {
            W!(0, u);
        };
    }
    macro_rules! L2 {
        () => {
            A!(1);
            D!(0);
            Q!(2);
        };
    }
    macro_rules! L3 {
        () => {
            A!(2);
            D!(0);
            E!();
            D!(2);
        };
    }
    macro_rules! L4 {
        () => {
            A!(3);
            D!(0);
            E!();
            D!(2);
            Q!(4);
        };
    }
    macro_rules! L5 {
        () => {
            A!(4);
            D!(0);
            E!();
            D!(2);
            E!();
            D!(4);
        };
    }
    macro_rules! L6 {
        () => {
            A!(5);
            D!(0);
            E!();
            D!(2);
            E!();
            D!(4);
            Q!(6);
        };
    }
    macro_rules! L7 {
        () => {
            A!(6);
            D!(0);
            E!();
            D!(2);
            E!();
            D!(4);
            E!();
            D!(6);
        };
    }
    macro_rules! L8 {
        () => {
            A!(7);
            D!(0);
            E!();
            D!(2);
            E!();
            D!(4);
            E!();
            D!(6);
            Q!(8);
        };
    }
    macro_rules! L9 {
        () => {
            A!(8);
            D!(0);
            E!();
            D!(2);
            E!();
            D!(4);
            E!();
            D!(6);
            E!();
            D!(8);
        };
    }

    macro_rules! LN {
        ($N:literal) => {
            paste! {
                [<L $N>]!();
            }
            unsafe {
                b = b.add($N + 1);
            }
        };
    }
    macro_rules! LZ {
        ($N:literal) => {
            paste! {
                [<L $N>]!();
            }
            f(unsafe {
                str::from_utf8_unchecked(slice::from_raw_parts(
                    buffer.as_ptr().cast::<u8>(),
                    b.add($N + 1)
                        .offset_from_unsigned(buffer.as_ptr().cast::<u8>()),
                ))
            });
            return;
        };
    }
    macro_rules! LG {
        ($F:ident) => {
            if u < 100 {
                if u < 10 {
                    $F!(0);
                } else {
                    $F!(1);
                }
            } else if u < 1000000 {
                if u < 10000 {
                    if u < 1000 {
                        $F!(2);
                    } else {
                        $F!(3);
                    }
                } else if u < 100000 {
                    $F!(4);
                } else {
                    $F!(5);
                }
            } else if u < 100000000 {
                if u < 10000000 {
                    $F!(6);
                } else {
                    $F!(7);
                }
            } else if u < 1000000000 {
                $F!(8);
            } else {
                $F!(9);
            }
        };
    }

    if (n >> 32) as u32 == 0 {
        u = n as u32;
        LG!(LZ);
    }

    let a = n / 100000000;

    if (a >> 32) as u32 == 0 {
        u = a as u32;
        LG!(LN);
    } else {
        u = (a / 100000000) as u32;
        LG!(LN);
        u = (a % 100000000) as u32;
        LN!(7);
    }

    u = (n % 100000000) as u32;
    LZ!(7);
}
