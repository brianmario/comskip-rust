#![allow(
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    pub type __sFILEX;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn get_char_in_latin_1(buffer: *mut libc::c_uchar, c: libc::c_uchar);
    fn get_char_in_unicode(buffer: *mut libc::c_uchar, c: libc::c_uchar);
    fn get_char_in_utf_8(buffer: *mut libc::c_uchar, c: libc::c_uchar) -> libc::c_int;
    fn cctolower(c: libc::c_uchar) -> libc::c_uchar;
    fn cctoupper(c: libc::c_uchar) -> libc::c_uchar;
    static mut c1global: libc::c_uint;
    static mut c2global: libc::c_uint;
    static mut cc_channel: libc::c_int;
    static mut encoding: libc::c_int;
    static mut direct_rollup: libc::c_int;
    static mut subs_delay: LONG;
    static mut extraction_start: boundary_time;
    static mut screens_to_process: LONG;
    static mut processed_enough: libc::c_int;
    static mut nofontcolor: libc::c_int;
    static mut sentence_cap: libc::c_int;
    fn totalblockswritten_thisfile() -> libc::c_uint;
    static mut debug_608: libc::c_int;
    static mut spell_lower: *mut *mut libc::c_char;
    static mut spell_correct: *mut *mut libc::c_char;
    static mut spell_words: libc::c_int;
    static mut write_format: libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn __maskrune(_: __darwin_ct_rune_t, _: libc::c_ulong) -> libc::c_int;
    static mut _DefaultRuneLocale: _RuneLocale;
}
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_longlong;
pub type __darwin_ct_rune_t = libc::c_int;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_wchar_t = libc::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
pub type __darwin_off_t = __int64_t;
pub type size_t = __darwin_size_t;
pub type fpos_t = __darwin_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut libc::c_uchar,
    pub _size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sFILE {
    pub _p: *mut libc::c_uchar,
    pub _r: libc::c_int,
    pub _w: libc::c_int,
    pub _flags: libc::c_short,
    pub _file: libc::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: libc::c_int,
    pub _cookie: *mut libc::c_void,
    pub _close: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub _read: Option<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_char, libc::c_int) -> libc::c_int,
    >,
    pub _seek: Option<unsafe extern "C" fn(*mut libc::c_void, fpos_t, libc::c_int) -> fpos_t>,
    pub _write: Option<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, libc::c_int) -> libc::c_int,
    >,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: libc::c_int,
    pub _ubuf: [libc::c_uchar; 3],
    pub _nbuf: [libc::c_uchar; 1],
    pub _lb: __sbuf,
    pub _blksize: libc::c_int,
    pub _offset: fpos_t,
}
pub type FILE = __sFILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct s_write {
    pub fh: *mut FILE,
    pub filename: *mut libc::c_char,
    pub buffer: *mut libc::c_uchar,
    pub used: libc::c_int,
    pub data608: *mut eia608,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct eia608 {
    pub buffer1: eia608_screen,
    pub buffer2: eia608_screen,
    pub cursor_row: libc::c_int,
    pub cursor_column: libc::c_int,
    pub visible_buffer: libc::c_int,
    pub srt_counter: libc::c_int,
    pub screenfuls_counter: libc::c_int,
    pub current_visible_start_cc: libc::c_uint,
    pub mode: libc::c_int,
    pub last_c1: libc::c_uchar,
    pub last_c2: libc::c_uchar,
    pub channel: libc::c_int,
    pub color: libc::c_uchar,
    pub font: libc::c_uchar,
    pub rollup_base_row: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct eia608_screen {
    pub characters: [[libc::c_uchar; 33]; 15],
    pub colors: [[libc::c_uchar; 33]; 15],
    pub fonts: [[libc::c_uchar; 33]; 15],
    pub row_used: [libc::c_int; 15],
    pub empty: libc::c_int,
}
pub const ENC_UNICODE: encoding_type = 0;
pub type LONG = libc::c_long;
pub const COL_WHITE: color_code = 0;
pub const FONT_ITALICS: font_bits = 1;
pub const FONT_UNDERLINED: font_bits = 2;
pub const COL_USERDEFINED: color_code = 7;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct boundary_time {
    pub hh: libc::c_int,
    pub mm: libc::c_int,
    pub ss: libc::c_int,
    pub time_in_ms: LONG,
    pub time_in_ccblocks: LONG,
    pub set: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneLocale {
    pub __magic: [libc::c_char; 8],
    pub __encoding: [libc::c_char; 32],
    pub __sgetrune: Option<
        unsafe extern "C" fn(
            *const libc::c_char,
            __darwin_size_t,
            *mut *const libc::c_char,
        ) -> __darwin_rune_t,
    >,
    pub __sputrune: Option<
        unsafe extern "C" fn(
            __darwin_rune_t,
            *mut libc::c_char,
            __darwin_size_t,
            *mut *mut libc::c_char,
        ) -> libc::c_int,
    >,
    pub __invalid_rune: __darwin_rune_t,
    pub __runetype: [__uint32_t; 256],
    pub __maplower: [__darwin_rune_t; 256],
    pub __mapupper: [__darwin_rune_t; 256],
    pub __runetype_ext: _RuneRange,
    pub __maplower_ext: _RuneRange,
    pub __mapupper_ext: _RuneRange,
    pub __variable: *mut libc::c_void,
    pub __variable_len: libc::c_int,
    pub __ncharclasses: libc::c_int,
    pub __charclasses: *mut _RuneCharClass,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneCharClass {
    pub __name: [libc::c_char; 14],
    pub __mask: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneRange {
    pub __nranges: libc::c_int,
    pub __ranges: *mut _RuneEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneEntry {
    pub __min: __darwin_rune_t,
    pub __max: __darwin_rune_t,
    pub __map: __darwin_rune_t,
    pub __types: *mut __uint32_t,
}
pub const MODE_ROLLUP_4: cc_modes = 3;
pub const MODE_ROLLUP_3: cc_modes = 2;
pub const MODE_ROLLUP_2: cc_modes = 1;
pub const MODE_POPUP: cc_modes = 0;
pub const MODE_TEXT: cc_modes = 4;
pub const FONT_REGULAR: font_bits = 0;
pub const FONT_UNDERLINED_ITALICS: font_bits = 3;
pub const COL_MAGENTA: color_code = 6;
pub const COL_YELLOW: color_code = 5;
pub const COL_RED: color_code = 4;
pub const COL_CYAN: color_code = 3;
pub const COL_BLUE: color_code = 2;
pub const COL_GREEN: color_code = 1;
pub const COM_ENDOFCAPTION: command_code = 3;
pub const COM_ERASEDISPLAYEDMEMORY: command_code = 1;
pub const COM_ERASENONDISPLAYEDMEMORY: command_code = 11;
pub const COM_CARRIAGERETURN: command_code = 10;
pub const COM_ROLLUP4: command_code = 9;
pub const COM_ROLLUP3: command_code = 8;
pub const COM_ROLLUP2: command_code = 7;
pub const COM_RESUMETEXTDISPLAY: command_code = 13;
pub const COM_RESUMECAPTIONLOADING: command_code = 2;
pub const COM_TABOFFSET3: command_code = 6;
pub const COM_TABOFFSET2: command_code = 5;
pub const COM_TABOFFSET1: command_code = 4;
pub const COM_BACKSPACE: command_code = 12;
pub const COM_UNKNOWN: command_code = 0;
pub type cc_modes = libc::c_uint;
pub type color_code = libc::c_uint;
pub type font_bits = libc::c_uint;
pub type command_code = libc::c_uint;
pub type encoding_type = libc::c_uint;
#[inline]
unsafe extern "C" fn isascii(mut _c: libc::c_int) -> libc::c_int {
    return (_c & !(0x7f as libc::c_int) == 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn __istype(mut _c: __darwin_ct_rune_t, mut _f: libc::c_ulong) -> libc::c_int {
    return if isascii(_c) != 0 {
        (_DefaultRuneLocale.__runetype[_c as usize] as libc::c_ulong & _f != 0) as libc::c_int
    } else {
        (__maskrune(_c, _f) != 0) as libc::c_int
    };
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn ispunct(mut _c: libc::c_int) -> libc::c_int {
    return __istype(_c, 0x2000 as libc::c_long as libc::c_ulong);
}
#[no_mangle]
pub static mut rowdata: [libc::c_int; 16] = [
    11 as libc::c_int,
    -(1 as libc::c_int),
    1 as libc::c_int,
    2 as libc::c_int,
    3 as libc::c_int,
    4 as libc::c_int,
    12 as libc::c_int,
    13 as libc::c_int,
    14 as libc::c_int,
    15 as libc::c_int,
    5 as libc::c_int,
    6 as libc::c_int,
    7 as libc::c_int,
    8 as libc::c_int,
    9 as libc::c_int,
    10 as libc::c_int,
];
#[no_mangle]
pub static mut pac2_attribs: [[libc::c_uchar; 3]; 32] = [
    [
        COL_WHITE as libc::c_int as libc::c_uchar,
        FONT_REGULAR as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        COL_WHITE as libc::c_int as libc::c_uchar,
        FONT_UNDERLINED as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        COL_GREEN as libc::c_int as libc::c_uchar,
        FONT_REGULAR as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        COL_GREEN as libc::c_int as libc::c_uchar,
        FONT_UNDERLINED as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        COL_BLUE as libc::c_int as libc::c_uchar,
        FONT_REGULAR as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        COL_BLUE as libc::c_int as libc::c_uchar,
        FONT_UNDERLINED as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        COL_CYAN as libc::c_int as libc::c_uchar,
        FONT_REGULAR as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        COL_CYAN as libc::c_int as libc::c_uchar,
        FONT_UNDERLINED as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        COL_RED as libc::c_int as libc::c_uchar,
        FONT_REGULAR as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        COL_RED as libc::c_int as libc::c_uchar,
        FONT_UNDERLINED as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        COL_YELLOW as libc::c_int as libc::c_uchar,
        FONT_REGULAR as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        COL_YELLOW as libc::c_int as libc::c_uchar,
        FONT_UNDERLINED as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        COL_MAGENTA as libc::c_int as libc::c_uchar,
        FONT_REGULAR as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        COL_MAGENTA as libc::c_int as libc::c_uchar,
        FONT_UNDERLINED as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        COL_WHITE as libc::c_int as libc::c_uchar,
        FONT_ITALICS as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        COL_WHITE as libc::c_int as libc::c_uchar,
        FONT_UNDERLINED_ITALICS as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        COL_WHITE as libc::c_int as libc::c_uchar,
        FONT_REGULAR as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        COL_WHITE as libc::c_int as libc::c_uchar,
        FONT_UNDERLINED as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        COL_WHITE as libc::c_int as libc::c_uchar,
        FONT_REGULAR as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
    ],
    [
        COL_WHITE as libc::c_int as libc::c_uchar,
        FONT_UNDERLINED as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
    ],
    [
        COL_WHITE as libc::c_int as libc::c_uchar,
        FONT_REGULAR as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
    ],
    [
        COL_WHITE as libc::c_int as libc::c_uchar,
        FONT_UNDERLINED as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
    ],
    [
        COL_WHITE as libc::c_int as libc::c_uchar,
        FONT_REGULAR as libc::c_int as libc::c_uchar,
        12 as libc::c_int as libc::c_uchar,
    ],
    [
        COL_WHITE as libc::c_int as libc::c_uchar,
        FONT_UNDERLINED as libc::c_int as libc::c_uchar,
        12 as libc::c_int as libc::c_uchar,
    ],
    [
        COL_WHITE as libc::c_int as libc::c_uchar,
        FONT_REGULAR as libc::c_int as libc::c_uchar,
        16 as libc::c_int as libc::c_uchar,
    ],
    [
        COL_WHITE as libc::c_int as libc::c_uchar,
        FONT_UNDERLINED as libc::c_int as libc::c_uchar,
        16 as libc::c_int as libc::c_uchar,
    ],
    [
        COL_WHITE as libc::c_int as libc::c_uchar,
        FONT_REGULAR as libc::c_int as libc::c_uchar,
        20 as libc::c_int as libc::c_uchar,
    ],
    [
        COL_WHITE as libc::c_int as libc::c_uchar,
        FONT_UNDERLINED as libc::c_int as libc::c_uchar,
        20 as libc::c_int as libc::c_uchar,
    ],
    [
        COL_WHITE as libc::c_int as libc::c_uchar,
        FONT_REGULAR as libc::c_int as libc::c_uchar,
        24 as libc::c_int as libc::c_uchar,
    ],
    [
        COL_WHITE as libc::c_int as libc::c_uchar,
        FONT_UNDERLINED as libc::c_int as libc::c_uchar,
        24 as libc::c_int as libc::c_uchar,
    ],
    [
        COL_WHITE as libc::c_int as libc::c_uchar,
        FONT_REGULAR as libc::c_int as libc::c_uchar,
        28 as libc::c_int as libc::c_uchar,
    ],
    [
        COL_WHITE as libc::c_int as libc::c_uchar,
        FONT_UNDERLINED as libc::c_int as libc::c_uchar,
        28 as libc::c_int as libc::c_uchar,
    ],
];
#[no_mangle]
pub static mut in_xds_mode: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut enc_buffer: [libc::c_uchar; 2048] = [0; 2048];
#[no_mangle]
pub static mut str: [libc::c_uchar; 2048] = [0; 2048];
#[no_mangle]
pub static mut enc_buffer_used: libc::c_uint = 0;
#[no_mangle]
pub static mut encoded_crlf: [libc::c_uchar; 16] = [0; 16];
#[no_mangle]
pub static mut encoded_crlf_length: libc::c_uint = 0;
#[no_mangle]
pub static mut encoded_br: [libc::c_uchar; 16] = [0; 16];
#[no_mangle]
pub static mut encoded_br_length: libc::c_uint = 0;
#[no_mangle]
pub static mut subline: *mut libc::c_uchar = 0 as *const libc::c_uchar as *mut libc::c_uchar;
#[no_mangle]
pub static mut new_sentence: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut usercolor_rgb: [libc::c_uchar; 8] =
    unsafe { *::std::mem::transmute::<&[u8; 8], &mut [libc::c_uchar; 8]>(b"\0\0\0\0\0\0\0\0") };
#[no_mangle]
pub static mut default_color: libc::c_int = COL_WHITE as libc::c_int;
#[no_mangle]
pub static mut command_type: [*const libc::c_char; 14] = [
    b"Unknown\0" as *const u8 as *const libc::c_char,
    b"EDM - EraseDisplayedMemory\0" as *const u8 as *const libc::c_char,
    b"RCL - ResumeCaptionLoading\0" as *const u8 as *const libc::c_char,
    b"EOC - End Of Caption\0" as *const u8 as *const libc::c_char,
    b"TO1 - Tab Offset, 1 column\0" as *const u8 as *const libc::c_char,
    b"TO2 - Tab Offset, 2 column\0" as *const u8 as *const libc::c_char,
    b"TO3 - Tab Offset, 3 column\0" as *const u8 as *const libc::c_char,
    b"RU2 - Roll up 2 rows\0" as *const u8 as *const libc::c_char,
    b"RU3 - Roll up 3 rows\0" as *const u8 as *const libc::c_char,
    b"RU4 - Roll up 4 rows\0" as *const u8 as *const libc::c_char,
    b"CR  - Carriage Return\0" as *const u8 as *const libc::c_char,
    b"ENM - Erase non-displayed memory\0" as *const u8 as *const libc::c_char,
    b"BS  - Backspace\0" as *const u8 as *const libc::c_char,
    b"RTD - Resume Text Display\0" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub static mut font_text: [*const libc::c_char; 4] = [
    b"regular\0" as *const u8 as *const libc::c_char,
    b"italics\0" as *const u8 as *const libc::c_char,
    b"underlined\0" as *const u8 as *const libc::c_char,
    b"underlined italics\0" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub static mut cc_modes_text: [*const libc::c_char; 1] =
    [b"Pop-Up captions\0" as *const u8 as *const libc::c_char];
#[no_mangle]
pub static mut color_text: [[*const libc::c_char; 2]; 8] = [
    [
        b"white\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    ],
    [
        b"green\0" as *const u8 as *const libc::c_char,
        b"<font color=\"#00ff00\">\0" as *const u8 as *const libc::c_char,
    ],
    [
        b"blue\0" as *const u8 as *const libc::c_char,
        b"<font color=\"#0000ff\">\0" as *const u8 as *const libc::c_char,
    ],
    [
        b"cyan\0" as *const u8 as *const libc::c_char,
        b"<font color=\"#00ffff\">\0" as *const u8 as *const libc::c_char,
    ],
    [
        b"red\0" as *const u8 as *const libc::c_char,
        b"<font color=\"#ff0000\">\0" as *const u8 as *const libc::c_char,
    ],
    [
        b"yellow\0" as *const u8 as *const libc::c_char,
        b"<font color=\"#ffff00\">\0" as *const u8 as *const libc::c_char,
    ],
    [
        b"magenta\0" as *const u8 as *const libc::c_char,
        b"<font color=\"#ff00ff\">\0" as *const u8 as *const libc::c_char,
    ],
    [
        b"userdefined\0" as *const u8 as *const libc::c_char,
        b"<font color=\"\0" as *const u8 as *const libc::c_char,
    ],
];
#[no_mangle]
pub unsafe extern "C" fn encode_line(
    mut buffer: *mut libc::c_uchar,
    mut text: *mut libc::c_uchar,
) -> libc::c_uint {
    let mut bytes: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while *text != 0 {
        match encoding {
            2 | 1 => {
                *buffer = *text;
                bytes = bytes.wrapping_add(1);
                buffer = buffer.offset(1);
            }
            0 => {
                *buffer = *text;
                *buffer.offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar;
                bytes = bytes.wrapping_add(2 as libc::c_int as libc::c_uint);
                buffer = buffer.offset(2 as libc::c_int as isize);
            }
            _ => {}
        }
        text = text.offset(1);
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn correct_case(mut line_num: libc::c_int, mut data: *mut eia608_screen) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < spell_words {
        let mut c: *mut libc::c_char =
            ((*data).characters[line_num as usize]).as_mut_ptr() as *mut libc::c_char;
        let mut len: size_t = strlen(*spell_correct.offset(i as isize));
        loop {
            c = strstr(c, *spell_lower.offset(i as isize));
            if c.is_null() {
                break;
            }
            let mut prev: libc::c_uchar = (if c
                == ((*data).characters[line_num as usize]).as_mut_ptr() as *mut libc::c_char
            {
                ' ' as i32
            } else {
                *c.offset(-(1 as libc::c_int as isize)) as libc::c_int
            }) as libc::c_uchar;
            let mut next: libc::c_uchar = *c.offset(len as isize) as libc::c_uchar;
            if (prev as libc::c_int == ' ' as i32
                || prev as libc::c_int == 0x89 as libc::c_int
                || ispunct(prev as libc::c_int) != 0)
                && (next as libc::c_int == ' ' as i32
                    || next as libc::c_int == 0x89 as libc::c_int
                    || ispunct(next as libc::c_int) != 0)
            {
                memcpy(
                    c as *mut libc::c_void,
                    *spell_correct.offset(i as isize) as *const libc::c_void,
                    len,
                );
            }
            c = c.offset(1);
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn capitalize(mut line_num: libc::c_int, mut data: *mut eia608_screen) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        let mut current_block_5: u64;
        match (*data).characters[line_num as usize][i as usize] as libc::c_int {
            32 | 137 => {
                current_block_5 = 13536709405535804910;
            }
            46 => {
                current_block_5 = 3739096821823288865;
            }
            63 | 33 => {
                current_block_5 = 3739096821823288865;
            }
            _ => {
                if new_sentence != 0 {
                    (*data).characters[line_num as usize][i as usize] =
                        cctoupper((*data).characters[line_num as usize][i as usize]);
                } else {
                    (*data).characters[line_num as usize][i as usize] =
                        cctolower((*data).characters[line_num as usize][i as usize]);
                }
                new_sentence = 0 as libc::c_int;
                current_block_5 = 13536709405535804910;
            }
        }
        match current_block_5 {
            3739096821823288865 => {
                new_sentence = 1 as libc::c_int;
            }
            _ => {}
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn get_decoder_line_encoded(
    mut buffer: *mut libc::c_uchar,
    mut line_num: libc::c_int,
    mut data: *mut eia608_screen,
) -> libc::c_uint {
    let mut is_underlined: libc::c_int = 0;
    let mut col: libc::c_int = COL_WHITE as libc::c_int;
    let mut underlined: libc::c_int = 0 as libc::c_int;
    let mut italics: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut bytes: libc::c_int = 0 as libc::c_int;
    let mut has_ita: libc::c_int = 0;
    let mut line: *mut libc::c_uchar = ((*data).characters[line_num as usize]).as_mut_ptr();
    let mut orig: *mut libc::c_uchar = buffer;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        let mut its_col: libc::c_int = (*data).colors[line_num as usize][i as usize] as libc::c_int;
        if its_col != col && nofontcolor == 0 {
            if col != COL_WHITE as libc::c_int {
                buffer = buffer.offset(encode_line(
                    buffer,
                    b"</font>\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar,
                ) as isize);
            }
            buffer = buffer.offset(encode_line(
                buffer,
                color_text[its_col as usize][1 as libc::c_int as usize] as *mut libc::c_uchar,
            ) as isize);
            if its_col == COL_USERDEFINED as libc::c_int {
                buffer = buffer.offset(encode_line(buffer, usercolor_rgb.as_mut_ptr()) as isize);
                buffer = buffer.offset(encode_line(
                    buffer,
                    b"\">\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar,
                ) as isize);
            }
            col = its_col;
        }
        is_underlined = (*data).fonts[line_num as usize][i as usize] as libc::c_int
            & FONT_UNDERLINED as libc::c_int;
        if is_underlined != 0 && underlined == 0 as libc::c_int {
            buffer = buffer.offset(encode_line(
                buffer,
                b"<u>\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar,
            ) as isize);
        }
        if is_underlined == 0 as libc::c_int && underlined != 0 {
            buffer = buffer.offset(encode_line(
                buffer,
                b"</u>\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar,
            ) as isize);
        }
        underlined = is_underlined;
        has_ita = (*data).fonts[line_num as usize][i as usize] as libc::c_int
            & FONT_ITALICS as libc::c_int;
        if has_ita != 0 && italics == 0 as libc::c_int {
            buffer = buffer.offset(encode_line(
                buffer,
                b"<i>\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar,
            ) as isize);
        }
        if has_ita == 0 as libc::c_int && italics != 0 {
            buffer = buffer.offset(encode_line(
                buffer,
                b"</i>\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar,
            ) as isize);
        }
        italics = has_ita;
        bytes = 0 as libc::c_int;
        match encoding {
            2 => {
                bytes = get_char_in_utf_8(buffer, *line.offset(i as isize));
            }
            1 => {
                get_char_in_latin_1(buffer, *line.offset(i as isize));
                bytes = 1 as libc::c_int;
            }
            0 => {
                get_char_in_unicode(buffer, *line.offset(i as isize));
                bytes = 2 as libc::c_int;
            }
            _ => {}
        }
        buffer = buffer.offset(bytes as isize);
        i += 1;
    }
    if italics != 0 {
        buffer = buffer.offset(encode_line(
            buffer,
            b"</i>\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar,
        ) as isize);
    }
    if underlined != 0 {
        buffer = buffer.offset(encode_line(
            buffer,
            b"</u>\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar,
        ) as isize);
    }
    if col != COL_WHITE as libc::c_int && nofontcolor == 0 {
        buffer = buffer.offset(encode_line(
            buffer,
            b"</font>\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar,
        ) as isize);
    }
    *buffer = 0 as libc::c_int as libc::c_uchar;
    return buffer.offset_from(orig) as libc::c_long as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn clear_eia608_cc_buffer(mut data: *mut eia608_screen) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 15 as libc::c_int {
        memset(
            ((*data).characters[i as usize]).as_mut_ptr() as *mut libc::c_void,
            ' ' as i32,
            32 as libc::c_int as libc::c_ulong,
        );
        (*data).characters[i as usize][32 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_uchar;
        memset(
            ((*data).colors[i as usize]).as_mut_ptr() as *mut libc::c_void,
            default_color,
            33 as libc::c_int as libc::c_ulong,
        );
        memset(
            ((*data).fonts[i as usize]).as_mut_ptr() as *mut libc::c_void,
            FONT_REGULAR as libc::c_int,
            33 as libc::c_int as libc::c_ulong,
        );
        (*data).row_used[i as usize] = 0 as libc::c_int;
        (*data).empty = 1 as libc::c_int;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn init_eia608(mut data: *mut eia608) {
    (*data).cursor_column = 0 as libc::c_int;
    (*data).cursor_row = 0 as libc::c_int;
    clear_eia608_cc_buffer(&mut (*data).buffer1);
    clear_eia608_cc_buffer(&mut (*data).buffer2);
    (*data).visible_buffer = 1 as libc::c_int;
    (*data).last_c1 = 0 as libc::c_int as libc::c_uchar;
    (*data).last_c2 = 0 as libc::c_int as libc::c_uchar;
    (*data).mode = MODE_POPUP as libc::c_int;
    (*data).current_visible_start_cc = 0 as libc::c_int as libc::c_uint;
    (*data).srt_counter = 0 as libc::c_int;
    (*data).screenfuls_counter = 0 as libc::c_int;
    (*data).channel = 1 as libc::c_int;
    (*data).color = default_color as libc::c_uchar;
    (*data).font = FONT_REGULAR as libc::c_int as libc::c_uchar;
    (*data).rollup_base_row = 14 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn get_writing_buffer(mut wb: *mut s_write) -> *mut eia608_screen {
    let mut use_buffer: *mut eia608_screen = 0 as *mut eia608_screen;
    match (*(*wb).data608).mode {
        0 => {
            if (*(*wb).data608).visible_buffer == 1 as libc::c_int {
                use_buffer = &mut (*(*wb).data608).buffer2;
            } else {
                use_buffer = &mut (*(*wb).data608).buffer1;
            }
        }
        1 | 2 | 3 => {
            if (*(*wb).data608).visible_buffer == 1 as libc::c_int {
                use_buffer = &mut (*(*wb).data608).buffer1;
            } else {
                use_buffer = &mut (*(*wb).data608).buffer2;
            }
        }
        _ => {
            printf(
                b"Caption mode has an illegal value at get_writing_buffer(), this is a bug.\n\0"
                    as *const u8 as *const libc::c_char,
            );
            exit(500 as libc::c_int);
        }
    }
    return use_buffer;
}
#[no_mangle]
pub unsafe extern "C" fn write_char(c: libc::c_uchar, mut wb: *mut s_write) {
    if (*(*wb).data608).mode != MODE_TEXT as libc::c_int {
        let mut use_buffer: *mut eia608_screen = get_writing_buffer(wb);
        (*use_buffer).characters[(*(*wb).data608).cursor_row as usize]
            [(*(*wb).data608).cursor_column as usize] = c;
        (*use_buffer).colors[(*(*wb).data608).cursor_row as usize]
            [(*(*wb).data608).cursor_column as usize] = (*(*wb).data608).color;
        (*use_buffer).fonts[(*(*wb).data608).cursor_row as usize]
            [(*(*wb).data608).cursor_column as usize] = (*(*wb).data608).font;
        (*use_buffer).row_used[(*(*wb).data608).cursor_row as usize] = 1 as libc::c_int;
        (*use_buffer).empty = 0 as libc::c_int;
        if (*(*wb).data608).cursor_column < 31 as libc::c_int {
            let ref mut fresh0 = (*(*wb).data608).cursor_column;
            *fresh0 += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn handle_text_attr(
    c1: libc::c_uchar,
    c2: libc::c_uchar,
    mut wb: *mut s_write,
) {
    if debug_608 != 0 {
        printf(
            b"\r608: text_attr: %02X %02X\n\0" as *const u8 as *const libc::c_char,
            c1 as libc::c_int,
            c2 as libc::c_int,
        );
    }
    if (c1 as libc::c_int != 0x11 as libc::c_int && c1 as libc::c_int != 0x19 as libc::c_int
        || ((c2 as libc::c_int) < 0x20 as libc::c_int || c2 as libc::c_int > 0x2f as libc::c_int))
        && debug_608 != 0
    {
        printf(b"\rThis is not a text attribute!\n\0" as *const u8 as *const libc::c_char);
    } else {
        let mut i: libc::c_int = c2 as libc::c_int - 0x20 as libc::c_int;
        (*(*wb).data608).color = pac2_attribs[i as usize][0 as libc::c_int as usize];
        (*(*wb).data608).font = pac2_attribs[i as usize][1 as libc::c_int as usize];
        if debug_608 != 0 {
            printf(
                b"\rColor: %s,  font: %s\n\0" as *const u8 as *const libc::c_char,
                color_text[(*(*wb).data608).color as usize][0 as libc::c_int as usize],
                font_text[(*(*wb).data608).font as usize],
            );
        }
        if (*(*wb).data608).cursor_column < 31 as libc::c_int {
            let ref mut fresh1 = (*(*wb).data608).cursor_column;
            *fresh1 += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn mstotime(
    mut milli: LONG,
    mut hours: *mut libc::c_uint,
    mut minutes: *mut libc::c_uint,
    mut seconds: *mut libc::c_uint,
    mut ms: *mut libc::c_uint,
) {
    *ms = (milli % 1000 as libc::c_int as libc::c_long) as libc::c_uint;
    milli = (milli - *ms as libc::c_long) / 1000 as libc::c_int as libc::c_long;
    *seconds = (milli % 60 as libc::c_int as libc::c_long) as libc::c_int as libc::c_uint;
    milli = (milli - *seconds as libc::c_long) / 60 as libc::c_int as libc::c_long;
    *minutes = (milli % 60 as libc::c_int as libc::c_long) as libc::c_int as libc::c_uint;
    milli = (milli - *minutes as libc::c_long) / 60 as libc::c_int as libc::c_long;
    *hours = milli as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn write_subtitle_file_footer(mut wb: *mut s_write) {
    match write_format {
        2 => {
            sprintf(
                str.as_mut_ptr() as *mut libc::c_char,
                b"<BODY><SAMI>\n\0" as *const u8 as *const libc::c_char,
            );
            if debug_608 != 0 && encoding != ENC_UNICODE as libc::c_int {
                printf(
                    b"\r%s\n\0" as *const u8 as *const libc::c_char,
                    str.as_mut_ptr(),
                );
            }
            enc_buffer_used = encode_line(enc_buffer.as_mut_ptr(), str.as_mut_ptr());
            fwrite(
                enc_buffer.as_mut_ptr() as *const libc::c_void,
                enc_buffer_used as libc::c_ulong,
                1 as libc::c_int as libc::c_ulong,
                (*wb).fh,
            );
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn fprintf_encoded(mut fh: *mut FILE, mut string: *mut libc::c_char) {
    enc_buffer_used = encode_line(enc_buffer.as_mut_ptr(), string as *mut libc::c_uchar);
    fwrite(
        enc_buffer.as_mut_ptr() as *const libc::c_void,
        enc_buffer_used as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        fh,
    );
}
#[no_mangle]
pub unsafe extern "C" fn write_subtitle_file_header(mut wb: *mut s_write) {
    match write_format {
        2 => {
            fprintf_encoded(
                (*wb).fh,
                b"<SAMI>\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            fprintf_encoded(
                (*wb).fh,
                b"<HEAD>\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            fprintf_encoded(
                (*wb).fh,
                b"<STYLE TYPE=\"text/css\">\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            fprintf_encoded(
                (*wb).fh,
                b"<--\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            fprintf_encoded(
                (*wb).fh,
                b"P {margin-left: 16pt; margin-right: 16pt; margin-bottom: 16pt; margin-top: 16pt;\n\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            fprintf_encoded(
                (*wb).fh,
                b"   text-align: center; font-size: 18pt; font-family: arial; font-weight: bold; color: #f0f0f0;}\n\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            fprintf_encoded(
                (*wb).fh,
                b".UNKNOWNCC {Name:Unknown; lang:en-US; SAMIType:CC;}\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
            fprintf_encoded(
                (*wb).fh,
                b"-->\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            fprintf_encoded(
                (*wb).fh,
                b"</STYLE>\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            fprintf_encoded(
                (*wb).fh,
                b"</HEAD>\n\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            fprintf_encoded(
                (*wb).fh,
                b"<BODY>\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        1 | _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn write_cc_buffer_as_srt(
    mut data: *mut eia608_screen,
    mut wb: *mut s_write,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut h1: libc::c_uint = 0;
    let mut m1: libc::c_uint = 0;
    let mut s1: libc::c_uint = 0;
    let mut ms1: libc::c_uint = 0;
    let mut h2: libc::c_uint = 0;
    let mut m2: libc::c_uint = 0;
    let mut s2: libc::c_uint = 0;
    let mut ms2: libc::c_uint = 0;
    let mut wrote_something: libc::c_int = 0 as libc::c_int;
    let mut pg: libc::c_uint = if c1global != 0 { c1global } else { c2global };
    let mut timeline: [libc::c_char; 128] = [0; 128];
    let mut ms_end: libc::c_long = 0;
    let mut ms_start: LONG = (((*(*wb).data608).current_visible_start_cc)
        .wrapping_add(pg)
        .wrapping_mul(1000 as libc::c_int as libc::c_uint)
        as libc::c_double
        / 29.97f64) as LONG;
    if extraction_start.set != 0 && extraction_start.time_in_ms > ms_start {
        return 0 as libc::c_int;
    }
    ms_start += subs_delay;
    if ms_start < 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int;
    }
    ms_end = ((totalblockswritten_thisfile())
        .wrapping_add(pg)
        .wrapping_mul(1000 as libc::c_int as libc::c_uint) as libc::c_double
        / 29.97f64) as libc::c_long
        + subs_delay;
    mstotime(ms_start, &mut h1, &mut m1, &mut s1, &mut ms1);
    mstotime(
        ms_end - 1 as libc::c_int as libc::c_long,
        &mut h2,
        &mut m2,
        &mut s2,
        &mut ms2,
    );
    let ref mut fresh2 = (*(*wb).data608).srt_counter;
    *fresh2 += 1;
    sprintf(
        timeline.as_mut_ptr(),
        b"%u\r\n\0" as *const u8 as *const libc::c_char,
        (*(*wb).data608).srt_counter,
    );
    enc_buffer_used = encode_line(
        enc_buffer.as_mut_ptr(),
        timeline.as_mut_ptr() as *mut libc::c_uchar,
    );
    fwrite(
        enc_buffer.as_mut_ptr() as *const libc::c_void,
        enc_buffer_used as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        (*wb).fh,
    );
    sprintf(
        timeline.as_mut_ptr(),
        b"%02u:%02u:%02u,%03u --> %02u:%02u:%02u,%03u\r\n\0" as *const u8 as *const libc::c_char,
        h1,
        m1,
        s1,
        ms1,
        h2,
        m2,
        s2,
        ms2,
    );
    enc_buffer_used = encode_line(
        enc_buffer.as_mut_ptr(),
        timeline.as_mut_ptr() as *mut libc::c_uchar,
    );
    if debug_608 != 0 {
        printf(b"\r\0" as *const u8 as *const libc::c_char);
        printf(
            b"%s\0" as *const u8 as *const libc::c_char,
            timeline.as_mut_ptr(),
        );
    }
    fwrite(
        enc_buffer.as_mut_ptr() as *const libc::c_void,
        enc_buffer_used as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        (*wb).fh,
    );
    i = 0 as libc::c_int;
    while i < 15 as libc::c_int {
        if (*data).row_used[i as usize] != 0 {
            let mut length: libc::c_int = 0;
            if sentence_cap != 0 {
                capitalize(i, data);
                correct_case(i, data);
            }
            length = get_decoder_line_encoded(subline, i, data) as libc::c_int;
            if debug_608 != 0 && encoding != ENC_UNICODE as libc::c_int {
                printf(b"\r\0" as *const u8 as *const libc::c_char);
                printf(b"%s\n\0" as *const u8 as *const libc::c_char, subline);
            }
            fwrite(
                subline as *const libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                length as libc::c_ulong,
                (*wb).fh,
            );
            fwrite(
                encoded_crlf.as_mut_ptr() as *const libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                encoded_crlf_length as libc::c_ulong,
                (*wb).fh,
            );
            wrote_something = 1 as libc::c_int;
        }
        i += 1;
    }
    if debug_608 != 0 {
        printf(b"\r\n\0" as *const u8 as *const libc::c_char);
    }
    fwrite(
        encoded_crlf.as_mut_ptr() as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        encoded_crlf_length as libc::c_ulong,
        (*wb).fh,
    );
    return wrote_something;
}
#[no_mangle]
pub unsafe extern "C" fn write_cc_buffer_as_sami(
    mut data: *mut eia608_screen,
    mut wb: *mut s_write,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut wrote_something: libc::c_int = 0 as libc::c_int;
    let mut pg: libc::c_uint = if c1global != 0 { c1global } else { c2global };
    let mut startms: LONG = (((*(*wb).data608).current_visible_start_cc)
        .wrapping_add(pg)
        .wrapping_mul(1000 as libc::c_int as libc::c_uint)
        as libc::c_double
        / 29.97f64) as LONG;
    let mut endms: LONG = 0;
    if extraction_start.set != 0 && extraction_start.time_in_ms > startms {
        return 0 as libc::c_int;
    }
    startms += subs_delay;
    if startms < 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int;
    }
    endms = ((totalblockswritten_thisfile())
        .wrapping_add(pg)
        .wrapping_mul(1000 as libc::c_int as libc::c_uint) as libc::c_double
        / 29.97f64) as LONG
        + subs_delay;
    endms -= 1;
    sprintf(
        str.as_mut_ptr() as *mut libc::c_char,
        b"<SYNC start=\"%ld\"><P class=\"UNKNOWNCC\">\r\n\0" as *const u8 as *const libc::c_char,
        startms,
    );
    if debug_608 != 0 && encoding != ENC_UNICODE as libc::c_int {
        printf(
            b"\r%s\n\0" as *const u8 as *const libc::c_char,
            str.as_mut_ptr(),
        );
    }
    enc_buffer_used = encode_line(enc_buffer.as_mut_ptr(), str.as_mut_ptr());
    fwrite(
        enc_buffer.as_mut_ptr() as *const libc::c_void,
        enc_buffer_used as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        (*wb).fh,
    );
    i = 0 as libc::c_int;
    while i < 15 as libc::c_int {
        if (*data).row_used[i as usize] != 0 {
            let mut length: libc::c_int = get_decoder_line_encoded(subline, i, data) as libc::c_int;
            if debug_608 != 0 && encoding != ENC_UNICODE as libc::c_int {
                printf(b"\r\0" as *const u8 as *const libc::c_char);
                printf(b"%s\n\0" as *const u8 as *const libc::c_char, subline);
            }
            fwrite(
                subline as *const libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                length as libc::c_ulong,
                (*wb).fh,
            );
            wrote_something = 1 as libc::c_int;
            if i != 14 as libc::c_int {
                fwrite(
                    encoded_br.as_mut_ptr() as *const libc::c_void,
                    1 as libc::c_int as libc::c_ulong,
                    encoded_br_length as libc::c_ulong,
                    (*wb).fh,
                );
            }
            fwrite(
                encoded_crlf.as_mut_ptr() as *const libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                encoded_crlf_length as libc::c_ulong,
                (*wb).fh,
            );
        }
        i += 1;
    }
    sprintf(
        str.as_mut_ptr() as *mut libc::c_char,
        b"</P></SYNC>\r\n\0" as *const u8 as *const libc::c_char,
    );
    if debug_608 != 0 && encoding != ENC_UNICODE as libc::c_int {
        printf(
            b"\r%s\n\0" as *const u8 as *const libc::c_char,
            str.as_mut_ptr(),
        );
    }
    enc_buffer_used = encode_line(enc_buffer.as_mut_ptr(), str.as_mut_ptr());
    fwrite(
        enc_buffer.as_mut_ptr() as *const libc::c_void,
        enc_buffer_used as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        (*wb).fh,
    );
    sprintf(
        str.as_mut_ptr() as *mut libc::c_char,
        b"<SYNC start=\"%ld\"><P class=\"UNKNOWNCC\">&nbsp</P></SYNC>\r\n\r\n\0" as *const u8
            as *const libc::c_char,
        endms,
    );
    if debug_608 != 0 && encoding != ENC_UNICODE as libc::c_int {
        printf(
            b"\r%s\n\0" as *const u8 as *const libc::c_char,
            str.as_mut_ptr(),
        );
    }
    enc_buffer_used = encode_line(enc_buffer.as_mut_ptr(), str.as_mut_ptr());
    fwrite(
        enc_buffer.as_mut_ptr() as *const libc::c_void,
        enc_buffer_used as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        (*wb).fh,
    );
    return wrote_something;
}
#[no_mangle]
pub unsafe extern "C" fn write_cc_buffer(mut wb: *mut s_write) -> libc::c_int {
    let mut data: *mut eia608_screen = 0 as *mut eia608_screen;
    let mut wrote_something: libc::c_int = 0 as libc::c_int;
    if screens_to_process != -(1 as libc::c_int) as libc::c_long
        && (*(*wb).data608).screenfuls_counter as libc::c_long >= screens_to_process
    {
        processed_enough = 1 as libc::c_int;
        return 0 as libc::c_int;
    }
    if (*(*wb).data608).visible_buffer == 1 as libc::c_int {
        data = &mut (*(*wb).data608).buffer1;
    } else {
        data = &mut (*(*wb).data608).buffer2;
    }
    if (*data).empty == 0 {
        new_sentence = 1 as libc::c_int;
        match write_format {
            1 => {
                wrote_something = write_cc_buffer_as_srt(data, wb);
            }
            2 => {
                wrote_something = write_cc_buffer_as_sami(data, wb);
            }
            _ => {}
        }
    }
    return wrote_something;
}
#[no_mangle]
pub unsafe extern "C" fn roll_up(mut wb: *mut s_write) {
    let mut i: libc::c_int = 0;
    let mut keep_lines: libc::c_int = 0;
    let mut lastrow: libc::c_int = 0;
    let mut rows_now: libc::c_int = 0 as libc::c_int;
    let mut use_buffer: *mut eia608_screen = 0 as *mut eia608_screen;
    if (*(*wb).data608).visible_buffer == 1 as libc::c_int {
        use_buffer = &mut (*(*wb).data608).buffer1;
    } else {
        use_buffer = &mut (*(*wb).data608).buffer2;
    }
    match (*(*wb).data608).mode {
        1 => {
            keep_lines = 2 as libc::c_int;
        }
        2 => {
            keep_lines = 3 as libc::c_int;
        }
        3 => {
            keep_lines = 4 as libc::c_int;
        }
        _ => {
            keep_lines = 0 as libc::c_int;
        }
    }
    i = 0 as libc::c_int;
    while i < 15 as libc::c_int {
        if (*use_buffer).row_used[i as usize] != 0 {
            rows_now += 1;
        }
        i += 1;
    }
    if rows_now > keep_lines {
        printf(b"Bug here.\n\0" as *const u8 as *const libc::c_char);
    }
    i = 14 as libc::c_int;
    while i >= 0 as libc::c_int && (*use_buffer).row_used[i as usize] == 0 as libc::c_int {
        i -= 1;
    }
    if i > 0 as libc::c_int {
        let mut j: libc::c_int = 0;
        lastrow = i;
        j = lastrow - keep_lines + 1 as libc::c_int;
        while j < lastrow {
            if j >= 0 as libc::c_int {
                memcpy(
                    ((*use_buffer).characters[j as usize]).as_mut_ptr() as *mut libc::c_void,
                    ((*use_buffer).characters[(j + 1 as libc::c_int) as usize]).as_mut_ptr()
                        as *const libc::c_void,
                    33 as libc::c_int as libc::c_ulong,
                );
                memcpy(
                    ((*use_buffer).colors[j as usize]).as_mut_ptr() as *mut libc::c_void,
                    ((*use_buffer).colors[(j + 1 as libc::c_int) as usize]).as_mut_ptr()
                        as *const libc::c_void,
                    33 as libc::c_int as libc::c_ulong,
                );
                memcpy(
                    ((*use_buffer).fonts[j as usize]).as_mut_ptr() as *mut libc::c_void,
                    ((*use_buffer).fonts[(j + 1 as libc::c_int) as usize]).as_mut_ptr()
                        as *const libc::c_void,
                    33 as libc::c_int as libc::c_ulong,
                );
                (*use_buffer).row_used[j as usize] =
                    (*use_buffer).row_used[(j + 1 as libc::c_int) as usize];
            }
            j += 1;
        }
        j = 0 as libc::c_int;
        while j < 1 as libc::c_int + (*(*wb).data608).cursor_row - keep_lines {
            memset(
                ((*use_buffer).characters[j as usize]).as_mut_ptr() as *mut libc::c_void,
                ' ' as i32,
                32 as libc::c_int as libc::c_ulong,
            );
            memset(
                ((*use_buffer).colors[j as usize]).as_mut_ptr() as *mut libc::c_void,
                COL_WHITE as libc::c_int,
                32 as libc::c_int as libc::c_ulong,
            );
            memset(
                ((*use_buffer).fonts[j as usize]).as_mut_ptr() as *mut libc::c_void,
                FONT_REGULAR as libc::c_int,
                32 as libc::c_int as libc::c_ulong,
            );
            (*use_buffer).characters[j as usize][32 as libc::c_int as usize] =
                0 as libc::c_int as libc::c_uchar;
            (*use_buffer).row_used[j as usize] = 0 as libc::c_int;
            j += 1;
        }
        memset(
            ((*use_buffer).characters[lastrow as usize]).as_mut_ptr() as *mut libc::c_void,
            ' ' as i32,
            32 as libc::c_int as libc::c_ulong,
        );
        memset(
            ((*use_buffer).colors[lastrow as usize]).as_mut_ptr() as *mut libc::c_void,
            COL_WHITE as libc::c_int,
            32 as libc::c_int as libc::c_ulong,
        );
        memset(
            ((*use_buffer).fonts[lastrow as usize]).as_mut_ptr() as *mut libc::c_void,
            FONT_REGULAR as libc::c_int,
            32 as libc::c_int as libc::c_ulong,
        );
        (*use_buffer).characters[lastrow as usize][32 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_uchar;
        (*use_buffer).row_used[lastrow as usize] = 0 as libc::c_int;
    }
    rows_now = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 15 as libc::c_int {
        if (*use_buffer).row_used[i as usize] != 0 {
            rows_now += 1;
        }
        i += 1;
    }
    if rows_now > keep_lines {
        printf(b"Bug here.\n\0" as *const u8 as *const libc::c_char);
    }
}
#[no_mangle]
pub unsafe extern "C" fn erase_memory(mut wb: *mut s_write, mut displayed: libc::c_int) {
    let mut buf: *mut eia608_screen = 0 as *mut eia608_screen;
    if displayed != 0 {
        if (*(*wb).data608).visible_buffer == 1 as libc::c_int {
            buf = &mut (*(*wb).data608).buffer1;
        } else {
            buf = &mut (*(*wb).data608).buffer2;
        }
    } else if (*(*wb).data608).visible_buffer == 1 as libc::c_int {
        buf = &mut (*(*wb).data608).buffer2;
    } else {
        buf = &mut (*(*wb).data608).buffer1;
    }
    clear_eia608_cc_buffer(buf);
}
#[no_mangle]
pub unsafe extern "C" fn handle_command(
    mut c1: libc::c_uchar,
    c2: libc::c_uchar,
    mut wb: *mut s_write,
) {
    let mut command: libc::c_int = COM_UNKNOWN as libc::c_int;
    if c1 as libc::c_int == 0x15 as libc::c_int {
        c1 = 0x14 as libc::c_int as libc::c_uchar;
    }
    if (c1 as libc::c_int == 0x14 as libc::c_int || c1 as libc::c_int == 0x1c as libc::c_int)
        && c2 as libc::c_int == 0x2c as libc::c_int
    {
        command = COM_ERASEDISPLAYEDMEMORY as libc::c_int;
    }
    if (c1 as libc::c_int == 0x14 as libc::c_int || c1 as libc::c_int == 0x1c as libc::c_int)
        && c2 as libc::c_int == 0x20 as libc::c_int
    {
        command = COM_RESUMECAPTIONLOADING as libc::c_int;
    }
    if (c1 as libc::c_int == 0x14 as libc::c_int || c1 as libc::c_int == 0x1c as libc::c_int)
        && c2 as libc::c_int == 0x2f as libc::c_int
    {
        command = COM_ENDOFCAPTION as libc::c_int;
    }
    if (c1 as libc::c_int == 0x17 as libc::c_int || c1 as libc::c_int == 0x1f as libc::c_int)
        && c2 as libc::c_int == 0x21 as libc::c_int
    {
        command = COM_TABOFFSET1 as libc::c_int;
    }
    if (c1 as libc::c_int == 0x17 as libc::c_int || c1 as libc::c_int == 0x1f as libc::c_int)
        && c2 as libc::c_int == 0x22 as libc::c_int
    {
        command = COM_TABOFFSET2 as libc::c_int;
    }
    if (c1 as libc::c_int == 0x17 as libc::c_int || c1 as libc::c_int == 0x1f as libc::c_int)
        && c2 as libc::c_int == 0x23 as libc::c_int
    {
        command = COM_TABOFFSET3 as libc::c_int;
    }
    if (c1 as libc::c_int == 0x14 as libc::c_int || c1 as libc::c_int == 0x1c as libc::c_int)
        && c2 as libc::c_int == 0x25 as libc::c_int
    {
        command = COM_ROLLUP2 as libc::c_int;
    }
    if (c1 as libc::c_int == 0x14 as libc::c_int || c1 as libc::c_int == 0x1c as libc::c_int)
        && c2 as libc::c_int == 0x26 as libc::c_int
    {
        command = COM_ROLLUP3 as libc::c_int;
    }
    if (c1 as libc::c_int == 0x14 as libc::c_int || c1 as libc::c_int == 0x1c as libc::c_int)
        && c2 as libc::c_int == 0x27 as libc::c_int
    {
        command = COM_ROLLUP4 as libc::c_int;
    }
    if (c1 as libc::c_int == 0x14 as libc::c_int || c1 as libc::c_int == 0x1c as libc::c_int)
        && c2 as libc::c_int == 0x2d as libc::c_int
    {
        command = COM_CARRIAGERETURN as libc::c_int;
    }
    if (c1 as libc::c_int == 0x14 as libc::c_int || c1 as libc::c_int == 0x1c as libc::c_int)
        && c2 as libc::c_int == 0x2e as libc::c_int
    {
        command = COM_ERASENONDISPLAYEDMEMORY as libc::c_int;
    }
    if (c1 as libc::c_int == 0x14 as libc::c_int || c1 as libc::c_int == 0x1c as libc::c_int)
        && c2 as libc::c_int == 0x21 as libc::c_int
    {
        command = COM_BACKSPACE as libc::c_int;
    }
    if (c1 as libc::c_int == 0x14 as libc::c_int || c1 as libc::c_int == 0x1c as libc::c_int)
        && c2 as libc::c_int == 0x2b as libc::c_int
    {
        command = COM_RESUMETEXTDISPLAY as libc::c_int;
    }
    if debug_608 != 0 {
        printf(
            b"\rCommand: %02X %02X (%s)\n\0" as *const u8 as *const libc::c_char,
            c1 as libc::c_int,
            c2 as libc::c_int,
            command_type[command as usize],
        );
    }
    match command {
        12 => {
            if (*(*wb).data608).cursor_column > 0 as libc::c_int {
                let ref mut fresh3 = (*(*wb).data608).cursor_column;
                *fresh3 -= 1;
                (*get_writing_buffer(wb)).characters[(*(*wb).data608).cursor_row as usize]
                    [(*(*wb).data608).cursor_column as usize] = ' ' as i32 as libc::c_uchar;
            }
        }
        4 => {
            if (*(*wb).data608).cursor_column < 31 as libc::c_int {
                let ref mut fresh4 = (*(*wb).data608).cursor_column;
                *fresh4 += 1;
            }
        }
        5 => {
            (*(*wb).data608).cursor_column += 2 as libc::c_int;
            if (*(*wb).data608).cursor_column > 31 as libc::c_int {
                (*(*wb).data608).cursor_column = 31 as libc::c_int;
            }
        }
        6 => {
            (*(*wb).data608).cursor_column += 3 as libc::c_int;
            if (*(*wb).data608).cursor_column > 31 as libc::c_int {
                (*(*wb).data608).cursor_column = 31 as libc::c_int;
            }
        }
        2 => {
            (*(*wb).data608).mode = MODE_POPUP as libc::c_int;
        }
        13 => {
            (*(*wb).data608).mode = MODE_TEXT as libc::c_int;
        }
        7 => {
            (*(*wb).data608).mode = MODE_ROLLUP_2 as libc::c_int;
            if (*(*wb).data608).mode == MODE_POPUP as libc::c_int {
                if write_cc_buffer(wb) != 0 {
                    let ref mut fresh5 = (*(*wb).data608).screenfuls_counter;
                    *fresh5 += 1;
                }
                erase_memory(wb, 1 as libc::c_int);
            }
            erase_memory(wb, 0 as libc::c_int);
            (*(*wb).data608).cursor_column = 0 as libc::c_int;
            (*(*wb).data608).cursor_row = (*(*wb).data608).rollup_base_row;
        }
        8 => {
            if (*(*wb).data608).mode == MODE_POPUP as libc::c_int {
                if write_cc_buffer(wb) != 0 {
                    let ref mut fresh6 = (*(*wb).data608).screenfuls_counter;
                    *fresh6 += 1;
                }
                erase_memory(wb, 1 as libc::c_int);
            }
            (*(*wb).data608).mode = MODE_ROLLUP_3 as libc::c_int;
            erase_memory(wb, 0 as libc::c_int);
            (*(*wb).data608).cursor_column = 0 as libc::c_int;
            (*(*wb).data608).cursor_row = (*(*wb).data608).rollup_base_row;
        }
        9 => {
            if (*(*wb).data608).mode == MODE_POPUP as libc::c_int {
                if write_cc_buffer(wb) != 0 {
                    let ref mut fresh7 = (*(*wb).data608).screenfuls_counter;
                    *fresh7 += 1;
                }
                erase_memory(wb, 1 as libc::c_int);
            }
            (*(*wb).data608).mode = MODE_ROLLUP_4 as libc::c_int;
            (*(*wb).data608).cursor_column = 0 as libc::c_int;
            (*(*wb).data608).cursor_row = (*(*wb).data608).rollup_base_row;
            erase_memory(wb, 0 as libc::c_int);
        }
        10 => {
            if write_cc_buffer(wb) != 0 {
                let ref mut fresh8 = (*(*wb).data608).screenfuls_counter;
                *fresh8 += 1;
            }
            roll_up(wb);
            (*(*wb).data608).current_visible_start_cc = totalblockswritten_thisfile();
            (*(*wb).data608).cursor_column = 0 as libc::c_int;
        }
        11 => {
            erase_memory(wb, 0 as libc::c_int);
        }
        1 => {
            if write_cc_buffer(wb) != 0 {
                let ref mut fresh9 = (*(*wb).data608).screenfuls_counter;
                *fresh9 += 1;
            }
            erase_memory(wb, 1 as libc::c_int);
            (*(*wb).data608).current_visible_start_cc = totalblockswritten_thisfile();
        }
        3 => {
            if write_cc_buffer(wb) != 0 {
                let ref mut fresh10 = (*(*wb).data608).screenfuls_counter;
                *fresh10 += 1;
            }
            (*(*wb).data608).visible_buffer = if (*(*wb).data608).visible_buffer == 1 as libc::c_int
            {
                2 as libc::c_int
            } else {
                1 as libc::c_int
            };
            (*(*wb).data608).current_visible_start_cc = totalblockswritten_thisfile();
            (*(*wb).data608).cursor_column = 0 as libc::c_int;
            (*(*wb).data608).cursor_row = 0 as libc::c_int;
            (*(*wb).data608).color = default_color as libc::c_uchar;
            (*(*wb).data608).font = FONT_REGULAR as libc::c_int as libc::c_uchar;
        }
        _ => {
            if debug_608 != 0 {
                printf(b"\rNot yet implemented.\n\0" as *const u8 as *const libc::c_char);
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn handle_double(c1: libc::c_uchar, c2: libc::c_uchar, mut wb: *mut s_write) {
    let mut c: libc::c_uchar = 0;
    if (*(*wb).data608).channel != cc_channel {
        return;
    }
    if c2 as libc::c_int >= 0x30 as libc::c_int && c2 as libc::c_int <= 0x3f as libc::c_int {
        c = (c2 as libc::c_int + 0x50 as libc::c_int) as libc::c_uchar;
        if debug_608 != 0 {
            printf(
                b"\rDouble: %02X %02X  -->  %c\n\0" as *const u8 as *const libc::c_char,
                c1 as libc::c_int,
                c2 as libc::c_int,
                c as libc::c_int,
            );
        }
        write_char(c, wb);
    }
}
#[no_mangle]
pub unsafe extern "C" fn handle_extended(
    mut hi: libc::c_uchar,
    mut lo: libc::c_uchar,
    mut wb: *mut s_write,
) -> libc::c_uchar {
    let mut c: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
    if debug_608 != 0 {
        printf(
            b"\rExtended: %02X %02X\n\0" as *const u8 as *const libc::c_char,
            hi as libc::c_int,
            lo as libc::c_int,
        );
    }
    if lo as libc::c_int >= 0x20 as libc::c_int
        && lo as libc::c_int <= 0x3f as libc::c_int
        && (hi as libc::c_int == 0x12 as libc::c_int || hi as libc::c_int == 0x13 as libc::c_int)
    {
        match hi as libc::c_int {
            18 => {
                c = (lo as libc::c_int + 0x70 as libc::c_int) as libc::c_uchar;
            }
            19 => {
                c = (lo as libc::c_int + 0x90 as libc::c_int) as libc::c_uchar;
            }
            _ => {}
        }
        if (*(*wb).data608).cursor_column > 0 as libc::c_int {
            let ref mut fresh11 = (*(*wb).data608).cursor_column;
            *fresh11 -= 1;
        }
        write_char(c, wb);
    }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn handle_pac(
    mut c1: libc::c_uchar,
    mut c2: libc::c_uchar,
    mut wb: *mut s_write,
) {
    let mut color: libc::c_int = 0;
    let mut font: libc::c_int = 0;
    let mut indent: libc::c_int = 0;
    let mut row: libc::c_int = rowdata[((c1 as libc::c_int) << 1 as libc::c_int & 14 as libc::c_int
        | c2 as libc::c_int >> 5 as libc::c_int & 1 as libc::c_int)
        as usize];
    if debug_608 != 0 {
        printf(
            b"\rPAC: %02X %02X\n\0" as *const u8 as *const libc::c_char,
            c1 as libc::c_int,
            c2 as libc::c_int,
        );
    }
    if c2 as libc::c_int >= 0x40 as libc::c_int && c2 as libc::c_int <= 0x5f as libc::c_int {
        c2 = (c2 as libc::c_int - 0x40 as libc::c_int) as libc::c_uchar;
    } else if c2 as libc::c_int >= 0x60 as libc::c_int && c2 as libc::c_int <= 0x7f as libc::c_int {
        c2 = (c2 as libc::c_int - 0x60 as libc::c_int) as libc::c_uchar;
    } else {
        if debug_608 != 0 {
            printf(b"\rThis is not a PAC!!!!!\n\0" as *const u8 as *const libc::c_char);
        }
        return;
    }
    color = pac2_attribs[c2 as usize][0 as libc::c_int as usize] as libc::c_int;
    font = pac2_attribs[c2 as usize][1 as libc::c_int as usize] as libc::c_int;
    indent = pac2_attribs[c2 as usize][2 as libc::c_int as usize] as libc::c_int;
    if debug_608 != 0 {
        printf(
            b"\rPosition: %d:%d, color: %s,  font: %s\n\0" as *const u8 as *const libc::c_char,
            row,
            indent,
            color_text[color as usize][0 as libc::c_int as usize],
            font_text[font as usize],
        );
    }
    if (*(*wb).data608).mode != MODE_TEXT as libc::c_int {
        (*(*wb).data608).cursor_row = row - 1 as libc::c_int;
    }
    (*(*wb).data608).rollup_base_row = row - 1 as libc::c_int;
    (*(*wb).data608).cursor_column = indent;
}
#[no_mangle]
pub unsafe extern "C" fn handle_single(c1: libc::c_uchar, mut wb: *mut s_write) {
    if (c1 as libc::c_int) < 0x20 as libc::c_int || (*(*wb).data608).channel != cc_channel {
        return;
    }
    write_char(c1, wb);
}
#[no_mangle]
pub unsafe extern "C" fn check_channel(mut c1: libc::c_uchar, mut wb: *mut s_write) {
    if (*(*wb).data608).channel != 1 as libc::c_int && c1 as libc::c_int == 0x14 as libc::c_int {
        if debug_608 != 0 {
            printf(b"\rChannel change, now 1\n\0" as *const u8 as *const libc::c_char);
        }
        (*(*wb).data608).channel = 1 as libc::c_int;
    }
    if (*(*wb).data608).channel != 2 as libc::c_int && c1 as libc::c_int == 0x1c as libc::c_int {
        if debug_608 != 0 {
            printf(b"\rChannel change, now 2\n\0" as *const u8 as *const libc::c_char);
        }
        (*(*wb).data608).channel = 2 as libc::c_int;
    }
    if (*(*wb).data608).channel != 3 as libc::c_int
        && (c1 as libc::c_int >= 0x1 as libc::c_int && c1 as libc::c_int <= 0xf as libc::c_int
            || c1 as libc::c_int == 0x15 as libc::c_int)
    {
        if debug_608 != 0 {
            printf(b"\rChannel change, now 3\n\0" as *const u8 as *const libc::c_char);
        }
        (*(*wb).data608).channel = 3 as libc::c_int;
    }
    if (*(*wb).data608).channel != 4 as libc::c_int && c1 as libc::c_int == 0x1d as libc::c_int {
        if debug_608 != 0 {
            printf(b"\rChannel change, now 4\n\0" as *const u8 as *const libc::c_char);
        }
        (*(*wb).data608).channel = 4 as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn disCommand(
    mut hi: libc::c_uchar,
    mut lo: libc::c_uchar,
    mut wb: *mut s_write,
) -> libc::c_int {
    let mut wrote_to_screen: libc::c_int = 0 as libc::c_int;
    if hi as libc::c_int >= 0x18 as libc::c_int && hi as libc::c_int <= 0x1f as libc::c_int {
        hi = (hi as libc::c_int - 8 as libc::c_int) as libc::c_uchar;
    }
    match hi as libc::c_int {
        16 => {
            if lo as libc::c_int >= 0x40 as libc::c_int && lo as libc::c_int <= 0x5f as libc::c_int
            {
                handle_pac(hi, lo, wb);
            }
        }
        17 => {
            if lo as libc::c_int >= 0x20 as libc::c_int && lo as libc::c_int <= 0x2f as libc::c_int
            {
                handle_text_attr(hi, lo, wb);
            }
            if lo as libc::c_int >= 0x30 as libc::c_int && lo as libc::c_int <= 0x3f as libc::c_int
            {
                wrote_to_screen = 1 as libc::c_int;
                handle_double(hi, lo, wb);
            }
            if lo as libc::c_int >= 0x40 as libc::c_int && lo as libc::c_int <= 0x7f as libc::c_int
            {
                handle_pac(hi, lo, wb);
            }
        }
        18 | 19 => {
            if lo as libc::c_int >= 0x20 as libc::c_int && lo as libc::c_int <= 0x3f as libc::c_int
            {
                handle_extended(hi, lo, wb);
                wrote_to_screen = 1 as libc::c_int;
            }
            if lo as libc::c_int >= 0x40 as libc::c_int && lo as libc::c_int <= 0x7f as libc::c_int
            {
                handle_pac(hi, lo, wb);
            }
        }
        20 | 21 => {
            if lo as libc::c_int >= 0x20 as libc::c_int && lo as libc::c_int <= 0x2f as libc::c_int
            {
                handle_command(hi, lo, wb);
            }
            if lo as libc::c_int >= 0x40 as libc::c_int && lo as libc::c_int <= 0x7f as libc::c_int
            {
                handle_pac(hi, lo, wb);
            }
        }
        22 => {
            if lo as libc::c_int >= 0x40 as libc::c_int && lo as libc::c_int <= 0x7f as libc::c_int
            {
                handle_pac(hi, lo, wb);
            }
        }
        23 => {
            if lo as libc::c_int >= 0x21 as libc::c_int && lo as libc::c_int <= 0x22 as libc::c_int
            {
                handle_command(hi, lo, wb);
            }
            if lo as libc::c_int >= 0x2e as libc::c_int && lo as libc::c_int <= 0x2f as libc::c_int
            {
                handle_text_attr(hi, lo, wb);
            }
            if lo as libc::c_int >= 0x40 as libc::c_int && lo as libc::c_int <= 0x7f as libc::c_int
            {
                handle_pac(hi, lo, wb);
            }
        }
        _ => {}
    }
    return wrote_to_screen;
}
#[no_mangle]
pub unsafe extern "C" fn process608(
    mut data: *const libc::c_uchar,
    mut length: libc::c_int,
    mut wb: *mut s_write,
) {
    if !data.is_null() {
        let mut i: libc::c_int = 0;
        let mut current_block_22: u64;
        i = 0 as libc::c_int;
        while i < length {
            let mut hi: libc::c_uchar = 0;
            let mut lo: libc::c_uchar = 0;
            let mut wrote_to_screen: libc::c_int = 0 as libc::c_int;
            hi = (*data.offset(i as isize) as libc::c_int & 0x7f as libc::c_int) as libc::c_uchar;
            lo = (*data.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                & 0x7f as libc::c_int) as libc::c_uchar;
            if !(hi as libc::c_int == 0 as libc::c_int && lo as libc::c_int == 0 as libc::c_int) {
                check_channel(hi, wb);
                if !((*(*wb).data608).channel != cc_channel) {
                    if hi as libc::c_int >= 0x1 as libc::c_int
                        && hi as libc::c_int <= 0xe as libc::c_int
                    {
                        in_xds_mode = 1 as libc::c_int;
                    }
                    if hi as libc::c_int == 0xf as libc::c_int {
                        in_xds_mode = 0 as libc::c_int;
                    } else {
                        if hi as libc::c_int >= 0x10 as libc::c_int
                            && (hi as libc::c_int) < 0x1f as libc::c_int
                        {
                            in_xds_mode = 0 as libc::c_int;
                            if (*(*wb).data608).last_c1 as libc::c_int == hi as libc::c_int
                                && (*(*wb).data608).last_c2 as libc::c_int == lo as libc::c_int
                            {
                                current_block_22 = 8258075665625361029;
                            } else {
                                (*(*wb).data608).last_c1 = hi;
                                (*(*wb).data608).last_c2 = lo;
                                wrote_to_screen = disCommand(hi, lo, wb);
                                current_block_22 = 5601891728916014340;
                            }
                        } else {
                            current_block_22 = 5601891728916014340;
                        }
                        match current_block_22 {
                            8258075665625361029 => {}
                            _ => {
                                if hi as libc::c_int >= 0x20 as libc::c_int {
                                    handle_single(hi, wb);
                                    handle_single(lo, wb);
                                    wrote_to_screen = 1 as libc::c_int;
                                    (*(*wb).data608).last_c1 = 0 as libc::c_int as libc::c_uchar;
                                    (*(*wb).data608).last_c2 = 0 as libc::c_int as libc::c_uchar;
                                }
                                if wrote_to_screen != 0
                                    && direct_rollup != 0
                                    && ((*(*wb).data608).mode == MODE_ROLLUP_2 as libc::c_int
                                        || (*(*wb).data608).mode == MODE_ROLLUP_3 as libc::c_int
                                        || (*(*wb).data608).mode == MODE_ROLLUP_4 as libc::c_int)
                                {
                                    write_cc_buffer(wb);
                                    (*(*wb).data608).current_visible_start_cc =
                                        totalblockswritten_thisfile();
                                }
                            }
                        }
                    }
                }
            }
            i = i + 2 as libc::c_int;
        }
    }
}
