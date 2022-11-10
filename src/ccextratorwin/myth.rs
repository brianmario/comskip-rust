extern "C" {
    static mut current_pts: LONG;
    static mut pts_set: libc::c_int;
    static mut c1count: libc::c_uint;
    static mut c2count: libc::c_uint;
    static mut c1count_total: libc::c_uint;
    fn buffered_read_opt(buffer: *mut libc::c_uchar, bytes: libc::c_uint) -> LONG;
    static mut filebuffer: *mut libc::c_uchar;
    static mut filebuffer_pos: libc::c_int;
    static mut bytesinbuffer: libc::c_int;
    #[link_name = "in"]
    static mut in_0: libc::c_int;
    static mut inputsize: LONG;
    static mut processed_enough: libc::c_int;
    fn printdata(
        data1: *const libc::c_uchar,
        length1: libc::c_int,
        data2: *const libc::c_uchar,
        length2: libc::c_int,
    );
    fn init_file_buffer() -> libc::c_int;
    fn buffered_seek(offset: libc::c_int);
    static mut last_reported_progress: libc::c_int;
    static mut buffer_input: libc::c_int;
    fn process_block(data: *mut libc::c_uchar, length: LONG) -> LONG;
    static mut result: LONG;
}
pub type __int64_t = libc::c_longlong;
pub type __darwin_off_t = __int64_t;
pub type int64_t = libc::c_longlong;
pub type off_t = __darwin_off_t;
pub type LONG = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVPacket {
    pub pts: LONG,
    pub dts: LONG,
    pub data: *mut libc::c_uchar,
    pub size: libc::c_int,
    pub stream_index: libc::c_int,
    pub flags: libc::c_int,
    pub duration: libc::c_int,
    pub destruct: Option<unsafe extern "C" fn(*mut AVPacket) -> ()>,
    pub priv_0: *mut libc::c_void,
    pub pos: LONG,
    pub codec_id: libc::c_int,
    pub type_0: libc::c_int,
}
pub const CODEC_TYPE_VIDEO: CodecType = 0;
pub const CODEC_ID_MPEG2VIDEO: CodecID = 2;
pub const CODEC_TYPE_DATA: CodecType = 2;
pub const CODEC_ID_MPEG2VBI: CodecID = 94210;
pub const CODEC_ID_DVD_SUBTITLE: CodecID = 94208;
pub const CODEC_TYPE_SUBTITLE: CodecType = 3;
pub const CODEC_ID_PCM_S16BE: CodecID = 65537;
pub const CODEC_TYPE_AUDIO: CodecType = 1;
pub const CODEC_ID_DTS: CodecID = 86021;
pub const CODEC_ID_AC3: CodecID = 86020;
pub const CODEC_ID_MP2: CodecID = 86016;
pub const CODEC_ID_CAVS: CodecID = 91;
pub const CODEC_ID_H264: CodecID = 29;
pub const CODEC_ID_MPEG4: CodecID = 14;
pub const CODEC_ID_AAC: CodecID = 86018;
pub const CODEC_ID_MP3: CodecID = 86017;
pub type CodecType = libc::c_int;
pub type CodecID = libc::c_uint;
#[no_mangle]
pub static mut header_state: libc::c_uint = 0;
#[no_mangle]
pub static mut psm_es_type: [libc::c_uchar; 256] = [0; 256];
#[no_mangle]
pub static mut cc608_parity_table: [libc::c_int; 256] = [0; 256];
#[no_mangle]
pub static mut av: AVPacket = AVPacket {
    pts: 0,
    dts: 0,
    data: 0 as *const libc::c_uchar as *mut libc::c_uchar,
    size: 0,
    stream_index: 0,
    flags: 0,
    duration: 0,
    destruct: None,
    priv_0: 0 as *const libc::c_void as *mut libc::c_void,
    pos: 0,
    codec_id: 0,
    type_0: 0,
};
#[no_mangle]
pub unsafe extern "C" fn get_be16() -> libc::c_int {
    let mut a: libc::c_uchar = 0;
    let mut b: libc::c_uchar = 0;
    if bytesinbuffer - filebuffer_pos != 0 {
        if !(&mut a as *mut libc::c_uchar).is_null() {
            a = *filebuffer.offset(filebuffer_pos as isize);
            filebuffer_pos += 1;
            result = 1 as libc::c_int as LONG;
        }
    } else {
        result = buffered_read_opt(&mut a, 1 as libc::c_int as libc::c_uint);
    }
    if bytesinbuffer - filebuffer_pos != 0 {
        if !(&mut b as *mut libc::c_uchar).is_null() {
            b = *filebuffer.offset(filebuffer_pos as isize);
            filebuffer_pos += 1;
            result = 1 as libc::c_int as LONG;
        }
    } else {
        result = buffered_read_opt(&mut b, 1 as libc::c_int as libc::c_uint);
    }
    return (a as libc::c_int) << 8 as libc::c_int | b as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn get_byte() -> libc::c_int {
    let mut b: libc::c_uchar = 0;
    if bytesinbuffer - filebuffer_pos != 0 {
        if !(&mut b as *mut libc::c_uchar).is_null() {
            b = *filebuffer.offset(filebuffer_pos as isize);
            filebuffer_pos += 1;
            result = 1 as libc::c_int as LONG;
        }
    } else {
        result = buffered_read_opt(&mut b, 1 as libc::c_int as libc::c_uint);
    }
    if result == 1 as libc::c_int as libc::c_long {
        return b as libc::c_int;
    } else {
        return 0 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn get_be32() -> libc::c_uint {
    let mut val: libc::c_uint = 0;
    val = (get_be16() << 16 as libc::c_int) as libc::c_uint;
    val |= get_be16() as libc::c_uint;
    return val;
}
unsafe extern "C" fn get_pts(mut c: libc::c_int) -> LONG {
    let mut pts: LONG = 0;
    let mut val: libc::c_int = 0;
    if c < 0 as libc::c_int {
        c = get_byte();
    }
    pts = ((c >> 1 as libc::c_int & 0x7 as libc::c_int) as LONG) << 30 as libc::c_int;
    val = get_be16();
    pts |= ((val >> 1 as libc::c_int) as LONG) << 15 as libc::c_int;
    val = get_be16();
    pts |= (val >> 1 as libc::c_int) as LONG;
    return pts;
}
unsafe extern "C" fn find_next_start_code(
    size_ptr: *mut libc::c_int,
    header_state_0: *mut libc::c_uint,
) -> libc::c_int {
    let current_block: u64;
    let mut state: libc::c_uint = 0;
    let mut v: libc::c_uint = 0;
    let mut val: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    state = *header_state_0;
    n = *size_ptr;
    loop {
        if !(n > 0 as libc::c_int) {
            current_block = 13797916685926291137;
            break;
        }
        let mut cx: libc::c_uchar = 0;
        if bytesinbuffer - filebuffer_pos != 0 {
            if !(&mut cx as *mut libc::c_uchar).is_null() {
                cx = *filebuffer.offset(filebuffer_pos as isize);
                filebuffer_pos += 1;
                result = 1 as libc::c_int as LONG;
            }
        } else {
            result = buffered_read_opt(&mut cx, 1 as libc::c_int as libc::c_uint);
        }
        if result != 1 as libc::c_int as libc::c_long {
            current_block = 13797916685926291137;
            break;
        }
        v = cx as libc::c_uint;
        n -= 1;
        if state == 0x1 as libc::c_int as libc::c_uint {
            state = (state << 8 as libc::c_int | v) & 0xffffff as libc::c_int as libc::c_uint;
            val = state as libc::c_int;
            current_block = 18295522109918506392;
            break;
        } else {
            state = (state << 8 as libc::c_int | v) & 0xffffff as libc::c_int as libc::c_uint;
        }
    }
    match current_block {
        13797916685926291137 => {
            val = -(1 as libc::c_int);
        }
        _ => {}
    }
    *header_state_0 = state;
    *size_ptr = n;
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn url_fskip(length: libc::c_int) {
    buffered_seek(length);
}
unsafe extern "C" fn mpegps_psm_parse() -> libc::c_long {
    let mut psm_length: libc::c_int = 0;
    let mut ps_info_length: libc::c_int = 0;
    let mut es_map_length: libc::c_int = 0;
    psm_length = get_be16();
    get_byte();
    get_byte();
    ps_info_length = get_be16();
    url_fskip(ps_info_length);
    es_map_length = get_be16();
    while es_map_length >= 4 as libc::c_int {
        let type_0: libc::c_uchar = get_byte() as libc::c_uchar;
        let es_id: libc::c_uchar = get_byte() as libc::c_uchar;
        let es_info_length: libc::c_uint = get_be16() as libc::c_uint;
        psm_es_type[es_id as usize] = type_0;
        url_fskip(es_info_length as libc::c_int);
        es_map_length = (es_map_length as libc::c_uint)
            .wrapping_sub((4 as libc::c_int as libc::c_uint).wrapping_add(es_info_length))
            as libc::c_int as libc::c_int;
    }
    get_be32();
    return (2 as libc::c_int + psm_length) as libc::c_long;
}
unsafe extern "C" fn mpegps_read_pes_header(
    pstart_code: *mut libc::c_int,
    ppts: *mut LONG,
    pdts: *mut LONG,
) -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut startcode: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut header_len: libc::c_int = 0;
    let mut pts: LONG = 0;
    let mut dts: LONG = 0;
    'c_8550: loop {
        header_state = 0xff as libc::c_int as libc::c_uint;
        size = 100000 as libc::c_int;
        startcode = find_next_start_code(&mut size, &mut header_state);
        if startcode < 0 as libc::c_int {
            return -(2 as libc::c_int);
        }
        if startcode as libc::c_uint == 0x1ba as libc::c_int as libc::c_uint {
            continue;
        }
        if startcode as libc::c_uint == 0x1bb as libc::c_int as libc::c_uint {
            continue;
        }
        if startcode == 0x1be as libc::c_int || startcode == 0x1bf as libc::c_int {
            len = get_be16();
        } else if startcode == 0x1bc as libc::c_int {
            mpegps_psm_parse();
        } else {
            if !(startcode >= 0x1c0 as libc::c_int && startcode <= 0x1df as libc::c_int
                || startcode >= 0x1e0 as libc::c_int && startcode <= 0x1ef as libc::c_int
                || startcode == 0x1bd as libc::c_int)
            {
                continue;
            }
            len = get_be16();
            pts = 0x8000000000000000 as libc::c_ulong as int64_t as LONG;
            dts = 0x8000000000000000 as libc::c_ulong as int64_t as LONG;
            loop {
                if len < 1 as libc::c_int {
                    continue 'c_8550;
                }
                c = get_byte();
                len -= 1;
                if c != 0xff as libc::c_int {
                    break;
                }
            }
            if c & 0xc0 as libc::c_int == 0x40 as libc::c_int {
                if len < 2 as libc::c_int {
                    continue;
                }
                get_byte();
                c = get_byte();
                len -= 2 as libc::c_int;
            }
            if c & 0xf0 as libc::c_int == 0x20 as libc::c_int {
                if len < 4 as libc::c_int {
                    continue;
                }
                pts = get_pts(c);
                dts = pts;
                len -= 4 as libc::c_int;
            } else if c & 0xf0 as libc::c_int == 0x30 as libc::c_int {
                if len < 9 as libc::c_int {
                    continue;
                }
                pts = get_pts(c);
                dts = get_pts(-(1 as libc::c_int));
                len -= 9 as libc::c_int;
            } else if c & 0xc0 as libc::c_int == 0x80 as libc::c_int {
                flags = get_byte();
                header_len = get_byte();
                len -= 2 as libc::c_int;
                if header_len > len {
                    continue;
                }
                if flags & 0xc0 as libc::c_int == 0x80 as libc::c_int {
                    pts = get_pts(-(1 as libc::c_int));
                    dts = pts;
                    if header_len < 5 as libc::c_int {
                        continue;
                    }
                    header_len -= 5 as libc::c_int;
                    len -= 5 as libc::c_int;
                }
                if flags & 0xc0 as libc::c_int == 0xc0 as libc::c_int {
                    pts = get_pts(-(1 as libc::c_int));
                    dts = get_pts(-(1 as libc::c_int));
                    if header_len < 10 as libc::c_int {
                        continue;
                    }
                    header_len -= 10 as libc::c_int;
                    len -= 10 as libc::c_int;
                }
                len -= header_len;
                while header_len > 0 as libc::c_int {
                    get_byte();
                    header_len -= 1;
                }
            } else if c != 0xf as libc::c_int {
                continue;
            }
            if !(startcode == 0x1bd as libc::c_int) {
                break;
            }
            if len < 1 as libc::c_int {
                continue;
            }
            startcode = get_byte();
            len -= 1;
            if !(startcode >= 0x80 as libc::c_int && startcode <= 0xbf as libc::c_int) {
                break;
            }
            if len < 3 as libc::c_int {
                continue;
            }
            get_byte();
            get_byte();
            get_byte();
            len -= 3 as libc::c_int;
            break;
        }
    }
    *pstart_code = startcode;
    *ppts = pts;
    *pdts = dts;
    return len;
}
unsafe extern "C" fn cc608_good_parity(
    parity_table: *const libc::c_int,
    data: libc::c_uint,
) -> libc::c_int {
    let ret: libc::c_int =
        (*parity_table.offset((data & 0xff as libc::c_int as libc::c_uint) as isize) != 0
            && *parity_table.offset(
                ((data & 0xff00 as libc::c_int as libc::c_uint) >> 8 as libc::c_int) as isize,
            ) != 0) as libc::c_int;
    ret == 0;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn ProcessVBIDataPacket() {
    static mut min_blank: libc::c_uint = 6 as libc::c_int as libc::c_uint;
    let mut linemask: LONG = 0 as libc::c_int as LONG;
    let mut meat: *const libc::c_uchar = av.data;
    let mut i: libc::c_uint = 0;
    if meat.is_null() {
        libc::printf(
            b"Warning: ProcessVBIDataPacket called with NULL data, ignoring.\n\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if *meat.offset(0 as libc::c_int as isize) as libc::c_int == 't' as i32
        && *meat.offset(1 as libc::c_int as isize) as libc::c_int == 'v' as i32
        && *meat.offset(2 as libc::c_int as isize) as libc::c_int == '0' as i32
    {
        libc::memcpy(
            &mut linemask as *mut LONG as *mut libc::c_void,
            meat.offset(3 as libc::c_int as isize) as *const libc::c_void,
            8,
        );
        meat = meat.offset(11 as libc::c_int as isize);
    } else if *meat.offset(0 as libc::c_int as isize) as libc::c_int == 'T' as i32
        && *meat.offset(1 as libc::c_int as isize) as libc::c_int == 'V' as i32
        && *meat.offset(2 as libc::c_int as isize) as libc::c_int == '0' as i32
    {
        linemask = 0xffffffffffffffff as libc::c_ulong as LONG;
        meat = meat.offset(3 as libc::c_int as isize);
    } else {
        libc::printf(b"Unknown VBI data stream\n\0" as *const u8 as *const libc::c_char);
        return;
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < 36 as libc::c_int as libc::c_uint {
        let mut line: libc::c_uint = 0;
        let mut field: libc::c_uint = 0;
        let mut id2: libc::c_uint = 0;
        if !(linemask >> i & 0x1 as libc::c_int as libc::c_long == 0) {
            line = (if i < 18 as libc::c_int as libc::c_uint {
                i
            } else {
                i.wrapping_sub(18 as libc::c_int as libc::c_uint)
            })
            .wrapping_add(min_blank);
            field = (if i < 18 as libc::c_int as libc::c_uint {
                0 as libc::c_int
            } else {
                1 as libc::c_int
            }) as libc::c_uint;
            id2 = (*meat as libc::c_int & 0xf as libc::c_int) as libc::c_uint;
            match id2 {
                4 => {
                    if 21 as libc::c_int as libc::c_uint == line {
                        let data: libc::c_int = (*meat.offset(2 as libc::c_int as isize)
                            as libc::c_int)
                            << 8 as libc::c_int
                            | *meat.offset(1 as libc::c_int as isize) as libc::c_int;
                        if cc608_good_parity(cc608_parity_table.as_mut_ptr(), data as libc::c_uint)
                            != 0
                        {
                            if field == 0 as libc::c_int as libc::c_uint {
                                printdata(
                                    meat.offset(1 as libc::c_int as isize),
                                    2 as libc::c_int,
                                    0 as *const libc::c_uchar,
                                    0 as libc::c_int,
                                );
                                c1count = c1count.wrapping_add(1);
                            } else {
                                printdata(
                                    0 as *const libc::c_uchar,
                                    0 as libc::c_int,
                                    meat.offset(1 as libc::c_int as isize),
                                    2 as libc::c_int,
                                );
                                c2count = c2count.wrapping_add(1);
                            }
                        }
                    }
                }
                7 => {}
                5 => {}
                1 | _ => {}
            }
            meat = meat.offset(43 as libc::c_int as isize);
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn mpegps_read_packet() -> libc::c_int {
    let mut current_block: u64;
    let mut pts: LONG = 0;
    let mut dts: LONG = 0;
    let mut len: libc::c_int = 0;
    let mut startcode: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut codec_id: libc::c_int = 0 as libc::c_int;
    let mut es_type: libc::c_int = 0;
    loop {
        len = mpegps_read_pes_header(&mut startcode, &mut pts, &mut dts);
        if len < 0 as libc::c_int {
            return len;
        }
        es_type = psm_es_type[(startcode & 0xff as libc::c_int) as usize] as libc::c_int;
        if es_type > 0 as libc::c_int {
            if es_type == 0x1 as libc::c_int {
                codec_id = CODEC_ID_MPEG2VIDEO as libc::c_int;
                type_0 = CODEC_TYPE_VIDEO as libc::c_int;
                current_block = 5028470053297453708;
            } else if es_type == 0x2 as libc::c_int {
                codec_id = CODEC_ID_MPEG2VIDEO as libc::c_int;
                type_0 = CODEC_TYPE_VIDEO as libc::c_int;
                current_block = 5028470053297453708;
            } else if es_type == 0x3 as libc::c_int || es_type == 0x4 as libc::c_int {
                codec_id = CODEC_ID_MP3 as libc::c_int;
                type_0 = CODEC_TYPE_AUDIO as libc::c_int;
                current_block = 5028470053297453708;
            } else if es_type == 0xf as libc::c_int {
                codec_id = CODEC_ID_AAC as libc::c_int;
                type_0 = CODEC_TYPE_AUDIO as libc::c_int;
                current_block = 5028470053297453708;
            } else if es_type == 0x10 as libc::c_int {
                codec_id = CODEC_ID_MPEG4 as libc::c_int;
                type_0 = CODEC_TYPE_VIDEO as libc::c_int;
                current_block = 5028470053297453708;
            } else if es_type == 0x1b as libc::c_int {
                codec_id = CODEC_ID_H264 as libc::c_int;
                type_0 = CODEC_TYPE_VIDEO as libc::c_int;
                current_block = 5028470053297453708;
            } else if es_type == 0x81 as libc::c_int {
                codec_id = CODEC_ID_AC3 as libc::c_int;
                type_0 = CODEC_TYPE_AUDIO as libc::c_int;
                current_block = 5028470053297453708;
            } else {
                current_block = 16088375578540436547;
            }
        } else if startcode >= 0x1e0 as libc::c_int && startcode <= 0x1ef as libc::c_int {
            static mut avs_seqh: [libc::c_uchar; 4] = [
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                1 as libc::c_int as libc::c_uchar,
                0xb0 as libc::c_int as libc::c_uchar,
            ];
            let mut buf: [libc::c_uchar; 8] = [0; 8];
            if 8 as libc::c_int <= bytesinbuffer - filebuffer_pos {
                if !buf.as_mut_ptr().is_null() {
                    libc::memcpy(
                        buf.as_mut_ptr() as *mut libc::c_void,
                        filebuffer.offset(filebuffer_pos as isize) as *const libc::c_void,
                        8,
                    );
                }
                filebuffer_pos += 8 as libc::c_int;
                result = 8 as libc::c_int as LONG;
            } else {
                result = buffered_read_opt(buf.as_mut_ptr(), 8 as libc::c_int as libc::c_uint);
            }
            buffered_seek(-(8 as libc::c_int));
            if libc::memcmp(
                buf.as_mut_ptr() as *const libc::c_void,
                avs_seqh.as_ptr() as *const libc::c_void,
                4,
            ) == 0
                && (buf[6 as libc::c_int as usize] as libc::c_int != 0 as libc::c_int
                    || buf[7 as libc::c_int as usize] as libc::c_int != 1 as libc::c_int)
            {
                codec_id = CODEC_ID_CAVS as libc::c_int;
            } else {
                codec_id = CODEC_ID_MPEG2VIDEO as libc::c_int;
            }
            type_0 = CODEC_TYPE_VIDEO as libc::c_int;
            current_block = 5028470053297453708;
        } else if startcode >= 0x1c0 as libc::c_int && startcode <= 0x1df as libc::c_int {
            type_0 = CODEC_TYPE_AUDIO as libc::c_int;
            codec_id = CODEC_ID_MP2 as libc::c_int;
            current_block = 5028470053297453708;
        } else if startcode >= 0x80 as libc::c_int && startcode <= 0x87 as libc::c_int {
            type_0 = CODEC_TYPE_AUDIO as libc::c_int;
            codec_id = CODEC_ID_AC3 as libc::c_int;
            current_block = 5028470053297453708;
        } else if startcode >= 0x88 as libc::c_int && startcode <= 0x9f as libc::c_int {
            type_0 = CODEC_TYPE_AUDIO as libc::c_int;
            codec_id = CODEC_ID_DTS as libc::c_int;
            current_block = 5028470053297453708;
        } else if startcode >= 0xa0 as libc::c_int && startcode <= 0xbf as libc::c_int {
            type_0 = CODEC_TYPE_AUDIO as libc::c_int;
            codec_id = CODEC_ID_PCM_S16BE as libc::c_int;
            current_block = 5028470053297453708;
        } else if startcode >= 0x20 as libc::c_int && startcode <= 0x3f as libc::c_int {
            type_0 = CODEC_TYPE_SUBTITLE as libc::c_int;
            codec_id = CODEC_ID_DVD_SUBTITLE as libc::c_int;
            current_block = 5028470053297453708;
        } else if startcode == 0x69 as libc::c_int || startcode == 0x49 as libc::c_int {
            type_0 = CODEC_TYPE_DATA as libc::c_int;
            codec_id = CODEC_ID_MPEG2VBI as libc::c_int;
            current_block = 5028470053297453708;
        } else {
            current_block = 16088375578540436547;
        }
        match current_block {
            5028470053297453708 => {
                if !(startcode >= 0xa0 as libc::c_int && startcode <= 0xbf as libc::c_int) {
                    break;
                }
                if !(len <= 3 as libc::c_int) {
                    get_byte();
                    get_byte();
                    get_byte();
                    len -= 3 as libc::c_int;
                    break;
                }
            }
            _ => {}
        }
        url_fskip(len);
    }
    av.size = len;
    av.data = libc::realloc(av.data as *mut libc::c_void, av.size as usize) as *mut libc::c_uchar;
    if (av.data).is_null() {
        libc::printf(
            b"\rNot enough memory, libc::realloc() failed. Giving up.\n\0" as *const u8
                as *const libc::c_char,
        );
        ::std::process::exit(-(3 as libc::c_int));
    }
    av.codec_id = codec_id;
    av.type_0 = type_0;
    if av.size <= bytesinbuffer - filebuffer_pos {
        if !(av.data).is_null() {
            libc::memcpy(
                av.data as *mut libc::c_void,
                filebuffer.offset(filebuffer_pos as isize) as *const libc::c_void,
                av.size as usize,
            );
        }
        filebuffer_pos += av.size;
        result = av.size as LONG;
    } else {
        result = buffered_read_opt(av.data, av.size as libc::c_uint);
    }
    av.pts = pts;
    av.dts = dts;
    return 0 as libc::c_int;
}
unsafe extern "C" fn cc608_parity(byte: libc::c_uint) -> libc::c_int {
    let mut ones: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 7 as libc::c_int {
        if byte & ((1 as libc::c_int) << i) as libc::c_uint != 0 {
            ones += 1;
        }
        i += 1;
    }
    return ones & 1 as libc::c_int;
}
unsafe extern "C" fn cc608_build_parity_table(parity_table: *mut libc::c_int) {
    let mut byte: libc::c_uint = 0;
    let mut parity_v: libc::c_int = 0;
    byte = 0 as libc::c_int as libc::c_uint;
    while byte <= 127 as libc::c_int as libc::c_uint {
        parity_v = cc608_parity(byte);
        *parity_table.offset(byte as isize) = parity_v;
        *parity_table.offset((byte | 0x80 as libc::c_int as libc::c_uint) as isize) =
            (parity_v == 0) as libc::c_int;
        byte = byte.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn build_parity_table() {
    cc608_build_parity_table(cc608_parity_table.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn myth_loop() {
    let mut rc: libc::c_int = 0;
    let mut has_vbi: libc::c_int = 0 as libc::c_int;
    let mut desp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut saved: LONG = 0;
    av.data = 0 as *mut libc::c_uchar;
    buffer_input = 1 as libc::c_int;
    if init_file_buffer() != 0 {
        libc::printf(b"Not enough memory.\n\0" as *const u8 as *const libc::c_char);
        ::std::process::exit(-(3 as libc::c_int));
    }
    desp = libc::malloc(65536) as *mut libc::c_uchar;
    saved = 0 as libc::c_int as LONG;
    while processed_enough == 0 && {
        rc = mpegps_read_packet();
        rc == 0 as libc::c_int
    } {
        if av.codec_id == CODEC_ID_MPEG2VBI as libc::c_int
            && av.type_0 == CODEC_TYPE_DATA as libc::c_int
        {
            if has_vbi == 0 {
                libc::printf(
                    b"\rDetected VBI data, disabling user-data packet analysis (not needed).\n\0"
                        as *const u8 as *const libc::c_char,
                );
                has_vbi = 1 as libc::c_int;
            }
            ProcessVBIDataPacket();
        }
        if av.codec_id == CODEC_ID_MPEG2VIDEO as libc::c_int
            && av.type_0 == CODEC_TYPE_VIDEO as libc::c_int
            && has_vbi == 0
        {
            let length: LONG = saved + av.size as libc::c_long;
            let mut used: LONG = 0;
            if length > 65536 as libc::c_int as libc::c_long {
                libc::printf(
                    b"Not enough memory. Please report this: 65536 bytes is not enough!\0"
                        as *const u8 as *const libc::c_char,
                );
                ::std::process::exit(-(500 as libc::c_int));
            }
            if av.pts as libc::c_longlong != 0x8000000000000000 as libc::c_ulong as int64_t {
                current_pts = av.pts;
                if pts_set == 0 as libc::c_int {
                    pts_set = 1 as libc::c_int;
                }
            }
            libc::memcpy(
                desp.offset(saved as isize) as *mut libc::c_void,
                av.data as *const libc::c_void,
                av.size as usize,
            );
            used = process_block(desp, length);
            libc::memmove(
                desp as *mut libc::c_void,
                desp.offset(used as isize) as *const libc::c_void,
                (length - used) as usize,
            );
            saved = length - used;
        }
        if inputsize > 0 as libc::c_int as libc::c_long {
            let mut cur_sec: libc::c_int = 0;
            let at: LONG = libc::lseek(in_0, 0 as libc::c_int as off_t, 1 as libc::c_int) as LONG;
            let progress: libc::c_int =
                ((at >> 8 as libc::c_int) * 100 as libc::c_int as libc::c_long
                    / (inputsize >> 8 as libc::c_int)) as libc::c_int;
            if last_reported_progress != progress {
                libc::printf(b"\r%3d%%\0" as *const u8 as *const libc::c_char, progress);
                cur_sec = (c1count.wrapping_add(c1count_total) as libc::c_double / 29.97f64)
                    as libc::c_int;
                libc::printf(
                    b"  |  %02d:%02d\0" as *const u8 as *const libc::c_char,
                    cur_sec / 60 as libc::c_int,
                    cur_sec % 60 as libc::c_int,
                );
                last_reported_progress = progress;
            }
        }
    }
    libc::free(av.data as *mut libc::c_void);
}
