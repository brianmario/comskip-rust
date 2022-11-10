#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
extern "C" {
    pub type __sFILEX;
    static mut _DefaultRuneLocale: _RuneLocale;
    fn __tolower(_: __darwin_ct_rune_t) -> __darwin_ct_rune_t;
    fn fclose(_: *mut FILE) -> libc::c_int;
    fn fgets(_: *mut libc::c_char, _: libc::c_int, _: *mut FILE) -> *mut libc::c_char;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn atoi(_: *const libc::c_char) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn time(_: *mut time_t) -> time_t;
    fn lseek(_: libc::c_int, _: off_t, _: libc::c_int) -> off_t;
    static mut usercolor_rgb: [libc::c_uchar; 8];
    static mut default_color: libc::c_int;
    fn init_file_buffer() -> libc::c_int;
    fn init_eia608(data: *mut eia608);
    fn encode_line(buffer: *mut libc::c_uchar, text: *mut libc::c_uchar) -> libc::c_uint;
    fn write_subtitle_file_header(wb: *mut s_write);
    fn build_parity_table();
    static mut net_fields: LONG;
    static mut encoded_crlf: [libc::c_uchar; 16];
    static mut encoded_crlf_length: libc::c_uint;
    static mut encoded_br: [libc::c_uchar; 16];
    static mut encoded_br_length: libc::c_uint;
    fn process608(data: *const libc::c_uchar, length: libc::c_int, wb: *mut s_write);
    static mut subline: *mut libc::c_uchar;
}
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_longlong;
pub type __darwin_ct_rune_t = libc::c_int;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_wchar_t = libc::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
pub type __darwin_time_t = libc::c_long;
pub type __darwin_off_t = __int64_t;
pub type size_t = __darwin_size_t;
pub type off_t = __darwin_off_t;
pub type time_t = __darwin_time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneEntry {
    pub __min: __darwin_rune_t,
    pub __max: __darwin_rune_t,
    pub __map: __darwin_rune_t,
    pub __types: *mut __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneRange {
    pub __nranges: libc::c_int,
    pub __ranges: *mut _RuneEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneCharClass {
    pub __name: [libc::c_char; 14],
    pub __mask: __uint32_t,
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
pub type color_code = libc::c_uint;
pub const COL_USERDEFINED: color_code = 7;
pub type LONG = libc::c_long;
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
pub type output_format = libc::c_uint;
pub const OF_SAMI: output_format = 2;
pub const OF_SRT: output_format = 1;
pub const OF_RAW: output_format = 0;
pub type encoding_type = libc::c_uint;
pub const ENC_UTF_8: encoding_type = 2;
pub const ENC_LATIN_1: encoding_type = 1;
pub const ENC_UNICODE: encoding_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gop_time_code {
    pub drop_frame_flag: libc::c_int,
    pub time_code_hours: libc::c_int,
    pub time_code_minutes: libc::c_int,
    pub marker_bit: libc::c_int,
    pub time_code_seconds: libc::c_int,
    pub time_code_pictures: libc::c_int,
    pub inited: libc::c_int,
    pub ccblocks: LONG,
}
#[inline]
unsafe extern "C" fn __isctype(
    mut _c: __darwin_ct_rune_t,
    mut _f: libc::c_ulong,
) -> __darwin_ct_rune_t {
    return if _c < 0 as libc::c_int || _c >= (1 as libc::c_int) << 8 as libc::c_int {
        0 as libc::c_int
    } else {
        (_DefaultRuneLocale.__runetype[_c as usize] as libc::c_ulong & _f != 0) as libc::c_int
    };
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn isdigit(mut _c: libc::c_int) -> libc::c_int {
    return __isctype(_c, 0x400 as libc::c_long as libc::c_ulong);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn tolower(mut _c: libc::c_int) -> libc::c_int {
    return __tolower(_c);
}
#[no_mangle]
pub static mut captions_buffer_1: [libc::c_uchar; 64] = [0; 64];
#[no_mangle]
pub static mut used_caption_buffer_1: libc::c_uint = 0 as libc::c_int as libc::c_uint;
#[no_mangle]
pub static mut captions_buffer_2: [libc::c_uchar; 64] = [0; 64];
#[no_mangle]
pub static mut used_caption_buffer_2: libc::c_uint = 0 as libc::c_int as libc::c_uint;
#[no_mangle]
pub static mut min_pts: LONG = 0;
#[no_mangle]
pub static mut max_pts: LONG = 0;
#[no_mangle]
pub static mut last_pts: LONG = 0;
#[no_mangle]
pub static mut pts_set: libc::c_int = 0;
#[no_mangle]
pub static mut c1count: libc::c_uint = 0;
#[no_mangle]
pub static mut c2count: libc::c_uint = 0;
#[no_mangle]
pub static mut c1count_total: libc::c_uint = 0;
#[no_mangle]
pub static mut c2count_total: libc::c_uint = 0;
#[no_mangle]
pub static mut c1global: libc::c_uint = 0 as libc::c_int as libc::c_uint;
#[no_mangle]
pub static mut c2global: libc::c_uint = 0 as libc::c_int as libc::c_uint;
#[no_mangle]
pub static mut pts_big_change: libc::c_uint = 0;
#[no_mangle]
pub static mut ptsdata: [libc::c_uchar; 5] = [0; 5];
#[no_mangle]
pub static mut lastptsdata: [libc::c_uchar; 5] = [0; 5];
#[no_mangle]
pub static mut fbuffer: *mut libc::c_uchar = 0 as *const libc::c_uchar as *mut libc::c_uchar;
#[no_mangle]
pub static mut past: LONG = 0;
#[no_mangle]
pub static mut pesheaderbuf: *mut libc::c_uchar = 0 as *const libc::c_uchar as *mut libc::c_uchar;
#[no_mangle]
pub static mut inputsize: LONG = 0;
#[no_mangle]
pub static mut last_reported_progress: libc::c_int = 0;
#[no_mangle]
pub static mut processed_enough: libc::c_int = 0;
#[no_mangle]
pub static mut startbytes: [libc::c_uchar; 1048576] = [0; 1048576];
#[no_mangle]
pub static mut startbytes_pos: libc::c_uint = 0;
#[no_mangle]
pub static mut startbytes_avail: libc::c_uint = 0;
#[no_mangle]
pub static mut stat_numuserheaders: libc::c_int = 0;
#[no_mangle]
pub static mut stat_dvdccheaders: libc::c_int = 0;
#[no_mangle]
pub static mut stat_replay5000headers: libc::c_int = 0;
#[no_mangle]
pub static mut stat_replay4000headers: libc::c_int = 0;
#[no_mangle]
pub static mut stat_dishheaders: libc::c_int = 0;
#[no_mangle]
pub static mut stat_hdtv: libc::c_int = 0;
#[no_mangle]
pub static mut total_frames_count: libc::c_uint = 0;
#[no_mangle]
pub static mut cc_stats: [libc::c_int; 4] = [0; 4];
#[no_mangle]
pub static mut false_pict_header: libc::c_int = 0;
#[no_mangle]
pub static mut gop_time: gop_time_code = gop_time_code {
    drop_frame_flag: 0,
    time_code_hours: 0,
    time_code_minutes: 0,
    marker_bit: 0,
    time_code_seconds: 0,
    time_code_pictures: 0,
    inited: 0,
    ccblocks: 0,
};
#[no_mangle]
pub static mut first_gop_time: gop_time_code = gop_time_code {
    drop_frame_flag: 0,
    time_code_hours: 0,
    time_code_minutes: 0,
    marker_bit: 0,
    time_code_seconds: 0,
    time_code_pictures: 0,
    inited: 0,
    ccblocks: 0,
};
#[no_mangle]
pub static mut printed_gop: gop_time_code = gop_time_code {
    drop_frame_flag: 0,
    time_code_hours: 0,
    time_code_minutes: 0,
    marker_bit: 0,
    time_code_seconds: 0,
    time_code_pictures: 0,
    inited: 0,
    ccblocks: 0,
};
#[no_mangle]
pub static mut frames_since_last_gop: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut gop_rollover: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut buffer_output: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut buffer_input: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut autopad: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut gop_pad: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut ff_cleanup: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut ts_mode: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut input_bin: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut auto_ts: libc::c_int = 2 as libc::c_int;
#[no_mangle]
pub static mut debug: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut fix_padding: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut rawmode: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut extract: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut cc_channel: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut debug_608: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut subs_delay: LONG = 0 as libc::c_int as LONG;
#[no_mangle]
pub static mut extraction_start: boundary_time = boundary_time {
    hh: 0,
    mm: 0,
    ss: 0,
    time_in_ms: 0,
    time_in_ccblocks: 0,
    set: 0,
};
#[no_mangle]
pub static mut extraction_end: boundary_time = boundary_time {
    hh: 0,
    mm: 0,
    ss: 0,
    time_in_ms: 0,
    time_in_ccblocks: 0,
    set: 0,
};
#[no_mangle]
pub static mut screens_to_process: LONG = -(1 as libc::c_int) as LONG;
#[no_mangle]
pub static mut basefilename: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut inputfile: *mut *mut libc::c_char =
    0 as *const *mut libc::c_char as *mut *mut libc::c_char;
#[no_mangle]
pub static mut direct_rollup: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut num_input_files: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut inputfile_capacity: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut nofontcolor: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut next_input_file: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut write_format: libc::c_int = OF_SRT as libc::c_int;
#[no_mangle]
pub static mut encoding: libc::c_int = ENC_LATIN_1 as libc::c_int;
#[no_mangle]
pub static mut auto_myth: libc::c_int = 2 as libc::c_int;
#[no_mangle]
pub static mut sentence_cap: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut sentence_cap_file: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut spell_lower: *mut *mut libc::c_char =
    0 as *const *mut libc::c_char as *mut *mut libc::c_char;
#[no_mangle]
pub static mut spell_correct: *mut *mut libc::c_char =
    0 as *const *mut libc::c_char as *mut *mut libc::c_char;
#[no_mangle]
pub static mut spell_words: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut spell_capacity: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut spell_builtin_added: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut wbout1: s_write = s_write {
    fh: 0 as *const FILE as *mut FILE,
    filename: 0 as *const libc::c_char as *mut libc::c_char,
    buffer: 0 as *const libc::c_uchar as *mut libc::c_uchar,
    used: 0,
    data608: 0 as *const eia608 as *mut eia608,
};
#[no_mangle]
pub static mut wbout2: s_write = s_write {
    fh: 0 as *const FILE as *mut FILE,
    filename: 0 as *const libc::c_char as *mut libc::c_char,
    buffer: 0 as *const libc::c_uchar as *mut libc::c_uchar,
    used: 0,
    data608: 0 as *const eia608 as *mut eia608,
};
#[no_mangle]
pub static mut loopcount: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut datacount: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut BROADCAST_HEADER: [libc::c_uchar; 4] = [
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
];
#[no_mangle]
pub static mut LITTLE_ENDIAN_BOM: [libc::c_uchar; 2] = [
    0xff as libc::c_int as libc::c_uchar,
    0xfe as libc::c_int as libc::c_uchar,
];
#[no_mangle]
pub static mut DVD_HEADER: [libc::c_uchar; 8] = [
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xb2 as libc::c_int as libc::c_uchar,
    0x43 as libc::c_int as libc::c_uchar,
    0x43 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xf8 as libc::c_int as libc::c_uchar,
];
#[no_mangle]
pub static mut lc1: [libc::c_uchar; 1] = [0x8a as libc::c_int as libc::c_uchar];
#[no_mangle]
pub static mut lc2: [libc::c_uchar; 1] = [0x8f as libc::c_int as libc::c_uchar];
#[no_mangle]
pub static mut lc3: [libc::c_uchar; 2] = [
    0x16 as libc::c_int as libc::c_uchar,
    0xfe as libc::c_int as libc::c_uchar,
];
#[no_mangle]
pub static mut lc4: [libc::c_uchar; 2] = [
    0x1e as libc::c_int as libc::c_uchar,
    0xfe as libc::c_int as libc::c_uchar,
];
#[no_mangle]
pub static mut lc5: [libc::c_uchar; 1] = [0xff as libc::c_int as libc::c_uchar];
#[no_mangle]
pub static mut lc6: [libc::c_uchar; 1] = [0xfe as libc::c_int as libc::c_uchar];
#[no_mangle]
pub static mut clean: *mut FILE = 0 as *const FILE as *mut FILE;
#[export_name = "in"]
pub static mut in_0: libc::c_int = 0;
#[no_mangle]
pub static mut framerates_values: [libc::c_double; 16] = [
    0 as libc::c_int as libc::c_double,
    23.976f64,
    24.0f64,
    25.0f64,
    29.97f64,
    30.0f64,
    50.0f64,
    59.94f64,
    60.0f64,
    0 as libc::c_int as libc::c_double,
    0 as libc::c_int as libc::c_double,
    0 as libc::c_int as libc::c_double,
    0 as libc::c_int as libc::c_double,
    0 as libc::c_int as libc::c_double,
    0.,
    0.,
];
#[no_mangle]
pub static mut framerates_types: [*const libc::c_char; 16] = [
    b"00 - forbidden\0" as *const u8 as *const libc::c_char,
    b"01 - 23.976\0" as *const u8 as *const libc::c_char,
    b"02 - 24\0" as *const u8 as *const libc::c_char,
    b"03 - 25\0" as *const u8 as *const libc::c_char,
    b"04 - 29.97\0" as *const u8 as *const libc::c_char,
    b"05 - 30\0" as *const u8 as *const libc::c_char,
    b"06 - 50\0" as *const u8 as *const libc::c_char,
    b"07 - 59.94\0" as *const u8 as *const libc::c_char,
    b"08 - 60\0" as *const u8 as *const libc::c_char,
    b"09 - reserved\0" as *const u8 as *const libc::c_char,
    b"10 - reserved\0" as *const u8 as *const libc::c_char,
    b"11 - reserved\0" as *const u8 as *const libc::c_char,
    b"12 - reserved\0" as *const u8 as *const libc::c_char,
    b"13 - reserved\0" as *const u8 as *const libc::c_char,
    b"14 - reserved\0" as *const u8 as *const libc::c_char,
    b"15 - reserved\0" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub static mut aspect_ratio_types: [*const libc::c_char; 16] = [
    b"00 - forbidden\0" as *const u8 as *const libc::c_char,
    b"01 - 1:1\0" as *const u8 as *const libc::c_char,
    b"02 - 4:3\0" as *const u8 as *const libc::c_char,
    b"03 - 16:9\0" as *const u8 as *const libc::c_char,
    b"04 - 2.21:1\0" as *const u8 as *const libc::c_char,
    b"05 - reserved\0" as *const u8 as *const libc::c_char,
    b"06 - reserved\0" as *const u8 as *const libc::c_char,
    b"07 - reserved\0" as *const u8 as *const libc::c_char,
    b"08 - reserved\0" as *const u8 as *const libc::c_char,
    b"09 - reserved\0" as *const u8 as *const libc::c_char,
    b"10 - reserved\0" as *const u8 as *const libc::c_char,
    b"11 - reserved\0" as *const u8 as *const libc::c_char,
    b"12 - reserved\0" as *const u8 as *const libc::c_char,
    b"13 - reserved\0" as *const u8 as *const libc::c_char,
    b"14 - reserved\0" as *const u8 as *const libc::c_char,
    b"15 - reserved\0" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub static mut pict_types: [*const libc::c_char; 8] = [
    b"00 - ilegal (0)\0" as *const u8 as *const libc::c_char,
    b"01 - I\0" as *const u8 as *const libc::c_char,
    b"02 - P\0" as *const u8 as *const libc::c_char,
    b"03 - B\0" as *const u8 as *const libc::c_char,
    b"04 - ilegal (D)\0" as *const u8 as *const libc::c_char,
    b"05 - ilegal (5)\0" as *const u8 as *const libc::c_char,
    b"06 - ilegal (6)\0" as *const u8 as *const libc::c_char,
    b"07 - ilegal (7)\0" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub static mut cc_types: [*const libc::c_char; 4] = [
    b"NTSC line 21 field 1 closed captions\0" as *const u8 as *const libc::c_char,
    b"NTSC line 21 field 2 closed captions\0" as *const u8 as *const libc::c_char,
    b"DTVCC Channel Packet Data\0" as *const u8 as *const libc::c_char,
    b"DTVCC Channel Packet Start\0" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub static mut spell_builtin: [*const libc::c_char; 30] = [
    b"I\0" as *const u8 as *const libc::c_char,
    b"I'd\0" as *const u8 as *const libc::c_char,
    b"I've\0" as *const u8 as *const libc::c_char,
    b"I'd\0" as *const u8 as *const libc::c_char,
    b"I'll\0" as *const u8 as *const libc::c_char,
    b"January\0" as *const u8 as *const libc::c_char,
    b"February\0" as *const u8 as *const libc::c_char,
    b"March\0" as *const u8 as *const libc::c_char,
    b"April\0" as *const u8 as *const libc::c_char,
    b"June\0" as *const u8 as *const libc::c_char,
    b"July\0" as *const u8 as *const libc::c_char,
    b"August\0" as *const u8 as *const libc::c_char,
    b"September\0" as *const u8 as *const libc::c_char,
    b"October\0" as *const u8 as *const libc::c_char,
    b"November\0" as *const u8 as *const libc::c_char,
    b"December\0" as *const u8 as *const libc::c_char,
    b"Monday\0" as *const u8 as *const libc::c_char,
    b"Tuesday\0" as *const u8 as *const libc::c_char,
    b"Wednesday\0" as *const u8 as *const libc::c_char,
    b"Thursday\0" as *const u8 as *const libc::c_char,
    b"Friday\0" as *const u8 as *const libc::c_char,
    b"Saturday\0" as *const u8 as *const libc::c_char,
    b"Sunday\0" as *const u8 as *const libc::c_char,
    b"Halloween\0" as *const u8 as *const libc::c_char,
    b"United States\0" as *const u8 as *const libc::c_char,
    b"Spain\0" as *const u8 as *const libc::c_char,
    b"France\0" as *const u8 as *const libc::c_char,
    b"Italy\0" as *const u8 as *const libc::c_char,
    b"England\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn getfilesize(in_1: libc::c_int) -> LONG {
    let current: LONG = lseek(in_1, 0 as libc::c_int as off_t, 1 as libc::c_int) as LONG;
    let length: LONG = lseek(in_1, 0 as libc::c_int as off_t, 2 as libc::c_int) as LONG;
    lseek(in_1, current as off_t, 0 as libc::c_int);
    return length;
}
#[no_mangle]
pub unsafe extern "C" fn header() {
    printf(b"CCExtractor v0.34, cfsmp3 at gmail\n\0" as *const u8 as *const libc::c_char);
    printf(b"----------------------------------\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn usage() {
    printf(
        b"Heavily based on McPoodle's tools. Check his page for lots of information\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(b"on closed captions technical details.\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"(http://www.geocities.com/mcpoodle43/SCC_TOOLS/DOCS/SCC_TOOLS.HTML)\n\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(b"This tool home page:\n\0" as *const u8 as *const libc::c_char);
    printf(b"http://ccextractor.sourceforge.net\n\0" as *const u8 as *const libc::c_char);
    printf(b"  Extracts closed captions from MPEG files.\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"    (DVB, .TS, ReplayTV 4000 and 5000, dvr-ms, bttv and Dish Network are known\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(b"     to work).\n\n\0" as *const u8 as *const libc::c_char);
    printf(b"  Syntax:\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"  ccextractor [options] inputfile1 [inputfile2...] [-o outputfilename]\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"               [-o1 outputfilename1] [-o2 outputfilename2]\n\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(b"File name related options:\n\0" as *const u8 as *const libc::c_char);
    printf(b"            inputfile: file(s) to process\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"    -o outputfilename: Use -o parameters to define output filename if you don't\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"                       like the default ones (same as infile plus _1 or _2 when\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"                       needed and .bin or .srt extension).\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"                           -o or -o1 -> Name of the first (maybe only) output\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"                                        file.\n\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b"                           -o2       -> Name of the second output file, when\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"                                        it applies.\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"         -cf filename: Write 'clean' data to a file. Cleans means the ES\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"                       without TS or PES headers.\n\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"You can pass as many input files as you need. They will be processed in order.\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"Output will be one single file (either raw or srt). Use this if you made your\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"recording in several cuts (to skip commercials for example) but you want one\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(b"subtitle file with contiguous timing.\n\n\0" as *const u8 as *const libc::c_char);
    printf(b"Options that affect what will be processed:\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"          -1, -2, -12: Output Field 1 data, Field 2 data, or both\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(b"                       (DEFAULT is -1)\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"                 -cc2: When in srt/sami mode, process captions in channel 2\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(b"                       instead channel 1.\n\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"In general, if you want English subtitles you don't need to use these options\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"as they are broadcast in field 1, channel 1. If you want the second language\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"(usually Spanish) you may need to try -2, or -cc2, or both.\n\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"Options that affect how input files will be processed.\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"                  -ts: Force Transport Stream mode.\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"                -nots: Disable Transport Stream mode.\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"                 -bin: Process a raw (bin) closed captions dump instead of a\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"                       MPEG files. Requires that either -srt or -sami is used\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(b"                       as well.\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"                -myth: Force MythTV code branch.\n\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b"              -nomyth: Disable MythTV code branch.\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"     -fp --fixpadding: Fix padding - some cards (or providers, or whatever)\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"                       seem to send 0000 as CC padding instead of 8080. If you\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"                       get bad timing, this might solve it.\n\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"Usually you only need to use -bin (if you want to produce srt/sami from a\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"dump of previously extracted closed captions). For MPEG files, transport\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"stream mode is autodetected. The MythTV branch is needed for analog captures\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"such as those with bttv cards (Hauppage 250 for example), which is detected\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"as well. You can however force whatever you need in case autodetection\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(b"doesn't work for you.\n\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"Options that affect what kind of output will be produced:\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"                   -d: Output raw captions in DVD format\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"                       (DEFAULT is broadcast format)\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"                 -srt: Generate .srt instead of .bin.\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"                -sami: Generate .sami instead of .bin.\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"                -utf8: Encode subtitles in UTF-8 instead of Latin-1\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"             -unicode: Encode subtitles in Unicode instead of Latin-1\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"  -nofc --nofontcolor: For .srt/.sami, don't add font color tags.\n\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"    -sc --sentencecap: Sentence capitalization. Use if you hate.\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"                       ALL CAPS in subtitles.\n\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b"  --capfile -caf file: Add the contents of 'file' to the list of words\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"                       that must be capitalized. For example, if file\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"                       is a plain text file that contains\n\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(b"                       Tony\n\0" as *const u8 as *const libc::c_char);
    printf(b"                       Alan\n\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"                       Whenever those words are found they will be written\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"                       exactly as they appear in the file.\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"                       Use one line per word. Lines starting with # are\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"                       considered comments and discarded.\n\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"Options that affect how ccextractor reads and writes (buffering):\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"    -bo -bufferoutput: Buffer writes. Might help a bit with performance.\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"     -bi -bufferinput: Forces input buffering.\n\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b" -nobi -nobufferinput: Disables input buffering.\n\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"Options that affect the built-in closed caption decoder:\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"                 -dru: Direct Roll-Up. When in roll-up mode, write character by\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"                       character instead of line by line. Note that this\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"                       produces (much) larger files.\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"                -noff: Disable FF clean-up. This is extra sanity check when\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"                       processing CC blocks. FF clean-up usually gets rid of\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"                       garbage produced by false CC block, but might cause\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"                       good characters to be missed. Use this option if you\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"                       prefer not to have any character discarded. Note that\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"                       this option is probably no longer needed and will\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(b"                       be removed soon.\n\n\0" as *const u8 as *const libc::c_char);
    printf(b"Options that affect timing:\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"    -noap --noautopad: Disable autopad. By default ccextractor pads closed\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"                       captions data to ensure that there's exactly 29.97 CC\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"                       2-byte blocks per second. Usually this fixes timing\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"                       issues, but you may disable it with this option.\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"                       Note that autopadding only happens in TS mode.\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"         -gp --goppad: Use GOP timing for padding instead of PTS. Use this\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"                       if you need padding on a non-TS file.\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"            -delay ms: For srt/sami, add this number of milliseconds to\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"                       all times. For example, -delay 400 makes subtitles\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"                       appear 400ms late. You can also use negative numbers\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"                       to make subs appear early.\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"Notes on times: -startat and -endat times are used first, then -delay.\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"So if you use -srt -startat 3:00 -endat 5:00 -delay 12000, ccextractor will\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"generate a .srt file, with only data from 3:00 to 5:00 in the input file(s)\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"and then add that (huge) delay, which would make the final file start at\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(b"5:00 and end at 7:00.\n\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"Options that affect what segment of the input file(s) to process:\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"        -startat time: For .srt/.sami, only write subtitles that start after\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"                       the given time. Time can be seconds, MM:SS or HH:MM:SS.\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"                       For example, -startat 3:00 means 'start writing from\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(b"                       minute 3.\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"                       This option is ignored in raw mode.\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"          -endat time: Stop processing after the given time (same format as\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"                       -startat). This option is honored in all output\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(b"                       formats.\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"-scr --screenfuls num: Write 'num' screenfuls and terminate processing.\n\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(b"Options that affect debug data:\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"               -debug: For HDTV dumps 'interesting' packets.\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"                 -608: Print debug traces from the EIA-608 decoder.\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"                       If you need to submit a bug report, please send\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"                       the output from this option.\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn totalblockswritten_thisfile() -> libc::c_uint {
    let mut blocks: libc::c_uint = c1count_total.wrapping_add(c1count);
    if blocks == 0 as libc::c_int as libc::c_uint {
        blocks = c2count_total.wrapping_add(c2count);
    }
    return blocks;
}
#[no_mangle]
pub unsafe extern "C" fn init_write(mut wb: *mut s_write) {
    let ref mut fresh0 = (*wb).fh;
    *fresh0 = 0 as *mut FILE;
    let ref mut fresh1 = (*wb).filename;
    *fresh1 = 0 as *mut libc::c_char;
    let ref mut fresh2 = (*wb).buffer;
    *fresh2 =
        malloc((256 as libc::c_int * 1024 as libc::c_int + 120 as libc::c_int) as libc::c_ulong)
            as *mut libc::c_uchar;
    (*wb).used = 0 as libc::c_int;
    let ref mut fresh3 = (*wb).data608;
    *fresh3 = malloc(::std::mem::size_of::<eia608>() as libc::c_ulong) as *mut eia608;
    init_eia608((*wb).data608);
}
#[no_mangle]
pub unsafe extern "C" fn writeraw(
    data: *const libc::c_uchar,
    length: libc::c_int,
    mut wb: *mut s_write,
) {
    if buffer_output != 0 {
        if data.is_null()
            || (*wb).used + length > 256 as libc::c_int * 1024 as libc::c_int + 120 as libc::c_int
        {
            fwrite(
                (*wb).buffer as *const libc::c_void,
                (*wb).used as libc::c_ulong,
                1 as libc::c_int as libc::c_ulong,
                (*wb).fh,
            );
            if !data.is_null() {
                fwrite(
                    data as *const libc::c_void,
                    length as libc::c_ulong,
                    1 as libc::c_int as libc::c_ulong,
                    (*wb).fh,
                );
            }
            (*wb).used = 0 as libc::c_int;
        } else {
            memcpy(
                ((*wb).buffer).offset((*wb).used as isize) as *mut libc::c_void,
                data as *const libc::c_void,
                length as libc::c_ulong,
            );
            (*wb).used += length;
        }
    } else {
        fwrite(
            data as *const libc::c_void,
            length as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            (*wb).fh,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn writedata(
    data: *const libc::c_uchar,
    length: libc::c_int,
    wb: *mut s_write,
) {
    if extraction_end.set != 0 {
        let pg: libc::c_uint = if c1global != 0 { c1global } else { c2global };
        if pg.wrapping_add(totalblockswritten_thisfile()) as libc::c_long
            > extraction_end.time_in_ccblocks
        {
            processed_enough = 1 as libc::c_int;
            return;
        }
    }
    if write_format == OF_RAW as libc::c_int {
        writeraw(data, length, wb);
    } else if !data.is_null() {
        process608(data, length, wb);
    }
}
#[no_mangle]
pub unsafe extern "C" fn flushbuffer(wb: *mut s_write, closefile: libc::c_int) {
    if buffer_output != 0 {
        writedata(0 as *const libc::c_uchar, 0 as libc::c_int, wb);
    }
    if closefile != 0 && !wb.is_null() && !((*wb).fh).is_null() {
        fclose((*wb).fh);
    }
}
#[no_mangle]
pub unsafe extern "C" fn too_many_blocks() -> libc::c_int {
    if first_gop_time.inited != 0 {
        let mis1: libc::c_int = (gop_time.ccblocks - first_gop_time.ccblocks
            + frames_since_last_gop as libc::c_long
            - c1count as libc::c_long) as libc::c_int;
        if mis1 < 0 as libc::c_int {
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn printdata(
    data1: *const libc::c_uchar,
    length1: libc::c_int,
    data2: *const libc::c_uchar,
    length2: libc::c_int,
) {
    if rawmode == 0 as libc::c_int {
        if length1 != 0 && extract != 2 as libc::c_int {
            writedata(data1, length1, &mut wbout1);
        }
        if length2 != 0 && extract != 1 as libc::c_int {
            writedata(data2, length2, &mut wbout2);
        }
    } else {
        if datacount == 0 as libc::c_int {
            writedata(
                DVD_HEADER.as_ptr(),
                ::std::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong as libc::c_int,
                &mut wbout1,
            );
            if loopcount == 1 as libc::c_int {
                writedata(
                    lc1.as_ptr(),
                    ::std::mem::size_of::<[libc::c_uchar; 1]>() as libc::c_ulong as libc::c_int,
                    &mut wbout1,
                );
            }
            if loopcount == 2 as libc::c_int {
                writedata(
                    lc2.as_ptr(),
                    ::std::mem::size_of::<[libc::c_uchar; 1]>() as libc::c_ulong as libc::c_int,
                    &mut wbout1,
                );
            }
            if loopcount == 3 as libc::c_int {
                writedata(
                    lc3.as_ptr(),
                    ::std::mem::size_of::<[libc::c_uchar; 2]>() as libc::c_ulong as libc::c_int,
                    &mut wbout1,
                );
                writedata(data2, length2, &mut wbout1);
            }
            if loopcount > 3 as libc::c_int {
                writedata(
                    lc4.as_ptr(),
                    ::std::mem::size_of::<[libc::c_uchar; 2]>() as libc::c_ulong as libc::c_int,
                    &mut wbout1,
                );
                writedata(data2, length2, &mut wbout1);
            }
        }
        datacount += 1;
        writedata(
            lc5.as_ptr(),
            ::std::mem::size_of::<[libc::c_uchar; 1]>() as libc::c_ulong as libc::c_int,
            &mut wbout1,
        );
        writedata(data1, length1, &mut wbout1);
        if loopcount == 1 as libc::c_int && datacount < 5 as libc::c_int
            || loopcount == 2 as libc::c_int && datacount < 8 as libc::c_int
            || loopcount == 3 as libc::c_int && datacount < 11 as libc::c_int
            || loopcount > 3 as libc::c_int && datacount < 15 as libc::c_int
        {
            writedata(
                lc6.as_ptr(),
                ::std::mem::size_of::<[libc::c_uchar; 1]>() as libc::c_ulong as libc::c_int,
                &mut wbout1,
            );
            writedata(data2, length2, &mut wbout1);
        } else {
            if loopcount == 1 as libc::c_int {
                writedata(
                    lc6.as_ptr(),
                    ::std::mem::size_of::<[libc::c_uchar; 1]>() as libc::c_ulong as libc::c_int,
                    &mut wbout1,
                );
                writedata(data2, length2, &mut wbout1);
            }
            loopcount += 1;
            datacount = 0 as libc::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn dump(start: *mut libc::c_uchar, mut l: libc::c_int) {
    let mut x: libc::c_int = 0;
    x = 0 as libc::c_int;
    while x < l {
        let mut j: libc::c_int = 0;
        printf(b"%03d | \0" as *const u8 as *const libc::c_char, x);
        j = 0 as libc::c_int;
        while j < 16 as libc::c_int {
            if x + j == 34 as libc::c_int {
                x += 4 as libc::c_int;
                l += 4 as libc::c_int;
            }
            if x + j < l {
                printf(
                    b"%02X \0" as *const u8 as *const libc::c_char,
                    *start.offset((x + j) as isize) as libc::c_int,
                );
            } else {
                printf(b"   \0" as *const u8 as *const libc::c_char);
            }
            j += 1;
        }
        printf(b" | \0" as *const u8 as *const libc::c_char);
        j = 0 as libc::c_int;
        while j < 16 as libc::c_int {
            if x + j <= l && *start.offset((x + j) as isize) as libc::c_int >= ' ' as i32 {
                printf(
                    b"%c\0" as *const u8 as *const libc::c_char,
                    *start.offset((x + j) as isize) as libc::c_int,
                );
            } else {
                printf(b" \0" as *const u8 as *const libc::c_char);
            }
            j += 1;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        x = x + 16 as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn prepare_for_new_file() {
    min_pts = 0xffffffff as libc::c_uint as LONG;
    max_pts = 0 as libc::c_int as LONG;
    last_pts = 0 as libc::c_int as LONG;
    pts_set = 0 as libc::c_int;
    inputsize = 0 as libc::c_int as LONG;
    last_reported_progress = -(1 as libc::c_int);
    stat_numuserheaders = 0 as libc::c_int;
    stat_dvdccheaders = 0 as libc::c_int;
    stat_replay5000headers = 0 as libc::c_int;
    stat_replay4000headers = 0 as libc::c_int;
    stat_dishheaders = 0 as libc::c_int;
    stat_hdtv = 0 as libc::c_int;
    total_frames_count = 0 as libc::c_int as libc::c_uint;
    cc_stats[0 as libc::c_int as usize] = 0 as libc::c_int;
    cc_stats[1 as libc::c_int as usize] = 0 as libc::c_int;
    cc_stats[2 as libc::c_int as usize] = 0 as libc::c_int;
    cc_stats[3 as libc::c_int as usize] = 0 as libc::c_int;
    false_pict_header = 0 as libc::c_int;
    frames_since_last_gop = 0 as libc::c_int;
    gop_time.inited = 0 as libc::c_int;
    first_gop_time.inited = 0 as libc::c_int;
    gop_rollover = 0 as libc::c_int;
    printed_gop.inited = 0 as libc::c_int;
    c1count = 0 as libc::c_int as libc::c_uint;
    c2count = 0 as libc::c_int as libc::c_uint;
    c1count_total = 0 as libc::c_int as libc::c_uint;
    c2count_total = 0 as libc::c_int as libc::c_uint;
    past = 0 as libc::c_int as LONG;
    pts_big_change = 0 as libc::c_int as libc::c_uint;
    startbytes_pos = 0 as libc::c_int as libc::c_uint;
    startbytes_avail = 0 as libc::c_int as libc::c_uint;
    net_fields = 20 as libc::c_int as LONG;
    init_file_buffer();
}
#[no_mangle]
pub unsafe extern "C" fn parsedelay(par: *mut libc::c_char) -> libc::c_int {
    let mut sign: libc::c_int = 0 as libc::c_int;
    let mut c: *mut libc::c_char = par;
    while *c != 0 {
        if *c as libc::c_int == '-' as i32 || *c as libc::c_int == '+' as i32 {
            if c != par {
                return 1 as libc::c_int;
            }
            if *c as libc::c_int == '-' as i32 {
                sign = 1 as libc::c_int;
            }
        } else {
            if isdigit(*c as libc::c_int) == 0 {
                return 1 as libc::c_int;
            }
            subs_delay = subs_delay * 10 as libc::c_int as libc::c_long
                + (*c as libc::c_int - '0' as i32) as libc::c_long;
        }
        c = c.offset(1);
    }
    if sign != 0 {
        subs_delay = -subs_delay;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn init_boundary_time(mut bt: *mut boundary_time) {
    (*bt).hh = 0 as libc::c_int;
    (*bt).mm = 0 as libc::c_int;
    (*bt).ss = 0 as libc::c_int;
    (*bt).set = 0 as libc::c_int;
    (*bt).time_in_ms = 0 as libc::c_int as LONG;
    (*bt).time_in_ccblocks = 0 as libc::c_int as LONG;
}
#[no_mangle]
pub unsafe extern "C" fn stringztoms(
    s: *mut libc::c_char,
    mut bt: *mut boundary_time,
) -> libc::c_int {
    let mut ss: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut mm: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut hh: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut value: libc::c_int = -(1 as libc::c_int);
    let mut colons: libc::c_int = 0 as libc::c_int;
    let mut secs: LONG = 0;
    let mut c: *mut libc::c_char = s;
    while *c != 0 {
        if *c as libc::c_int == ':' as i32 {
            if value == -(1 as libc::c_int) {
                return -(1 as libc::c_int);
            }
            colons += 1;
            if colons > 2 as libc::c_int {
                return -(1 as libc::c_int);
            }
            hh = mm;
            mm = ss;
            ss = value as libc::c_uint;
            value = -(1 as libc::c_int);
        } else {
            if isdigit(*c as libc::c_int) == 0 {
                return -(1 as libc::c_int);
            }
            if value == -(1 as libc::c_int) {
                value = *c as libc::c_int - '0' as i32;
            } else {
                value = value * 10 as libc::c_int + *c as libc::c_int - '0' as i32;
            }
        }
        c = c.offset(1);
    }
    hh = mm;
    mm = ss;
    ss = value as libc::c_uint;
    if mm > 59 as libc::c_int as libc::c_uint || ss > 59 as libc::c_int as libc::c_uint {
        return -(1 as libc::c_int);
    }
    (*bt).set = 1 as libc::c_int;
    (*bt).hh = hh as libc::c_int;
    (*bt).mm = mm as libc::c_int;
    (*bt).ss = ss as libc::c_int;
    secs = hh
        .wrapping_mul(3600 as libc::c_int as libc::c_uint)
        .wrapping_add(mm.wrapping_mul(60 as libc::c_int as libc::c_uint))
        .wrapping_add(ss) as LONG;
    (*bt).time_in_ms = secs * 1000 as libc::c_int as libc::c_long;
    (*bt).time_in_ccblocks = (secs as libc::c_double * 29.97f64) as LONG;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn add_word(word: *const libc::c_char) -> libc::c_int {
    let mut len: size_t = 0;
    let mut i: size_t = 0;
    let mut new_lower: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut new_correct: *mut libc::c_char = 0 as *mut libc::c_char;
    if spell_words == spell_capacity {
        spell_capacity += 50 as libc::c_int;
        spell_lower = realloc(
            spell_lower as *mut libc::c_void,
            (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                .wrapping_mul(spell_capacity as libc::c_ulong),
        ) as *mut *mut libc::c_char;
        spell_correct = realloc(
            spell_correct as *mut libc::c_void,
            (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                .wrapping_mul(spell_capacity as libc::c_ulong),
        ) as *mut *mut libc::c_char;
    }
    len = strlen(word);
    new_lower = malloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong)) as *mut libc::c_char;
    new_correct = malloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong)) as *mut libc::c_char;
    if spell_lower.is_null()
        || spell_correct.is_null()
        || new_lower.is_null()
        || new_correct.is_null()
    {
        printf(b"\rNot enough memory.\n\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    strcpy(new_correct, word);
    i = 0 as libc::c_int as size_t;
    while i < len {
        let mut c: libc::c_char = 0;
        c = *new_correct.offset(i as isize);
        c = tolower(c as libc::c_int) as libc::c_char;
        *new_lower.offset(i as isize) = c;
        i = i.wrapping_add(1);
    }
    *new_lower.offset(len as isize) = 0 as libc::c_int as libc::c_char;
    let ref mut fresh4 = *spell_lower.offset(spell_words as isize);
    *fresh4 = new_lower;
    let ref mut fresh5 = *spell_correct.offset(spell_words as isize);
    *fresh5 = new_correct;
    spell_words += 1;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn add_built_in_words() -> libc::c_int {
    if spell_builtin_added == 0 {
        let mut i: libc::c_int = 0 as libc::c_int;
        while !(spell_builtin[i as usize]).is_null() {
            if add_word(spell_builtin[i as usize]) != 0 {
                return -(1 as libc::c_int);
            }
            i += 1;
        }
        spell_builtin_added = 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn process_cap_file(filename: *mut libc::c_char) -> libc::c_int {
    let fi: *mut FILE = fopen(filename, b"rt\0" as *const u8 as *const libc::c_char);
    let mut num: libc::c_int = 0 as libc::c_int;
    let mut line: [libc::c_char; 35] = [0; 35];
    if fi.is_null() {
        printf(
            b"\rUnable to open capitalization file: %s\n\0" as *const u8 as *const libc::c_char,
            filename,
        );
        return -(1 as libc::c_int);
    }
    while !(fgets(line.as_mut_ptr(), 35 as libc::c_int, fi)).is_null() {
        let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
        num += 1;
        if line[0 as libc::c_int as usize] as libc::c_int == '#' as i32 {
            continue;
        }
        c = line
            .as_mut_ptr()
            .offset(strlen(line.as_mut_ptr()) as isize)
            .offset(-(1 as libc::c_int as isize));
        while c >= line.as_mut_ptr()
            && (*c as libc::c_int == 0xd as libc::c_int || *c as libc::c_int == 0xa as libc::c_int)
        {
            *c = 0 as libc::c_int as libc::c_char;
            c = c.offset(-1);
        }
        if strlen(line.as_mut_ptr()) > 32 as libc::c_int as libc::c_ulong {
            printf(
                b"Word in line %d too long, max = 32 characters.\n\0" as *const u8
                    as *const libc::c_char,
                num,
            );
            fclose(fi);
            return -(1 as libc::c_int);
        }
        if strlen(line.as_mut_ptr()) > 0 as libc::c_int as libc::c_ulong {
            if add_word(line.as_mut_ptr()) != 0 {
                return -(1 as libc::c_int);
            }
        }
    }
    fclose(fi);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CEW_reinit() {
    if !(wbout1.fh).is_null() {
        fclose(wbout1.fh);
    }
    if !(wbout1.filename).is_null() && strlen(wbout1.filename) > 0 as libc::c_int as libc::c_ulong {
        wbout1.fh = fopen(wbout1.filename, b"wb\0" as *const u8 as *const libc::c_char);
    }
    init_boundary_time(&mut extraction_start);
    init_boundary_time(&mut extraction_end);
    wbout1.used = 0 as libc::c_int;
    if !(wbout1.data608).is_null() {
        init_eia608(wbout1.data608);
    }
    prepare_for_new_file();
    if !(wbout1.fh).is_null() {
        if write_format == OF_RAW as libc::c_int {
            writeraw(
                BROADCAST_HEADER.as_ptr(),
                ::std::mem::size_of::<[libc::c_uchar; 4]>() as libc::c_ulong as libc::c_int,
                &mut wbout1,
            );
        } else {
            if encoding == ENC_UNICODE as libc::c_int {
                writeraw(
                    LITTLE_ENDIAN_BOM.as_ptr(),
                    ::std::mem::size_of::<[libc::c_uchar; 2]>() as libc::c_ulong as libc::c_int,
                    &mut wbout1,
                );
            }
            write_subtitle_file_header(&mut wbout1);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn CEW_init(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut output_filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut clean_filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut extension: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut start: time_t = 0;
    init_write(&mut wbout1);
    init_write(&mut wbout2);
    init_boundary_time(&mut extraction_start);
    init_boundary_time(&mut extraction_end);
    i = 1 as libc::c_int;
    while i < argc {
        if *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize) as libc::c_int
            != '-' as i32
        {
            if inputfile_capacity <= num_input_files {
                inputfile_capacity += 10 as libc::c_int;
                inputfile = realloc(
                    inputfile as *mut libc::c_void,
                    (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                        .wrapping_mul(inputfile_capacity as libc::c_ulong),
                ) as *mut *mut libc::c_char;
            }
            let ref mut fresh6 = *inputfile.offset(num_input_files as isize);
            *fresh6 = *argv.offset(i as isize);
            num_input_files += 1;
        }
        if strcmp(
            *argv.offset(i as isize),
            b"-bo\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            || strcmp(
                *argv.offset(i as isize),
                b"--bufferoutput\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            buffer_output = 1 as libc::c_int;
        }
        if strcmp(
            *argv.offset(i as isize),
            b"-bi\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            || strcmp(
                *argv.offset(i as isize),
                b"--bufferinput\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            buffer_input = 1 as libc::c_int;
        }
        if strcmp(
            *argv.offset(i as isize),
            b"-nobi\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            || strcmp(
                *argv.offset(i as isize),
                b"--nobufferinput\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            buffer_input = 0 as libc::c_int;
        }
        if strcmp(
            *argv.offset(i as isize),
            b"-d\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            rawmode = 1 as libc::c_int;
        }
        if strcmp(
            *argv.offset(i as isize),
            b"-dru\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            direct_rollup = 1 as libc::c_int;
        }
        if strcmp(
            *argv.offset(i as isize),
            b"-nots\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            auto_ts = 0 as libc::c_int;
        }
        if strcmp(
            *argv.offset(i as isize),
            b"-nofc\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            || strcmp(
                *argv.offset(i as isize),
                b"--nofontcolor\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            nofontcolor = 1 as libc::c_int;
        }
        if strcmp(
            *argv.offset(i as isize),
            b"-ts\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            auto_ts = 1 as libc::c_int;
        }
        if strcmp(
            *argv.offset(i as isize),
            b"-12\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            extract = 12 as libc::c_int;
        }
        if strcmp(
            *argv.offset(i as isize),
            b"-noff\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            ff_cleanup = 0 as libc::c_int;
        }
        if strcmp(
            *argv.offset(i as isize),
            b"-fp\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            || strcmp(
                *argv.offset(i as isize),
                b"--fixpadding\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            fix_padding = 1 as libc::c_int;
        }
        if strcmp(
            *argv.offset(i as isize),
            b"-noap\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            || strcmp(
                *argv.offset(i as isize),
                b"--noautopad\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            autopad = 0 as libc::c_int;
        }
        if strcmp(
            *argv.offset(i as isize),
            b"-gp\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            || strcmp(
                *argv.offset(i as isize),
                b"--goppad\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            gop_pad = 1 as libc::c_int;
        }
        if strcmp(
            *argv.offset(i as isize),
            b"-debug\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            debug = 1 as libc::c_int;
        }
        if strcmp(
            *argv.offset(i as isize),
            b"--sentencecap\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            || strcmp(
                *argv.offset(i as isize),
                b"-sc\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            if add_built_in_words() != 0 {
                exit(-(1 as libc::c_int));
            }
            sentence_cap = 1 as libc::c_int;
        }
        if (strcmp(
            *argv.offset(i as isize),
            b"--capfile\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            || strcmp(
                *argv.offset(i as isize),
                b"-caf\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int)
            && i < argc - 1 as libc::c_int
        {
            if add_built_in_words() != 0 {
                exit(-(1 as libc::c_int));
            }
            if process_cap_file(*argv.offset((i + 1 as libc::c_int) as isize)) != 0 as libc::c_int {
                exit(-(1 as libc::c_int));
            }
            sentence_cap = 1 as libc::c_int;
            sentence_cap_file = *argv.offset((i + 1 as libc::c_int) as isize);
            i += 1;
        }
        if (strcmp(
            *argv.offset(i as isize),
            b"--defaultcolor\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            || strcmp(
                *argv.offset(i as isize),
                b"-dc\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int)
            && i < argc - 1 as libc::c_int
        {
            if strlen(*argv.offset((i + 1 as libc::c_int) as isize))
                != 7 as libc::c_int as libc::c_ulong
                || *(*argv.offset((i + 1 as libc::c_int) as isize))
                    .offset(0 as libc::c_int as isize) as libc::c_int
                    != '#' as i32
            {
                printf(
                    b"\r--defaultcolor expects a 7 character parameter that starts with #\n\0"
                        as *const u8 as *const libc::c_char,
                );
                exit(-(1 as libc::c_int));
            }
            strcpy(
                usercolor_rgb.as_mut_ptr() as *mut libc::c_char,
                *argv.offset((i + 1 as libc::c_int) as isize),
            );
            default_color = COL_USERDEFINED as libc::c_int;
            i += 1;
        }
        if strcmp(
            *argv.offset(i as isize),
            b"-delay\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            && i < argc - 1 as libc::c_int
        {
            if parsedelay(*argv.offset((i + 1 as libc::c_int) as isize)) != 0 {
                printf(
                    b"\r-delay only accept integers (such as -300 or 300)\n\0" as *const u8
                        as *const libc::c_char,
                );
                exit(-(1 as libc::c_int));
            }
            i += 1;
        }
        if (strcmp(
            *argv.offset(i as isize),
            b"-scr\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            || strcmp(
                *argv.offset(i as isize),
                b"--screenfuls\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int)
            && i < argc - 1 as libc::c_int
        {
            screens_to_process = atoi(*argv.offset((i + 1 as libc::c_int) as isize)) as LONG;
            if screens_to_process < 0 as libc::c_int as libc::c_long {
                printf(
                    b"\r--screenfuls only accepts positive integers.\n\0" as *const u8
                        as *const libc::c_char,
                );
                exit(-(1 as libc::c_int));
            }
            i += 1;
        }
        if strcmp(
            *argv.offset(i as isize),
            b"-startat\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            && i < argc - 1 as libc::c_int
        {
            if stringztoms(
                *argv.offset((i + 1 as libc::c_int) as isize),
                &mut extraction_start,
            ) == -(1 as libc::c_int)
            {
                printf(
                    b"\r-startat only accepts SS, MM:SS or HH:MM:SS\n\0" as *const u8
                        as *const libc::c_char,
                );
                exit(-(1 as libc::c_int));
            }
            i += 1;
        }
        if strcmp(
            *argv.offset(i as isize),
            b"-endat\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            && i < argc - 1 as libc::c_int
        {
            if stringztoms(
                *argv.offset((i + 1 as libc::c_int) as isize),
                &mut extraction_end,
            ) == -(1 as libc::c_int)
            {
                printf(
                    b"\r-endat only accepts SS, MM:SS or HH:MM:SS\n\0" as *const u8
                        as *const libc::c_char,
                );
                exit(-(1 as libc::c_int));
            }
            i += 1;
        }
        if strcmp(
            *argv.offset(i as isize),
            b"-1\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            extract = 1 as libc::c_int;
        }
        if strcmp(
            *argv.offset(i as isize),
            b"-2\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            extract = 2 as libc::c_int;
        }
        if strcmp(
            *argv.offset(i as isize),
            b"-srt\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            write_format = OF_SRT as libc::c_int;
        }
        if strcmp(
            *argv.offset(i as isize),
            b"-sami\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            write_format = OF_SAMI as libc::c_int;
        }
        if strcmp(
            *argv.offset(i as isize),
            b"-bin\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            input_bin = 1 as libc::c_int;
        }
        if strcmp(
            *argv.offset(i as isize),
            b"-cc2\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            || strcmp(
                *argv.offset(i as isize),
                b"-CC2\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            cc_channel = 2 as libc::c_int;
        }
        if strcmp(
            *argv.offset(i as isize),
            b"-608\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            debug_608 = 1 as libc::c_int;
        }
        if !(strstr(
            *argv.offset(i as isize),
            b"-unicode\0" as *const u8 as *const libc::c_char,
        ))
        .is_null()
        {
            encoding = ENC_UNICODE as libc::c_int;
        }
        if !(strstr(
            *argv.offset(i as isize),
            b"-utf8\0" as *const u8 as *const libc::c_char,
        ))
        .is_null()
        {
            encoding = ENC_UTF_8 as libc::c_int;
        }
        if !(strstr(
            *argv.offset(i as isize),
            b"-myth\0" as *const u8 as *const libc::c_char,
        ))
        .is_null()
        {
            auto_myth = 1 as libc::c_int;
        }
        if !(strstr(
            *argv.offset(i as isize),
            b"-nomyth\0" as *const u8 as *const libc::c_char,
        ))
        .is_null()
        {
            auto_myth = 0 as libc::c_int;
        }
        if strcmp(
            *argv.offset(i as isize),
            b"-o\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            && i < argc - 1 as libc::c_int
        {
            output_filename = *argv.offset((i + 1 as libc::c_int) as isize);
            i += 1;
        }
        if strcmp(
            *argv.offset(i as isize),
            b"-cf\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            && i < argc - 1 as libc::c_int
        {
            clean_filename = *argv.offset((i + 1 as libc::c_int) as isize);
            i += 1;
        }
        if strcmp(
            *argv.offset(i as isize),
            b"-o1\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            && i < argc - 1 as libc::c_int
        {
            wbout1.filename = *argv.offset((i + 1 as libc::c_int) as isize);
            i += 1;
        }
        if strcmp(
            *argv.offset(i as isize),
            b"-o2\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            && i < argc - 1 as libc::c_int
        {
            wbout2.filename = *argv.offset((i + 1 as libc::c_int) as isize);
            i += 1;
        }
        i += 1;
    }
    if num_input_files == 0 as libc::c_int {
        usage();
        exit(2 as libc::c_int);
    }
    if !output_filename.is_null() {
        if extract == 2 as libc::c_int {
            wbout2.filename = output_filename;
        } else {
            wbout1.filename = output_filename;
        }
    }
    match write_format {
        0 => {
            extension = b".bin\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        1 => {
            extension = b".srt\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        2 => {
            extension = b".smi\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        _ => {
            printf(
                b"write_format doesn't have any legal value, this is a bug.\n\0" as *const u8
                    as *const libc::c_char,
            );
            exit(500 as libc::c_int);
        }
    }
    if input_bin != 0 && write_format == OF_RAW as libc::c_int {
        printf(
            b"-bin can only be used if the output is a subtitle file.\n\0" as *const u8
                as *const libc::c_char,
        );
        printf(
            b"If you want to produce a raw closed captions dump from\n\0" as *const u8
                as *const libc::c_char,
        );
        printf(
            b"a raw closed captions dump just copy the file.\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(-(5 as libc::c_int));
    }
    fbuffer =
        malloc((256 as libc::c_int * 1024 as libc::c_int + 120 as libc::c_int) as libc::c_ulong)
            as *mut libc::c_uchar;
    subline = malloc(2048 as libc::c_int as libc::c_ulong) as *mut libc::c_uchar;
    pesheaderbuf = malloc(188 as libc::c_int as libc::c_ulong) as *mut libc::c_uchar;
    basefilename = malloc(
        (strlen(*inputfile.offset(0 as libc::c_int as isize)))
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    if (wbout1.filename).is_null() {
        wbout1.filename = malloc(
            (strlen(*inputfile.offset(0 as libc::c_int as isize)))
                .wrapping_add(3 as libc::c_int as libc::c_ulong)
                .wrapping_add(strlen(extension)),
        ) as *mut libc::c_char;
        *(wbout1.filename).offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    }
    if (wbout2.filename).is_null() {
        wbout2.filename = malloc(
            (strlen(*inputfile.offset(0 as libc::c_int as isize)))
                .wrapping_add(3 as libc::c_int as libc::c_ulong)
                .wrapping_add(strlen(extension)),
        ) as *mut libc::c_char;
        *(wbout2.filename).offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    }
    if fbuffer.is_null()
        || basefilename.is_null()
        || pesheaderbuf.is_null()
        || (wbout1.filename).is_null()
        || (wbout2.filename).is_null()
        || subline.is_null()
        || init_file_buffer() != 0
    {
        printf(b"Not enough memory\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    strcpy(basefilename, *inputfile.offset(0 as libc::c_int as isize));
    c = basefilename
        .offset(strlen(basefilename) as isize)
        .offset(-(1 as libc::c_int as isize));
    while c > basefilename && *c as libc::c_int != '.' as i32 {
        c = c.offset(-1);
    }
    if *c as libc::c_int == '.' as i32 {
        *c = 0 as libc::c_int as libc::c_char;
    }
    if rawmode == 1 as libc::c_int {
        if *(wbout1.filename).offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int {
            strcpy(wbout1.filename, basefilename);
            strcat(
                wbout1.filename,
                b".bin\0" as *const u8 as *const libc::c_char,
            );
        }
        printf(
            b"Creating %s\n\0" as *const u8 as *const libc::c_char,
            wbout1.filename,
        );
        wbout1.fh = fopen(wbout1.filename, b"wb\0" as *const u8 as *const libc::c_char);
        if (wbout1.fh).is_null() {
            printf(b"Failed\n\0" as *const u8 as *const libc::c_char);
            exit(3 as libc::c_int);
        }
    } else {
        if extract != 2 as libc::c_int {
            if *(wbout1.filename).offset(0 as libc::c_int as isize) as libc::c_int
                == 0 as libc::c_int
            {
                strcpy(wbout1.filename, basefilename);
                strcat(wbout1.filename, extension as *const libc::c_char);
            }
            printf(
                b"Creating %s\n\0" as *const u8 as *const libc::c_char,
                wbout1.filename,
            );
            wbout1.fh = fopen(wbout1.filename, b"wb\0" as *const u8 as *const libc::c_char);
            if (wbout1.fh).is_null() {
                printf(b"Failed\n\0" as *const u8 as *const libc::c_char);
                exit(3 as libc::c_int);
            }
            if write_format == OF_RAW as libc::c_int {
                writeraw(
                    BROADCAST_HEADER.as_ptr(),
                    ::std::mem::size_of::<[libc::c_uchar; 4]>() as libc::c_ulong as libc::c_int,
                    &mut wbout1,
                );
            } else {
                if encoding == ENC_UNICODE as libc::c_int {
                    writeraw(
                        LITTLE_ENDIAN_BOM.as_ptr(),
                        ::std::mem::size_of::<[libc::c_uchar; 2]>() as libc::c_ulong as libc::c_int,
                        &mut wbout1,
                    );
                }
                write_subtitle_file_header(&mut wbout1);
            }
        }
        if extract == 12 as libc::c_int {
            printf(b" and \n\0" as *const u8 as *const libc::c_char);
        }
        if extract != 1 as libc::c_int {
            if *(wbout2.filename).offset(0 as libc::c_int as isize) as libc::c_int
                == 0 as libc::c_int
            {
                strcpy(wbout2.filename, basefilename);
                strcat(wbout2.filename, b"_2\0" as *const u8 as *const libc::c_char);
                strcat(wbout2.filename, extension as *const libc::c_char);
            }
            printf(
                b"Creating %s\n\0" as *const u8 as *const libc::c_char,
                wbout2.filename,
            );
            wbout2.fh = fopen(wbout2.filename, b"wb\0" as *const u8 as *const libc::c_char);
            if (wbout2.fh).is_null() {
                printf(b"Failed\n\0" as *const u8 as *const libc::c_char);
                exit(3 as libc::c_int);
            }
            if write_format == OF_RAW as libc::c_int {
                writeraw(
                    BROADCAST_HEADER.as_ptr(),
                    ::std::mem::size_of::<[libc::c_uchar; 4]>() as libc::c_ulong as libc::c_int,
                    &mut wbout2,
                );
            } else {
                if encoding == ENC_UNICODE as libc::c_int {
                    writeraw(
                        LITTLE_ENDIAN_BOM.as_ptr(),
                        ::std::mem::size_of::<[libc::c_uchar; 2]>() as libc::c_ulong as libc::c_int,
                        &mut wbout1,
                    );
                }
                write_subtitle_file_header(&mut wbout2);
            }
        }
    }
    clean = 0 as *mut FILE;
    if !clean_filename.is_null() {
        clean = fopen(clean_filename, b"wb\0" as *const u8 as *const libc::c_char);
        if clean.is_null() {
            printf(
                b"Unable to open clean file: %s\n\0" as *const u8 as *const libc::c_char,
                clean_filename,
            );
            exit(-(4 as libc::c_int));
        }
    }
    encoded_crlf_length = encode_line(
        encoded_crlf.as_mut_ptr(),
        b"\r\n\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar,
    );
    encoded_br_length = encode_line(
        encoded_br.as_mut_ptr(),
        b"<br>\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar,
    );
    build_parity_table();
    time(&mut start);
    return 0 as libc::c_int;
}
