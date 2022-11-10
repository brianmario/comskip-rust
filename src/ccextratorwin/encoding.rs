#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
pub type __darwin_ct_rune_t = libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn get_char_in_latin_1(buffer: *mut libc::c_uchar, c: libc::c_uchar) {
    let mut c1: libc::c_uchar = '?' as i32 as libc::c_uchar;
    if (c as libc::c_int) < 0x80 as libc::c_int {
        match c as libc::c_int {
            42 => {
                c1 = 0xe1 as libc::c_int as libc::c_uchar;
            }
            92 => {
                c1 = 0xe9 as libc::c_int as libc::c_uchar;
            }
            94 => {
                c1 = 0xed as libc::c_int as libc::c_uchar;
            }
            95 => {
                c1 = 0xf3 as libc::c_int as libc::c_uchar;
            }
            96 => {
                c1 = 0xfa as libc::c_int as libc::c_uchar;
            }
            123 => {
                c1 = 0xe7 as libc::c_int as libc::c_uchar;
            }
            124 => {
                c1 = 0xf7 as libc::c_int as libc::c_uchar;
            }
            125 => {
                c1 = 0xd1 as libc::c_int as libc::c_uchar;
            }
            126 => {
                c1 = 0xf1 as libc::c_int as libc::c_uchar;
            }
            _ => {
                c1 = c;
            }
        }
        *buffer = c1;
        return;
    }
    let mut current_block_89: u64;
    match c as libc::c_int {
        128 => {
            c1 = 0xae as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        129 => {
            c1 = 0xb0 as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        130 => {
            c1 = 0xbd as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        131 => {
            c1 = 0xbf as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        132 => {
            current_block_89 = 5710330377809666066;
        }
        133 => {
            c1 = 0xa2 as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        134 => {
            c1 = 0xa3 as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        135 => {
            c1 = 0xb6 as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        136 => {
            c1 = 0xe0 as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        137 => {
            c1 = 0x20 as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        138 => {
            c1 = 0xe8 as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        139 => {
            c1 = 0xe2 as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        140 => {
            c1 = 0xea as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        141 => {
            c1 = 0xee as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        142 => {
            c1 = 0xf4 as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        143 => {
            c1 = 0xfb as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        144 => {
            c1 = 0xc1 as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        145 => {
            c1 = 0xc9 as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        146 => {
            c1 = 0xd3 as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        147 => {
            c1 = 0xda as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        148 => {
            c1 = 0xdc as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        149 => {
            c1 = 0xfc as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        150 => {
            c1 = 0x27 as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        151 => {
            c1 = 0xa1 as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        152 => {
            c1 = 0x2a as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        153 => {
            c1 = 0x27 as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        154 => {
            c1 = 0x2d as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        155 => {
            c1 = 0xa9 as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        156 => {
            current_block_89 = 5710330377809666066;
        }
        157 => {
            c1 = 0x2e as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        158 => {
            c1 = 0x22 as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        159 => {
            c1 = 0x22 as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        160 => {
            c1 = 0xc0 as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        161 => {
            c1 = 0xc2 as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        162 => {
            c1 = 0xc7 as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        163 => {
            c1 = 0xc8 as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        164 => {
            c1 = 0xca as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        165 => {
            c1 = 0xcb as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        166 => {
            c1 = 0xeb as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        167 => {
            c1 = 0xce as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        168 => {
            c1 = 0xcf as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        169 => {
            c1 = 0xef as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        170 => {
            c1 = 0xd4 as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        171 => {
            c1 = 0xd9 as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        172 => {
            c1 = 0xf9 as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        173 => {
            c1 = 0xdb as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        174 => {
            c1 = 0xab as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        175 => {
            c1 = 0xbb as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        176 => {
            c1 = 0xc3 as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        177 => {
            c1 = 0xe3 as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        178 => {
            c1 = 0xcd as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        179 => {
            c1 = 0xcc as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        180 => {
            c1 = 0xec as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        181 => {
            c1 = 0xd2 as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        182 => {
            c1 = 0xf2 as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        183 => {
            c1 = 0xd5 as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        184 => {
            c1 = 0xf5 as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        185 => {
            c1 = 0x7b as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        186 => {
            c1 = 0x7d as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        187 => {
            c1 = 0x5c as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        188 => {
            c1 = 0x5e as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        189 => {
            c1 = 0x5f as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        190 => {
            c1 = 0xa6 as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        191 => {
            c1 = 0x7e as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        192 => {
            c1 = 0xc4 as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        193 => {
            c1 = 0xe3 as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        194 => {
            c1 = 0xd6 as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        195 => {
            c1 = 0xf6 as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        196 => {
            c1 = 0xdf as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        197 => {
            c1 = 0xa5 as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        198 => {
            c1 = 0xa4 as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        199 => {
            c1 = 0x7c as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        200 => {
            c1 = 0xc5 as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        201 => {
            c1 = 0xe5 as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        202 => {
            c1 = 0xd8 as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        203 => {
            c1 = 0xf8 as libc::c_int as libc::c_uchar;
            current_block_89 = 5710330377809666066;
        }
        204 => {
            current_block_89 = 4755981973329894317;
        }
        205 => {
            current_block_89 = 4755981973329894317;
        }
        206 => {
            current_block_89 = 13781690407670640666;
        }
        207 | _ => {
            current_block_89 = 14280379611056559621;
        }
    }
    match current_block_89 {
        4755981973329894317 => {
            current_block_89 = 13781690407670640666;
        }
        _ => {}
    }
    match current_block_89 {
        13781690407670640666 => {
            current_block_89 = 14280379611056559621;
        }
        _ => {}
    }
    match current_block_89 {
        14280379611056559621 => {
            *buffer = '?' as i32 as libc::c_uchar;
        }
        _ => {}
    }
    *buffer = c1;
}
#[no_mangle]
pub unsafe extern "C" fn get_char_in_unicode(buffer: *mut libc::c_uchar, c: libc::c_uchar) {
    let mut c1: libc::c_uchar = 0;
    let mut c2: libc::c_uchar = 0;
    match c as libc::c_int {
        132 => {
            c2 = 0x21 as libc::c_int as libc::c_uchar;
            c1 = 0x22 as libc::c_int as libc::c_uchar;
        }
        135 => {
            c2 = 0x26 as libc::c_int as libc::c_uchar;
            c1 = 0x6a as libc::c_int as libc::c_uchar;
        }
        156 => {
            c2 = 0x21 as libc::c_int as libc::c_uchar;
            c1 = 0x20 as libc::c_int as libc::c_uchar;
        }
        204 => {
            c2 = 0x23 as libc::c_int as libc::c_uchar;
            c1 = 0x1c as libc::c_int as libc::c_uchar;
        }
        205 => {
            c2 = 0x23 as libc::c_int as libc::c_uchar;
            c1 = 0x1d as libc::c_int as libc::c_uchar;
        }
        206 => {
            c2 = 0x23 as libc::c_int as libc::c_uchar;
            c1 = 0x1e as libc::c_int as libc::c_uchar;
        }
        207 => {
            c2 = 0x23 as libc::c_int as libc::c_uchar;
            c1 = 0x1f as libc::c_int as libc::c_uchar;
        }
        _ => {
            get_char_in_latin_1(&mut c1, c);
            c2 = 0 as libc::c_int as libc::c_uchar;
        }
    }
    *buffer = c1;
    *buffer.offset(1 as libc::c_int as isize) = c2;
}
#[no_mangle]
pub unsafe extern "C" fn get_char_in_utf_8(
    buffer: *mut libc::c_uchar,
    c: libc::c_uchar,
) -> libc::c_int {
    if (c as libc::c_int) < 0x80 as libc::c_int {
        match c as libc::c_int {
            42 => {
                *buffer = 0xc3 as libc::c_int as libc::c_uchar;
                *buffer.offset(1 as libc::c_int as isize) = 0xa1 as libc::c_int as libc::c_uchar;
                return 2 as libc::c_int;
            }
            92 => {
                *buffer = 0xc3 as libc::c_int as libc::c_uchar;
                *buffer.offset(1 as libc::c_int as isize) = 0xa9 as libc::c_int as libc::c_uchar;
                return 2 as libc::c_int;
            }
            94 => {
                *buffer = 0xc3 as libc::c_int as libc::c_uchar;
                *buffer.offset(1 as libc::c_int as isize) = 0xad as libc::c_int as libc::c_uchar;
                return 2 as libc::c_int;
            }
            95 => {
                *buffer = 0xc3 as libc::c_int as libc::c_uchar;
                *buffer.offset(1 as libc::c_int as isize) = 0xb3 as libc::c_int as libc::c_uchar;
                return 2 as libc::c_int;
            }
            96 => {
                *buffer = 0xc3 as libc::c_int as libc::c_uchar;
                *buffer.offset(1 as libc::c_int as isize) = 0xba as libc::c_int as libc::c_uchar;
                return 2 as libc::c_int;
            }
            123 => {
                *buffer = 0xc3 as libc::c_int as libc::c_uchar;
                *buffer.offset(1 as libc::c_int as isize) = 0xa7 as libc::c_int as libc::c_uchar;
                return 2 as libc::c_int;
            }
            124 => {
                *buffer = 0xc3 as libc::c_int as libc::c_uchar;
                *buffer.offset(1 as libc::c_int as isize) = 0xb7 as libc::c_int as libc::c_uchar;
                return 2 as libc::c_int;
            }
            125 => {
                *buffer = 0xc3 as libc::c_int as libc::c_uchar;
                *buffer.offset(1 as libc::c_int as isize) = 0x91 as libc::c_int as libc::c_uchar;
                return 2 as libc::c_int;
            }
            126 => {
                *buffer = 0xc3 as libc::c_int as libc::c_uchar;
                *buffer.offset(1 as libc::c_int as isize) = 0xb1 as libc::c_int as libc::c_uchar;
                return 2 as libc::c_int;
            }
            _ => {
                *buffer = c;
                return 1 as libc::c_int;
            }
        }
    }
    match c as libc::c_int {
        128 => {
            *buffer = 0xc2 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0xae as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        129 => {
            *buffer = 0xc2 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0xb0 as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        130 => {
            *buffer = 0xc2 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0xbd as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        131 => {
            *buffer = 0xc2 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0xbf as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        132 => {
            *buffer = 0xe2 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0x84 as libc::c_int as libc::c_uchar;
            *buffer.offset(2 as libc::c_int as isize) = 0xa2 as libc::c_int as libc::c_uchar;
            return 3 as libc::c_int;
        }
        133 => {
            *buffer = 0xc2 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0xa2 as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        134 => {
            *buffer = 0xc2 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0xa3 as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        135 => {
            *buffer = 0xe2 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0x99 as libc::c_int as libc::c_uchar;
            *buffer.offset(2 as libc::c_int as isize) = 0xaa as libc::c_int as libc::c_uchar;
            return 3 as libc::c_int;
        }
        136 => {
            *buffer = 0xc3 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0xa0 as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        137 => {
            *buffer = 0x20 as libc::c_int as libc::c_uchar;
            return 1 as libc::c_int;
        }
        138 => {
            *buffer = 0xc3 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0xa8 as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        139 => {
            *buffer = 0xc3 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0xa2 as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        140 => {
            *buffer = 0xc3 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0xaa as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        141 => {
            *buffer = 0xc3 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0xae as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        142 => {
            *buffer = 0xc3 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0xb4 as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        143 => {
            *buffer = 0xc3 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0xbb as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        144 => {
            *buffer = 0xc3 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0x81 as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        145 => {
            *buffer = 0xc3 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0x89 as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        146 => {
            *buffer = 0xc3 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0x93 as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        147 => {
            *buffer = 0xc3 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0x9a as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        148 => {
            *buffer = 0xc3 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0x9c as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        149 => {
            *buffer = 0xc3 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0xbc as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        150 => {
            *buffer = 0x27 as libc::c_int as libc::c_uchar;
            return 1 as libc::c_int;
        }
        151 => {
            *buffer = 0xc1 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0xa1 as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        152 => {
            *buffer = 0x2a as libc::c_int as libc::c_uchar;
            return 1 as libc::c_int;
        }
        153 => {
            *buffer = 0x27 as libc::c_int as libc::c_uchar;
            return 1 as libc::c_int;
        }
        154 => {
            *buffer = 0x2d as libc::c_int as libc::c_uchar;
            return 1 as libc::c_int;
        }
        155 => {
            *buffer = 0xc2 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0xa9 as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        156 => {
            *buffer = 0xe2 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0x84 as libc::c_int as libc::c_uchar;
            *buffer.offset(2 as libc::c_int as isize) = 0xa0 as libc::c_int as libc::c_uchar;
            return 3 as libc::c_int;
        }
        157 => {
            *buffer = 0x2e as libc::c_int as libc::c_uchar;
            return 1 as libc::c_int;
        }
        158 => {
            *buffer = 0x22 as libc::c_int as libc::c_uchar;
            return 1 as libc::c_int;
        }
        159 => {
            *buffer = 0x22 as libc::c_int as libc::c_uchar;
            return 1 as libc::c_int;
        }
        160 => {
            *buffer = 0xc3 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0x80 as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        161 => {
            *buffer = 0xc3 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0x82 as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        162 => {
            *buffer = 0xc3 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0x87 as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        163 => {
            *buffer = 0xc3 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0x88 as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        164 => {
            *buffer = 0xc3 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0x8a as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        165 => {
            *buffer = 0xc3 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0x8b as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        166 => {
            *buffer = 0xc3 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0xab as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        167 => {
            *buffer = 0xc3 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0x8e as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        168 => {
            *buffer = 0xc3 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0x8f as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        169 => {
            *buffer = 0xc3 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0xaf as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        170 => {
            *buffer = 0xc3 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0x94 as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        171 => {
            *buffer = 0xc3 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0x99 as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        172 => {
            *buffer = 0xc3 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0xb9 as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        173 => {
            *buffer = 0xc3 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0x9b as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        174 => {
            *buffer = 0xc2 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0xab as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        175 => {
            *buffer = 0xc2 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0xbb as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        176 => {
            *buffer = 0xc3 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0x83 as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        177 => {
            *buffer = 0xc3 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0xa3 as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        178 => {
            *buffer = 0xc3 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0x8d as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        179 => {
            *buffer = 0xc3 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0x8c as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        180 => {
            *buffer = 0xc3 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0xac as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        181 => {
            *buffer = 0xc3 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0x92 as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        182 => {
            *buffer = 0xc3 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0xb2 as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        183 => {
            *buffer = 0xc3 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0x95 as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        184 => {
            *buffer = 0xc3 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0xb5 as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        185 => {
            *buffer = 0x7b as libc::c_int as libc::c_uchar;
            return 1 as libc::c_int;
        }
        186 => {
            *buffer = 0x7d as libc::c_int as libc::c_uchar;
            return 1 as libc::c_int;
        }
        187 => {
            *buffer = 0x5c as libc::c_int as libc::c_uchar;
            return 1 as libc::c_int;
        }
        188 => {
            *buffer = 0x5e as libc::c_int as libc::c_uchar;
            return 1 as libc::c_int;
        }
        189 => {
            *buffer = 0x5f as libc::c_int as libc::c_uchar;
            return 1 as libc::c_int;
        }
        190 => {
            *buffer = 0xc2 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0xa6 as libc::c_int as libc::c_uchar;
            return 1 as libc::c_int;
        }
        191 => {
            *buffer = 0x7e as libc::c_int as libc::c_uchar;
            return 1 as libc::c_int;
        }
        192 => {
            *buffer = 0xc3 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0x84 as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        193 => {
            *buffer = 0xc3 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0xa4 as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        194 => {
            *buffer = 0xc3 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0x96 as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        195 => {
            *buffer = 0xc3 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0xb6 as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        196 => {
            *buffer = 0xc3 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0x9f as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        197 => {
            *buffer = 0xc2 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0xa5 as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        198 => {
            *buffer = 0xc2 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0xa4 as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        199 => {
            *buffer = 0x7c as libc::c_int as libc::c_uchar;
            return 1 as libc::c_int;
        }
        200 => {
            *buffer = 0xc3 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0x85 as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        201 => {
            *buffer = 0xc3 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0xa5 as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        202 => {
            *buffer = 0xc3 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0x98 as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        203 => {
            *buffer = 0xc3 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0xb8 as libc::c_int as libc::c_uchar;
            return 2 as libc::c_int;
        }
        204 => {
            *buffer = 0xe2 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0x8c as libc::c_int as libc::c_uchar;
            *buffer.offset(2 as libc::c_int as isize) = 0x9c as libc::c_int as libc::c_uchar;
            return 3 as libc::c_int;
        }
        205 => {
            *buffer = 0xe2 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0x8c as libc::c_int as libc::c_uchar;
            *buffer.offset(2 as libc::c_int as isize) = 0x9d as libc::c_int as libc::c_uchar;
            return 3 as libc::c_int;
        }
        206 => {
            *buffer = 0xe2 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0x8c as libc::c_int as libc::c_uchar;
            *buffer.offset(2 as libc::c_int as isize) = 0x9e as libc::c_int as libc::c_uchar;
            return 3 as libc::c_int;
        }
        207 => {
            *buffer = 0xe2 as libc::c_int as libc::c_uchar;
            *buffer.offset(1 as libc::c_int as isize) = 0x8c as libc::c_int as libc::c_uchar;
            *buffer.offset(2 as libc::c_int as isize) = 0x9f as libc::c_int as libc::c_uchar;
            return 3 as libc::c_int;
        }
        _ => {
            *buffer = '?' as i32 as libc::c_uchar;
            return 1 as libc::c_int;
        }
    };
}
use crate::ccextratorwin::ccextractor;
#[no_mangle]
pub unsafe extern "C" fn cctolower(c: libc::c_uchar) -> libc::c_uchar {
    if c as libc::c_int >= 'A' as i32 && c as libc::c_int <= 'Z' as i32 {
        return ccextractor::tolower(c as libc::c_int) as libc::c_uchar;
    }
    match c as libc::c_int {
        125 => return 0x7e as libc::c_int as libc::c_uchar,
        144 => return 0x2a as libc::c_int as libc::c_uchar,
        145 => return 0x5c as libc::c_int as libc::c_uchar,
        146 => return 0x5f as libc::c_int as libc::c_uchar,
        147 => return 0x60 as libc::c_int as libc::c_uchar,
        162 => return 0x7b as libc::c_int as libc::c_uchar,
        160 => return 0x88 as libc::c_int as libc::c_uchar,
        163 => return 0x8a as libc::c_int as libc::c_uchar,
        161 => return 0x8b as libc::c_int as libc::c_uchar,
        164 => return 0x8c as libc::c_int as libc::c_uchar,
        167 => return 0x8d as libc::c_int as libc::c_uchar,
        170 => return 0x8e as libc::c_int as libc::c_uchar,
        173 => return 0x8f as libc::c_int as libc::c_uchar,
        148 => return 0x95 as libc::c_int as libc::c_uchar,
        165 => return 0xa6 as libc::c_int as libc::c_uchar,
        168 => return 0xa9 as libc::c_int as libc::c_uchar,
        171 => return 0xac as libc::c_int as libc::c_uchar,
        176 => return 0xb1 as libc::c_int as libc::c_uchar,
        178 => return 0x5e as libc::c_int as libc::c_uchar,
        179 => return 0xb4 as libc::c_int as libc::c_uchar,
        181 => return 0xb6 as libc::c_int as libc::c_uchar,
        183 => return 0xb8 as libc::c_int as libc::c_uchar,
        192 => return 0xc1 as libc::c_int as libc::c_uchar,
        194 => return 0xc3 as libc::c_int as libc::c_uchar,
        200 => return 0xc9 as libc::c_int as libc::c_uchar,
        202 => return 0xcb as libc::c_int as libc::c_uchar,
        _ => {}
    }
    return c;
}
use crate::platform;
#[no_mangle]
pub unsafe extern "C" fn cctoupper(c: libc::c_uchar) -> libc::c_uchar {
    if c as libc::c_int >= 'a' as i32 && c as libc::c_int <= 'z' as i32 {
        return platform::toupper(c as libc::c_int) as libc::c_uchar;
    }
    match c as libc::c_int {
        126 => return 0x7d as libc::c_int as libc::c_uchar,
        42 => return 0x90 as libc::c_int as libc::c_uchar,
        92 => return 0x91 as libc::c_int as libc::c_uchar,
        94 => return 0xb2 as libc::c_int as libc::c_uchar,
        95 => return 0x92 as libc::c_int as libc::c_uchar,
        96 => return 0x93 as libc::c_int as libc::c_uchar,
        123 => return 0xa2 as libc::c_int as libc::c_uchar,
        136 => return 0xa0 as libc::c_int as libc::c_uchar,
        138 => return 0xa3 as libc::c_int as libc::c_uchar,
        139 => return 0xa1 as libc::c_int as libc::c_uchar,
        140 => return 0xa4 as libc::c_int as libc::c_uchar,
        141 => return 0xa7 as libc::c_int as libc::c_uchar,
        142 => return 0xaa as libc::c_int as libc::c_uchar,
        143 => return 0xad as libc::c_int as libc::c_uchar,
        149 => return 0x94 as libc::c_int as libc::c_uchar,
        166 => return 0xa5 as libc::c_int as libc::c_uchar,
        169 => return 0xa8 as libc::c_int as libc::c_uchar,
        172 => return 0xab as libc::c_int as libc::c_uchar,
        177 => return 0xb0 as libc::c_int as libc::c_uchar,
        180 => return 0xb3 as libc::c_int as libc::c_uchar,
        182 => return 0xb5 as libc::c_int as libc::c_uchar,
        184 => return 0xb7 as libc::c_int as libc::c_uchar,
        193 => return 0xc0 as libc::c_int as libc::c_uchar,
        195 => return 0xc2 as libc::c_int as libc::c_uchar,
        201 => return 0xc8 as libc::c_int as libc::c_uchar,
        203 => return 0xca as libc::c_int as libc::c_uchar,
        _ => {}
    }
    return c;
}
