// SPDX-License-Identifier: BSD-2-Clause
// See LICENSE file in the project root for full license text.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![deny(unsafe_op_in_unsafe_fn)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::c_int;
    use std::mem;

    #[cfg(feature = "approx")]
    #[test]
    fn reganexec() {
        let mut preg = mem::MaybeUninit::<regex_t>::uninit();
        if unsafe {
            tre_regcomp(
                preg.as_mut_ptr(),
                b"Hello!\0".as_ptr() as *const _,
                REG_ICASE as c_int,
            )
        } != 0
        {
            panic!("tre_regcomp");
        }
        let preg = unsafe { preg.assume_init() };

        let params = regaparams_t {
            cost_ins: 1,
            cost_del: 1,
            cost_subst: 1,
            max_cost: 2,
            max_del: 2,
            max_ins: 2,
            max_subst: 2,
            max_err: 2,
        };

        let mut pmatch: Vec<regmatch_t> = vec![Default::default(); 1];
        let mut amatch = regamatch_t {
            nmatch: 1,
            pmatch: pmatch.as_mut_ptr(),
            ..Default::default()
        };

        if unsafe {
            tre_reganexec(
                &preg,
                b"Hullo!".as_ptr() as *const _,
                6,
                &mut amatch,
                params,
                0,
            )
        } != 0
        {
            panic!("tre_regaexec");
        }

        assert_eq!(amatch.cost, 1);
        assert_eq!(pmatch[0].rm_so, 0);
        assert_eq!(pmatch[0].rm_eo, 6);
    }

    #[test]
    fn regexec() {
        let mut preg = mem::MaybeUninit::<regex_t>::uninit();
        if unsafe {
            tre_regcomp(
                preg.as_mut_ptr(),
                b"Hello(, [[:alpha:]]+)?!\0".as_ptr() as *const _,
                (REG_EXTENDED | REG_ICASE) as c_int,
            )
        } != 0
        {
            panic!("tre_regcomp");
        }

        let preg = unsafe { preg.assume_init() };

        let nmatch = 1;
        let mut pmatch: Vec<regmatch_t> = vec![regmatch_t { rm_so: 0, rm_eo: 0 }; 1];
        if unsafe {
            tre_regexec(
                &preg,
                b"Hello!".as_ptr() as *const _,
                nmatch,
                pmatch.as_mut_ptr(),
                0,
            )
        } != 0
        {
            panic!("tre_regexec");
        }

        assert!(pmatch[0].rm_so == 0, "Bad starting offset");
        assert!(pmatch[0].rm_eo == 6, "Bad ending offset");

        pmatch[0].rm_eo = 0;

        let nmatch = 2;
        pmatch.push(regmatch_t { rm_so: 0, rm_eo: 0 });
        if unsafe {
            tre_regexec(
                &preg,
                b"Hello, world!\0".as_ptr() as *const _,
                nmatch,
                pmatch.as_mut_ptr(),
                0,
            )
        } != 0
        {
            panic!("tre_regexec");
        }

        assert!(pmatch[0].rm_so == 0, "Bad starting offset");
        assert!(pmatch[0].rm_eo == 13, "Bad ending offset");
        assert!(pmatch[1].rm_so == 5, "Bad starting offset for match group");
        assert!(pmatch[1].rm_eo == 12, "Bad ending offset for match group");
    }

    #[cfg(feature = "vendored")]
    #[test]
    fn regnexecb_with_embedded_nul() {
        let pattern = b"a\0b";
        let string = b"xxa\0byy";
        let mut preg = mem::MaybeUninit::<regex_t>::uninit();

        assert_eq!(
            unsafe {
                tre_regncompb(
                    preg.as_mut_ptr(),
                    pattern.as_ptr().cast(),
                    pattern.len(),
                    REG_EXTENDED as c_int,
                )
            },
            0
        );
        let mut preg = unsafe { preg.assume_init() };
        let mut pmatch = regmatch_t::default();

        assert_eq!(
            unsafe {
                tre_regnexecb(
                    &preg,
                    string.as_ptr().cast(),
                    string.len(),
                    1,
                    &mut pmatch,
                    0,
                )
            },
            0
        );
        assert_eq!(pmatch.rm_so, 2);
        assert_eq!(pmatch.rm_eo, 5);

        unsafe { tre_regfree(&mut preg) };
    }

    #[cfg(feature = "vendored")]
    #[test]
    fn regexecb_with_non_utf8_bytes() {
        let mut preg = mem::MaybeUninit::<regex_t>::uninit();
        assert_eq!(
            unsafe {
                tre_regcompb(
                    preg.as_mut_ptr(),
                    b"\xff\0".as_ptr().cast(),
                    REG_EXTENDED as c_int,
                )
            },
            0
        );
        let mut preg = unsafe { preg.assume_init() };
        let mut pmatch = regmatch_t::default();

        assert_eq!(
            unsafe { tre_regexecb(&preg, b"x\xff\0".as_ptr().cast(), 1, &mut pmatch, 0,) },
            0
        );
        assert_eq!(pmatch.rm_so, 1);
        assert_eq!(pmatch.rm_eo, 2);

        unsafe { tre_regfree(&mut preg) };
    }

    #[cfg(all(feature = "vendored", feature = "approx"))]
    #[test]
    fn regaexecb_with_non_utf8_bytes() {
        let mut preg = mem::MaybeUninit::<regex_t>::uninit();
        assert_eq!(
            unsafe {
                tre_regcompb(
                    preg.as_mut_ptr(),
                    b"\xff\0".as_ptr().cast(),
                    REG_EXTENDED as c_int,
                )
            },
            0
        );
        let mut preg = unsafe { preg.assume_init() };
        let params = regaparams_t {
            cost_ins: 1,
            cost_del: 1,
            cost_subst: 1,
            max_cost: 1,
            max_del: 1,
            max_ins: 1,
            max_subst: 1,
            max_err: 1,
        };
        let mut pmatch = regmatch_t::default();
        let mut amatch = regamatch_t {
            nmatch: 1,
            pmatch: &mut pmatch,
            ..Default::default()
        };

        assert_eq!(
            unsafe { tre_regaexecb(&preg, b"\xfe\0".as_ptr().cast(), &mut amatch, params, 0,) },
            0
        );
        assert_eq!(amatch.cost, 1);
        assert_eq!(pmatch.rm_so, 0);
        assert_eq!(pmatch.rm_eo, 1);

        unsafe { tre_regfree(&mut preg) };
    }

    #[test]
    fn reguexec() {
        use std::ffi::{c_int, c_uint, c_void};
        #[repr(C)]
        struct Data<'a>(pub &'a [u8], pub usize);

        #[inline(never)]
        #[unsafe(no_mangle)]
        unsafe extern "C" fn get_next_char(
            c: *mut tre_char_t,
            pos_add: *mut c_uint,
            context: *mut c_void,
        ) -> c_int {
            let data = context as *mut Data;
            let (string, i) = unsafe { ((*data).0, (*data).1) };

            if i >= string.len() {
                unsafe { *c = b'\0' as tre_char_t };
                return -1;
            }

            unsafe {
                *c = string[i] as tre_char_t;
                *pos_add = 1;
                (*data).1 += 1;
            }
            0
        }

        #[inline(never)]
        #[unsafe(no_mangle)]
        unsafe extern "C" fn rewind(pos: usize, context: *mut c_void) {
            let data = context as *mut Data;
            unsafe { (*data).1 = pos };
        }

        #[inline(never)]
        #[unsafe(no_mangle)]
        unsafe extern "C" fn compare(
            pos1: usize,
            pos2: usize,
            len: usize,
            context: *mut c_void,
        ) -> c_int {
            let data = context as *mut Data;
            let string = unsafe { (*data).0 };
            let slen = string.len();

            if pos1 > slen || pos2 > slen {
                return -1;
            }

            let mut i1_s = pos1;
            let mut i1_e = if i1_s + len > string.len() {
                slen - 1
            } else {
                i1_s + len
            };

            let mut i2_s = pos2;
            let mut i2_e = if i2_s + len > string.len() {
                slen - 1
            } else {
                i2_s + len
            };

            if (i1_s > i1_e || i2_s > i2_e) || ((i1_e - i1_s) != (i2_e - i2_s)) {
                // Different lengths, therefore unequal
                return -1;
            }

            if i1_s > i2_s {
                // Swap
                std::mem::swap(&mut i1_s, &mut i2_s);
                std::mem::swap(&mut i1_e, &mut i2_e);
            }

            if string[i1_s..i1_e] == string[i2_s..i2_e] {
                return 0;
            }

            -1
        }

        let mut preg = mem::MaybeUninit::<regex_t>::uninit();
        if unsafe {
            tre_regcomp(
                preg.as_mut_ptr(),
                b"(abracadabra)(\\1)*\0".as_ptr() as *const _,
                (REG_ICASE | REG_EXTENDED) as c_int,
            )
        } != 0
        {
            panic!("tre_regcomp");
        }
        let preg = unsafe { preg.assume_init() };

        let string = b"abracadabraabracadabra";
        let mut data = Data(string, 0);
        let source = tre_str_source {
            get_next_char: Some(get_next_char),
            rewind: Some(rewind),
            compare: Some(compare),
            context: &mut data as *mut _ as *mut c_void,
        };

        let mut matches = vec![regmatch_t::default(); 1];
        if unsafe { tre_reguexec(&preg, &source, 1, matches.as_mut_ptr(), 0) } != 0 {
            panic!("tre_reguexec");
        }

        assert_eq!(matches[0].rm_so, 0);
        assert_eq!(matches[0].rm_eo, 22);
    }
}
