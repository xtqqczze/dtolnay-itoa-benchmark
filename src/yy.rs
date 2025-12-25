/*
 * Integer to ascii conversion (ANSI C)
 *
 * Description
 *     The itoa function converts an integer value to a character string in
 *     decimal and stores the result in the buffer. If value is negative, the
 *     resulting string is preceded with a minus sign (-).
 *
 * Parameters
 *     val: Value to be converted.
 *     buf: Buffer that holds the result of the conversion.
 *
 * Return Value
 *     A pointer to the end of resulting string.
 *
 * Notice
 *     The resulting string is not null-terminated.
 *     The buffer should be large enough to hold any possible result:
 *         uint32_t: 10 bytes
 *         uint64_t: 20 bytes
 *         int32_t: 11 bytes
 *         int64_t: 20 bytes
 *
 * Copyright (c) 2018 YaoYuan <ibireme@gmail.com>.
 * Released under the MIT license (MIT).
 */

use crate::digitslut::DIGITS_LUT as DIGIT_TABLE;
use std::mem::MaybeUninit;
use std::ptr;
use std::slice;

unsafe fn byte_copy_2(dst: *mut u8, src: *const u8) {
    unsafe {
        ptr::copy_nonoverlapping(src, dst, 2);
    }
}

unsafe fn itoa_u64_impl_len_8(val: u32, buf: *mut u8) -> *mut u8 {
    let aabb = ((u64::from(val) * 109951163) >> 40) as u32; // val / 10000
    let ccdd = val - aabb * 10000; // val % 10000
    let aa = (aabb * 5243) >> 19; // aabb / 100
    let cc = (ccdd * 5243) >> 19; // ccdd / 100
    let bb = aabb - aa * 100; // aabb % 100
    let dd = ccdd - cc * 100; // ccdd % 100
    unsafe {
        byte_copy_2(buf, DIGIT_TABLE.as_ptr().add(aa as usize * 2));
        byte_copy_2(buf.add(2), DIGIT_TABLE.as_ptr().add(bb as usize * 2));
        byte_copy_2(buf.add(4), DIGIT_TABLE.as_ptr().add(cc as usize * 2));
        byte_copy_2(buf.add(6), DIGIT_TABLE.as_ptr().add(dd as usize * 2));
        buf.add(8)
    }
}

unsafe fn itoa_u64_impl_len_4(val: u32, buf: *mut u8) -> *mut u8 {
    let aa = (val * 5243) >> 19; // val / 100
    let bb = val - aa * 100; // val % 100
    unsafe {
        byte_copy_2(buf, DIGIT_TABLE.as_ptr().add(aa as usize * 2));
        byte_copy_2(buf.add(2), DIGIT_TABLE.as_ptr().add(bb as usize * 2));
        buf.add(4)
    }
}

unsafe fn itoa_u64_impl_len_1_to_8(val: u32, mut buf: *mut u8) -> *mut u8 {
    if val < 100 {
        // 1-2 digits: aa
        let lz = usize::from(val < 10);
        unsafe {
            byte_copy_2(buf, DIGIT_TABLE.as_ptr().add(val as usize * 2 + lz));
            buf = buf.wrapping_sub(lz);
            buf.wrapping_add(2)
        }
    } else if val < 10000 {
        // 3-4 digits: aabb
        let aa = (val * 5243) >> 19; // val / 100
        let bb = val - aa * 100; // val % 100
        let lz = usize::from(aa < 10);
        unsafe {
            byte_copy_2(buf, DIGIT_TABLE.as_ptr().add(aa as usize * 2 + lz));
            buf = buf.wrapping_sub(lz);
            byte_copy_2(
                buf.wrapping_add(2),
                DIGIT_TABLE.as_ptr().add(bb as usize * 2),
            );
            buf.wrapping_add(4)
        }
    } else if val < 1000000 {
        // 5-6 digits: aabbcc
        let aa = ((u64::from(val) * 429497) >> 32) as u32; // val / 10000
        let bbcc = val - aa * 10000; // val % 10000
        let bb = (bbcc * 5243) >> 19; // bbcc / 100
        let cc = bbcc - bb * 100; // bbcc % 100
        let lz = usize::from(aa < 10);
        unsafe {
            byte_copy_2(buf, DIGIT_TABLE.as_ptr().add(aa as usize * 2 + lz));
            buf = buf.wrapping_sub(lz);
            byte_copy_2(
                buf.wrapping_add(2),
                DIGIT_TABLE.as_ptr().add(bb as usize * 2),
            );
            byte_copy_2(
                buf.wrapping_add(4),
                DIGIT_TABLE.as_ptr().add(cc as usize * 2),
            );
            buf.wrapping_add(6)
        }
    } else {
        // 7-8 digits: aabbccdd
        let aabb = ((u64::from(val) * 109951163) >> 40) as u32; // val / 10000
        let ccdd = val - aabb * 10000; // val % 10000
        let aa = (aabb * 5243) >> 19; // aabb / 100
        let cc = (ccdd * 5243) >> 19; // ccdd / 100
        let bb = aabb - aa * 100; // aabb % 100
        let dd = ccdd - cc * 100; // ccdd % 100
        let lz = usize::from(aa < 10);
        unsafe {
            byte_copy_2(buf, DIGIT_TABLE.as_ptr().add(aa as usize * 2 + lz));
            buf = buf.wrapping_sub(lz);
            byte_copy_2(
                buf.wrapping_add(2),
                DIGIT_TABLE.as_ptr().add(bb as usize * 2),
            );
            byte_copy_2(
                buf.wrapping_add(4),
                DIGIT_TABLE.as_ptr().add(cc as usize * 2),
            );
            byte_copy_2(
                buf.wrapping_add(6),
                DIGIT_TABLE.as_ptr().add(dd as usize * 2),
            );
            buf.wrapping_add(8)
        }
    }
}

unsafe fn itoa_u64_impl_len_5_to_8(val: u32, mut buf: *mut u8) -> *mut u8 {
    if val < 1000000 {
        // 5-6 digits: aabbcc
        let aa = ((u64::from(val) * 429497) >> 32) as u32; // val / 10000
        let bbcc = val - aa * 10000; // val % 10000
        let bb = (bbcc * 5243) >> 19; // bbcc / 100
        let cc = bbcc - bb * 100; // bbcc % 100
        let lz = usize::from(aa < 10);
        unsafe {
            byte_copy_2(buf, DIGIT_TABLE.as_ptr().add(aa as usize * 2 + lz));
            buf = buf.wrapping_sub(lz);
            byte_copy_2(
                buf.wrapping_add(2),
                DIGIT_TABLE.as_ptr().add(bb as usize * 2),
            );
            byte_copy_2(
                buf.wrapping_add(4),
                DIGIT_TABLE.as_ptr().add(cc as usize * 2),
            );
            buf.wrapping_add(6)
        }
    } else {
        // 7-8 digits: aabbccdd
        let aabb = ((u64::from(val) * 109951163) >> 40) as u32; // val / 10000
        let ccdd = val - aabb * 10000; // val % 10000
        let aa = (aabb * 5243) >> 19; // aabb / 100
        let cc = (ccdd * 5243) >> 19; // ccdd / 100
        let bb = aabb - aa * 100; // aabb % 100
        let dd = ccdd - cc * 100; // ccdd % 100
        let lz = usize::from(aa < 10);
        unsafe {
            byte_copy_2(buf, DIGIT_TABLE.as_ptr().add(aa as usize * 2 + lz));
            buf = buf.wrapping_sub(lz);
            byte_copy_2(
                buf.wrapping_add(2),
                DIGIT_TABLE.as_ptr().add(bb as usize * 2),
            );
            byte_copy_2(
                buf.wrapping_add(4),
                DIGIT_TABLE.as_ptr().add(cc as usize * 2),
            );
            byte_copy_2(
                buf.wrapping_add(6),
                DIGIT_TABLE.as_ptr().add(dd as usize * 2),
            );
            buf.wrapping_add(8)
        }
    }
}

unsafe fn itoa_u64_impl(val: u64, mut buf: *mut u8) -> *mut u8 {
    if val < 100000000 {
        // 1-8 digits
        unsafe {
            buf = itoa_u64_impl_len_1_to_8(val as u32, buf);
        }
    } else if val < 100000000 * 100000000 {
        // 9-16 digits
        let hgh = val / 100000000;
        let low = (val - hgh * 100000000) as u32; // val % 100000000
        unsafe {
            buf = itoa_u64_impl_len_1_to_8(hgh as u32, buf);
            buf = itoa_u64_impl_len_8(low, buf);
        }
    } else {
        // 17-20 digits
        let tmp = val / 100000000;
        let low = (val - tmp * 100000000) as u32; // val % 100000000
        let hgh = u64::from((tmp / 10000) as u32);
        let mid = (tmp - hgh * 10000) as u32; // tmp % 10000
        unsafe {
            buf = itoa_u64_impl_len_5_to_8(hgh as u32, buf);
            buf = itoa_u64_impl_len_4(mid, buf);
            buf = itoa_u64_impl_len_8(low, buf);
        }
    }
    buf
}

pub fn u64toa_yy(v: u64, f: &dyn Fn(&str)) {
    let mut buffer = [MaybeUninit::<u8>::uninit(); 20];
    unsafe {
        let end = itoa_u64_impl(v, buffer.as_mut_ptr().cast::<u8>());
        f(str::from_utf8_unchecked(slice::from_raw_parts(
            buffer.as_ptr().cast::<u8>(),
            end.cast_const()
                .offset_from_unsigned(buffer.as_ptr().cast::<u8>()),
        )));
    }
}
