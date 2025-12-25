/*===----------------------------------------------------------------------===*
 * itoa_ljust_impl.h - Fast integer to ascii decimal conversion
 *
 * This file is meant to be included in only one .cpp file/compilation unit.
 * Uses recursive function templates, compile with -O3 for best performance.
 *
 * The implementation strives to perform well with random input values
 * where CPU branch prediction becomes ineffective:
 *
 *   1 ) reduce the number of conditional branches used to determine
 *       the number of digits and use uninterrupted sequence of
 *       instructions to generate multiple digits, this was inspired by
 *       the implementation of FastUInt32ToBufferLeft in
 *           https://github.com/google/protobuf/blob/master/
 *                 src/google/protobuf/stubs/strutil.cc
 *
 *   2 ) avoid branches altogether by allowing overwriting of characters
 *       in the output buffer when the difference is only one character
 *          a) minus sign
 *          b) alignment to even # digits
 *
 *   3 ) use hints to the compiler to indicate which conditional branches
 *       are likely to be taken so the compiler arranges the likely
 *       case to be the fallthrough, branch not taken
 *
 * Other Performance considerations
 *
 *   4 ) use a lookup table to convert binary numbers 0..99 into 2 characters
 *       This technique is used by all fast implementations.
 *       See Andrei Alexandrescu's engineering notes
 *           https://www.facebook.com/notes/facebook-engineering/
 *                 three-optimization-tips-for-c/10151361643253920/
 *
 *   5 ) use memcpy to store 2 digits at a time - most compilers treat
 *       memcpy as a builtin/intrinsic, this lets the compiler
 *       generate a 2-byte store instruction in platforms that support
 *       unaligned access
 *
 *   6 ) use explicit multiplicative inverse to perform integer division
 *       See Terje Mathisen's algoritm in Agner Fog's
 *           http://www.agner.org/optimize/optimizing_assembly.pdf
 *       By exploiting knowledge of the restricted domain of the dividend
 *       the multiplicative inverse factor is smaller and can fit in the
 *       immediate operand of x86 multiply instructions, resulting in
 *       fewer instructions
 *
 *   7 ) inline the recursive call to FastUInt64ToBufferLeft in the original
 *       Google Protocol Buffers, as in itoa-benchmark/src/unrolledlut.cpp
 *       by nyronium@genthree.io
 *
 * Correctness considerations
 *
 *   8 ) Avoids unary minus of signed types - undefined behavior if value
 *       is INT_MIN in platforms using two's complement representation
 *
 *===----------------------------------------------------------------------===*
 *
 * The MIT License (MIT)
 *
 * Copyright (c) 2016-2017 Arturo Martin-de-Nicolas
 * arturomdn@gmail.com
 * https://github.com/amdn/itoa_ljust/
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 *     The above copyright notice and this permission notice shall be included
 *     in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *===----------------------------------------------------------------------===*/

use crate::digitslut::DIGITS_LUT;
use std::mem::MaybeUninit;
use std::ptr;
use std::slice;

const fn p10(e: u32) -> u32 {
    if e != 0 { 10 * p10(e - 1) } else { 1 }
}

const fn q10_u32<const E: u32>(u: u32) -> u32 {
    if E == 0 {
        u
    } else if E == 2 {
        ((u as u64 * 5243) >> 19) as u32 // u < 10^4
    } else if E == 4 {
        (((1 + u as u64) * 858993) >> 33) as u32 // u < 10^6
    } else if E == 6 {
        (((1 + u as u64) * 8796093) >> 43) as u32 // u < 10^8
    } else if E == 8 {
        ((u as u64 * 1441151881) >> 57) as u32 // u < 2^32
    } else {
        unimplemented!()
    }
}

const fn q10_u64<const E: u32>(u: u64) -> u64 {
    u / p10(E) as u64
}

struct QR<T> {
    q: T,
    r: T,
}

impl QR<u32> {
    const fn new_u32<const E: u32>(u: u32) -> Self {
        let q = q10_u32::<E>(u);
        QR {
            q,
            r: u - q * p10(E),
        }
    }

    const fn new_u64<const E: u32>(u: u64) -> Self {
        let q = q10_u64::<E>(u) as u32;
        QR {
            q,
            r: (u - (q.wrapping_mul(p10(E))) as u64) as u32,
        }
    }
}

impl QR<u64> {
    const fn new_u64<const E: u32>(u: u64) -> Self {
        let q = q10_u64::<E>(u);
        QR {
            q,
            r: u - q * p10(E) as u64,
        }
    }
}

macro_rules! cvt_even {
    ($cvt:ident, $E:expr, $minus2:ident) => {
        unsafe fn $cvt(out: *mut u8, u: u32) -> *mut u8 {
            let d = QR::<u32>::new_u32::<{ $E - 2 }>(u);
            unsafe {
                ptr::copy_nonoverlapping(DIGITS_LUT.as_ptr().add(d.q as usize * 2), out, 2);
                $minus2(out.add(2), d.r)
            }
        }
    };
}

cvt_even!(cvt_2, 2, cvt_0);
cvt_even!(cvt_4, 4, cvt_2);
cvt_even!(cvt_6, 6, cvt_4);
cvt_even!(cvt_8, 8, cvt_6);

unsafe fn cvt_0(out: *mut u8, _: u32) -> *mut u8 {
    out
}

macro_rules! cvt_odd {
    ($cvt:ident, $E:expr, $minus1:ident) => {
        unsafe fn $cvt(mut out: *mut u8, u: u32) -> *mut u8 {
            let d = QR::<u32>::new_u32::<{ $E - 1 }>(u);
            unsafe {
                let mut src = DIGITS_LUT.as_ptr().add(d.q as usize * 2);
                *out = *src;
                src = src.add(1);
                out = out.add(usize::from(d.q > 9));
                *out = *src;
                $minus1(out.add(1), d.r)
            }
        }
    };
}

cvt_odd!(cvt_1, 1, cvt_0);
cvt_odd!(cvt_3, 3, cvt_2);
cvt_odd!(cvt_5, 5, cvt_4);
cvt_odd!(cvt_7, 7, cvt_6);
cvt_odd!(cvt_9, 9, cvt_8);

unsafe fn to_dec_u32(out: *mut u8, u: u32) -> *mut u8 {
    if u >= p10(8) {
        unsafe { cvt_9(out, u) }
    } else if u < p10(2) {
        unsafe { cvt_1(out, u) }
    } else if u < p10(4) {
        unsafe { cvt_3(out, u) }
    } else if u < p10(6) {
        unsafe { cvt_5(out, u) }
    } else {
        unsafe { cvt_7(out, u) }
    }
}

unsafe fn to_dec_u64(mut out: *mut u8, u: u64) -> *mut u8 {
    let low = u as u32;
    if u64::from(low) == u {
        return unsafe { to_dec_u32(out, low) };
    }
    let mid = QR::<u64>::new_u64::<8>(u);
    let mid32 = mid.q as u32;
    if u64::from(mid32) == mid.q {
        unsafe {
            out = to_dec_u32(out, mid32);
            cvt_8(out, mid.r as u32)
        }
    } else {
        let d = QR::<u32>::new_u64::<8>(mid.q);
        unsafe {
            out = if d.q < p10(2) {
                cvt_1(out, d.q)
            } else {
                cvt_3(out, d.q)
            };
            out = cvt_8(out, d.r);
            cvt_8(out, mid.r as u32)
        }
    }
}

pub fn u64toa_amartin(v: u64, f: &dyn Fn(&str)) {
    let mut buffer = [MaybeUninit::<u8>::uninit(); 20];
    unsafe {
        let end = to_dec_u64(buffer.as_mut_ptr().cast::<u8>(), v);
        f(str::from_utf8_unchecked(slice::from_raw_parts(
            buffer.as_ptr().cast::<u8>(),
            end.cast_const()
                .offset_from_unsigned(buffer.as_ptr().cast::<u8>()),
        )));
    }
}
