extern "C" {
    static mut gop_time: gop_time_code;
    static mut first_gop_time: gop_time_code;
    static mut gop_rollover: libc::c_int;
    static mut min_pts: LONG;
    static mut max_pts: LONG;
    static mut last_pts: LONG;
    static mut ts_mode: libc::c_int;
    static mut fbuffer: *mut libc::c_uchar;
    static mut past: LONG;
    static mut startbytes_pos: libc::c_uint;
    static mut pesheaderbuf: *mut libc::c_uchar;
    static mut pts_set: libc::c_int;
    static mut c1count: libc::c_uint;
    static mut c2count: libc::c_uint;
    static mut c1count_total: libc::c_uint;
    static mut c2count_total: libc::c_uint;
    static mut pts_big_change: libc::c_uint;
    static mut ptsdata: [libc::c_uchar; 5];
    static mut lastptsdata: [libc::c_uchar; 5];
    static mut clean: *mut libc::FILE;
    #[link_name = "in"]
    static mut in_0: libc::c_int;
    static mut stat_dvdccheaders: libc::c_int;
    static mut stat_replay5000headers: libc::c_int;
    static mut stat_replay4000headers: libc::c_int;
    static mut stat_dishheaders: libc::c_int;
    static mut stat_hdtv: libc::c_int;
    static mut autopad: libc::c_int;
    static mut gop_pad: libc::c_int;
    static mut ff_cleanup: libc::c_int;
    static mut debug: libc::c_int;
    static mut fix_padding: libc::c_int;
    static mut cc_stats: [libc::c_int; 4];
    static mut inputsize: LONG;
    static mut processed_enough: libc::c_int;
    fn dump(start: *mut libc::c_uchar, l: libc::c_int);
    fn printdata(
        data1: *const libc::c_uchar,
        length1: libc::c_int,
        data2: *const libc::c_uchar,
        length2: libc::c_int,
    );
    fn too_many_blocks() -> libc::c_int;
    static mut last_reported_progress: libc::c_int;
    static mut buffer_input: libc::c_int;
    static mut frames_since_last_gop: libc::c_int;
    static mut cc608_parity_table: [libc::c_int; 256];
}
pub type __int64_t = libc::c_longlong;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_ssize_t = libc::c_long;
pub type __darwin_off_t = __int64_t;
pub type size_t = __darwin_size_t;
pub type int64_t = libc::c_longlong;
pub type off_t = __darwin_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut libc::c_uchar,
    pub _size: libc::c_int,
}
pub type LONG = libc::c_long;
pub type frame_type = libc::c_uint;
pub const B_FRAME: frame_type = 3;
pub const P_FRAME: frame_type = 2;
pub const RESET_OR_UNKNOWN: frame_type = 0;
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
#[no_mangle]
pub static mut current_hor_size: libc::c_uint = 0 as libc::c_int as libc::c_uint;
#[no_mangle]
pub static mut current_vert_size: libc::c_uint = 0 as libc::c_int as libc::c_uint;
#[no_mangle]
pub static mut current_aspect_ratio: libc::c_uint = 0 as libc::c_int as libc::c_uint;
#[no_mangle]
pub static mut current_frame_rate: libc::c_uint = 0 as libc::c_int as libc::c_uint;
#[no_mangle]
pub static mut current_bit_rate: libc::c_uint = 0 as libc::c_int as libc::c_uint;
#[no_mangle]
pub static mut current_pts: LONG = 0 as libc::c_int as LONG;
#[no_mangle]
pub static mut ts_headers_total: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut result: LONG = 0;
#[no_mangle]
pub static mut net_fields: LONG = 20 as libc::c_int as LONG;
#[no_mangle]
pub static mut DO_NOTHING: [libc::c_uchar; 2] = [
    0x80 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
];
#[no_mangle]
pub static mut full_pes: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut inbuf: LONG = 0 as libc::c_int as LONG;
#[no_mangle]
pub static mut tsheader: [libc::c_uchar; 4] = [0; 4];
#[no_mangle]
pub static mut next_ts_header_read: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut end_of_file: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut current_picture_coding_type: libc::c_int = RESET_OR_UNKNOWN as libc::c_int;
#[no_mangle]
pub static mut last_picture_coding_type: libc::c_int = RESET_OR_UNKNOWN as libc::c_int;
#[no_mangle]
pub static mut twoback_picture_coding_type: libc::c_int = RESET_OR_UNKNOWN as libc::c_int;
#[no_mangle]
pub static mut p_caption_size: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut p_caption_capacity: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut p_caption: *mut libc::c_uchar = 0 as *const libc::c_uchar as *mut libc::c_uchar;
#[no_mangle]
pub static mut non_compliant_DVD: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn calculate_ccblock_gop_time(mut g: *mut gop_time_code) {
    let seconds: libc::c_int = (*g).time_code_hours * 3600 as libc::c_int
        + (*g).time_code_minutes * 60 as libc::c_int
        + (*g).time_code_seconds;
    (*g).ccblocks =
        ((seconds as libc::c_double * 29.97f64) as libc::c_int + (*g).time_code_pictures) as LONG;
    if gop_rollover != 0 {
        let ref mut fresh0 = (*g).ccblocks;
        *fresh0 +=
            (86400 as libc::c_int as libc::c_double * 29.97f64) as libc::c_int as libc::c_long;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gop_accepted(g: *mut gop_time_code) -> libc::c_int {
    if !((*g).time_code_hours <= 23 as libc::c_int
        && (*g).time_code_minutes <= 59 as libc::c_int
        && (*g).time_code_seconds <= 59 as libc::c_int
        && (*g).time_code_pictures <= 59 as libc::c_int)
    {
        return 0 as libc::c_int;
    }
    if gop_time.time_code_hours == 23 as libc::c_int
        && gop_time.time_code_minutes == 59 as libc::c_int
        && (*g).time_code_hours == 0 as libc::c_int
        && (*g).time_code_minutes == 0 as libc::c_int
    {
        gop_rollover = 1 as libc::c_int;
        return 1 as libc::c_int;
    }
    if gop_time.inited != 0 {
        if gop_time.ccblocks > (*g).ccblocks {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn set_cc_pts(pts: int64_t) {
    current_pts = pts as LONG;
    if pts_set == 0 as libc::c_int {
        pts_set = 1 as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn update_clock() {
    if pts_set != 0 as libc::c_int {
        let mut dif: libc::c_int = 0;
        pts_set = 2 as libc::c_int;
        if current_pts < min_pts {
            min_pts = current_pts;
        }
        if current_pts > max_pts {
            max_pts = current_pts;
        }
        dif = (current_pts - last_pts) as libc::c_int;
        dif = dif / 90000 as libc::c_int;
        if dif < 0 as libc::c_int {
            libc::printf(
                b"\nThe clock is going backwards -  %ld seconds)\n\0" as *const u8
                    as *const libc::c_char,
                (last_pts - current_pts) / 90000 as libc::c_int as libc::c_long,
            );
        }
        if dif < 0 as libc::c_int || dif >= 5 as libc::c_int {
            libc::printf(
                b"\nWarning: Reference clock has changed abruptly (%d seconds), attempting to synchronize\n\0"
                    as *const u8 as *const libc::c_char,
                dif,
            );
            libc::printf(
                b"Last PTS value: %lu\n\0" as *const u8 as *const libc::c_char,
                last_pts,
            );
            dump(lastptsdata.as_mut_ptr(), 5 as libc::c_int);
            libc::printf(
                b"Current PTS value: %lu\n\0" as *const u8 as *const libc::c_char,
                current_pts,
            );
            dump(ptsdata.as_mut_ptr(), 5 as libc::c_int);
            c1count_total = c1count_total.wrapping_add(c1count);
            c2count_total = c2count_total.wrapping_add(c2count);
            c1count = 0 as libc::c_int as libc::c_uint;
            c2count = 0 as libc::c_int as libc::c_uint;
            pts_set = 1 as libc::c_int;
            min_pts = 0xffffffff as libc::c_uint as LONG;
            max_pts = 0 as libc::c_int as LONG;
            pts_big_change = 1 as libc::c_int as libc::c_uint;
        }
        last_pts = current_pts;
        libc::memcpy(
            lastptsdata.as_mut_ptr() as *mut libc::c_void,
            ptsdata.as_mut_ptr() as *const libc::c_void,
            5,
        );
    }
}
#[no_mangle]
pub static mut filebuffer: *mut libc::c_uchar = 0 as *const libc::c_uchar as *mut libc::c_uchar;
#[no_mangle]
pub static mut filebuffer_start: LONG = 0;
#[no_mangle]
pub static mut filebuffer_pos: libc::c_int = 0;
#[no_mangle]
pub static mut bytesinbuffer: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn do_padding(mis1: libc::c_int, mis2: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < mis1 {
        printdata(
            DO_NOTHING.as_ptr(),
            2 as libc::c_int,
            0 as *const libc::c_uchar,
            0 as libc::c_int,
        );
        c1count = c1count.wrapping_add(1);
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < mis2 {
        printdata(
            0 as *const libc::c_uchar,
            0 as libc::c_int,
            DO_NOTHING.as_ptr(),
            2 as libc::c_int,
        );
        c2count = c2count.wrapping_add(1);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gop_padding() {
    if first_gop_time.inited != 0 {
        let mis1: libc::c_int = (gop_time.ccblocks + frames_since_last_gop as libc::c_long
            - first_gop_time.ccblocks
            - c1count as libc::c_long) as libc::c_int;
        let mis2: libc::c_int = (gop_time.ccblocks + frames_since_last_gop as libc::c_long
            - first_gop_time.ccblocks
            - c2count as libc::c_long) as libc::c_int;
        do_padding(mis1, mis2);
    }
}
#[no_mangle]
pub unsafe extern "C" fn pts_padding() {
    let exp: libc::c_int = ((current_pts - min_pts) as libc::c_double * 29.97f64
        / 90000 as libc::c_int as libc::c_double) as libc::c_int;
    let mis1: libc::c_int = (exp as libc::c_uint).wrapping_sub(c1count) as libc::c_int;
    let mis2: libc::c_int = (exp as libc::c_uint).wrapping_sub(c2count) as libc::c_int;
    do_padding(mis1, mis2);
}
#[no_mangle]
pub unsafe extern "C" fn init_file_buffer() -> libc::c_int {
    filebuffer_start = 0 as libc::c_int as LONG;
    filebuffer_pos = 0 as libc::c_int;
    bytesinbuffer = 0 as libc::c_int;
    if filebuffer.is_null() {
        filebuffer = libc::malloc((1024 * 1024 * 16) as usize) as *mut libc::c_uchar;
    }
    if filebuffer.is_null() {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn buffered_seek(offset: libc::c_int) {
    if offset < 0 as libc::c_int {
        filebuffer_pos += offset;
        if filebuffer_pos < 0 as libc::c_int {
            startbytes_pos = startbytes_pos.wrapping_add(filebuffer_pos as libc::c_uint);
            filebuffer_pos = 0 as libc::c_int;
            if startbytes_pos <= 0 as libc::c_int as libc::c_uint {
                libc::printf(
                    b"PANIC: Attempt to seek before buffer start, this is a bug!\0" as *const u8
                        as *const libc::c_char,
                );
                ::std::process::exit(-(4 as libc::c_int));
            }
        }
    } else {
        buffered_read_opt(0 as *mut libc::c_uchar, offset as libc::c_uint);
    };
}
#[no_mangle]
pub unsafe extern "C" fn buffered_read_opt(
    mut buffer: *mut libc::c_uchar,
    mut bytes: libc::c_uint,
) -> LONG {
    let mut copied: LONG = 0 as libc::c_int as LONG;
    let mut keep: libc::c_int = 0;
    let mut copy: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if buffer_input != 0 || filebuffer_pos < bytesinbuffer {
        let mut eof: libc::c_int = 0 as libc::c_int;
        while eof == 0 && bytes != 0 {
            let mut ready: size_t = (bytesinbuffer - filebuffer_pos) as size_t;
            if ready == 0 as libc::c_int as libc::c_ulong {
                if buffer_input == 0 {
                    let mut i_0: libc::c_int = 0;
                    loop {
                        i_0 = libc::read(in_0, fbuffer as *mut libc::c_void, bytes as usize)
                            as libc::c_int;
                        copied += i_0 as libc::c_long;
                        bytes = bytes.wrapping_sub(i_0 as libc::c_uint);
                        fbuffer = fbuffer.offset(i_0 as isize);
                        if !(i_0 != 0 && bytes != 0) {
                            break;
                        }
                    }
                    return copied;
                }
                keep = if bytesinbuffer > 8 as libc::c_int {
                    8 as libc::c_int
                } else {
                    bytesinbuffer
                };
                libc::memmove(
                    filebuffer as *mut libc::c_void,
                    filebuffer.offset(
                        (1024 as libc::c_int * 1024 as libc::c_int * 16 as libc::c_int - keep)
                            as isize,
                    ) as *const libc::c_void,
                    keep as usize,
                );
                i = libc::read(
                    in_0,
                    filebuffer.offset(keep as isize) as *mut libc::c_void,
                    (1024 * 1024 * 16 - keep) as usize,
                ) as libc::c_int;
                if i == 0 as libc::c_int {
                    eof = 1 as libc::c_int;
                }
                filebuffer_pos = keep;
                bytesinbuffer = i + keep;
                ready = i as size_t;
            }
            copy = (if ready >= bytes as libc::c_ulong {
                bytes as libc::c_ulong
            } else {
                ready
            }) as libc::c_int;
            if !fbuffer.is_null() {
                libc::memcpy(
                    fbuffer as *mut libc::c_void,
                    filebuffer.offset(filebuffer_pos as isize) as *const libc::c_void,
                    copy as usize,
                );
                buffer = buffer.offset(copy as isize);
            }
            filebuffer_pos += copy;
            bytes = bytes.wrapping_sub(copy as libc::c_uint);
            copied += copy as libc::c_long;
        }
        return copied;
    } else {
        if !fbuffer.is_null() {
            return copied + libc::read(in_0, fbuffer as *mut libc::c_void, bytes as usize) as i64;
        }
        return (copied as libc::c_longlong + libc::lseek(in_0, bytes as off_t, 1 as libc::c_int))
            as LONG;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_getmoredata() -> libc::c_long {
    let mut paystart: libc::c_int = 0 as libc::c_int;
    let mut pes_start_in_this_pass: libc::c_int = 0 as libc::c_int;
    let mut enough: libc::c_int = 0 as libc::c_int;
    let mut got_pes_header: libc::c_int = 0 as libc::c_int;
    let mut payload_read: libc::c_int = 0 as libc::c_int;
    let mut dump_tspacket: libc::c_int = 0 as libc::c_int;
    let mut payload_start: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut payload_length: libc::c_uint = 0;
    let mut ts_adaptation: libc::c_int = 0 as libc::c_int;
    let mut error: libc::c_uint = 0;
    let mut pid: libc::c_int = 0;
    let mut adapt: libc::c_uint = 0;
    let mut want: libc::c_int = 0;
    full_pes = 0 as libc::c_int;
    let mut current_block_143: u64;
    loop {
        if (256 as libc::c_int * 1024 as libc::c_int + 120 as libc::c_int) as libc::c_long - inbuf
            < 184 as libc::c_int as libc::c_long
        {
            enough = 1 as libc::c_int;
        } else {
            if next_ts_header_read != 0 {
                result = 4 as libc::c_int as LONG;
                next_ts_header_read = 0 as libc::c_int;
            } else {
                if 4 as libc::c_int <= bytesinbuffer - filebuffer_pos {
                    if !tsheader.as_mut_ptr().is_null() {
                        tsheader[0 as libc::c_int as usize] =
                            *filebuffer.offset(filebuffer_pos as isize);
                        tsheader[1 as libc::c_int as usize] =
                            *filebuffer.offset((filebuffer_pos + 1 as libc::c_int) as isize);
                        tsheader[2 as libc::c_int as usize] =
                            *filebuffer.offset((filebuffer_pos + 2 as libc::c_int) as isize);
                        tsheader[3 as libc::c_int as usize] =
                            *filebuffer.offset((filebuffer_pos + 3 as libc::c_int) as isize);
                        filebuffer_pos += 4 as libc::c_int;
                        result = 4 as libc::c_int as LONG;
                    }
                } else {
                    result =
                        buffered_read_opt(tsheader.as_mut_ptr(), 4 as libc::c_int as libc::c_uint);
                }
                if result != 4 as libc::c_int as libc::c_long {
                    end_of_file = 1 as libc::c_int;
                    break;
                } else {
                    past += result;
                    ts_headers_total += 1;
                }
            }
            if tsheader[0 as libc::c_int as usize] as libc::c_int != 0x47 as libc::c_int {
                libc::printf(
                    b"\nProblem: No TS header mark. Received bytes:\n\0" as *const u8
                        as *const libc::c_char,
                );
                dump(tsheader.as_mut_ptr(), 4 as libc::c_int);
                libc::printf(b"Trying to continue anyway.\n\0" as *const u8 as *const libc::c_char);
            }
            error = ((tsheader[1 as libc::c_int as usize] as libc::c_int & 0x80 as libc::c_int)
                >> 7 as libc::c_int) as libc::c_uint;
            payload_start = ((tsheader[1 as libc::c_int as usize] as libc::c_int
                & 0x40 as libc::c_int)
                >> 6 as libc::c_int) as libc::c_uint;
            pid = ((tsheader[1 as libc::c_int as usize] as libc::c_int & 0x1f as libc::c_int)
                << 8 as libc::c_int
                | tsheader[2 as libc::c_int as usize] as libc::c_int)
                & 0x1fff as libc::c_int;
            if pid < 0x10 as libc::c_int || pid >= 0x1fff as libc::c_int {
                if 184 as libc::c_int <= bytesinbuffer - filebuffer_pos {
                    filebuffer_pos += 184 as libc::c_int;
                    result = 184 as libc::c_int as LONG;
                } else {
                    result = buffered_read_opt(
                        0 as *mut libc::c_uchar,
                        184 as libc::c_int as libc::c_uint,
                    );
                }
            } else {
                adapt = ((tsheader[3 as libc::c_int as usize] as libc::c_int & 0x30 as libc::c_int)
                    >> 4 as libc::c_int) as libc::c_uint;
                if error != 0 {
                    libc::printf(
                        b"Warning: Defective TS packet: %u\n\0" as *const u8 as *const libc::c_char,
                        error,
                    );
                    dump_tspacket = 1 as libc::c_int;
                }
                ts_adaptation = (adapt & 2 as libc::c_int as libc::c_uint) as libc::c_int;
                payload_length = 184 as libc::c_int as libc::c_uint;
                if payload_start != 0 {
                    paystart += 1;
                    if got_pes_header != 0 {
                        next_ts_header_read = 1 as libc::c_int;
                        full_pes = 1 as libc::c_int;
                        break;
                    } else {
                        let mut stream_id: libc::c_uint = 0;
                        if ts_adaptation != 0 {
                            let mut adlength: libc::c_uchar = 0;
                            if 1 as libc::c_int <= bytesinbuffer - filebuffer_pos {
                                if !(&mut adlength as *mut libc::c_uchar).is_null() {
                                    libc::memcpy(
                                        &mut adlength as *mut libc::c_uchar as *mut libc::c_void,
                                        filebuffer.offset(filebuffer_pos as isize)
                                            as *const libc::c_void,
                                        1,
                                    );
                                }
                                filebuffer_pos += 1 as libc::c_int;
                                result = 1 as libc::c_int as LONG;
                            } else {
                                result = buffered_read_opt(
                                    &mut adlength,
                                    1 as libc::c_int as libc::c_uint,
                                );
                            }
                            past = past + result;
                            payload_length = payload_length
                                .wrapping_sub(adlength as libc::c_uint)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint);
                            if adlength as libc::c_int <= bytesinbuffer - filebuffer_pos {
                                filebuffer_pos += adlength as libc::c_int;
                                result = adlength as LONG;
                            } else {
                                result = buffered_read_opt(
                                    0 as *mut libc::c_uchar,
                                    adlength as libc::c_uint,
                                );
                            }
                            past = past + adlength as libc::c_long;
                        }
                        pes_start_in_this_pass = 1 as libc::c_int;
                        if 6 as libc::c_int <= bytesinbuffer - filebuffer_pos {
                            if !pesheaderbuf.is_null() {
                                libc::memcpy(
                                    pesheaderbuf as *mut libc::c_void,
                                    filebuffer.offset(filebuffer_pos as isize)
                                        as *const libc::c_void,
                                    6,
                                );
                            }
                            filebuffer_pos += 6 as libc::c_int;
                            result = 6 as libc::c_int as LONG;
                        } else {
                            result =
                                buffered_read_opt(pesheaderbuf, 6 as libc::c_int as libc::c_uint);
                        }
                        past = past + result;
                        payload_length =
                            payload_length.wrapping_sub(6 as libc::c_int as libc::c_uint);
                        if *pesheaderbuf.offset(0 as libc::c_int as isize) as libc::c_int
                            != 0 as libc::c_int
                            || *pesheaderbuf.offset(1 as libc::c_int as isize) as libc::c_int
                                != 0 as libc::c_int
                            || *pesheaderbuf.offset(2 as libc::c_int as isize) as libc::c_int
                                != 0x1 as libc::c_int
                        {
                            if payload_length as libc::c_int <= bytesinbuffer - filebuffer_pos {
                                filebuffer_pos += payload_length as libc::c_int;
                                result = payload_length as libc::c_int as LONG;
                            } else {
                                result = buffered_read_opt(
                                    0 as *mut libc::c_uchar,
                                    payload_length as libc::c_int as libc::c_uint,
                                );
                            }
                            current_block_143 = 5720623009719927633;
                        } else {
                            got_pes_header = 1 as libc::c_int;
                            stream_id =
                                *pesheaderbuf.offset(3 as libc::c_int as isize) as libc::c_uint;
                            if stream_id != 0xbe as libc::c_int as libc::c_uint
                                && stream_id != 0xbf as libc::c_int as libc::c_uint
                            {
                                let mut PTS_present: libc::c_uint = 0;
                                let mut pes_header_length: libc::c_uint = 0;
                                let mut need_to_skip: libc::c_int = 0;
                                if 3 as libc::c_int <= bytesinbuffer - filebuffer_pos {
                                    if !pesheaderbuf.offset(6 as libc::c_int as isize).is_null() {
                                        libc::memcpy(
                                            pesheaderbuf.offset(6 as libc::c_int as isize)
                                                as *mut libc::c_void,
                                            filebuffer.offset(filebuffer_pos as isize)
                                                as *const libc::c_void,
                                            3,
                                        );
                                    }
                                    filebuffer_pos += 3 as libc::c_int;
                                    result = 3 as libc::c_int as LONG;
                                } else {
                                    result = buffered_read_opt(
                                        pesheaderbuf.offset(6 as libc::c_int as isize),
                                        3 as libc::c_int as libc::c_uint,
                                    );
                                }
                                past = past + result;
                                payload_length =
                                    payload_length.wrapping_sub(3 as libc::c_int as libc::c_uint);
                                PTS_present = ((*pesheaderbuf.offset(7 as libc::c_int as isize)
                                    as libc::c_int
                                    & 0x80 as libc::c_int)
                                    >> 7 as libc::c_int)
                                    as libc::c_uint;
                                pes_header_length =
                                    *pesheaderbuf.offset(8 as libc::c_int as isize) as libc::c_uint;
                                need_to_skip = pes_header_length as libc::c_int;
                                if PTS_present != 0 {
                                    let mut pts_raw: [libc::c_uchar; 5] = [0; 5];
                                    if 5 as libc::c_int <= bytesinbuffer - filebuffer_pos {
                                        if !pts_raw.as_mut_ptr().is_null() {
                                            libc::memcpy(
                                                pts_raw.as_mut_ptr() as *mut libc::c_void,
                                                filebuffer.offset(filebuffer_pos as isize)
                                                    as *const libc::c_void,
                                                5,
                                            );
                                        }
                                        filebuffer_pos += 5 as libc::c_int;
                                        result = 5 as libc::c_int as LONG;
                                    } else {
                                        result = buffered_read_opt(
                                            pts_raw.as_mut_ptr(),
                                            5 as libc::c_int as libc::c_uint,
                                        );
                                    }
                                    past = past + result;
                                    payload_length = payload_length
                                        .wrapping_sub(5 as libc::c_int as libc::c_uint);
                                    need_to_skip = need_to_skip - 5 as libc::c_int;
                                    if pts_raw[0 as libc::c_int as usize] as libc::c_int
                                        & 1 as libc::c_int
                                        != 0
                                        && pts_raw[2 as libc::c_int as usize] as libc::c_int
                                            & 1 as libc::c_int
                                            != 0
                                        && pts_raw[4 as libc::c_int as usize] as libc::c_int
                                            & 1 as libc::c_int
                                            != 0
                                    {
                                        let bits_9: libc::c_uint =
                                            ((pts_raw[0 as libc::c_int as usize] as libc::c_int
                                                & 0xe as libc::c_int)
                                                << 28 as libc::c_int)
                                                as libc::c_uint;
                                        let bits_10: libc::c_uint =
                                            ((pts_raw[1 as libc::c_int as usize] as libc::c_int)
                                                << 22 as libc::c_int)
                                                as libc::c_uint;
                                        let bits_11: libc::c_uint =
                                            ((pts_raw[2 as libc::c_int as usize] as libc::c_int
                                                & 0xfe as libc::c_int)
                                                << 14 as libc::c_int)
                                                as libc::c_uint;
                                        let bits_12: libc::c_uint =
                                            ((pts_raw[3 as libc::c_int as usize] as libc::c_int)
                                                << 7 as libc::c_int)
                                                as libc::c_uint;
                                        let bits_13: libc::c_uint =
                                            (pts_raw[4 as libc::c_int as usize] as libc::c_int
                                                >> 1 as libc::c_int)
                                                as libc::c_uint;
                                        current_pts =
                                            (bits_9 | bits_10 | bits_11 | bits_12 | bits_13)
                                                as LONG;
                                        if pts_set == 0 as libc::c_int {
                                            pts_set = 1 as libc::c_int;
                                        }
                                    }
                                    libc::memcpy(
                                        ptsdata.as_mut_ptr() as *mut libc::c_void,
                                        pts_raw.as_mut_ptr() as *const libc::c_void,
                                        5,
                                    );
                                }
                                if need_to_skip < 0 as libc::c_int {
                                    libc::printf(
                                        b"Something's wrong here.\n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                }
                                if need_to_skip <= bytesinbuffer - filebuffer_pos {
                                    filebuffer_pos += need_to_skip;
                                    result = need_to_skip as LONG;
                                } else {
                                    result = buffered_read_opt(
                                        0 as *mut libc::c_uchar,
                                        need_to_skip as libc::c_uint,
                                    );
                                }
                                past = past + need_to_skip as libc::c_long;
                                payload_length =
                                    payload_length.wrapping_sub(need_to_skip as libc::c_uint);
                            }
                            current_block_143 = 7761909515536616543;
                        }
                    }
                } else {
                    current_block_143 = 7761909515536616543;
                }
                match current_block_143 {
                    5720623009719927633 => {}
                    _ => {
                        want = (if (256 as libc::c_int * 1024 as libc::c_int + 120 as libc::c_int)
                            as libc::c_long
                            - inbuf
                            > payload_length as libc::c_long
                        {
                            payload_length as libc::c_long
                        } else {
                            (256 as libc::c_int * 1024 as libc::c_int + 120 as libc::c_int)
                                as libc::c_long
                                - inbuf
                        }) as libc::c_int;
                        if want <= bytesinbuffer - filebuffer_pos {
                            if !fbuffer.offset(inbuf as isize).is_null() {
                                libc::memcpy(
                                    fbuffer.offset(inbuf as isize) as *mut libc::c_void,
                                    filebuffer.offset(filebuffer_pos as isize)
                                        as *const libc::c_void,
                                    want as usize,
                                );
                            }
                            filebuffer_pos += want;
                            result = want as LONG;
                        } else {
                            result = buffered_read_opt(
                                fbuffer.offset(inbuf as isize),
                                want as libc::c_uint,
                            );
                        }
                        if result > 0 as libc::c_int as libc::c_long {
                            payload_read += result as libc::c_int;
                        }
                        past = past + result;
                        if dump_tspacket != 0 {
                            libc::printf(b"Payload dump:\n\0" as *const u8 as *const libc::c_char);
                            dump_tspacket = 0 as libc::c_int;
                            dump(fbuffer.offset(inbuf as isize), 184 as libc::c_int);
                        }
                        inbuf += result;
                    }
                }
            }
        }
        if !(result != 0 as libc::c_int as libc::c_long
            && enough == 0
            && (256 as libc::c_int * 1024 as libc::c_int + 120 as libc::c_int) as libc::c_long
                != inbuf)
        {
            break;
        }
    }
    if (pes_start_in_this_pass == 0 as libc::c_int || full_pes == 0 as libc::c_int) && result != 0 {
        libc::printf(
            b"Warning: We don't have the complete PES in buffer.\n\0" as *const u8
                as *const libc::c_char,
        );
        libc::printf(
            b"Things may start to go wrong from this point.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    return payload_read as libc::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn general_getmoredata() -> LONG {
    loop {
        let want: libc::c_int = ((256 as libc::c_int * 1024 as libc::c_int + 120 as libc::c_int)
            as libc::c_long
            - inbuf) as libc::c_int;
        if want <= bytesinbuffer - filebuffer_pos {
            if !fbuffer.offset(inbuf as isize).is_null() {
                libc::memcpy(
                    fbuffer.offset(inbuf as isize) as *mut libc::c_void,
                    filebuffer.offset(filebuffer_pos as isize) as *const libc::c_void,
                    want as usize,
                );
            }
            filebuffer_pos += want;
            result = want as LONG;
        } else {
            result = buffered_read_opt(fbuffer.offset(inbuf as isize), want as libc::c_uint);
        }
        past = past + result;
        inbuf += result;
        if !(result != 0 as libc::c_int as libc::c_long
            && (256 as libc::c_int * 1024 as libc::c_int + 120 as libc::c_int) as libc::c_long
                != inbuf)
        {
            break;
        }
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn raw_loop() {
    let mut i: libc::c_long = 0;
    loop {
        inbuf = 0 as libc::c_int as LONG;
        general_getmoredata();
        i = 0 as libc::c_int as libc::c_long;
        while i < inbuf {
            if !(c1count < 2 as libc::c_int as libc::c_uint
                && *fbuffer.offset(i as isize) as libc::c_int == 0xff as libc::c_int
                && *fbuffer.offset(i as isize).offset(1 as libc::c_int as isize) as libc::c_int
                    == 0xff as libc::c_int)
            {
                printdata(
                    fbuffer.offset(i as isize),
                    2 as libc::c_int,
                    0 as *const libc::c_uchar,
                    0 as libc::c_int,
                );
                c1count = c1count.wrapping_add(1);
            }
            i = i + 2 as libc::c_int as libc::c_long;
        }
        if !(inbuf != 0) {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn process_block(data: *mut libc::c_uchar, length: LONG) -> LONG {
    let mut limit: libc::c_int = 0;
    let mut printed: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut header: *mut libc::c_uchar = data;
    let endofbuffer: *mut libc::c_uchar = data.offset(length as isize);
    if length < 4 as libc::c_int as libc::c_long {
        return length;
    }
    while !(header.offset(4 as libc::c_int as isize) > endofbuffer) {
        if *header.offset(0 as libc::c_int as isize) as libc::c_int == 0x43 as libc::c_int
            && *header.offset(1 as libc::c_int as isize) as libc::c_int == 0x43 as libc::c_int
        {
            let mut pattern: libc::c_uchar = 0;
            let mut field1packet: libc::c_int = 0;
            let mut i: libc::c_int = 0;
            let mut j_0: libc::c_int = 0;
            let mut capcount: libc::c_int = 0;
            if non_compliant_DVD != 0
                && current_picture_coding_type == B_FRAME as libc::c_int
                && autopad != 0
            {
                gop_padding();
            }
            stat_dvdccheaders += 1;
            header = header.offset(4 as libc::c_int as isize);
            pattern = (*header.offset(0 as libc::c_int as isize) as libc::c_int
                & 0x80 as libc::c_int) as libc::c_uchar;
            field1packet = 0 as libc::c_int;
            if pattern as libc::c_int == 0 as libc::c_int {
                field1packet = 1 as libc::c_int;
            }
            capcount = (*header.offset(0 as libc::c_int as isize) as libc::c_int
                & 0x1e as libc::c_int)
                / 2 as libc::c_int;
            header = header.offset(1);
            i = 0 as libc::c_int;
            while i < capcount {
                let mut data1: [libc::c_uchar; 2] = [
                    0x80 as libc::c_int as libc::c_uchar,
                    0x80 as libc::c_int as libc::c_uchar,
                ];
                let mut data2: [libc::c_uchar; 2] = [
                    0x80 as libc::c_int as libc::c_uchar,
                    0x80 as libc::c_int as libc::c_uchar,
                ];
                j_0 = 0 as libc::c_int;
                while j_0 < 2 as libc::c_int {
                    let mut data_0: [libc::c_uchar; 3] = [0; 3];
                    data_0[0 as libc::c_int as usize] = *header.offset(0 as libc::c_int as isize);
                    data_0[1 as libc::c_int as usize] = *header.offset(1 as libc::c_int as isize);
                    data_0[2 as libc::c_int as usize] = *header.offset(2 as libc::c_int as isize);
                    header = header.offset(3 as libc::c_int as isize);
                    if data_0[0 as libc::c_int as usize] as libc::c_int == 0xff as libc::c_int
                        && j_0 == field1packet
                    {
                        data1[0 as libc::c_int as usize] = data_0[1 as libc::c_int as usize];
                        data1[1 as libc::c_int as usize] = data_0[2 as libc::c_int as usize];
                    } else {
                        data2[0 as libc::c_int as usize] = data_0[1 as libc::c_int as usize];
                        data2[1 as libc::c_int as usize] = data_0[2 as libc::c_int as usize];
                    }
                    j_0 += 1;
                }
                if non_compliant_DVD == 0 as libc::c_int
                    || data1[0 as libc::c_int as usize] as libc::c_int != 0x80 as libc::c_int
                        && data1[0 as libc::c_int as usize] as libc::c_int != 0 as libc::c_int
                    || data1[1 as libc::c_int as usize] as libc::c_int != 0x80 as libc::c_int
                        && data1[1 as libc::c_int as usize] as libc::c_int != 0 as libc::c_int
                    || too_many_blocks() == 0
                {
                    printdata(
                        data1.as_mut_ptr(),
                        2 as libc::c_int,
                        data2.as_mut_ptr(),
                        2 as libc::c_int,
                    );
                    c1count = c1count.wrapping_add(1);
                    c2count = c2count.wrapping_add(1);
                }
                i += 1;
            }
            while *header.offset(0 as libc::c_int as isize) as libc::c_int == 0xfe as libc::c_int
                || *header.offset(0 as libc::c_int as isize) as libc::c_int == 0xff as libc::c_int
            {
                let mut j_1: libc::c_int = 0;
                let mut data1_0: [libc::c_uchar; 2] = [
                    0x80 as libc::c_int as libc::c_uchar,
                    0x80 as libc::c_int as libc::c_uchar,
                ];
                let mut data2_0: [libc::c_uchar; 2] = [
                    0x80 as libc::c_int as libc::c_uchar,
                    0x80 as libc::c_int as libc::c_uchar,
                ];
                j_1 = 0 as libc::c_int;
                while j_1 < 2 as libc::c_int {
                    let mut data_1: [libc::c_uchar; 3] = [0; 3];
                    data_1[0 as libc::c_int as usize] = *header.offset(0 as libc::c_int as isize);
                    data_1[1 as libc::c_int as usize] = *header.offset(1 as libc::c_int as isize);
                    data_1[2 as libc::c_int as usize] = *header.offset(2 as libc::c_int as isize);
                    header = header.offset(3 as libc::c_int as isize);
                    if data_1[0 as libc::c_int as usize] as libc::c_int == 0xff as libc::c_int
                        && j_1 == field1packet
                    {
                        data1_0[0 as libc::c_int as usize] = data_1[1 as libc::c_int as usize];
                        data1_0[1 as libc::c_int as usize] = data_1[2 as libc::c_int as usize];
                    } else {
                        data2_0[0 as libc::c_int as usize] = data_1[1 as libc::c_int as usize];
                        data2_0[1 as libc::c_int as usize] = data_1[2 as libc::c_int as usize];
                    }
                    j_1 += 1;
                }
                if data1_0[0 as libc::c_int as usize] as libc::c_int != 0x80 as libc::c_int
                    && data1_0[0 as libc::c_int as usize] as libc::c_int != 0 as libc::c_int
                    || data1_0[1 as libc::c_int as usize] as libc::c_int != 0x80 as libc::c_int
                        && data1_0[1 as libc::c_int as usize] as libc::c_int != 0 as libc::c_int
                    || too_many_blocks() == 0
                {
                    non_compliant_DVD = 1 as libc::c_int;
                    printdata(
                        data1_0.as_mut_ptr(),
                        2 as libc::c_int,
                        data2_0.as_mut_ptr(),
                        2 as libc::c_int,
                    );
                    c1count = c1count.wrapping_add(1);
                    c2count = c2count.wrapping_add(1);
                }
            }
        } else if *header.offset(0 as libc::c_int as isize) as libc::c_int == 0xbb as libc::c_int
            && *header.offset(1 as libc::c_int as isize) as libc::c_int == 0x2 as libc::c_int
            || *header.offset(2 as libc::c_int as isize) as libc::c_int == 0x99 as libc::c_int
                && *header.offset(3 as libc::c_int as isize) as libc::c_int == 0x2 as libc::c_int
        {
            let mut data1_1: [libc::c_uchar; 2] = [0; 2];
            let mut data2_1: [libc::c_uchar; 2] = [0; 2];
            if *header.offset(0 as libc::c_int as isize) as libc::c_int == 0xbb as libc::c_int {
                stat_replay4000headers += 1;
            } else {
                stat_replay5000headers += 1;
            }
            header = header.offset(2 as libc::c_int as isize);
            data2_1[0 as libc::c_int as usize] = *header.offset(0 as libc::c_int as isize);
            data2_1[1 as libc::c_int as usize] = *header.offset(1 as libc::c_int as isize);
            header = header.offset(4 as libc::c_int as isize);
            data1_1[0 as libc::c_int as usize] = *header.offset(0 as libc::c_int as isize);
            data1_1[1 as libc::c_int as usize] = *header.offset(1 as libc::c_int as isize);
            printdata(
                data1_1.as_mut_ptr(),
                2 as libc::c_int,
                data2_1.as_mut_ptr(),
                2 as libc::c_int,
            );
            c1count = c1count.wrapping_add(1);
            c2count = c2count.wrapping_add(1);
        } else if *header.offset(0 as libc::c_int as isize) as libc::c_int == 0x47 as libc::c_int
            && *header.offset(1 as libc::c_int as isize) as libc::c_int == 0x41 as libc::c_int
            && *header.offset(2 as libc::c_int as isize) as libc::c_int == 0x39 as libc::c_int
            && *header.offset(3 as libc::c_int as isize) as libc::c_int == 0x34 as libc::c_int
        {
            if header.offset(5 as libc::c_int as isize) > endofbuffer {
                header = header.offset(-(4 as libc::c_int as isize));
                break;
            } else {
                stat_hdtv += 1;
                if *header.offset(4 as libc::c_int as isize) as libc::c_int == 0x3 as libc::c_int {
                    let mut cc_data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                    let mut ud_header: libc::c_uchar = 0;
                    let mut cc_count: libc::c_uchar = 0;
                    let mut process_cc_data_flag: libc::c_uchar = 0;
                    if current_picture_coding_type == RESET_OR_UNKNOWN as libc::c_int {
                        if last_picture_coding_type == P_FRAME as libc::c_int {
                            current_picture_coding_type = B_FRAME as libc::c_int;
                            twoback_picture_coding_type = P_FRAME as libc::c_int;
                            last_picture_coding_type = B_FRAME as libc::c_int;
                        }
                        if last_picture_coding_type == B_FRAME as libc::c_int
                            && twoback_picture_coding_type == B_FRAME as libc::c_int
                        {
                            current_picture_coding_type = P_FRAME as libc::c_int;
                            twoback_picture_coding_type = B_FRAME as libc::c_int;
                            last_picture_coding_type = P_FRAME as libc::c_int;
                        }
                        if last_picture_coding_type == B_FRAME as libc::c_int
                            && twoback_picture_coding_type == P_FRAME as libc::c_int
                        {
                            current_picture_coding_type = B_FRAME as libc::c_int;
                            twoback_picture_coding_type = B_FRAME as libc::c_int;
                            last_picture_coding_type = B_FRAME as libc::c_int;
                        }
                        if debug != 0 {
                            libc::printf(
                                b"\rPicture time assumed to be: %d\n\0" as *const u8
                                    as *const libc::c_char,
                                last_picture_coding_type,
                            );
                        }
                    }
                    current_picture_coding_type != B_FRAME as libc::c_int;
                    if current_picture_coding_type == B_FRAME as libc::c_int && autopad != 0 {
                        if gop_pad != 0 {
                            gop_padding();
                        } else if pts_set == 2 as libc::c_int {
                            pts_padding();
                        }
                    }
                    ud_header = *header.offset(5 as libc::c_int as isize);
                    cc_count = (ud_header as libc::c_int & 0x1f as libc::c_int) as libc::c_uchar;
                    process_cc_data_flag = ((ud_header as libc::c_int & 0x40 as libc::c_int)
                        >> 6 as libc::c_int)
                        as libc::c_uchar;
                    if process_cc_data_flag != 0 {
                        let mut j_2: libc::c_int = 0;
                        let mut bail: libc::c_int = 0;
                        let mut proceed: libc::c_int = 1 as libc::c_int;
                        cc_data = header.offset(7 as libc::c_int as isize);
                        if cc_data.offset((cc_count as libc::c_int * 3 as libc::c_int) as isize)
                            > endofbuffer
                        {
                            header = header.offset(-(4 as libc::c_int as isize));
                            break;
                        } else {
                            if *cc_data
                                .offset((cc_count as libc::c_int * 3 as libc::c_int) as isize)
                                as libc::c_int
                                != 0xff as libc::c_int
                            {
                                proceed = 0 as libc::c_int;
                            }
                            limit = cc_count as libc::c_int * 3 as libc::c_int;
                            printed = 0 as libc::c_int;
                            if proceed == 0 && debug != 0 {
                                libc::printf(
                                    b"\rThe following payload seems to be CC but is not properly terminated.\n\0"
                                        as *const u8 as *const libc::c_char,
                                );
                                libc::printf(
                                    b"(it will be processed anyway).\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                                dump(
                                    header.offset(-(4 as libc::c_int as isize)),
                                    128 as libc::c_int,
                                );
                                printed = 1 as libc::c_int;
                            }
                            bail = 0 as libc::c_int;
                            if proceed == 0 && debug != 0 {
                                libc::printf(
                                    b"This packet is not correctly terminated.\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                                libc::printf(
                                    b"Data start at offset %d of a %d bytes block.\n\0" as *const u8
                                        as *const libc::c_char,
                                    (header.offset_from(data) as libc::c_long
                                        - 4 as libc::c_int as libc::c_long)
                                        as libc::c_int,
                                    length as libc::c_int,
                                );
                                dump(data, 256 as libc::c_int);
                            }
                            j_2 = 0 as libc::c_int;
                            while j_2 < limit {
                                let mut cc_valid: libc::c_uchar = 0;
                                let mut cc_type: libc::c_uchar = 0;
                                if proceed == 0 && ff_cleanup != 0 {
                                    if *cc_data.offset(j_2 as isize) as libc::c_int
                                        == 0xfa as libc::c_int
                                        && *cc_data.offset((j_2 + 1 as libc::c_int) as isize)
                                            as libc::c_int
                                            == 0 as libc::c_int
                                        && *cc_data.offset((j_2 + 2 as libc::c_int) as isize)
                                            as libc::c_int
                                            == 0 as libc::c_int
                                    {
                                        break;
                                    }
                                    if cc608_parity_table[*cc_data
                                        .offset((j_2 + 1 as libc::c_int) as isize)
                                        as usize]
                                        == 0
                                        || cc608_parity_table[*cc_data
                                            .offset((j_2 + 2 as libc::c_int) as isize)
                                            as usize]
                                            == 0
                                    {
                                        break;
                                    }
                                }
                                cc_valid = ((*cc_data.offset(j_2 as isize) as libc::c_int
                                    & 4 as libc::c_int)
                                    >> 2 as libc::c_int)
                                    as libc::c_uchar;
                                cc_type = (*cc_data.offset(j_2 as isize) as libc::c_int
                                    & 3 as libc::c_int)
                                    as libc::c_uchar;
                                if cc_valid as libc::c_int == 0 as libc::c_int
                                    && *cc_data.offset((j_2 + 1 as libc::c_int) as isize)
                                        as libc::c_int
                                        == 0 as libc::c_int
                                    && *cc_data.offset((j_2 + 2 as libc::c_int) as isize)
                                        as libc::c_int
                                        == 0 as libc::c_int
                                    && fix_padding != 0
                                {
                                    cc_valid = 1 as libc::c_int as libc::c_uchar;
                                    *cc_data.offset((j_2 + 1 as libc::c_int) as isize) =
                                        0x80 as libc::c_int as libc::c_uchar;
                                    *cc_data.offset((j_2 + 2 as libc::c_int) as isize) =
                                        0x80 as libc::c_int as libc::c_uchar;
                                }
                                if cc_valid != 0 {
                                    cc_stats[cc_type as usize] += 1;
                                    if bail != 0
                                        && (cc_type as libc::c_int == 0 as libc::c_int
                                            || cc_type as libc::c_int == 1 as libc::c_int)
                                        && debug != 0
                                    {
                                        if printed == 0 {
                                            libc::printf(
                                                b"\rThis packet is not supposed to have more CC but it does!\n\0"
                                                    as *const u8 as *const libc::c_char,
                                            );
                                            dump(
                                                header.offset(-(4 as libc::c_int as isize)),
                                                128 as libc::c_int,
                                            );
                                        } else {
                                            libc::printf(
                                                b"\rThe PREVIOUSLY dumped packet was not supposed to have more CC but it did!\n\0"
                                                    as *const u8 as *const libc::c_char,
                                            );
                                        }
                                    }
                                    if cc_type as libc::c_int == 0 as libc::c_int {
                                        printdata(
                                            cc_data
                                                .offset(j_2 as isize)
                                                .offset(1 as libc::c_int as isize),
                                            2 as libc::c_int,
                                            0 as *const libc::c_uchar,
                                            0 as libc::c_int,
                                        );
                                        c1count = c1count.wrapping_add(1);
                                    } else if cc_type as libc::c_int == 1 as libc::c_int {
                                        printdata(
                                            0 as *const libc::c_uchar,
                                            0 as libc::c_int,
                                            cc_data
                                                .offset(j_2 as isize)
                                                .offset(1 as libc::c_int as isize),
                                            2 as libc::c_int,
                                        );
                                        c2count = c2count.wrapping_add(1);
                                    } else {
                                        bail = 1 as libc::c_int;
                                    }
                                }
                                j_2 = j_2 + 3 as libc::c_int;
                            }
                            header = header
                                .offset(7 as libc::c_int as isize)
                                .offset(limit as isize);
                            if header > endofbuffer {
                                header = endofbuffer;
                            }
                        }
                    }
                    twoback_picture_coding_type = last_picture_coding_type;
                    if current_picture_coding_type == B_FRAME as libc::c_int {
                        last_picture_coding_type = B_FRAME as libc::c_int;
                    } else {
                        last_picture_coding_type = P_FRAME as libc::c_int;
                    }
                    current_picture_coding_type = RESET_OR_UNKNOWN as libc::c_int;
                }
                header = header.offset(1);
            }
        } else if *header.offset(0 as libc::c_int as isize) as libc::c_int == 0x5 as libc::c_int
            && *header.offset(1 as libc::c_int as isize) as libc::c_int == 0x2 as libc::c_int
        {
            let mut type_0: libc::c_uchar = 0;
            let mut hi: libc::c_uchar = 0;
            let mut data1_2: [libc::c_uchar; 2] = [0; 2];
            if current_picture_coding_type == B_FRAME as libc::c_int && autopad != 0 {
                gop_padding();
            }
            stat_dishheaders += 1;
            header = header.offset(7 as libc::c_int as isize);
            type_0 = *header.offset(0 as libc::c_int as isize);
            header = header.offset(1);
            match type_0 as libc::c_int {
                2 => {
                    header = header.offset(1);
                    data1_2[0 as libc::c_int as usize] = *header.offset(0 as libc::c_int as isize);
                    data1_2[1 as libc::c_int as usize] = *header.offset(1 as libc::c_int as isize);
                    header = header.offset(2 as libc::c_int as isize);
                    type_0 = *header.offset(0 as libc::c_int as isize);
                    header = header.offset(1);
                    if data1_2[0 as libc::c_int as usize] as libc::c_int != 0x80 as libc::c_int
                        && data1_2[0 as libc::c_int as usize] as libc::c_int != 0 as libc::c_int
                        || data1_2[1 as libc::c_int as usize] as libc::c_int != 0x80 as libc::c_int
                            && data1_2[1 as libc::c_int as usize] as libc::c_int != 0 as libc::c_int
                        || too_many_blocks() == 0
                    {
                        printdata(
                            data1_2.as_mut_ptr(),
                            2 as libc::c_int,
                            0 as *const libc::c_uchar,
                            0 as libc::c_int,
                        );
                        c1count = c1count.wrapping_add(1);
                    }
                    hi = (data1_2[0 as libc::c_int as usize] as libc::c_int & 0x7f as libc::c_int)
                        as libc::c_uchar;
                    if type_0 as libc::c_int == 0x4 as libc::c_int
                        && (hi as libc::c_int) < 32 as libc::c_int
                    {
                        if data1_2[0 as libc::c_int as usize] as libc::c_int != 0x80 as libc::c_int
                            && data1_2[0 as libc::c_int as usize] as libc::c_int != 0 as libc::c_int
                            || data1_2[1 as libc::c_int as usize] as libc::c_int
                                != 0x80 as libc::c_int
                                && data1_2[1 as libc::c_int as usize] as libc::c_int
                                    != 0 as libc::c_int
                            || too_many_blocks() == 0
                        {
                            printdata(
                                data1_2.as_mut_ptr(),
                                2 as libc::c_int,
                                DO_NOTHING.as_ptr(),
                                ::std::mem::size_of::<[libc::c_uchar; 2]>() as libc::c_ulong
                                    as libc::c_int,
                            );
                            c1count = c1count.wrapping_add(1);
                            c2count = c2count.wrapping_add(1);
                        }
                    }
                    header = header.offset(3 as libc::c_int as isize);
                }
                4 => {
                    header = header.offset(1);
                    data1_2[0 as libc::c_int as usize] = *header.offset(0 as libc::c_int as isize);
                    data1_2[1 as libc::c_int as usize] = *header.offset(1 as libc::c_int as isize);
                    header = header.offset(2 as libc::c_int as isize);
                    if data1_2[0 as libc::c_int as usize] as libc::c_int != 0x80 as libc::c_int
                        && data1_2[0 as libc::c_int as usize] as libc::c_int != 0 as libc::c_int
                        || data1_2[1 as libc::c_int as usize] as libc::c_int != 0x80 as libc::c_int
                            && data1_2[1 as libc::c_int as usize] as libc::c_int != 0 as libc::c_int
                        || too_many_blocks() == 0
                    {
                        printdata(
                            data1_2.as_mut_ptr(),
                            2 as libc::c_int,
                            DO_NOTHING.as_ptr(),
                            ::std::mem::size_of::<[libc::c_uchar; 2]>() as libc::c_ulong
                                as libc::c_int,
                        );
                        c1count = c1count.wrapping_add(1);
                        c2count = c2count.wrapping_add(1);
                    }
                    data1_2[0 as libc::c_int as usize] = *header.offset(0 as libc::c_int as isize);
                    data1_2[1 as libc::c_int as usize] = *header.offset(1 as libc::c_int as isize);
                    header = header.offset(2 as libc::c_int as isize);
                    if data1_2[0 as libc::c_int as usize] as libc::c_int != 0x80 as libc::c_int
                        && data1_2[0 as libc::c_int as usize] as libc::c_int != 0 as libc::c_int
                        || data1_2[1 as libc::c_int as usize] as libc::c_int != 0x80 as libc::c_int
                            && data1_2[1 as libc::c_int as usize] as libc::c_int != 0 as libc::c_int
                        || too_many_blocks() == 0
                    {
                        printdata(
                            data1_2.as_mut_ptr(),
                            2 as libc::c_int,
                            DO_NOTHING.as_ptr(),
                            ::std::mem::size_of::<[libc::c_uchar; 2]>() as libc::c_ulong
                                as libc::c_int,
                        );
                        c1count = c1count.wrapping_add(1);
                        c2count = c2count.wrapping_add(1);
                    }
                    header = header.offset(4 as libc::c_int as isize);
                }
                5 => {
                    j = 0 as libc::c_int;
                    while j < p_caption_size {
                        printdata(
                            p_caption.offset(j as isize),
                            2 as libc::c_int,
                            0 as *const libc::c_uchar,
                            0 as libc::c_int,
                        );
                        c1count = c1count.wrapping_add(1);
                        j += 2 as libc::c_int;
                    }
                    p_caption_size = 0 as libc::c_int;
                    header = header.offset(6 as libc::c_int as isize);
                    type_0 = *header.offset(0 as libc::c_int as isize);
                    header = header.offset(2 as libc::c_int as isize);
                    data1_2[0 as libc::c_int as usize] = *header.offset(0 as libc::c_int as isize);
                    data1_2[1 as libc::c_int as usize] = *header.offset(1 as libc::c_int as isize);
                    header = header.offset(2 as libc::c_int as isize);
                    if p_caption_capacity < 2 as libc::c_int {
                        p_caption = libc::realloc(
                            p_caption as *mut libc::c_void,
                            1024,
                        ) as *mut libc::c_uchar;
                        p_caption_capacity = 1024 as libc::c_int;
                    }
                    *p_caption.offset(0 as libc::c_int as isize) =
                        data1_2[0 as libc::c_int as usize];
                    *p_caption.offset(1 as libc::c_int as isize) =
                        data1_2[1 as libc::c_int as usize];
                    p_caption_size = 2 as libc::c_int;
                    if type_0 as libc::c_int == 0x2 as libc::c_int {
                        type_0 = *header.offset(0 as libc::c_int as isize);
                        header = header.offset(1);
                        hi = (data1_2[0 as libc::c_int as usize] as libc::c_int
                            & 0x7f as libc::c_int) as libc::c_uchar;
                        if type_0 as libc::c_int == 0x4 as libc::c_int
                            && (hi as libc::c_int) < 32 as libc::c_int
                        {
                            if p_caption_capacity < p_caption_size + 2 as libc::c_int {
                                p_caption = libc::realloc(
                                    p_caption as *mut libc::c_void,
                                    (p_caption_capacity + 1024 as libc::c_int) as usize,
                                ) as *mut libc::c_uchar;
                                p_caption_capacity += 1024 as libc::c_int;
                            }
                            *p_caption.offset(p_caption_size as isize) =
                                data1_2[0 as libc::c_int as usize];
                            *p_caption.offset((p_caption_size + 1 as libc::c_int) as isize) =
                                data1_2[1 as libc::c_int as usize];
                            p_caption_size += 2 as libc::c_int;
                        }
                    } else {
                        data1_2[0 as libc::c_int as usize] =
                            *header.offset(0 as libc::c_int as isize);
                        data1_2[1 as libc::c_int as usize] =
                            *header.offset(1 as libc::c_int as isize);
                        header = header.offset(2 as libc::c_int as isize);
                        if p_caption_capacity < p_caption_size + 2 as libc::c_int {
                            p_caption = libc::realloc(
                                p_caption as *mut libc::c_void,
                                (p_caption_capacity + 1024 as libc::c_int) as usize,
                            ) as *mut libc::c_uchar;
                            p_caption_capacity += 1024 as libc::c_int;
                        }
                        *p_caption.offset(p_caption_size as isize) =
                            data1_2[0 as libc::c_int as usize];
                        *p_caption.offset((p_caption_size + 1 as libc::c_int) as isize) =
                            data1_2[1 as libc::c_int as usize];
                        p_caption_size += 2 as libc::c_int;
                        header = header.offset(1);
                    }
                    header = header.offset(3 as libc::c_int as isize);
                }
                _ => {}
            }
            header = header.offset(1);
        } else {
            header = header.offset(1);
        }
    }
    return header.offset_from(data) as libc::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn general_loop() {
    let mut overlap: LONG = 0 as libc::c_int as LONG;
    let mut pos: LONG = 0 as libc::c_int as LONG;
    end_of_file = 0 as libc::c_int;
    current_picture_coding_type = 0 as libc::c_int;
    p_caption_size = 0 as libc::c_int;
    p_caption_capacity = 0 as libc::c_int;
    p_caption = 0 as *mut libc::c_uchar;
    while end_of_file == 0 && processed_enough == 0 {
        let mut i: LONG = 0;
        let mut got: LONG = 0;
        overlap = inbuf - pos;
        libc::memmove(
            fbuffer as *mut libc::c_void,
            fbuffer.offset(pos as isize) as *const libc::c_void,
            (inbuf - pos) as usize,
        );
        inbuf -= pos;
        pos = 0 as libc::c_int as LONG;
        if ts_mode != 0 {
            i = ts_getmoredata();
        } else {
            i = general_getmoredata();
        }
        if !clean.is_null() {
            libc::fwrite(
                fbuffer.offset(overlap as isize) as *const libc::c_void,
                1,
                (inbuf - overlap) as usize,
                clean,
            );
        }
        if i == 0 as libc::c_int as libc::c_long {
            end_of_file = 1 as libc::c_int;
            libc::memset(
                fbuffer.offset(inbuf as isize) as *mut libc::c_void,
                0 as libc::c_int,
                ((256 as libc::c_int * 1024 as libc::c_int + 120 as libc::c_int) as libc::c_long
                    - inbuf) as usize,
            );
        }
        if inbuf == 0 as libc::c_int as libc::c_long {
            break;
        }
        got = process_block(fbuffer, inbuf);
        if got > inbuf {
            libc::printf(b"BUG BUG\n\0" as *const u8 as *const libc::c_char);
        }
        pos += got;
        if inputsize > 0 as libc::c_int as libc::c_long {
            let progress: libc::c_int =
                ((past >> 8 as libc::c_int) * 100 as libc::c_int as libc::c_long
                    / (inputsize >> 8 as libc::c_int)) as libc::c_int;
            if last_reported_progress != progress {
                let mut cur_sec: libc::c_int = 0;
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
}
