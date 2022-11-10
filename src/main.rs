#![allow(
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_variables,
    unused_must_use
)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool, c_variadic, linkage)]
use std::arch::asm;
extern crate c2rust_bitfields;
extern crate libm;

mod ccextratorwin;
mod comskip;

extern "C" {
    pub type AVOptionRanges;
    pub type AVOption;
    pub type AVBuffer;
    pub type AVDictionary;
    pub type AVCodecDefault;
    pub type MpegEncContext;
    pub type AVCodecInternal;
    pub type AVCodecHWConfigInternal;
    pub type AVFormatInternal;
    pub type AVStreamInternal;
    pub type AVDeviceCapabilitiesQuery;
    pub type AVDeviceInfoList;
    pub type AVCodecTag;
    pub type SwsContext;
    static mut __stderrp: *mut libc::FILE;
    fn signal(
        _: libc::c_int,
        _: Option<unsafe extern "C" fn(libc::c_int) -> ()>,
    ) -> Option<unsafe extern "C" fn(libc::c_int) -> ()>;
    fn av_mallocz(size: size_t) -> *mut libc::c_void;
    fn av_log_get_level() -> libc::c_int;
    fn av_log_set_level(level: libc::c_int);
    fn av_log_set_callback(
        callback: Option<
            unsafe extern "C" fn(
                *mut libc::c_void,
                libc::c_int,
                *const libc::c_char,
                ::std::ffi::VaList,
            ) -> (),
        >,
    );
    fn av_log_format_line(
        ptr: *mut libc::c_void,
        level: libc::c_int,
        fmt: *const libc::c_char,
        vl: ::std::ffi::VaList,
        line: *mut libc::c_char,
        line_size: libc::c_int,
        print_prefix: *mut libc::c_int,
    );
    fn av_log_set_flags(arg: libc::c_int);
    fn av_get_bytes_per_sample(sample_fmt: AVSampleFormat) -> libc::c_int;
    fn av_sample_fmt_is_planar(sample_fmt: AVSampleFormat) -> libc::c_int;
    fn av_samples_get_buffer_size(
        linesize: *mut libc::c_int,
        nb_channels: libc::c_int,
        nb_samples: libc::c_int,
        sample_fmt: AVSampleFormat,
        align: libc::c_int,
    ) -> libc::c_int;
    fn av_dict_set_int(
        pm: *mut *mut AVDictionary,
        key: *const libc::c_char,
        value: int64_t,
        flags: libc::c_int,
    ) -> libc::c_int;
    fn av_frame_get_best_effort_timestamp(frame: *const AVFrame) -> int64_t;
    fn av_frame_alloc() -> *mut AVFrame;
    fn av_frame_free(frame: *mut *mut AVFrame);
    fn av_frame_unref(frame: *mut AVFrame);
    fn av_frame_get_buffer(frame: *mut AVFrame, align: libc::c_int) -> libc::c_int;
    fn av_frame_copy_props(dst: *mut AVFrame, src: *const AVFrame) -> libc::c_int;
    fn av_packet_unref(pkt: *mut AVPacket);
    fn avcodec_find_decoder(id: AVCodecID) -> *mut AVCodec;
    fn avcodec_find_decoder_by_name(name: *const libc::c_char) -> *mut AVCodec;
    fn av_codec_get_max_lowres(codec: *const AVCodec) -> libc::c_int;
    fn avcodec_register_all();
    fn avcodec_open2(
        avctx: *mut AVCodecContext,
        codec: *const AVCodec,
        options: *mut *mut AVDictionary,
    ) -> libc::c_int;
    fn avcodec_close(avctx: *mut AVCodecContext) -> libc::c_int;
    fn avcodec_decode_audio4(
        avctx: *mut AVCodecContext,
        frame: *mut AVFrame,
        got_frame_ptr: *mut libc::c_int,
        avpkt: *const AVPacket,
    ) -> libc::c_int;
    fn avcodec_decode_video2(
        avctx: *mut AVCodecContext,
        picture: *mut AVFrame,
        got_picture_ptr: *mut libc::c_int,
        avpkt: *const AVPacket,
    ) -> libc::c_int;
    fn avcodec_flush_buffers(avctx: *mut AVCodecContext);
    fn avio_seek(s: *mut AVIOContext, offset: int64_t, whence: libc::c_int) -> int64_t;
    fn avio_size(s: *mut AVIOContext) -> int64_t;
    fn av_stream_get_parser(s: *const AVStream) -> *mut AVCodecParserContext;
    fn av_register_all();
    fn avformat_network_init() -> libc::c_int;
    fn avformat_network_deinit() -> libc::c_int;
    fn avformat_alloc_context() -> *mut AVFormatContext;
    fn avformat_open_input(
        ps: *mut *mut AVFormatContext,
        url: *const libc::c_char,
        fmt: *mut AVInputFormat,
        options: *mut *mut AVDictionary,
    ) -> libc::c_int;
    fn avformat_find_stream_info(
        ic: *mut AVFormatContext,
        options: *mut *mut AVDictionary,
    ) -> libc::c_int;
    fn av_find_best_stream(
        ic: *mut AVFormatContext,
        type_0: AVMediaType,
        wanted_stream_nb: libc::c_int,
        related_stream: libc::c_int,
        decoder_ret: *mut *mut AVCodec,
        flags: libc::c_int,
    ) -> libc::c_int;
    fn av_read_frame(s: *mut AVFormatContext, pkt: *mut AVPacket) -> libc::c_int;
    fn av_seek_frame(
        s: *mut AVFormatContext,
        stream_index_0: libc::c_int,
        timestamp: int64_t,
        flags: libc::c_int,
    ) -> libc::c_int;
    fn avformat_close_input(s: *mut *mut AVFormatContext);
    fn av_dump_format(
        ic: *mut AVFormatContext,
        index: libc::c_int,
        url: *const libc::c_char,
        is_output: libc::c_int,
    );
    fn sws_scale(
        c: *mut SwsContext,
        srcSlice: *const *const uint8_t,
        srcStride: *const libc::c_int,
        srcSliceY: libc::c_int,
        srcSliceH: libc::c_int,
        dst: *const *mut uint8_t,
        dstStride: *const libc::c_int,
    ) -> libc::c_int;
    fn sws_getCachedContext(
        context: *mut SwsContext,
        srcW: libc::c_int,
        srcH: libc::c_int,
        srcFormat: AVPixelFormat,
        dstW: libc::c_int,
        dstH: libc::c_int,
        dstFormat: AVPixelFormat,
        flags: libc::c_int,
        srcFilter: *mut SwsFilter,
        dstFilter: *mut SwsFilter,
        param: *const libc::c_double,
    ) -> *mut SwsContext;
    static mut hardware_decode: libc::c_int;
    static mut width: libc::c_int;
    static mut height: libc::c_int;
    static mut audio_channels: libc::c_int;
    fn BuildCommListAsYouGo();
    static mut lastFrameCommCalculated: libc::c_int;
    static mut thread_count: libc::c_int;
    static mut timeline_repair: libc::c_int;
    static mut inbasename: [libc::c_char; 0];
    static mut skip_B_frames: libc::c_int;
    static mut lowres: libc::c_int;
    static mut selftest: libc::c_int;
    static mut frame_count: libc::c_int;
    static mut HomeDir: [libc::c_char; 256];
    static mut processCC: libc::c_int;
    static mut live_tv: libc::c_int;
    static mut ccData: [uint8_t; 500];
    static mut ccDataLen: libc::c_int;
    static mut videowidth: libc::c_int;
    static mut output_debugwindow: libc::c_int;
    static mut output_timing: libc::c_int;
    static mut output_srt: libc::c_int;
    static mut output_smi: libc::c_int;
    static mut frame_ptr: *mut libc::c_uchar;
    static mut live_tv_retries: libc::c_int;
    fn set_fps(
        frame_delay: libc::c_double,
        dfps: libc::c_double,
        ticks: libc::c_int,
        rfps: libc::c_double,
        afps: libc::c_double,
    );
    fn dump_video(start: *mut libc::c_char, end: *mut libc::c_char);
    fn dump_audio(start: *mut libc::c_char, end: *mut libc::c_char);
    fn Debug(level: libc::c_int, fmt: *mut libc::c_char, _: ...);
    fn dump_video_start();
    fn dump_audio_start();
    fn DetectCommercials(_: libc::c_int, _: libc::c_double) -> libc::c_int;
    fn BuildMasterCommList() -> libc::c_int;
    fn LoadSettings(argc: libc::c_int, argv: *mut *mut libc::c_char) -> *mut libc::FILE;
    fn ProcessCCData();
    fn dump_data(start: *mut libc::c_char, length: libc::c_int);
    fn close_data();
    fn get_fps() -> libc::c_double;
    fn set_frame_volume(framenr: uint32_t, volume: libc::c_int);
    fn get_frame_pts(f: libc::c_int) -> libc::c_double;
    fn CEW_reinit();
    fn process_block(data: *mut libc::c_uchar, length: libc::c_long) -> libc::c_long;
    static mut mpegfilename: [libc::c_char; 0];
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_longlong;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_time_t = libc::c_long;
pub type __darwin_off_t = __int64_t;
pub type __darwin_suseconds_t = __int32_t;
pub type __darwin_useconds_t = __uint32_t;
pub type size_t = __darwin_size_t;
pub type int8_t = libc::c_schar;
pub type int64_t = libc::c_longlong;
pub type uint64_t = libc::c_ulonglong;
pub type uint32_t = libc::c_uint;
pub type useconds_t = __darwin_useconds_t;
pub type va_list = __builtin_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut libc::c_uchar,
    pub _size: libc::c_int,
}
pub type uint8_t = libc::c_uchar;
pub type uint16_t = libc::c_ushort;
pub type fileh = *mut libc::FILE;
pub type AVMediaType = libc::c_int;
pub const AVMEDIA_TYPE_NB: AVMediaType = 5;
pub const AVMEDIA_TYPE_ATTACHMENT: AVMediaType = 4;
pub const AVMEDIA_TYPE_SUBTITLE: AVMediaType = 3;
pub const AVMEDIA_TYPE_DATA: AVMediaType = 2;
pub const AVMEDIA_TYPE_AUDIO: AVMediaType = 1;
pub const AVMEDIA_TYPE_VIDEO: AVMediaType = 0;
pub const AVMEDIA_TYPE_UNKNOWN: AVMediaType = -1;
pub type AVPictureType = libc::c_uint;
pub const AV_PICTURE_TYPE_BI: AVPictureType = 7;
pub const AV_PICTURE_TYPE_SP: AVPictureType = 6;
pub const AV_PICTURE_TYPE_SI: AVPictureType = 5;
pub const AV_PICTURE_TYPE_S: AVPictureType = 4;
pub const AV_PICTURE_TYPE_B: AVPictureType = 3;
pub const AV_PICTURE_TYPE_P: AVPictureType = 2;
pub const AV_PICTURE_TYPE_I: AVPictureType = 1;
pub const AV_PICTURE_TYPE_NONE: AVPictureType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVRational {
    pub num: libc::c_int,
    pub den: libc::c_int,
}
pub type AVClassCategory = libc::c_uint;
pub const AV_CLASS_CATEGORY_NB: AVClassCategory = 46;
pub const AV_CLASS_CATEGORY_DEVICE_INPUT: AVClassCategory = 45;
pub const AV_CLASS_CATEGORY_DEVICE_OUTPUT: AVClassCategory = 44;
pub const AV_CLASS_CATEGORY_DEVICE_AUDIO_INPUT: AVClassCategory = 43;
pub const AV_CLASS_CATEGORY_DEVICE_AUDIO_OUTPUT: AVClassCategory = 42;
pub const AV_CLASS_CATEGORY_DEVICE_VIDEO_INPUT: AVClassCategory = 41;
pub const AV_CLASS_CATEGORY_DEVICE_VIDEO_OUTPUT: AVClassCategory = 40;
pub const AV_CLASS_CATEGORY_SWRESAMPLER: AVClassCategory = 10;
pub const AV_CLASS_CATEGORY_SWSCALER: AVClassCategory = 9;
pub const AV_CLASS_CATEGORY_BITSTREAM_FILTER: AVClassCategory = 8;
pub const AV_CLASS_CATEGORY_FILTER: AVClassCategory = 7;
pub const AV_CLASS_CATEGORY_DECODER: AVClassCategory = 6;
pub const AV_CLASS_CATEGORY_ENCODER: AVClassCategory = 5;
pub const AV_CLASS_CATEGORY_DEMUXER: AVClassCategory = 4;
pub const AV_CLASS_CATEGORY_MUXER: AVClassCategory = 3;
pub const AV_CLASS_CATEGORY_OUTPUT: AVClassCategory = 2;
pub const AV_CLASS_CATEGORY_INPUT: AVClassCategory = 1;
pub const AV_CLASS_CATEGORY_NA: AVClassCategory = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVClass {
    pub class_name: *const libc::c_char,
    pub item_name: Option<unsafe extern "C" fn(*mut libc::c_void) -> *const libc::c_char>,
    pub option: *const AVOption,
    pub version: libc::c_int,
    pub log_level_offset_offset: libc::c_int,
    pub parent_log_context_offset: libc::c_int,
    pub child_next:
        Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> *mut libc::c_void>,
    pub child_class_next: Option<unsafe extern "C" fn(*const AVClass) -> *const AVClass>,
    pub category: AVClassCategory,
    pub get_category: Option<unsafe extern "C" fn(*mut libc::c_void) -> AVClassCategory>,
    pub query_ranges: Option<
        unsafe extern "C" fn(
            *mut *mut AVOptionRanges,
            *mut libc::c_void,
            *const libc::c_char,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub child_class_iterate: Option<unsafe extern "C" fn(*mut *mut libc::c_void) -> *const AVClass>,
}
pub type AVPixelFormat = libc::c_int;
pub const AV_PIX_FMT_NB: AVPixelFormat = 198;
pub const AV_PIX_FMT_X2RGB10BE: AVPixelFormat = 197;
pub const AV_PIX_FMT_X2RGB10LE: AVPixelFormat = 196;
pub const AV_PIX_FMT_Y210LE: AVPixelFormat = 195;
pub const AV_PIX_FMT_Y210BE: AVPixelFormat = 194;
pub const AV_PIX_FMT_VULKAN: AVPixelFormat = 193;
pub const AV_PIX_FMT_NV42: AVPixelFormat = 192;
pub const AV_PIX_FMT_NV24: AVPixelFormat = 191;
pub const AV_PIX_FMT_YUVA444P12LE: AVPixelFormat = 190;
pub const AV_PIX_FMT_YUVA444P12BE: AVPixelFormat = 189;
pub const AV_PIX_FMT_YUVA422P12LE: AVPixelFormat = 188;
pub const AV_PIX_FMT_YUVA422P12BE: AVPixelFormat = 187;
pub const AV_PIX_FMT_GRAYF32LE: AVPixelFormat = 186;
pub const AV_PIX_FMT_GRAYF32BE: AVPixelFormat = 185;
pub const AV_PIX_FMT_GRAY14LE: AVPixelFormat = 184;
pub const AV_PIX_FMT_GRAY14BE: AVPixelFormat = 183;
pub const AV_PIX_FMT_OPENCL: AVPixelFormat = 182;
pub const AV_PIX_FMT_DRM_PRIME: AVPixelFormat = 181;
pub const AV_PIX_FMT_GBRAPF32LE: AVPixelFormat = 180;
pub const AV_PIX_FMT_GBRAPF32BE: AVPixelFormat = 179;
pub const AV_PIX_FMT_GBRPF32LE: AVPixelFormat = 178;
pub const AV_PIX_FMT_GBRPF32BE: AVPixelFormat = 177;
pub const AV_PIX_FMT_GRAY9LE: AVPixelFormat = 176;
pub const AV_PIX_FMT_GRAY9BE: AVPixelFormat = 175;
pub const AV_PIX_FMT_D3D11: AVPixelFormat = 174;
pub const AV_PIX_FMT_P016BE: AVPixelFormat = 173;
pub const AV_PIX_FMT_P016LE: AVPixelFormat = 172;
pub const AV_PIX_FMT_GRAY10LE: AVPixelFormat = 171;
pub const AV_PIX_FMT_GRAY10BE: AVPixelFormat = 170;
pub const AV_PIX_FMT_GRAY12LE: AVPixelFormat = 169;
pub const AV_PIX_FMT_GRAY12BE: AVPixelFormat = 168;
pub const AV_PIX_FMT_MEDIACODEC: AVPixelFormat = 167;
pub const AV_PIX_FMT_GBRAP10LE: AVPixelFormat = 166;
pub const AV_PIX_FMT_GBRAP10BE: AVPixelFormat = 165;
pub const AV_PIX_FMT_GBRAP12LE: AVPixelFormat = 164;
pub const AV_PIX_FMT_GBRAP12BE: AVPixelFormat = 163;
pub const AV_PIX_FMT_P010BE: AVPixelFormat = 162;
pub const AV_PIX_FMT_P010LE: AVPixelFormat = 161;
pub const AV_PIX_FMT_VIDEOTOOLBOX: AVPixelFormat = 160;
pub const AV_PIX_FMT_AYUV64BE: AVPixelFormat = 159;
pub const AV_PIX_FMT_AYUV64LE: AVPixelFormat = 158;
pub const AV_PIX_FMT_YUV440P12BE: AVPixelFormat = 157;
pub const AV_PIX_FMT_YUV440P12LE: AVPixelFormat = 156;
pub const AV_PIX_FMT_YUV440P10BE: AVPixelFormat = 155;
pub const AV_PIX_FMT_YUV440P10LE: AVPixelFormat = 154;
pub const AV_PIX_FMT_XVMC: AVPixelFormat = 153;
pub const AV_PIX_FMT_BAYER_GRBG16BE: AVPixelFormat = 152;
pub const AV_PIX_FMT_BAYER_GRBG16LE: AVPixelFormat = 151;
pub const AV_PIX_FMT_BAYER_GBRG16BE: AVPixelFormat = 150;
pub const AV_PIX_FMT_BAYER_GBRG16LE: AVPixelFormat = 149;
pub const AV_PIX_FMT_BAYER_RGGB16BE: AVPixelFormat = 148;
pub const AV_PIX_FMT_BAYER_RGGB16LE: AVPixelFormat = 147;
pub const AV_PIX_FMT_BAYER_BGGR16BE: AVPixelFormat = 146;
pub const AV_PIX_FMT_BAYER_BGGR16LE: AVPixelFormat = 145;
pub const AV_PIX_FMT_BAYER_GRBG8: AVPixelFormat = 144;
pub const AV_PIX_FMT_BAYER_GBRG8: AVPixelFormat = 143;
pub const AV_PIX_FMT_BAYER_RGGB8: AVPixelFormat = 142;
pub const AV_PIX_FMT_BAYER_BGGR8: AVPixelFormat = 141;
pub const AV_PIX_FMT_YUVJ411P: AVPixelFormat = 140;
pub const AV_PIX_FMT_GBRP14LE: AVPixelFormat = 139;
pub const AV_PIX_FMT_GBRP14BE: AVPixelFormat = 138;
pub const AV_PIX_FMT_GBRP12LE: AVPixelFormat = 137;
pub const AV_PIX_FMT_GBRP12BE: AVPixelFormat = 136;
pub const AV_PIX_FMT_YUV444P14LE: AVPixelFormat = 135;
pub const AV_PIX_FMT_YUV444P14BE: AVPixelFormat = 134;
pub const AV_PIX_FMT_YUV444P12LE: AVPixelFormat = 133;
pub const AV_PIX_FMT_YUV444P12BE: AVPixelFormat = 132;
pub const AV_PIX_FMT_YUV422P14LE: AVPixelFormat = 131;
pub const AV_PIX_FMT_YUV422P14BE: AVPixelFormat = 130;
pub const AV_PIX_FMT_YUV422P12LE: AVPixelFormat = 129;
pub const AV_PIX_FMT_YUV422P12BE: AVPixelFormat = 128;
pub const AV_PIX_FMT_YUV420P14LE: AVPixelFormat = 127;
pub const AV_PIX_FMT_YUV420P14BE: AVPixelFormat = 126;
pub const AV_PIX_FMT_YUV420P12LE: AVPixelFormat = 125;
pub const AV_PIX_FMT_YUV420P12BE: AVPixelFormat = 124;
pub const AV_PIX_FMT_BGR0: AVPixelFormat = 123;
pub const AV_PIX_FMT_0BGR: AVPixelFormat = 122;
pub const AV_PIX_FMT_RGB0: AVPixelFormat = 121;
pub const AV_PIX_FMT_0RGB: AVPixelFormat = 120;
pub const AV_PIX_FMT_CUDA: AVPixelFormat = 119;
pub const AV_PIX_FMT_D3D11VA_VLD: AVPixelFormat = 118;
pub const AV_PIX_FMT_MMAL: AVPixelFormat = 117;
pub const AV_PIX_FMT_QSV: AVPixelFormat = 116;
pub const AV_PIX_FMT_GBRAP16LE: AVPixelFormat = 115;
pub const AV_PIX_FMT_GBRAP16BE: AVPixelFormat = 114;
pub const AV_PIX_FMT_GBRAP: AVPixelFormat = 113;
pub const AV_PIX_FMT_YA16LE: AVPixelFormat = 112;
pub const AV_PIX_FMT_YA16BE: AVPixelFormat = 111;
pub const AV_PIX_FMT_YVYU422: AVPixelFormat = 110;
pub const AV_PIX_FMT_BGRA64LE: AVPixelFormat = 109;
pub const AV_PIX_FMT_BGRA64BE: AVPixelFormat = 108;
pub const AV_PIX_FMT_RGBA64LE: AVPixelFormat = 107;
pub const AV_PIX_FMT_RGBA64BE: AVPixelFormat = 106;
pub const AV_PIX_FMT_NV20BE: AVPixelFormat = 105;
pub const AV_PIX_FMT_NV20LE: AVPixelFormat = 104;
pub const AV_PIX_FMT_NV16: AVPixelFormat = 103;
pub const AV_PIX_FMT_XYZ12BE: AVPixelFormat = 102;
pub const AV_PIX_FMT_XYZ12LE: AVPixelFormat = 101;
pub const AV_PIX_FMT_VDPAU: AVPixelFormat = 100;
pub const AV_PIX_FMT_YUVA444P16LE: AVPixelFormat = 99;
pub const AV_PIX_FMT_YUVA444P16BE: AVPixelFormat = 98;
pub const AV_PIX_FMT_YUVA422P16LE: AVPixelFormat = 97;
pub const AV_PIX_FMT_YUVA422P16BE: AVPixelFormat = 96;
pub const AV_PIX_FMT_YUVA420P16LE: AVPixelFormat = 95;
pub const AV_PIX_FMT_YUVA420P16BE: AVPixelFormat = 94;
pub const AV_PIX_FMT_YUVA444P10LE: AVPixelFormat = 93;
pub const AV_PIX_FMT_YUVA444P10BE: AVPixelFormat = 92;
pub const AV_PIX_FMT_YUVA422P10LE: AVPixelFormat = 91;
pub const AV_PIX_FMT_YUVA422P10BE: AVPixelFormat = 90;
pub const AV_PIX_FMT_YUVA420P10LE: AVPixelFormat = 89;
pub const AV_PIX_FMT_YUVA420P10BE: AVPixelFormat = 88;
pub const AV_PIX_FMT_YUVA444P9LE: AVPixelFormat = 87;
pub const AV_PIX_FMT_YUVA444P9BE: AVPixelFormat = 86;
pub const AV_PIX_FMT_YUVA422P9LE: AVPixelFormat = 85;
pub const AV_PIX_FMT_YUVA422P9BE: AVPixelFormat = 84;
pub const AV_PIX_FMT_YUVA420P9LE: AVPixelFormat = 83;
pub const AV_PIX_FMT_YUVA420P9BE: AVPixelFormat = 82;
pub const AV_PIX_FMT_YUVA444P: AVPixelFormat = 81;
pub const AV_PIX_FMT_YUVA422P: AVPixelFormat = 80;
pub const AV_PIX_FMT_GBRP16LE: AVPixelFormat = 79;
pub const AV_PIX_FMT_GBRP16BE: AVPixelFormat = 78;
pub const AV_PIX_FMT_GBRP10LE: AVPixelFormat = 77;
pub const AV_PIX_FMT_GBRP10BE: AVPixelFormat = 76;
pub const AV_PIX_FMT_GBRP9LE: AVPixelFormat = 75;
pub const AV_PIX_FMT_GBRP9BE: AVPixelFormat = 74;
pub const AV_PIX_FMT_GBR24P: AVPixelFormat = 73;
pub const AV_PIX_FMT_GBRP: AVPixelFormat = 73;
pub const AV_PIX_FMT_YUV422P9LE: AVPixelFormat = 72;
pub const AV_PIX_FMT_YUV422P9BE: AVPixelFormat = 71;
pub const AV_PIX_FMT_YUV444P10LE: AVPixelFormat = 70;
pub const AV_PIX_FMT_YUV444P10BE: AVPixelFormat = 69;
pub const AV_PIX_FMT_YUV444P9LE: AVPixelFormat = 68;
pub const AV_PIX_FMT_YUV444P9BE: AVPixelFormat = 67;
pub const AV_PIX_FMT_YUV422P10LE: AVPixelFormat = 66;
pub const AV_PIX_FMT_YUV422P10BE: AVPixelFormat = 65;
pub const AV_PIX_FMT_YUV420P10LE: AVPixelFormat = 64;
pub const AV_PIX_FMT_YUV420P10BE: AVPixelFormat = 63;
pub const AV_PIX_FMT_YUV420P9LE: AVPixelFormat = 62;
pub const AV_PIX_FMT_YUV420P9BE: AVPixelFormat = 61;
pub const AV_PIX_FMT_BGR48LE: AVPixelFormat = 60;
pub const AV_PIX_FMT_BGR48BE: AVPixelFormat = 59;
pub const AV_PIX_FMT_GRAY8A: AVPixelFormat = 58;
pub const AV_PIX_FMT_Y400A: AVPixelFormat = 58;
pub const AV_PIX_FMT_YA8: AVPixelFormat = 58;
pub const AV_PIX_FMT_BGR444BE: AVPixelFormat = 57;
pub const AV_PIX_FMT_BGR444LE: AVPixelFormat = 56;
pub const AV_PIX_FMT_RGB444BE: AVPixelFormat = 55;
pub const AV_PIX_FMT_RGB444LE: AVPixelFormat = 54;
pub const AV_PIX_FMT_DXVA2_VLD: AVPixelFormat = 53;
pub const AV_PIX_FMT_YUV444P16BE: AVPixelFormat = 52;
pub const AV_PIX_FMT_YUV444P16LE: AVPixelFormat = 51;
pub const AV_PIX_FMT_YUV422P16BE: AVPixelFormat = 50;
pub const AV_PIX_FMT_YUV422P16LE: AVPixelFormat = 49;
pub const AV_PIX_FMT_YUV420P16BE: AVPixelFormat = 48;
pub const AV_PIX_FMT_YUV420P16LE: AVPixelFormat = 47;
pub const AV_PIX_FMT_VAAPI: AVPixelFormat = 46;
pub const AV_PIX_FMT_VAAPI_VLD: AVPixelFormat = 46;
pub const AV_PIX_FMT_VAAPI_IDCT: AVPixelFormat = 45;
pub const AV_PIX_FMT_VAAPI_MOCO: AVPixelFormat = 44;
pub const AV_PIX_FMT_BGR555LE: AVPixelFormat = 43;
pub const AV_PIX_FMT_BGR555BE: AVPixelFormat = 42;
pub const AV_PIX_FMT_BGR565LE: AVPixelFormat = 41;
pub const AV_PIX_FMT_BGR565BE: AVPixelFormat = 40;
pub const AV_PIX_FMT_RGB555LE: AVPixelFormat = 39;
pub const AV_PIX_FMT_RGB555BE: AVPixelFormat = 38;
pub const AV_PIX_FMT_RGB565LE: AVPixelFormat = 37;
pub const AV_PIX_FMT_RGB565BE: AVPixelFormat = 36;
pub const AV_PIX_FMT_RGB48LE: AVPixelFormat = 35;
pub const AV_PIX_FMT_RGB48BE: AVPixelFormat = 34;
pub const AV_PIX_FMT_YUVA420P: AVPixelFormat = 33;
pub const AV_PIX_FMT_YUVJ440P: AVPixelFormat = 32;
pub const AV_PIX_FMT_YUV440P: AVPixelFormat = 31;
pub const AV_PIX_FMT_GRAY16LE: AVPixelFormat = 30;
pub const AV_PIX_FMT_GRAY16BE: AVPixelFormat = 29;
pub const AV_PIX_FMT_BGRA: AVPixelFormat = 28;
pub const AV_PIX_FMT_ABGR: AVPixelFormat = 27;
pub const AV_PIX_FMT_RGBA: AVPixelFormat = 26;
pub const AV_PIX_FMT_ARGB: AVPixelFormat = 25;
pub const AV_PIX_FMT_NV21: AVPixelFormat = 24;
pub const AV_PIX_FMT_NV12: AVPixelFormat = 23;
pub const AV_PIX_FMT_RGB4_BYTE: AVPixelFormat = 22;
pub const AV_PIX_FMT_RGB4: AVPixelFormat = 21;
pub const AV_PIX_FMT_RGB8: AVPixelFormat = 20;
pub const AV_PIX_FMT_BGR4_BYTE: AVPixelFormat = 19;
pub const AV_PIX_FMT_BGR4: AVPixelFormat = 18;
pub const AV_PIX_FMT_BGR8: AVPixelFormat = 17;
pub const AV_PIX_FMT_UYYVYY411: AVPixelFormat = 16;
pub const AV_PIX_FMT_UYVY422: AVPixelFormat = 15;
pub const AV_PIX_FMT_YUVJ444P: AVPixelFormat = 14;
pub const AV_PIX_FMT_YUVJ422P: AVPixelFormat = 13;
pub const AV_PIX_FMT_YUVJ420P: AVPixelFormat = 12;
pub const AV_PIX_FMT_PAL8: AVPixelFormat = 11;
pub const AV_PIX_FMT_MONOBLACK: AVPixelFormat = 10;
pub const AV_PIX_FMT_MONOWHITE: AVPixelFormat = 9;
pub const AV_PIX_FMT_GRAY8: AVPixelFormat = 8;
pub const AV_PIX_FMT_YUV411P: AVPixelFormat = 7;
pub const AV_PIX_FMT_YUV410P: AVPixelFormat = 6;
pub const AV_PIX_FMT_YUV444P: AVPixelFormat = 5;
pub const AV_PIX_FMT_YUV422P: AVPixelFormat = 4;
pub const AV_PIX_FMT_BGR24: AVPixelFormat = 3;
pub const AV_PIX_FMT_RGB24: AVPixelFormat = 2;
pub const AV_PIX_FMT_YUYV422: AVPixelFormat = 1;
pub const AV_PIX_FMT_YUV420P: AVPixelFormat = 0;
pub const AV_PIX_FMT_NONE: AVPixelFormat = -1;
pub type AVColorPrimaries = libc::c_uint;
pub const AVCOL_PRI_NB: AVColorPrimaries = 23;
pub const AVCOL_PRI_JEDEC_P22: AVColorPrimaries = 22;
pub const AVCOL_PRI_EBU3213: AVColorPrimaries = 22;
pub const AVCOL_PRI_SMPTE432: AVColorPrimaries = 12;
pub const AVCOL_PRI_SMPTE431: AVColorPrimaries = 11;
pub const AVCOL_PRI_SMPTEST428_1: AVColorPrimaries = 10;
pub const AVCOL_PRI_SMPTE428: AVColorPrimaries = 10;
pub const AVCOL_PRI_BT2020: AVColorPrimaries = 9;
pub const AVCOL_PRI_FILM: AVColorPrimaries = 8;
pub const AVCOL_PRI_SMPTE240M: AVColorPrimaries = 7;
pub const AVCOL_PRI_SMPTE170M: AVColorPrimaries = 6;
pub const AVCOL_PRI_BT470BG: AVColorPrimaries = 5;
pub const AVCOL_PRI_BT470M: AVColorPrimaries = 4;
pub const AVCOL_PRI_RESERVED: AVColorPrimaries = 3;
pub const AVCOL_PRI_UNSPECIFIED: AVColorPrimaries = 2;
pub const AVCOL_PRI_BT709: AVColorPrimaries = 1;
pub const AVCOL_PRI_RESERVED0: AVColorPrimaries = 0;
pub type AVColorTransferCharacteristic = libc::c_uint;
pub const AVCOL_TRC_NB: AVColorTransferCharacteristic = 19;
pub const AVCOL_TRC_ARIB_STD_B67: AVColorTransferCharacteristic = 18;
pub const AVCOL_TRC_SMPTEST428_1: AVColorTransferCharacteristic = 17;
pub const AVCOL_TRC_SMPTE428: AVColorTransferCharacteristic = 17;
pub const AVCOL_TRC_SMPTEST2084: AVColorTransferCharacteristic = 16;
pub const AVCOL_TRC_SMPTE2084: AVColorTransferCharacteristic = 16;
pub const AVCOL_TRC_BT2020_12: AVColorTransferCharacteristic = 15;
pub const AVCOL_TRC_BT2020_10: AVColorTransferCharacteristic = 14;
pub const AVCOL_TRC_IEC61966_2_1: AVColorTransferCharacteristic = 13;
pub const AVCOL_TRC_BT1361_ECG: AVColorTransferCharacteristic = 12;
pub const AVCOL_TRC_IEC61966_2_4: AVColorTransferCharacteristic = 11;
pub const AVCOL_TRC_LOG_SQRT: AVColorTransferCharacteristic = 10;
pub const AVCOL_TRC_LOG: AVColorTransferCharacteristic = 9;
pub const AVCOL_TRC_LINEAR: AVColorTransferCharacteristic = 8;
pub const AVCOL_TRC_SMPTE240M: AVColorTransferCharacteristic = 7;
pub const AVCOL_TRC_SMPTE170M: AVColorTransferCharacteristic = 6;
pub const AVCOL_TRC_GAMMA28: AVColorTransferCharacteristic = 5;
pub const AVCOL_TRC_GAMMA22: AVColorTransferCharacteristic = 4;
pub const AVCOL_TRC_RESERVED: AVColorTransferCharacteristic = 3;
pub const AVCOL_TRC_UNSPECIFIED: AVColorTransferCharacteristic = 2;
pub const AVCOL_TRC_BT709: AVColorTransferCharacteristic = 1;
pub const AVCOL_TRC_RESERVED0: AVColorTransferCharacteristic = 0;
pub type AVColorSpace = libc::c_uint;
pub const AVCOL_SPC_NB: AVColorSpace = 15;
pub const AVCOL_SPC_ICTCP: AVColorSpace = 14;
pub const AVCOL_SPC_CHROMA_DERIVED_CL: AVColorSpace = 13;
pub const AVCOL_SPC_CHROMA_DERIVED_NCL: AVColorSpace = 12;
pub const AVCOL_SPC_SMPTE2085: AVColorSpace = 11;
pub const AVCOL_SPC_BT2020_CL: AVColorSpace = 10;
pub const AVCOL_SPC_BT2020_NCL: AVColorSpace = 9;
pub const AVCOL_SPC_YCOCG: AVColorSpace = 8;
pub const AVCOL_SPC_YCGCO: AVColorSpace = 8;
pub const AVCOL_SPC_SMPTE240M: AVColorSpace = 7;
pub const AVCOL_SPC_SMPTE170M: AVColorSpace = 6;
pub const AVCOL_SPC_BT470BG: AVColorSpace = 5;
pub const AVCOL_SPC_FCC: AVColorSpace = 4;
pub const AVCOL_SPC_RESERVED: AVColorSpace = 3;
pub const AVCOL_SPC_UNSPECIFIED: AVColorSpace = 2;
pub const AVCOL_SPC_BT709: AVColorSpace = 1;
pub const AVCOL_SPC_RGB: AVColorSpace = 0;
pub type AVColorRange = libc::c_uint;
pub const AVCOL_RANGE_NB: AVColorRange = 3;
pub const AVCOL_RANGE_JPEG: AVColorRange = 2;
pub const AVCOL_RANGE_MPEG: AVColorRange = 1;
pub const AVCOL_RANGE_UNSPECIFIED: AVColorRange = 0;
pub type AVChromaLocation = libc::c_uint;
pub const AVCHROMA_LOC_NB: AVChromaLocation = 7;
pub const AVCHROMA_LOC_BOTTOM: AVChromaLocation = 6;
pub const AVCHROMA_LOC_BOTTOMLEFT: AVChromaLocation = 5;
pub const AVCHROMA_LOC_TOP: AVChromaLocation = 4;
pub const AVCHROMA_LOC_TOPLEFT: AVChromaLocation = 3;
pub const AVCHROMA_LOC_CENTER: AVChromaLocation = 2;
pub const AVCHROMA_LOC_LEFT: AVChromaLocation = 1;
pub const AVCHROMA_LOC_UNSPECIFIED: AVChromaLocation = 0;
pub type AVSampleFormat = libc::c_int;
pub const AV_SAMPLE_FMT_NB: AVSampleFormat = 12;
pub const AV_SAMPLE_FMT_S64P: AVSampleFormat = 11;
pub const AV_SAMPLE_FMT_S64: AVSampleFormat = 10;
pub const AV_SAMPLE_FMT_DBLP: AVSampleFormat = 9;
pub const AV_SAMPLE_FMT_FLTP: AVSampleFormat = 8;
pub const AV_SAMPLE_FMT_S32P: AVSampleFormat = 7;
pub const AV_SAMPLE_FMT_S16P: AVSampleFormat = 6;
pub const AV_SAMPLE_FMT_U8P: AVSampleFormat = 5;
pub const AV_SAMPLE_FMT_DBL: AVSampleFormat = 4;
pub const AV_SAMPLE_FMT_FLT: AVSampleFormat = 3;
pub const AV_SAMPLE_FMT_S32: AVSampleFormat = 2;
pub const AV_SAMPLE_FMT_S16: AVSampleFormat = 1;
pub const AV_SAMPLE_FMT_U8: AVSampleFormat = 0;
pub const AV_SAMPLE_FMT_NONE: AVSampleFormat = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVBufferRef {
    pub buffer: *mut AVBuffer,
    pub data: *mut uint8_t,
    pub size: libc::c_int,
}
pub type AVFrameSideDataType = libc::c_uint;
pub const AV_FRAME_DATA_FILM_GRAIN_PARAMS: AVFrameSideDataType = 23;
pub const AV_FRAME_DATA_SEI_UNREGISTERED: AVFrameSideDataType = 22;
pub const AV_FRAME_DATA_VIDEO_ENC_PARAMS: AVFrameSideDataType = 21;
pub const AV_FRAME_DATA_REGIONS_OF_INTEREST: AVFrameSideDataType = 20;
pub const AV_FRAME_DATA_DYNAMIC_HDR_PLUS: AVFrameSideDataType = 19;
pub const AV_FRAME_DATA_S12M_TIMECODE: AVFrameSideDataType = 18;
pub const AV_FRAME_DATA_QP_TABLE_DATA: AVFrameSideDataType = 17;
pub const AV_FRAME_DATA_QP_TABLE_PROPERTIES: AVFrameSideDataType = 16;
pub const AV_FRAME_DATA_ICC_PROFILE: AVFrameSideDataType = 15;
pub const AV_FRAME_DATA_CONTENT_LIGHT_LEVEL: AVFrameSideDataType = 14;
pub const AV_FRAME_DATA_SPHERICAL: AVFrameSideDataType = 13;
pub const AV_FRAME_DATA_GOP_TIMECODE: AVFrameSideDataType = 12;
pub const AV_FRAME_DATA_MASTERING_DISPLAY_METADATA: AVFrameSideDataType = 11;
pub const AV_FRAME_DATA_AUDIO_SERVICE_TYPE: AVFrameSideDataType = 10;
pub const AV_FRAME_DATA_SKIP_SAMPLES: AVFrameSideDataType = 9;
pub const AV_FRAME_DATA_MOTION_VECTORS: AVFrameSideDataType = 8;
pub const AV_FRAME_DATA_AFD: AVFrameSideDataType = 7;
pub const AV_FRAME_DATA_DISPLAYMATRIX: AVFrameSideDataType = 6;
pub const AV_FRAME_DATA_REPLAYGAIN: AVFrameSideDataType = 5;
pub const AV_FRAME_DATA_DOWNMIX_INFO: AVFrameSideDataType = 4;
pub const AV_FRAME_DATA_MATRIXENCODING: AVFrameSideDataType = 3;
pub const AV_FRAME_DATA_STEREO3D: AVFrameSideDataType = 2;
pub const AV_FRAME_DATA_A53_CC: AVFrameSideDataType = 1;
pub const AV_FRAME_DATA_PANSCAN: AVFrameSideDataType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFrameSideData {
    pub type_0: AVFrameSideDataType,
    pub data: *mut uint8_t,
    pub size: libc::c_int,
    pub metadata: *mut AVDictionary,
    pub buf: *mut AVBufferRef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFrame {
    pub data: [*mut uint8_t; 8],
    pub linesize: [libc::c_int; 8],
    pub extended_data: *mut *mut uint8_t,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub nb_samples: libc::c_int,
    pub format: libc::c_int,
    pub key_frame: libc::c_int,
    pub pict_type: AVPictureType,
    pub sample_aspect_ratio: AVRational,
    pub pts: int64_t,
    pub pkt_pts: int64_t,
    pub pkt_dts: int64_t,
    pub coded_picture_number: libc::c_int,
    pub display_picture_number: libc::c_int,
    pub quality: libc::c_int,
    pub opaque: *mut libc::c_void,
    pub error: [uint64_t; 8],
    pub repeat_pict: libc::c_int,
    pub interlaced_frame: libc::c_int,
    pub top_field_first: libc::c_int,
    pub palette_has_changed: libc::c_int,
    pub reordered_opaque: int64_t,
    pub sample_rate: libc::c_int,
    pub channel_layout: uint64_t,
    pub buf: [*mut AVBufferRef; 8],
    pub extended_buf: *mut *mut AVBufferRef,
    pub nb_extended_buf: libc::c_int,
    pub side_data: *mut *mut AVFrameSideData,
    pub nb_side_data: libc::c_int,
    pub flags: libc::c_int,
    pub color_range: AVColorRange,
    pub color_primaries: AVColorPrimaries,
    pub color_trc: AVColorTransferCharacteristic,
    pub colorspace: AVColorSpace,
    pub chroma_location: AVChromaLocation,
    pub best_effort_timestamp: int64_t,
    pub pkt_pos: int64_t,
    pub pkt_duration: int64_t,
    pub metadata: *mut AVDictionary,
    pub decode_error_flags: libc::c_int,
    pub channels: libc::c_int,
    pub pkt_size: libc::c_int,
    pub qscale_table: *mut int8_t,
    pub qstride: libc::c_int,
    pub qscale_type: libc::c_int,
    pub qp_table_buf: *mut AVBufferRef,
    pub hw_frames_ctx: *mut AVBufferRef,
    pub opaque_ref: *mut AVBufferRef,
    pub crop_top: size_t,
    pub crop_bottom: size_t,
    pub crop_left: size_t,
    pub crop_right: size_t,
    pub private_ref: *mut AVBufferRef,
}
pub type AVCodecID = libc::c_uint;
pub const AV_CODEC_ID_WRAPPED_AVFRAME: AVCodecID = 135169;
pub const AV_CODEC_ID_FFMETADATA: AVCodecID = 135168;
pub const AV_CODEC_ID_MPEG4SYSTEMS: AVCodecID = 131073;
pub const AV_CODEC_ID_MPEG2TS: AVCodecID = 131072;
pub const AV_CODEC_ID_PROBE: AVCodecID = 102400;
pub const AV_CODEC_ID_BIN_DATA: AVCodecID = 100359;
pub const AV_CODEC_ID_TIMED_ID3: AVCodecID = 100358;
pub const AV_CODEC_ID_DVD_NAV: AVCodecID = 100357;
pub const AV_CODEC_ID_SMPTE_KLV: AVCodecID = 100356;
pub const AV_CODEC_ID_OTF: AVCodecID = 100355;
pub const AV_CODEC_ID_IDF: AVCodecID = 100354;
pub const AV_CODEC_ID_XBIN: AVCodecID = 100353;
pub const AV_CODEC_ID_BINTEXT: AVCodecID = 100352;
pub const AV_CODEC_ID_EPG: AVCodecID = 98306;
pub const AV_CODEC_ID_SCTE_35: AVCodecID = 98305;
pub const AV_CODEC_ID_TTF: AVCodecID = 98304;
pub const AV_CODEC_ID_FIRST_UNKNOWN: AVCodecID = 98304;
pub const AV_CODEC_ID_ARIB_CAPTION: AVCodecID = 96272;
pub const AV_CODEC_ID_TTML: AVCodecID = 96271;
pub const AV_CODEC_ID_HDMV_TEXT_SUBTITLE: AVCodecID = 96270;
pub const AV_CODEC_ID_ASS: AVCodecID = 96269;
pub const AV_CODEC_ID_PJS: AVCodecID = 96268;
pub const AV_CODEC_ID_VPLAYER: AVCodecID = 96267;
pub const AV_CODEC_ID_MPL2: AVCodecID = 96266;
pub const AV_CODEC_ID_WEBVTT: AVCodecID = 96265;
pub const AV_CODEC_ID_SUBRIP: AVCodecID = 96264;
pub const AV_CODEC_ID_SUBVIEWER: AVCodecID = 96263;
pub const AV_CODEC_ID_SUBVIEWER1: AVCodecID = 96262;
pub const AV_CODEC_ID_STL: AVCodecID = 96261;
pub const AV_CODEC_ID_REALTEXT: AVCodecID = 96260;
pub const AV_CODEC_ID_SAMI: AVCodecID = 96259;
pub const AV_CODEC_ID_JACOSUB: AVCodecID = 96258;
pub const AV_CODEC_ID_EIA_608: AVCodecID = 96257;
pub const AV_CODEC_ID_MICRODVD: AVCodecID = 96256;
pub const AV_CODEC_ID_SRT: AVCodecID = 94216;
pub const AV_CODEC_ID_DVB_TELETEXT: AVCodecID = 94215;
pub const AV_CODEC_ID_HDMV_PGS_SUBTITLE: AVCodecID = 94214;
pub const AV_CODEC_ID_MOV_TEXT: AVCodecID = 94213;
pub const AV_CODEC_ID_SSA: AVCodecID = 94212;
pub const AV_CODEC_ID_XSUB: AVCodecID = 94211;
pub const AV_CODEC_ID_TEXT: AVCodecID = 94210;
pub const AV_CODEC_ID_DVB_SUBTITLE: AVCodecID = 94209;
pub const AV_CODEC_ID_DVD_SUBTITLE: AVCodecID = 94208;
pub const AV_CODEC_ID_FIRST_SUBTITLE: AVCodecID = 94208;
pub const AV_CODEC_ID_FASTAUDIO: AVCodecID = 88090;
pub const AV_CODEC_ID_HCA: AVCodecID = 88089;
pub const AV_CODEC_ID_SIREN: AVCodecID = 88088;
pub const AV_CODEC_ID_MPEGH_3D_AUDIO: AVCodecID = 88087;
pub const AV_CODEC_ID_ACELP_KELVIN: AVCodecID = 88086;
pub const AV_CODEC_ID_HCOM: AVCodecID = 88085;
pub const AV_CODEC_ID_ATRAC9: AVCodecID = 88084;
pub const AV_CODEC_ID_SBC: AVCodecID = 88083;
pub const AV_CODEC_ID_APTX_HD: AVCodecID = 88082;
pub const AV_CODEC_ID_APTX: AVCodecID = 88081;
pub const AV_CODEC_ID_DOLBY_E: AVCodecID = 88080;
pub const AV_CODEC_ID_ATRAC3PAL: AVCodecID = 88079;
pub const AV_CODEC_ID_ATRAC3AL: AVCodecID = 88078;
pub const AV_CODEC_ID_DST: AVCodecID = 88077;
pub const AV_CODEC_ID_XMA2: AVCodecID = 88076;
pub const AV_CODEC_ID_XMA1: AVCodecID = 88075;
pub const AV_CODEC_ID_INTERPLAY_ACM: AVCodecID = 88074;
pub const AV_CODEC_ID_4GV: AVCodecID = 88073;
pub const AV_CODEC_ID_DSD_MSBF_PLANAR: AVCodecID = 88072;
pub const AV_CODEC_ID_DSD_LSBF_PLANAR: AVCodecID = 88071;
pub const AV_CODEC_ID_DSD_MSBF: AVCodecID = 88070;
pub const AV_CODEC_ID_DSD_LSBF: AVCodecID = 88069;
pub const AV_CODEC_ID_SMV: AVCodecID = 88068;
pub const AV_CODEC_ID_EVRC: AVCodecID = 88067;
pub const AV_CODEC_ID_SONIC_LS: AVCodecID = 88066;
pub const AV_CODEC_ID_SONIC: AVCodecID = 88065;
pub const AV_CODEC_ID_FFWAVESYNTH: AVCodecID = 88064;
pub const AV_CODEC_ID_CODEC2: AVCodecID = 86083;
pub const AV_CODEC_ID_DSS_SP: AVCodecID = 86082;
pub const AV_CODEC_ID_ON2AVC: AVCodecID = 86081;
pub const AV_CODEC_ID_PAF_AUDIO: AVCodecID = 86080;
pub const AV_CODEC_ID_METASOUND: AVCodecID = 86079;
pub const AV_CODEC_ID_TAK: AVCodecID = 86078;
pub const AV_CODEC_ID_COMFORT_NOISE: AVCodecID = 86077;
pub const AV_CODEC_ID_OPUS: AVCodecID = 86076;
pub const AV_CODEC_ID_ILBC: AVCodecID = 86075;
pub const AV_CODEC_ID_IAC: AVCodecID = 86074;
pub const AV_CODEC_ID_RALF: AVCodecID = 86073;
pub const AV_CODEC_ID_BMV_AUDIO: AVCodecID = 86072;
pub const AV_CODEC_ID_8SVX_FIB: AVCodecID = 86071;
pub const AV_CODEC_ID_8SVX_EXP: AVCodecID = 86070;
pub const AV_CODEC_ID_G729: AVCodecID = 86069;
pub const AV_CODEC_ID_G723_1: AVCodecID = 86068;
pub const AV_CODEC_ID_CELT: AVCodecID = 86067;
pub const AV_CODEC_ID_QDMC: AVCodecID = 86066;
pub const AV_CODEC_ID_AAC_LATM: AVCodecID = 86065;
pub const AV_CODEC_ID_BINKAUDIO_DCT: AVCodecID = 86064;
pub const AV_CODEC_ID_BINKAUDIO_RDFT: AVCodecID = 86063;
pub const AV_CODEC_ID_ATRAC1: AVCodecID = 86062;
pub const AV_CODEC_ID_MP4ALS: AVCodecID = 86061;
pub const AV_CODEC_ID_TRUEHD: AVCodecID = 86060;
pub const AV_CODEC_ID_TWINVQ: AVCodecID = 86059;
pub const AV_CODEC_ID_MP1: AVCodecID = 86058;
pub const AV_CODEC_ID_SIPR: AVCodecID = 86057;
pub const AV_CODEC_ID_EAC3: AVCodecID = 86056;
pub const AV_CODEC_ID_ATRAC3P: AVCodecID = 86055;
pub const AV_CODEC_ID_WMALOSSLESS: AVCodecID = 86054;
pub const AV_CODEC_ID_WMAPRO: AVCodecID = 86053;
pub const AV_CODEC_ID_WMAVOICE: AVCodecID = 86052;
pub const AV_CODEC_ID_SPEEX: AVCodecID = 86051;
pub const AV_CODEC_ID_MUSEPACK8: AVCodecID = 86050;
pub const AV_CODEC_ID_NELLYMOSER: AVCodecID = 86049;
pub const AV_CODEC_ID_APE: AVCodecID = 86048;
pub const AV_CODEC_ID_ATRAC3: AVCodecID = 86047;
pub const AV_CODEC_ID_GSM_MS: AVCodecID = 86046;
pub const AV_CODEC_ID_MLP: AVCodecID = 86045;
pub const AV_CODEC_ID_MUSEPACK7: AVCodecID = 86044;
pub const AV_CODEC_ID_IMC: AVCodecID = 86043;
pub const AV_CODEC_ID_DSICINAUDIO: AVCodecID = 86042;
pub const AV_CODEC_ID_WAVPACK: AVCodecID = 86041;
pub const AV_CODEC_ID_QCELP: AVCodecID = 86040;
pub const AV_CODEC_ID_SMACKAUDIO: AVCodecID = 86039;
pub const AV_CODEC_ID_TTA: AVCodecID = 86038;
pub const AV_CODEC_ID_TRUESPEECH: AVCodecID = 86037;
pub const AV_CODEC_ID_COOK: AVCodecID = 86036;
pub const AV_CODEC_ID_QDM2: AVCodecID = 86035;
pub const AV_CODEC_ID_GSM: AVCodecID = 86034;
pub const AV_CODEC_ID_WESTWOOD_SND1: AVCodecID = 86033;
pub const AV_CODEC_ID_ALAC: AVCodecID = 86032;
pub const AV_CODEC_ID_SHORTEN: AVCodecID = 86031;
pub const AV_CODEC_ID_MP3ON4: AVCodecID = 86030;
pub const AV_CODEC_ID_MP3ADU: AVCodecID = 86029;
pub const AV_CODEC_ID_FLAC: AVCodecID = 86028;
pub const AV_CODEC_ID_VMDAUDIO: AVCodecID = 86027;
pub const AV_CODEC_ID_MACE6: AVCodecID = 86026;
pub const AV_CODEC_ID_MACE3: AVCodecID = 86025;
pub const AV_CODEC_ID_WMAV2: AVCodecID = 86024;
pub const AV_CODEC_ID_WMAV1: AVCodecID = 86023;
pub const AV_CODEC_ID_DVAUDIO: AVCodecID = 86022;
pub const AV_CODEC_ID_VORBIS: AVCodecID = 86021;
pub const AV_CODEC_ID_DTS: AVCodecID = 86020;
pub const AV_CODEC_ID_AC3: AVCodecID = 86019;
pub const AV_CODEC_ID_AAC: AVCodecID = 86018;
pub const AV_CODEC_ID_MP3: AVCodecID = 86017;
pub const AV_CODEC_ID_MP2: AVCodecID = 86016;
pub const AV_CODEC_ID_DERF_DPCM: AVCodecID = 83970;
pub const AV_CODEC_ID_GREMLIN_DPCM: AVCodecID = 83969;
pub const AV_CODEC_ID_SDX2_DPCM: AVCodecID = 83968;
pub const AV_CODEC_ID_SOL_DPCM: AVCodecID = 81923;
pub const AV_CODEC_ID_XAN_DPCM: AVCodecID = 81922;
pub const AV_CODEC_ID_INTERPLAY_DPCM: AVCodecID = 81921;
pub const AV_CODEC_ID_ROQ_DPCM: AVCodecID = 81920;
pub const AV_CODEC_ID_RA_288: AVCodecID = 77825;
pub const AV_CODEC_ID_RA_144: AVCodecID = 77824;
pub const AV_CODEC_ID_AMR_WB: AVCodecID = 73729;
pub const AV_CODEC_ID_AMR_NB: AVCodecID = 73728;
pub const AV_CODEC_ID_ADPCM_IMA_MOFLEX: AVCodecID = 71698;
pub const AV_CODEC_ID_ADPCM_IMA_CUNNING: AVCodecID = 71697;
pub const AV_CODEC_ID_ADPCM_IMA_MTF: AVCodecID = 71696;
pub const AV_CODEC_ID_ADPCM_IMA_ALP: AVCodecID = 71695;
pub const AV_CODEC_ID_ADPCM_IMA_APM: AVCodecID = 71694;
pub const AV_CODEC_ID_ADPCM_ZORK: AVCodecID = 71693;
pub const AV_CODEC_ID_ADPCM_IMA_SSI: AVCodecID = 71692;
pub const AV_CODEC_ID_ADPCM_ARGO: AVCodecID = 71691;
pub const AV_CODEC_ID_ADPCM_AGM: AVCodecID = 71690;
pub const AV_CODEC_ID_ADPCM_MTAF: AVCodecID = 71689;
pub const AV_CODEC_ID_ADPCM_IMA_DAT4: AVCodecID = 71688;
pub const AV_CODEC_ID_ADPCM_AICA: AVCodecID = 71687;
pub const AV_CODEC_ID_ADPCM_PSX: AVCodecID = 71686;
pub const AV_CODEC_ID_ADPCM_THP_LE: AVCodecID = 71685;
pub const AV_CODEC_ID_ADPCM_G726LE: AVCodecID = 71684;
pub const AV_CODEC_ID_ADPCM_IMA_RAD: AVCodecID = 71683;
pub const AV_CODEC_ID_ADPCM_DTK: AVCodecID = 71682;
pub const AV_CODEC_ID_ADPCM_IMA_OKI: AVCodecID = 71681;
pub const AV_CODEC_ID_ADPCM_AFC: AVCodecID = 71680;
pub const AV_CODEC_ID_ADPCM_VIMA: AVCodecID = 69662;
pub const AV_CODEC_ID_ADPCM_IMA_APC: AVCodecID = 69661;
pub const AV_CODEC_ID_ADPCM_G722: AVCodecID = 69660;
pub const AV_CODEC_ID_ADPCM_IMA_ISS: AVCodecID = 69659;
pub const AV_CODEC_ID_ADPCM_EA_MAXIS_XA: AVCodecID = 69658;
pub const AV_CODEC_ID_ADPCM_EA_XAS: AVCodecID = 69657;
pub const AV_CODEC_ID_ADPCM_IMA_EA_EACS: AVCodecID = 69656;
pub const AV_CODEC_ID_ADPCM_IMA_EA_SEAD: AVCodecID = 69655;
pub const AV_CODEC_ID_ADPCM_EA_R2: AVCodecID = 69654;
pub const AV_CODEC_ID_ADPCM_EA_R3: AVCodecID = 69653;
pub const AV_CODEC_ID_ADPCM_EA_R1: AVCodecID = 69652;
pub const AV_CODEC_ID_ADPCM_IMA_AMV: AVCodecID = 69651;
pub const AV_CODEC_ID_ADPCM_THP: AVCodecID = 69650;
pub const AV_CODEC_ID_ADPCM_SBPRO_2: AVCodecID = 69649;
pub const AV_CODEC_ID_ADPCM_SBPRO_3: AVCodecID = 69648;
pub const AV_CODEC_ID_ADPCM_SBPRO_4: AVCodecID = 69647;
pub const AV_CODEC_ID_ADPCM_YAMAHA: AVCodecID = 69646;
pub const AV_CODEC_ID_ADPCM_SWF: AVCodecID = 69645;
pub const AV_CODEC_ID_ADPCM_CT: AVCodecID = 69644;
pub const AV_CODEC_ID_ADPCM_G726: AVCodecID = 69643;
pub const AV_CODEC_ID_ADPCM_EA: AVCodecID = 69642;
pub const AV_CODEC_ID_ADPCM_ADX: AVCodecID = 69641;
pub const AV_CODEC_ID_ADPCM_XA: AVCodecID = 69640;
pub const AV_CODEC_ID_ADPCM_4XM: AVCodecID = 69639;
pub const AV_CODEC_ID_ADPCM_MS: AVCodecID = 69638;
pub const AV_CODEC_ID_ADPCM_IMA_SMJPEG: AVCodecID = 69637;
pub const AV_CODEC_ID_ADPCM_IMA_WS: AVCodecID = 69636;
pub const AV_CODEC_ID_ADPCM_IMA_DK4: AVCodecID = 69635;
pub const AV_CODEC_ID_ADPCM_IMA_DK3: AVCodecID = 69634;
pub const AV_CODEC_ID_ADPCM_IMA_WAV: AVCodecID = 69633;
pub const AV_CODEC_ID_ADPCM_IMA_QT: AVCodecID = 69632;
pub const AV_CODEC_ID_PCM_SGA: AVCodecID = 67589;
pub const AV_CODEC_ID_PCM_VIDC: AVCodecID = 67588;
pub const AV_CODEC_ID_PCM_F24LE: AVCodecID = 67587;
pub const AV_CODEC_ID_PCM_F16LE: AVCodecID = 67586;
pub const AV_CODEC_ID_PCM_S64BE: AVCodecID = 67585;
pub const AV_CODEC_ID_PCM_S64LE: AVCodecID = 67584;
pub const AV_CODEC_ID_PCM_S16BE_PLANAR: AVCodecID = 65566;
pub const AV_CODEC_ID_PCM_S32LE_PLANAR: AVCodecID = 65565;
pub const AV_CODEC_ID_PCM_S24LE_PLANAR: AVCodecID = 65564;
pub const AV_CODEC_ID_PCM_S8_PLANAR: AVCodecID = 65563;
pub const AV_CODEC_ID_S302M: AVCodecID = 65562;
pub const AV_CODEC_ID_PCM_LXF: AVCodecID = 65561;
pub const AV_CODEC_ID_PCM_BLURAY: AVCodecID = 65560;
pub const AV_CODEC_ID_PCM_F64LE: AVCodecID = 65559;
pub const AV_CODEC_ID_PCM_F64BE: AVCodecID = 65558;
pub const AV_CODEC_ID_PCM_F32LE: AVCodecID = 65557;
pub const AV_CODEC_ID_PCM_F32BE: AVCodecID = 65556;
pub const AV_CODEC_ID_PCM_DVD: AVCodecID = 65555;
pub const AV_CODEC_ID_PCM_S16LE_PLANAR: AVCodecID = 65554;
pub const AV_CODEC_ID_PCM_ZORK: AVCodecID = 65553;
pub const AV_CODEC_ID_PCM_S24DAUD: AVCodecID = 65552;
pub const AV_CODEC_ID_PCM_U24BE: AVCodecID = 65551;
pub const AV_CODEC_ID_PCM_U24LE: AVCodecID = 65550;
pub const AV_CODEC_ID_PCM_S24BE: AVCodecID = 65549;
pub const AV_CODEC_ID_PCM_S24LE: AVCodecID = 65548;
pub const AV_CODEC_ID_PCM_U32BE: AVCodecID = 65547;
pub const AV_CODEC_ID_PCM_U32LE: AVCodecID = 65546;
pub const AV_CODEC_ID_PCM_S32BE: AVCodecID = 65545;
pub const AV_CODEC_ID_PCM_S32LE: AVCodecID = 65544;
pub const AV_CODEC_ID_PCM_ALAW: AVCodecID = 65543;
pub const AV_CODEC_ID_PCM_MULAW: AVCodecID = 65542;
pub const AV_CODEC_ID_PCM_U8: AVCodecID = 65541;
pub const AV_CODEC_ID_PCM_S8: AVCodecID = 65540;
pub const AV_CODEC_ID_PCM_U16BE: AVCodecID = 65539;
pub const AV_CODEC_ID_PCM_U16LE: AVCodecID = 65538;
pub const AV_CODEC_ID_PCM_S16BE: AVCodecID = 65537;
pub const AV_CODEC_ID_PCM_S16LE: AVCodecID = 65536;
pub const AV_CODEC_ID_FIRST_AUDIO: AVCodecID = 65536;
pub const AV_CODEC_ID_SGA_VIDEO: AVCodecID = 32827;
pub const AV_CODEC_ID_SIMBIOSIS_IMX: AVCodecID = 32826;
pub const AV_CODEC_ID_CRI: AVCodecID = 32825;
pub const AV_CODEC_ID_ARGO: AVCodecID = 32824;
pub const AV_CODEC_ID_IPU: AVCodecID = 32823;
pub const AV_CODEC_ID_PHOTOCD: AVCodecID = 32822;
pub const AV_CODEC_ID_MOBICLIP: AVCodecID = 32821;
pub const AV_CODEC_ID_PFM: AVCodecID = 32820;
pub const AV_CODEC_ID_NOTCHLC: AVCodecID = 32819;
pub const AV_CODEC_ID_MV30: AVCodecID = 32818;
pub const AV_CODEC_ID_CDTOONS: AVCodecID = 32817;
pub const AV_CODEC_ID_MVHA: AVCodecID = 32816;
pub const AV_CODEC_ID_MVDV: AVCodecID = 32815;
pub const AV_CODEC_ID_IMM5: AVCodecID = 32814;
pub const AV_CODEC_ID_VP4: AVCodecID = 32813;
pub const AV_CODEC_ID_LSCR: AVCodecID = 32812;
pub const AV_CODEC_ID_AGM: AVCodecID = 32811;
pub const AV_CODEC_ID_ARBC: AVCodecID = 32810;
pub const AV_CODEC_ID_HYMT: AVCodecID = 32809;
pub const AV_CODEC_ID_RASC: AVCodecID = 32808;
pub const AV_CODEC_ID_WCMV: AVCodecID = 32807;
pub const AV_CODEC_ID_MWSC: AVCodecID = 32806;
pub const AV_CODEC_ID_PROSUMER: AVCodecID = 32805;
pub const AV_CODEC_ID_IMM4: AVCodecID = 32804;
pub const AV_CODEC_ID_FITS: AVCodecID = 32803;
pub const AV_CODEC_ID_GDV: AVCodecID = 32802;
pub const AV_CODEC_ID_SVG: AVCodecID = 32801;
pub const AV_CODEC_ID_SRGC: AVCodecID = 32800;
pub const AV_CODEC_ID_MSCC: AVCodecID = 32799;
pub const AV_CODEC_ID_BITPACKED: AVCodecID = 32798;
pub const AV_CODEC_ID_AV1: AVCodecID = 32797;
pub const AV_CODEC_ID_XPM: AVCodecID = 32796;
pub const AV_CODEC_ID_CLEARVIDEO: AVCodecID = 32795;
pub const AV_CODEC_ID_SCPR: AVCodecID = 32794;
pub const AV_CODEC_ID_FMVC: AVCodecID = 32793;
pub const AV_CODEC_ID_SPEEDHQ: AVCodecID = 32792;
pub const AV_CODEC_ID_PIXLET: AVCodecID = 32791;
pub const AV_CODEC_ID_PSD: AVCodecID = 32790;
pub const AV_CODEC_ID_YLC: AVCodecID = 32789;
pub const AV_CODEC_ID_SHEERVIDEO: AVCodecID = 32788;
pub const AV_CODEC_ID_MAGICYUV: AVCodecID = 32787;
pub const AV_CODEC_ID_M101: AVCodecID = 32786;
pub const AV_CODEC_ID_TRUEMOTION2RT: AVCodecID = 32785;
pub const AV_CODEC_ID_CFHD: AVCodecID = 32784;
pub const AV_CODEC_ID_DAALA: AVCodecID = 32783;
pub const AV_CODEC_ID_APNG: AVCodecID = 32782;
pub const AV_CODEC_ID_SMVJPEG: AVCodecID = 32781;
pub const AV_CODEC_ID_SNOW: AVCodecID = 32780;
pub const AV_CODEC_ID_XFACE: AVCodecID = 32779;
pub const AV_CODEC_ID_CPIA: AVCodecID = 32778;
pub const AV_CODEC_ID_AVRN: AVCodecID = 32777;
pub const AV_CODEC_ID_YUV4: AVCodecID = 32776;
pub const AV_CODEC_ID_V408: AVCodecID = 32775;
pub const AV_CODEC_ID_V308: AVCodecID = 32774;
pub const AV_CODEC_ID_TARGA_Y216: AVCodecID = 32773;
pub const AV_CODEC_ID_AYUV: AVCodecID = 32772;
pub const AV_CODEC_ID_AVUI: AVCodecID = 32771;
pub const AV_CODEC_ID_012V: AVCodecID = 32770;
pub const AV_CODEC_ID_AVRP: AVCodecID = 32769;
pub const AV_CODEC_ID_Y41P: AVCodecID = 32768;
pub const AV_CODEC_ID_VVC: AVCodecID = 196;
pub const AV_CODEC_ID_MSP2: AVCodecID = 195;
pub const AV_CODEC_ID_AVS3: AVCodecID = 194;
pub const AV_CODEC_ID_PGX: AVCodecID = 193;
pub const AV_CODEC_ID_AVS2: AVCodecID = 192;
pub const AV_CODEC_ID_RSCC: AVCodecID = 191;
pub const AV_CODEC_ID_SCREENPRESSO: AVCodecID = 190;
pub const AV_CODEC_ID_DXV: AVCodecID = 189;
pub const AV_CODEC_ID_DDS: AVCodecID = 188;
pub const AV_CODEC_ID_HAP: AVCodecID = 187;
pub const AV_CODEC_ID_HQ_HQA: AVCodecID = 186;
pub const AV_CODEC_ID_TDSC: AVCodecID = 185;
pub const AV_CODEC_ID_HQX: AVCodecID = 184;
pub const AV_CODEC_ID_MVC2: AVCodecID = 183;
pub const AV_CODEC_ID_MVC1: AVCodecID = 182;
pub const AV_CODEC_ID_SGIRLE: AVCodecID = 181;
pub const AV_CODEC_ID_SANM: AVCodecID = 180;
pub const AV_CODEC_ID_VP7: AVCodecID = 179;
pub const AV_CODEC_ID_EXR: AVCodecID = 178;
pub const AV_CODEC_ID_PAF_VIDEO: AVCodecID = 177;
pub const AV_CODEC_ID_BRENDER_PIX: AVCodecID = 176;
pub const AV_CODEC_ID_ALIAS_PIX: AVCodecID = 175;
pub const AV_CODEC_ID_FIC: AVCodecID = 174;
pub const AV_CODEC_ID_HEVC: AVCodecID = 173;
pub const AV_CODEC_ID_HNM4_VIDEO: AVCodecID = 172;
pub const AV_CODEC_ID_WEBP: AVCodecID = 171;
pub const AV_CODEC_ID_G2M: AVCodecID = 170;
pub const AV_CODEC_ID_ESCAPE130: AVCodecID = 169;
pub const AV_CODEC_ID_AIC: AVCodecID = 168;
pub const AV_CODEC_ID_VP9: AVCodecID = 167;
pub const AV_CODEC_ID_MSS2: AVCodecID = 166;
pub const AV_CODEC_ID_CLLC: AVCodecID = 165;
pub const AV_CODEC_ID_MTS2: AVCodecID = 164;
pub const AV_CODEC_ID_TSCC2: AVCodecID = 163;
pub const AV_CODEC_ID_MSA1: AVCodecID = 162;
pub const AV_CODEC_ID_MSS1: AVCodecID = 161;
pub const AV_CODEC_ID_ZEROCODEC: AVCodecID = 160;
pub const AV_CODEC_ID_XBM: AVCodecID = 159;
pub const AV_CODEC_ID_CDXL: AVCodecID = 158;
pub const AV_CODEC_ID_XWD: AVCodecID = 157;
pub const AV_CODEC_ID_V410: AVCodecID = 156;
pub const AV_CODEC_ID_DXTORY: AVCodecID = 155;
pub const AV_CODEC_ID_VBLE: AVCodecID = 154;
pub const AV_CODEC_ID_BMV_VIDEO: AVCodecID = 153;
pub const AV_CODEC_ID_UTVIDEO: AVCodecID = 152;
pub const AV_CODEC_ID_VC1IMAGE: AVCodecID = 151;
pub const AV_CODEC_ID_WMV3IMAGE: AVCodecID = 150;
pub const AV_CODEC_ID_DFA: AVCodecID = 149;
pub const AV_CODEC_ID_JV: AVCodecID = 148;
pub const AV_CODEC_ID_PRORES: AVCodecID = 147;
pub const AV_CODEC_ID_LAGARITH: AVCodecID = 146;
pub const AV_CODEC_ID_MXPEG: AVCodecID = 145;
pub const AV_CODEC_ID_R10K: AVCodecID = 144;
pub const AV_CODEC_ID_A64_MULTI5: AVCodecID = 143;
pub const AV_CODEC_ID_A64_MULTI: AVCodecID = 142;
pub const AV_CODEC_ID_ANSI: AVCodecID = 141;
pub const AV_CODEC_ID_PICTOR: AVCodecID = 140;
pub const AV_CODEC_ID_VP8: AVCodecID = 139;
pub const AV_CODEC_ID_YOP: AVCodecID = 138;
pub const AV_CODEC_ID_KGV1: AVCodecID = 137;
pub const AV_CODEC_ID_IFF_ILBM: AVCodecID = 136;
pub const AV_CODEC_ID_BINKVIDEO: AVCodecID = 135;
pub const AV_CODEC_ID_ANM: AVCodecID = 134;
pub const AV_CODEC_ID_R210: AVCodecID = 133;
pub const AV_CODEC_ID_CDGRAPHICS: AVCodecID = 132;
pub const AV_CODEC_ID_FLASHSV2: AVCodecID = 131;
pub const AV_CODEC_ID_FRWU: AVCodecID = 130;
pub const AV_CODEC_ID_MAD: AVCodecID = 129;
pub const AV_CODEC_ID_DPX: AVCodecID = 128;
pub const AV_CODEC_ID_V210: AVCodecID = 127;
pub const AV_CODEC_ID_TMV: AVCodecID = 126;
pub const AV_CODEC_ID_V210X: AVCodecID = 125;
pub const AV_CODEC_ID_AURA2: AVCodecID = 124;
pub const AV_CODEC_ID_AURA: AVCodecID = 123;
pub const AV_CODEC_ID_TQI: AVCodecID = 122;
pub const AV_CODEC_ID_TGQ: AVCodecID = 121;
pub const AV_CODEC_ID_TGV: AVCodecID = 120;
pub const AV_CODEC_ID_MOTIONPIXELS: AVCodecID = 119;
pub const AV_CODEC_ID_CMV: AVCodecID = 118;
pub const AV_CODEC_ID_BFI: AVCodecID = 117;
pub const AV_CODEC_ID_DIRAC: AVCodecID = 116;
pub const AV_CODEC_ID_ESCAPE124: AVCodecID = 115;
pub const AV_CODEC_ID_RL2: AVCodecID = 114;
pub const AV_CODEC_ID_MIMIC: AVCodecID = 113;
pub const AV_CODEC_ID_INDEO5: AVCodecID = 112;
pub const AV_CODEC_ID_INDEO4: AVCodecID = 111;
pub const AV_CODEC_ID_SUNRAST: AVCodecID = 110;
pub const AV_CODEC_ID_PCX: AVCodecID = 109;
pub const AV_CODEC_ID_VB: AVCodecID = 108;
pub const AV_CODEC_ID_AMV: AVCodecID = 107;
pub const AV_CODEC_ID_VP6A: AVCodecID = 106;
pub const AV_CODEC_ID_TXD: AVCodecID = 105;
pub const AV_CODEC_ID_PTX: AVCodecID = 104;
pub const AV_CODEC_ID_BETHSOFTVID: AVCodecID = 103;
pub const AV_CODEC_ID_C93: AVCodecID = 102;
pub const AV_CODEC_ID_SGI: AVCodecID = 101;
pub const AV_CODEC_ID_THP: AVCodecID = 100;
pub const AV_CODEC_ID_DNXHD: AVCodecID = 99;
pub const AV_CODEC_ID_DXA: AVCodecID = 98;
pub const AV_CODEC_ID_GIF: AVCodecID = 97;
pub const AV_CODEC_ID_TIFF: AVCodecID = 96;
pub const AV_CODEC_ID_TIERTEXSEQVIDEO: AVCodecID = 95;
pub const AV_CODEC_ID_DSICINVIDEO: AVCodecID = 94;
pub const AV_CODEC_ID_TARGA: AVCodecID = 93;
pub const AV_CODEC_ID_VP6F: AVCodecID = 92;
pub const AV_CODEC_ID_VP6: AVCodecID = 91;
pub const AV_CODEC_ID_VP5: AVCodecID = 90;
pub const AV_CODEC_ID_VMNC: AVCodecID = 89;
pub const AV_CODEC_ID_JPEG2000: AVCodecID = 88;
pub const AV_CODEC_ID_CAVS: AVCodecID = 87;
pub const AV_CODEC_ID_FLASHSV: AVCodecID = 86;
pub const AV_CODEC_ID_KMVC: AVCodecID = 85;
pub const AV_CODEC_ID_NUV: AVCodecID = 84;
pub const AV_CODEC_ID_SMACKVIDEO: AVCodecID = 83;
pub const AV_CODEC_ID_AVS: AVCodecID = 82;
pub const AV_CODEC_ID_ZMBV: AVCodecID = 81;
pub const AV_CODEC_ID_MMVIDEO: AVCodecID = 80;
pub const AV_CODEC_ID_CSCD: AVCodecID = 79;
pub const AV_CODEC_ID_BMP: AVCodecID = 78;
pub const AV_CODEC_ID_TRUEMOTION2: AVCodecID = 77;
pub const AV_CODEC_ID_FRAPS: AVCodecID = 76;
pub const AV_CODEC_ID_INDEO2: AVCodecID = 75;
pub const AV_CODEC_ID_AASC: AVCodecID = 74;
pub const AV_CODEC_ID_WNV1: AVCodecID = 73;
pub const AV_CODEC_ID_LOCO: AVCodecID = 72;
pub const AV_CODEC_ID_WMV3: AVCodecID = 71;
pub const AV_CODEC_ID_VC1: AVCodecID = 70;
pub const AV_CODEC_ID_RV40: AVCodecID = 69;
pub const AV_CODEC_ID_RV30: AVCodecID = 68;
pub const AV_CODEC_ID_FFVHUFF: AVCodecID = 67;
pub const AV_CODEC_ID_PAM: AVCodecID = 66;
pub const AV_CODEC_ID_PGMYUV: AVCodecID = 65;
pub const AV_CODEC_ID_PGM: AVCodecID = 64;
pub const AV_CODEC_ID_PBM: AVCodecID = 63;
pub const AV_CODEC_ID_PPM: AVCodecID = 62;
pub const AV_CODEC_ID_PNG: AVCodecID = 61;
pub const AV_CODEC_ID_QPEG: AVCodecID = 60;
pub const AV_CODEC_ID_VIXL: AVCodecID = 59;
pub const AV_CODEC_ID_QDRAW: AVCodecID = 58;
pub const AV_CODEC_ID_ULTI: AVCodecID = 57;
pub const AV_CODEC_ID_TSCC: AVCodecID = 56;
pub const AV_CODEC_ID_QTRLE: AVCodecID = 55;
pub const AV_CODEC_ID_ZLIB: AVCodecID = 54;
pub const AV_CODEC_ID_MSZH: AVCodecID = 53;
pub const AV_CODEC_ID_VMDVIDEO: AVCodecID = 52;
pub const AV_CODEC_ID_TRUEMOTION1: AVCodecID = 51;
pub const AV_CODEC_ID_FLIC: AVCodecID = 50;
pub const AV_CODEC_ID_SMC: AVCodecID = 49;
pub const AV_CODEC_ID_8BPS: AVCodecID = 48;
pub const AV_CODEC_ID_IDCIN: AVCodecID = 47;
pub const AV_CODEC_ID_MSVIDEO1: AVCodecID = 46;
pub const AV_CODEC_ID_MSRLE: AVCodecID = 45;
pub const AV_CODEC_ID_WS_VQA: AVCodecID = 44;
pub const AV_CODEC_ID_CINEPAK: AVCodecID = 43;
pub const AV_CODEC_ID_RPZA: AVCodecID = 42;
pub const AV_CODEC_ID_XAN_WC4: AVCodecID = 41;
pub const AV_CODEC_ID_XAN_WC3: AVCodecID = 40;
pub const AV_CODEC_ID_INTERPLAY_VIDEO: AVCodecID = 39;
pub const AV_CODEC_ID_ROQ: AVCodecID = 38;
pub const AV_CODEC_ID_MDEC: AVCodecID = 37;
pub const AV_CODEC_ID_CLJR: AVCodecID = 36;
pub const AV_CODEC_ID_VCR1: AVCodecID = 35;
pub const AV_CODEC_ID_4XM: AVCodecID = 34;
pub const AV_CODEC_ID_FFV1: AVCodecID = 33;
pub const AV_CODEC_ID_ASV2: AVCodecID = 32;
pub const AV_CODEC_ID_ASV1: AVCodecID = 31;
pub const AV_CODEC_ID_THEORA: AVCodecID = 30;
pub const AV_CODEC_ID_VP3: AVCodecID = 29;
pub const AV_CODEC_ID_INDEO3: AVCodecID = 28;
pub const AV_CODEC_ID_H264: AVCodecID = 27;
pub const AV_CODEC_ID_CYUV: AVCodecID = 26;
pub const AV_CODEC_ID_HUFFYUV: AVCodecID = 25;
pub const AV_CODEC_ID_DVVIDEO: AVCodecID = 24;
pub const AV_CODEC_ID_SVQ3: AVCodecID = 23;
pub const AV_CODEC_ID_SVQ1: AVCodecID = 22;
pub const AV_CODEC_ID_FLV1: AVCodecID = 21;
pub const AV_CODEC_ID_H263I: AVCodecID = 20;
pub const AV_CODEC_ID_H263P: AVCodecID = 19;
pub const AV_CODEC_ID_WMV2: AVCodecID = 18;
pub const AV_CODEC_ID_WMV1: AVCodecID = 17;
pub const AV_CODEC_ID_MSMPEG4V3: AVCodecID = 16;
pub const AV_CODEC_ID_MSMPEG4V2: AVCodecID = 15;
pub const AV_CODEC_ID_MSMPEG4V1: AVCodecID = 14;
pub const AV_CODEC_ID_RAWVIDEO: AVCodecID = 13;
pub const AV_CODEC_ID_MPEG4: AVCodecID = 12;
pub const AV_CODEC_ID_JPEGLS: AVCodecID = 11;
pub const AV_CODEC_ID_SP5X: AVCodecID = 10;
pub const AV_CODEC_ID_LJPEG: AVCodecID = 9;
pub const AV_CODEC_ID_MJPEGB: AVCodecID = 8;
pub const AV_CODEC_ID_MJPEG: AVCodecID = 7;
pub const AV_CODEC_ID_RV20: AVCodecID = 6;
pub const AV_CODEC_ID_RV10: AVCodecID = 5;
pub const AV_CODEC_ID_H263: AVCodecID = 4;
pub const AV_CODEC_ID_H261: AVCodecID = 3;
pub const AV_CODEC_ID_MPEG2VIDEO: AVCodecID = 2;
pub const AV_CODEC_ID_MPEG1VIDEO: AVCodecID = 1;
pub const AV_CODEC_ID_NONE: AVCodecID = 0;
pub type AVFieldOrder = libc::c_uint;
pub const AV_FIELD_BT: AVFieldOrder = 5;
pub const AV_FIELD_TB: AVFieldOrder = 4;
pub const AV_FIELD_BB: AVFieldOrder = 3;
pub const AV_FIELD_TT: AVFieldOrder = 2;
pub const AV_FIELD_PROGRESSIVE: AVFieldOrder = 1;
pub const AV_FIELD_UNKNOWN: AVFieldOrder = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodecParameters {
    pub codec_type: AVMediaType,
    pub codec_id: AVCodecID,
    pub codec_tag: uint32_t,
    pub extradata: *mut uint8_t,
    pub extradata_size: libc::c_int,
    pub format: libc::c_int,
    pub bit_rate: int64_t,
    pub bits_per_coded_sample: libc::c_int,
    pub bits_per_raw_sample: libc::c_int,
    pub profile: libc::c_int,
    pub level: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub sample_aspect_ratio: AVRational,
    pub field_order: AVFieldOrder,
    pub color_range: AVColorRange,
    pub color_primaries: AVColorPrimaries,
    pub color_trc: AVColorTransferCharacteristic,
    pub color_space: AVColorSpace,
    pub chroma_location: AVChromaLocation,
    pub video_delay: libc::c_int,
    pub channel_layout: uint64_t,
    pub channels: libc::c_int,
    pub sample_rate: libc::c_int,
    pub block_align: libc::c_int,
    pub frame_size: libc::c_int,
    pub initial_padding: libc::c_int,
    pub trailing_padding: libc::c_int,
    pub seek_preroll: libc::c_int,
}
pub type AVPacketSideDataType = libc::c_uint;
pub const AV_PKT_DATA_NB: AVPacketSideDataType = 31;
pub const AV_PKT_DATA_S12M_TIMECODE: AVPacketSideDataType = 30;
pub const AV_PKT_DATA_DOVI_CONF: AVPacketSideDataType = 29;
pub const AV_PKT_DATA_ICC_PROFILE: AVPacketSideDataType = 28;
pub const AV_PKT_DATA_PRFT: AVPacketSideDataType = 27;
pub const AV_PKT_DATA_AFD: AVPacketSideDataType = 26;
pub const AV_PKT_DATA_ENCRYPTION_INFO: AVPacketSideDataType = 25;
pub const AV_PKT_DATA_ENCRYPTION_INIT_INFO: AVPacketSideDataType = 24;
pub const AV_PKT_DATA_A53_CC: AVPacketSideDataType = 23;
pub const AV_PKT_DATA_CONTENT_LIGHT_LEVEL: AVPacketSideDataType = 22;
pub const AV_PKT_DATA_SPHERICAL: AVPacketSideDataType = 21;
pub const AV_PKT_DATA_MASTERING_DISPLAY_METADATA: AVPacketSideDataType = 20;
pub const AV_PKT_DATA_MPEGTS_STREAM_ID: AVPacketSideDataType = 19;
pub const AV_PKT_DATA_METADATA_UPDATE: AVPacketSideDataType = 18;
pub const AV_PKT_DATA_WEBVTT_SETTINGS: AVPacketSideDataType = 17;
pub const AV_PKT_DATA_WEBVTT_IDENTIFIER: AVPacketSideDataType = 16;
pub const AV_PKT_DATA_MATROSKA_BLOCKADDITIONAL: AVPacketSideDataType = 15;
pub const AV_PKT_DATA_SUBTITLE_POSITION: AVPacketSideDataType = 14;
pub const AV_PKT_DATA_STRINGS_METADATA: AVPacketSideDataType = 13;
pub const AV_PKT_DATA_JP_DUALMONO: AVPacketSideDataType = 12;
pub const AV_PKT_DATA_SKIP_SAMPLES: AVPacketSideDataType = 11;
pub const AV_PKT_DATA_CPB_PROPERTIES: AVPacketSideDataType = 10;
pub const AV_PKT_DATA_FALLBACK_TRACK: AVPacketSideDataType = 9;
pub const AV_PKT_DATA_QUALITY_STATS: AVPacketSideDataType = 8;
pub const AV_PKT_DATA_AUDIO_SERVICE_TYPE: AVPacketSideDataType = 7;
pub const AV_PKT_DATA_STEREO3D: AVPacketSideDataType = 6;
pub const AV_PKT_DATA_DISPLAYMATRIX: AVPacketSideDataType = 5;
pub const AV_PKT_DATA_REPLAYGAIN: AVPacketSideDataType = 4;
pub const AV_PKT_DATA_H263_MB_INFO: AVPacketSideDataType = 3;
pub const AV_PKT_DATA_PARAM_CHANGE: AVPacketSideDataType = 2;
pub const AV_PKT_DATA_NEW_EXTRADATA: AVPacketSideDataType = 1;
pub const AV_PKT_DATA_PALETTE: AVPacketSideDataType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVPacketSideData {
    pub data: *mut uint8_t,
    pub size: libc::c_int,
    pub type_0: AVPacketSideDataType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVPacket {
    pub buf: *mut AVBufferRef,
    pub pts: int64_t,
    pub dts: int64_t,
    pub data: *mut uint8_t,
    pub size: libc::c_int,
    pub stream_index: libc::c_int,
    pub flags: libc::c_int,
    pub side_data: *mut AVPacketSideData,
    pub side_data_elems: libc::c_int,
    pub duration: int64_t,
    pub pos: int64_t,
    pub convergence_duration: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVProfile {
    pub profile: libc::c_int,
    pub name: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodecContext {
    pub av_class: *const AVClass,
    pub log_level_offset: libc::c_int,
    pub codec_type: AVMediaType,
    pub codec: *const AVCodec,
    pub codec_id: AVCodecID,
    pub codec_tag: libc::c_uint,
    pub priv_data: *mut libc::c_void,
    pub internal: *mut AVCodecInternal,
    pub opaque: *mut libc::c_void,
    pub bit_rate: int64_t,
    pub bit_rate_tolerance: libc::c_int,
    pub global_quality: libc::c_int,
    pub compression_level: libc::c_int,
    pub flags: libc::c_int,
    pub flags2: libc::c_int,
    pub extradata: *mut uint8_t,
    pub extradata_size: libc::c_int,
    pub time_base: AVRational,
    pub ticks_per_frame: libc::c_int,
    pub delay: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub coded_width: libc::c_int,
    pub coded_height: libc::c_int,
    pub gop_size: libc::c_int,
    pub pix_fmt: AVPixelFormat,
    pub draw_horiz_band: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *const AVFrame,
            *mut libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub get_format:
        Option<unsafe extern "C" fn(*mut AVCodecContext, *const AVPixelFormat) -> AVPixelFormat>,
    pub max_b_frames: libc::c_int,
    pub b_quant_factor: libc::c_float,
    pub b_frame_strategy: libc::c_int,
    pub b_quant_offset: libc::c_float,
    pub has_b_frames: libc::c_int,
    pub mpeg_quant: libc::c_int,
    pub i_quant_factor: libc::c_float,
    pub i_quant_offset: libc::c_float,
    pub lumi_masking: libc::c_float,
    pub temporal_cplx_masking: libc::c_float,
    pub spatial_cplx_masking: libc::c_float,
    pub p_masking: libc::c_float,
    pub dark_masking: libc::c_float,
    pub slice_count: libc::c_int,
    pub prediction_method: libc::c_int,
    pub slice_offset: *mut libc::c_int,
    pub sample_aspect_ratio: AVRational,
    pub me_cmp: libc::c_int,
    pub me_sub_cmp: libc::c_int,
    pub mb_cmp: libc::c_int,
    pub ildct_cmp: libc::c_int,
    pub dia_size: libc::c_int,
    pub last_predictor_count: libc::c_int,
    pub pre_me: libc::c_int,
    pub me_pre_cmp: libc::c_int,
    pub pre_dia_size: libc::c_int,
    pub me_subpel_quality: libc::c_int,
    pub me_range: libc::c_int,
    pub slice_flags: libc::c_int,
    pub mb_decision: libc::c_int,
    pub intra_matrix: *mut uint16_t,
    pub inter_matrix: *mut uint16_t,
    pub scenechange_threshold: libc::c_int,
    pub noise_reduction: libc::c_int,
    pub intra_dc_precision: libc::c_int,
    pub skip_top: libc::c_int,
    pub skip_bottom: libc::c_int,
    pub mb_lmin: libc::c_int,
    pub mb_lmax: libc::c_int,
    pub me_penalty_compensation: libc::c_int,
    pub bidir_refine: libc::c_int,
    pub brd_scale: libc::c_int,
    pub keyint_min: libc::c_int,
    pub refs: libc::c_int,
    pub chromaoffset: libc::c_int,
    pub mv0_threshold: libc::c_int,
    pub b_sensitivity: libc::c_int,
    pub color_primaries: AVColorPrimaries,
    pub color_trc: AVColorTransferCharacteristic,
    pub colorspace: AVColorSpace,
    pub color_range: AVColorRange,
    pub chroma_sample_location: AVChromaLocation,
    pub slices: libc::c_int,
    pub field_order: AVFieldOrder,
    pub sample_rate: libc::c_int,
    pub channels: libc::c_int,
    pub sample_fmt: AVSampleFormat,
    pub frame_size: libc::c_int,
    pub frame_number: libc::c_int,
    pub block_align: libc::c_int,
    pub cutoff: libc::c_int,
    pub channel_layout: uint64_t,
    pub request_channel_layout: uint64_t,
    pub audio_service_type: AVAudioServiceType,
    pub request_sample_fmt: AVSampleFormat,
    pub get_buffer2:
        Option<unsafe extern "C" fn(*mut AVCodecContext, *mut AVFrame, libc::c_int) -> libc::c_int>,
    pub refcounted_frames: libc::c_int,
    pub qcompress: libc::c_float,
    pub qblur: libc::c_float,
    pub qmin: libc::c_int,
    pub qmax: libc::c_int,
    pub max_qdiff: libc::c_int,
    pub rc_buffer_size: libc::c_int,
    pub rc_override_count: libc::c_int,
    pub rc_override: *mut RcOverride,
    pub rc_max_rate: int64_t,
    pub rc_min_rate: int64_t,
    pub rc_max_available_vbv_use: libc::c_float,
    pub rc_min_vbv_overflow_use: libc::c_float,
    pub rc_initial_buffer_occupancy: libc::c_int,
    pub coder_type: libc::c_int,
    pub context_model: libc::c_int,
    pub frame_skip_threshold: libc::c_int,
    pub frame_skip_factor: libc::c_int,
    pub frame_skip_exp: libc::c_int,
    pub frame_skip_cmp: libc::c_int,
    pub trellis: libc::c_int,
    pub min_prediction_order: libc::c_int,
    pub max_prediction_order: libc::c_int,
    pub timecode_frame_start: int64_t,
    pub rtp_callback: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *mut libc::c_void,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub rtp_payload_size: libc::c_int,
    pub mv_bits: libc::c_int,
    pub header_bits: libc::c_int,
    pub i_tex_bits: libc::c_int,
    pub p_tex_bits: libc::c_int,
    pub i_count: libc::c_int,
    pub p_count: libc::c_int,
    pub skip_count: libc::c_int,
    pub misc_bits: libc::c_int,
    pub frame_bits: libc::c_int,
    pub stats_out: *mut libc::c_char,
    pub stats_in: *mut libc::c_char,
    pub workaround_bugs: libc::c_int,
    pub strict_std_compliance: libc::c_int,
    pub error_concealment: libc::c_int,
    pub debug: libc::c_int,
    pub err_recognition: libc::c_int,
    pub reordered_opaque: int64_t,
    pub hwaccel: *const AVHWAccel,
    pub hwaccel_context: *mut libc::c_void,
    pub error: [uint64_t; 8],
    pub dct_algo: libc::c_int,
    pub idct_algo: libc::c_int,
    pub bits_per_coded_sample: libc::c_int,
    pub bits_per_raw_sample: libc::c_int,
    pub lowres: libc::c_int,
    pub coded_frame: *mut AVFrame,
    pub thread_count: libc::c_int,
    pub thread_type: libc::c_int,
    pub active_thread_type: libc::c_int,
    pub thread_safe_callbacks: libc::c_int,
    pub execute: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            Option<unsafe extern "C" fn(*mut AVCodecContext, *mut libc::c_void) -> libc::c_int>,
            *mut libc::c_void,
            *mut libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub execute2: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            Option<
                unsafe extern "C" fn(
                    *mut AVCodecContext,
                    *mut libc::c_void,
                    libc::c_int,
                    libc::c_int,
                ) -> libc::c_int,
            >,
            *mut libc::c_void,
            *mut libc::c_int,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub nsse_weight: libc::c_int,
    pub profile: libc::c_int,
    pub level: libc::c_int,
    pub skip_loop_filter: AVDiscard,
    pub skip_idct: AVDiscard,
    pub skip_frame: AVDiscard,
    pub subtitle_header: *mut uint8_t,
    pub subtitle_header_size: libc::c_int,
    pub vbv_delay: uint64_t,
    pub side_data_only_packets: libc::c_int,
    pub initial_padding: libc::c_int,
    pub framerate: AVRational,
    pub sw_pix_fmt: AVPixelFormat,
    pub pkt_timebase: AVRational,
    pub codec_descriptor: *const AVCodecDescriptor,
    pub pts_correction_num_faulty_pts: int64_t,
    pub pts_correction_num_faulty_dts: int64_t,
    pub pts_correction_last_pts: int64_t,
    pub pts_correction_last_dts: int64_t,
    pub sub_charenc: *mut libc::c_char,
    pub sub_charenc_mode: libc::c_int,
    pub skip_alpha: libc::c_int,
    pub seek_preroll: libc::c_int,
    pub debug_mv: libc::c_int,
    pub chroma_intra_matrix: *mut uint16_t,
    pub dump_separator: *mut uint8_t,
    pub codec_whitelist: *mut libc::c_char,
    pub properties: libc::c_uint,
    pub coded_side_data: *mut AVPacketSideData,
    pub nb_coded_side_data: libc::c_int,
    pub hw_frames_ctx: *mut AVBufferRef,
    pub sub_text_format: libc::c_int,
    pub trailing_padding: libc::c_int,
    pub max_pixels: int64_t,
    pub hw_device_ctx: *mut AVBufferRef,
    pub hwaccel_flags: libc::c_int,
    pub apply_cropping: libc::c_int,
    pub extra_hw_frames: libc::c_int,
    pub discard_damaged_percentage: libc::c_int,
    pub max_samples: int64_t,
    pub export_side_data: libc::c_int,
    pub get_encode_buffer: Option<
        unsafe extern "C" fn(*mut AVCodecContext, *mut AVPacket, libc::c_int) -> libc::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodecDescriptor {
    pub id: AVCodecID,
    pub type_0: AVMediaType,
    pub name: *const libc::c_char,
    pub long_name: *const libc::c_char,
    pub props: libc::c_int,
    pub mime_types: *const *const libc::c_char,
    pub profiles: *const AVProfile,
}
pub type AVDiscard = libc::c_int;
pub const AVDISCARD_ALL: AVDiscard = 48;
pub const AVDISCARD_NONKEY: AVDiscard = 32;
pub const AVDISCARD_NONINTRA: AVDiscard = 24;
pub const AVDISCARD_BIDIR: AVDiscard = 16;
pub const AVDISCARD_NONREF: AVDiscard = 8;
pub const AVDISCARD_DEFAULT: AVDiscard = 0;
pub const AVDISCARD_NONE: AVDiscard = -16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVHWAccel {
    pub name: *const libc::c_char,
    pub type_0: AVMediaType,
    pub id: AVCodecID,
    pub pix_fmt: AVPixelFormat,
    pub capabilities: libc::c_int,
    pub alloc_frame: Option<unsafe extern "C" fn(*mut AVCodecContext, *mut AVFrame) -> libc::c_int>,
    pub start_frame:
        Option<unsafe extern "C" fn(*mut AVCodecContext, *const uint8_t, uint32_t) -> libc::c_int>,
    pub decode_params: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            libc::c_int,
            *const uint8_t,
            uint32_t,
        ) -> libc::c_int,
    >,
    pub decode_slice:
        Option<unsafe extern "C" fn(*mut AVCodecContext, *const uint8_t, uint32_t) -> libc::c_int>,
    pub end_frame: Option<unsafe extern "C" fn(*mut AVCodecContext) -> libc::c_int>,
    pub frame_priv_data_size: libc::c_int,
    pub decode_mb: Option<unsafe extern "C" fn(*mut MpegEncContext) -> ()>,
    pub init: Option<unsafe extern "C" fn(*mut AVCodecContext) -> libc::c_int>,
    pub uninit: Option<unsafe extern "C" fn(*mut AVCodecContext) -> libc::c_int>,
    pub priv_data_size: libc::c_int,
    pub caps_internal: libc::c_int,
    pub frame_params:
        Option<unsafe extern "C" fn(*mut AVCodecContext, *mut AVBufferRef) -> libc::c_int>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RcOverride {
    pub start_frame: libc::c_int,
    pub end_frame: libc::c_int,
    pub qscale: libc::c_int,
    pub quality_factor: libc::c_float,
}
pub type AVAudioServiceType = libc::c_uint;
pub const AV_AUDIO_SERVICE_TYPE_NB: AVAudioServiceType = 9;
pub const AV_AUDIO_SERVICE_TYPE_KARAOKE: AVAudioServiceType = 8;
pub const AV_AUDIO_SERVICE_TYPE_VOICE_OVER: AVAudioServiceType = 7;
pub const AV_AUDIO_SERVICE_TYPE_EMERGENCY: AVAudioServiceType = 6;
pub const AV_AUDIO_SERVICE_TYPE_COMMENTARY: AVAudioServiceType = 5;
pub const AV_AUDIO_SERVICE_TYPE_DIALOGUE: AVAudioServiceType = 4;
pub const AV_AUDIO_SERVICE_TYPE_HEARING_IMPAIRED: AVAudioServiceType = 3;
pub const AV_AUDIO_SERVICE_TYPE_VISUALLY_IMPAIRED: AVAudioServiceType = 2;
pub const AV_AUDIO_SERVICE_TYPE_EFFECTS: AVAudioServiceType = 1;
pub const AV_AUDIO_SERVICE_TYPE_MAIN: AVAudioServiceType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodec {
    pub name: *const libc::c_char,
    pub long_name: *const libc::c_char,
    pub type_0: AVMediaType,
    pub id: AVCodecID,
    pub capabilities: libc::c_int,
    pub supported_framerates: *const AVRational,
    pub pix_fmts: *const AVPixelFormat,
    pub supported_samplerates: *const libc::c_int,
    pub sample_fmts: *const AVSampleFormat,
    pub channel_layouts: *const uint64_t,
    pub max_lowres: uint8_t,
    pub priv_class: *const AVClass,
    pub profiles: *const AVProfile,
    pub wrapper_name: *const libc::c_char,
    pub priv_data_size: libc::c_int,
    pub next: *mut AVCodec,
    pub update_thread_context:
        Option<unsafe extern "C" fn(*mut AVCodecContext, *const AVCodecContext) -> libc::c_int>,
    pub defaults: *const AVCodecDefault,
    pub init_static_data: Option<unsafe extern "C" fn(*mut AVCodec) -> ()>,
    pub init: Option<unsafe extern "C" fn(*mut AVCodecContext) -> libc::c_int>,
    pub encode_sub: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *mut uint8_t,
            libc::c_int,
            *const AVSubtitle,
        ) -> libc::c_int,
    >,
    pub encode2: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *mut AVPacket,
            *const AVFrame,
            *mut libc::c_int,
        ) -> libc::c_int,
    >,
    pub decode: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *mut libc::c_void,
            *mut libc::c_int,
            *mut AVPacket,
        ) -> libc::c_int,
    >,
    pub close: Option<unsafe extern "C" fn(*mut AVCodecContext) -> libc::c_int>,
    pub receive_packet:
        Option<unsafe extern "C" fn(*mut AVCodecContext, *mut AVPacket) -> libc::c_int>,
    pub receive_frame:
        Option<unsafe extern "C" fn(*mut AVCodecContext, *mut AVFrame) -> libc::c_int>,
    pub flush: Option<unsafe extern "C" fn(*mut AVCodecContext) -> ()>,
    pub caps_internal: libc::c_int,
    pub bsfs: *const libc::c_char,
    pub hw_configs: *const *const AVCodecHWConfigInternal,
    pub codec_tags: *const uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVSubtitle {
    pub format: uint16_t,
    pub start_display_time: uint32_t,
    pub end_display_time: uint32_t,
    pub num_rects: libc::c_uint,
    pub rects: *mut *mut AVSubtitleRect,
    pub pts: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVSubtitleRect {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub w: libc::c_int,
    pub h: libc::c_int,
    pub nb_colors: libc::c_int,
    pub pict: AVPicture,
    pub data: [*mut uint8_t; 4],
    pub linesize: [libc::c_int; 4],
    pub type_0: AVSubtitleType,
    pub text: *mut libc::c_char,
    pub ass: *mut libc::c_char,
    pub flags: libc::c_int,
}
pub type AVSubtitleType = libc::c_uint;
pub const SUBTITLE_ASS: AVSubtitleType = 3;
pub const SUBTITLE_TEXT: AVSubtitleType = 2;
pub const SUBTITLE_BITMAP: AVSubtitleType = 1;
pub const SUBTITLE_NONE: AVSubtitleType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVPicture {
    pub data: [*mut uint8_t; 8],
    pub linesize: [libc::c_int; 8],
}
pub type AVPictureStructure = libc::c_uint;
pub const AV_PICTURE_STRUCTURE_FRAME: AVPictureStructure = 3;
pub const AV_PICTURE_STRUCTURE_BOTTOM_FIELD: AVPictureStructure = 2;
pub const AV_PICTURE_STRUCTURE_TOP_FIELD: AVPictureStructure = 1;
pub const AV_PICTURE_STRUCTURE_UNKNOWN: AVPictureStructure = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodecParserContext {
    pub priv_data: *mut libc::c_void,
    pub parser: *mut AVCodecParser,
    pub frame_offset: int64_t,
    pub cur_offset: int64_t,
    pub next_frame_offset: int64_t,
    pub pict_type: libc::c_int,
    pub repeat_pict: libc::c_int,
    pub pts: int64_t,
    pub dts: int64_t,
    pub last_pts: int64_t,
    pub last_dts: int64_t,
    pub fetch_timestamp: libc::c_int,
    pub cur_frame_start_index: libc::c_int,
    pub cur_frame_offset: [int64_t; 4],
    pub cur_frame_pts: [int64_t; 4],
    pub cur_frame_dts: [int64_t; 4],
    pub flags: libc::c_int,
    pub offset: int64_t,
    pub cur_frame_end: [int64_t; 4],
    pub key_frame: libc::c_int,
    pub convergence_duration: int64_t,
    pub dts_sync_point: libc::c_int,
    pub dts_ref_dts_delta: libc::c_int,
    pub pts_dts_delta: libc::c_int,
    pub cur_frame_pos: [int64_t; 4],
    pub pos: int64_t,
    pub last_pos: int64_t,
    pub duration: libc::c_int,
    pub field_order: AVFieldOrder,
    pub picture_structure: AVPictureStructure,
    pub output_picture_number: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub coded_width: libc::c_int,
    pub coded_height: libc::c_int,
    pub format: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodecParser {
    pub codec_ids: [libc::c_int; 5],
    pub priv_data_size: libc::c_int,
    pub parser_init: Option<unsafe extern "C" fn(*mut AVCodecParserContext) -> libc::c_int>,
    pub parser_parse: Option<
        unsafe extern "C" fn(
            *mut AVCodecParserContext,
            *mut AVCodecContext,
            *mut *const uint8_t,
            *mut libc::c_int,
            *const uint8_t,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub parser_close: Option<unsafe extern "C" fn(*mut AVCodecParserContext) -> ()>,
    pub split: Option<
        unsafe extern "C" fn(*mut AVCodecContext, *const uint8_t, libc::c_int) -> libc::c_int,
    >,
    pub next: *mut AVCodecParser,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVIOInterruptCB {
    pub callback: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub opaque: *mut libc::c_void,
}
pub type AVIODataMarkerType = libc::c_uint;
pub const AVIO_DATA_MARKER_FLUSH_POINT: AVIODataMarkerType = 5;
pub const AVIO_DATA_MARKER_TRAILER: AVIODataMarkerType = 4;
pub const AVIO_DATA_MARKER_UNKNOWN: AVIODataMarkerType = 3;
pub const AVIO_DATA_MARKER_BOUNDARY_POINT: AVIODataMarkerType = 2;
pub const AVIO_DATA_MARKER_SYNC_POINT: AVIODataMarkerType = 1;
pub const AVIO_DATA_MARKER_HEADER: AVIODataMarkerType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVIOContext {
    pub av_class: *const AVClass,
    pub buffer: *mut libc::c_uchar,
    pub buffer_size: libc::c_int,
    pub buf_ptr: *mut libc::c_uchar,
    pub buf_end: *mut libc::c_uchar,
    pub opaque: *mut libc::c_void,
    pub read_packet:
        Option<unsafe extern "C" fn(*mut libc::c_void, *mut uint8_t, libc::c_int) -> libc::c_int>,
    pub write_packet:
        Option<unsafe extern "C" fn(*mut libc::c_void, *mut uint8_t, libc::c_int) -> libc::c_int>,
    pub seek: Option<unsafe extern "C" fn(*mut libc::c_void, int64_t, libc::c_int) -> int64_t>,
    pub pos: int64_t,
    pub eof_reached: libc::c_int,
    pub write_flag: libc::c_int,
    pub max_packet_size: libc::c_int,
    pub checksum: libc::c_ulong,
    pub checksum_ptr: *mut libc::c_uchar,
    pub update_checksum:
        Option<unsafe extern "C" fn(libc::c_ulong, *const uint8_t, libc::c_uint) -> libc::c_ulong>,
    pub error: libc::c_int,
    pub read_pause: Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> libc::c_int>,
    pub read_seek: Option<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_int, int64_t, libc::c_int) -> int64_t,
    >,
    pub seekable: libc::c_int,
    pub maxsize: int64_t,
    pub direct: libc::c_int,
    pub bytes_read: int64_t,
    pub seek_count: libc::c_int,
    pub writeout_count: libc::c_int,
    pub orig_buffer_size: libc::c_int,
    pub short_seek_threshold: libc::c_int,
    pub protocol_whitelist: *const libc::c_char,
    pub protocol_blacklist: *const libc::c_char,
    pub write_data_type: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut uint8_t,
            libc::c_int,
            AVIODataMarkerType,
            int64_t,
        ) -> libc::c_int,
    >,
    pub ignore_boundary_point: libc::c_int,
    pub current_type: AVIODataMarkerType,
    pub last_time: int64_t,
    pub short_seek_get: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub written: int64_t,
    pub buf_ptr_max: *mut libc::c_uchar,
    pub min_packet_size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFormatContext {
    pub av_class: *const AVClass,
    pub iformat: *mut AVInputFormat,
    pub oformat: *mut AVOutputFormat,
    pub priv_data: *mut libc::c_void,
    pub pb: *mut AVIOContext,
    pub ctx_flags: libc::c_int,
    pub nb_streams: libc::c_uint,
    pub streams: *mut *mut AVStream,
    pub filename: [libc::c_char; 1024],
    pub url: *mut libc::c_char,
    pub start_time: int64_t,
    pub duration: int64_t,
    pub bit_rate: int64_t,
    pub packet_size: libc::c_uint,
    pub max_delay: libc::c_int,
    pub flags: libc::c_int,
    pub probesize: int64_t,
    pub max_analyze_duration: int64_t,
    pub key: *const uint8_t,
    pub keylen: libc::c_int,
    pub nb_programs: libc::c_uint,
    pub programs: *mut *mut AVProgram,
    pub video_codec_id: AVCodecID,
    pub audio_codec_id: AVCodecID,
    pub subtitle_codec_id: AVCodecID,
    pub max_index_size: libc::c_uint,
    pub max_picture_buffer: libc::c_uint,
    pub nb_chapters: libc::c_uint,
    pub chapters: *mut *mut AVChapter,
    pub metadata: *mut AVDictionary,
    pub start_time_realtime: int64_t,
    pub fps_probe_size: libc::c_int,
    pub error_recognition: libc::c_int,
    pub interrupt_callback: AVIOInterruptCB,
    pub debug: libc::c_int,
    pub max_interleave_delta: int64_t,
    pub strict_std_compliance: libc::c_int,
    pub event_flags: libc::c_int,
    pub max_ts_probe: libc::c_int,
    pub avoid_negative_ts: libc::c_int,
    pub ts_id: libc::c_int,
    pub audio_preload: libc::c_int,
    pub max_chunk_duration: libc::c_int,
    pub max_chunk_size: libc::c_int,
    pub use_wallclock_as_timestamps: libc::c_int,
    pub avio_flags: libc::c_int,
    pub duration_estimation_method: AVDurationEstimationMethod,
    pub skip_initial_bytes: int64_t,
    pub correct_ts_overflow: libc::c_uint,
    pub seek2any: libc::c_int,
    pub flush_packets: libc::c_int,
    pub probe_score: libc::c_int,
    pub format_probesize: libc::c_int,
    pub codec_whitelist: *mut libc::c_char,
    pub format_whitelist: *mut libc::c_char,
    pub internal: *mut AVFormatInternal,
    pub io_repositioned: libc::c_int,
    pub video_codec: *mut AVCodec,
    pub audio_codec: *mut AVCodec,
    pub subtitle_codec: *mut AVCodec,
    pub data_codec: *mut AVCodec,
    pub metadata_header_padding: libc::c_int,
    pub opaque: *mut libc::c_void,
    pub control_message_cb: av_format_control_message,
    pub output_ts_offset: int64_t,
    pub dump_separator: *mut uint8_t,
    pub data_codec_id: AVCodecID,
    pub open_cb: Option<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            *mut *mut AVIOContext,
            *const libc::c_char,
            libc::c_int,
            *const AVIOInterruptCB,
            *mut *mut AVDictionary,
        ) -> libc::c_int,
    >,
    pub protocol_whitelist: *mut libc::c_char,
    pub io_open: Option<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            *mut *mut AVIOContext,
            *const libc::c_char,
            libc::c_int,
            *mut *mut AVDictionary,
        ) -> libc::c_int,
    >,
    pub io_close: Option<unsafe extern "C" fn(*mut AVFormatContext, *mut AVIOContext) -> ()>,
    pub protocol_blacklist: *mut libc::c_char,
    pub max_streams: libc::c_int,
    pub skip_estimate_duration_from_pts: libc::c_int,
    pub max_probe_packets: libc::c_int,
}
pub type av_format_control_message = Option<
    unsafe extern "C" fn(
        *mut AVFormatContext,
        libc::c_int,
        *mut libc::c_void,
        size_t,
    ) -> libc::c_int,
>;
pub type AVDurationEstimationMethod = libc::c_uint;
pub const AVFMT_DURATION_FROM_BITRATE: AVDurationEstimationMethod = 2;
pub const AVFMT_DURATION_FROM_STREAM: AVDurationEstimationMethod = 1;
pub const AVFMT_DURATION_FROM_PTS: AVDurationEstimationMethod = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVChapter {
    pub id: libc::c_int,
    pub time_base: AVRational,
    pub start: int64_t,
    pub end: int64_t,
    pub metadata: *mut AVDictionary,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVProgram {
    pub id: libc::c_int,
    pub flags: libc::c_int,
    pub discard: AVDiscard,
    pub stream_index: *mut libc::c_uint,
    pub nb_stream_indexes: libc::c_uint,
    pub metadata: *mut AVDictionary,
    pub program_num: libc::c_int,
    pub pmt_pid: libc::c_int,
    pub pcr_pid: libc::c_int,
    pub pmt_version: libc::c_int,
    pub start_time: int64_t,
    pub end_time: int64_t,
    pub pts_wrap_reference: int64_t,
    pub pts_wrap_behavior: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVStream {
    pub index: libc::c_int,
    pub id: libc::c_int,
    pub codec: *mut AVCodecContext,
    pub priv_data: *mut libc::c_void,
    pub time_base: AVRational,
    pub start_time: int64_t,
    pub duration: int64_t,
    pub nb_frames: int64_t,
    pub disposition: libc::c_int,
    pub discard: AVDiscard,
    pub sample_aspect_ratio: AVRational,
    pub metadata: *mut AVDictionary,
    pub avg_frame_rate: AVRational,
    pub attached_pic: AVPacket,
    pub side_data: *mut AVPacketSideData,
    pub nb_side_data: libc::c_int,
    pub event_flags: libc::c_int,
    pub r_frame_rate: AVRational,
    pub recommended_encoder_configuration: *mut libc::c_char,
    pub codecpar: *mut AVCodecParameters,
    pub unused: *mut libc::c_void,
    pub pts_wrap_bits: libc::c_int,
    pub first_dts: int64_t,
    pub cur_dts: int64_t,
    pub last_IP_pts: int64_t,
    pub last_IP_duration: libc::c_int,
    pub probe_packets: libc::c_int,
    pub codec_info_nb_frames: libc::c_int,
    pub need_parsing: AVStreamParseType,
    pub parser: *mut AVCodecParserContext,
    pub unused7: *mut libc::c_void,
    pub unused6: AVProbeData,
    pub unused5: [int64_t; 17],
    pub index_entries: *mut AVIndexEntry,
    pub nb_index_entries: libc::c_int,
    pub index_entries_allocated_size: libc::c_uint,
    pub stream_identifier: libc::c_int,
    pub unused8: libc::c_int,
    pub unused9: libc::c_int,
    pub unused10: libc::c_int,
    pub internal: *mut AVStreamInternal,
}
#[derive(Copy, Clone, c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct AVIndexEntry {
    pub pos: int64_t,
    pub timestamp: int64_t,
    #[bitfield(name = "flags", ty = "libc::c_int", bits = "0..=1")]
    #[bitfield(name = "size", ty = "libc::c_int", bits = "2..=31")]
    pub flags_size: [u8; 4],
    pub min_distance: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVProbeData {
    pub filename: *const libc::c_char,
    pub buf: *mut libc::c_uchar,
    pub buf_size: libc::c_int,
    pub mime_type: *const libc::c_char,
}
pub type AVStreamParseType = libc::c_uint;
pub const AVSTREAM_PARSE_FULL_RAW: AVStreamParseType = 5;
pub const AVSTREAM_PARSE_FULL_ONCE: AVStreamParseType = 4;
pub const AVSTREAM_PARSE_TIMESTAMPS: AVStreamParseType = 3;
pub const AVSTREAM_PARSE_HEADERS: AVStreamParseType = 2;
pub const AVSTREAM_PARSE_FULL: AVStreamParseType = 1;
pub const AVSTREAM_PARSE_NONE: AVStreamParseType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVOutputFormat {
    pub name: *const libc::c_char,
    pub long_name: *const libc::c_char,
    pub mime_type: *const libc::c_char,
    pub extensions: *const libc::c_char,
    pub audio_codec: AVCodecID,
    pub video_codec: AVCodecID,
    pub subtitle_codec: AVCodecID,
    pub flags: libc::c_int,
    pub codec_tag: *const *const AVCodecTag,
    pub priv_class: *const AVClass,
    pub next: *mut AVOutputFormat,
    pub priv_data_size: libc::c_int,
    pub write_header: Option<unsafe extern "C" fn(*mut AVFormatContext) -> libc::c_int>,
    pub write_packet:
        Option<unsafe extern "C" fn(*mut AVFormatContext, *mut AVPacket) -> libc::c_int>,
    pub write_trailer: Option<unsafe extern "C" fn(*mut AVFormatContext) -> libc::c_int>,
    pub interleave_packet: Option<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            *mut AVPacket,
            *mut AVPacket,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub query_codec: Option<unsafe extern "C" fn(AVCodecID, libc::c_int) -> libc::c_int>,
    pub get_output_timestamp: Option<
        unsafe extern "C" fn(*mut AVFormatContext, libc::c_int, *mut int64_t, *mut int64_t) -> (),
    >,
    pub control_message: Option<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            libc::c_int,
            *mut libc::c_void,
            size_t,
        ) -> libc::c_int,
    >,
    pub write_uncoded_frame: Option<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            libc::c_int,
            *mut *mut AVFrame,
            libc::c_uint,
        ) -> libc::c_int,
    >,
    pub get_device_list:
        Option<unsafe extern "C" fn(*mut AVFormatContext, *mut AVDeviceInfoList) -> libc::c_int>,
    pub create_device_capabilities: Option<
        unsafe extern "C" fn(*mut AVFormatContext, *mut AVDeviceCapabilitiesQuery) -> libc::c_int,
    >,
    pub free_device_capabilities: Option<
        unsafe extern "C" fn(*mut AVFormatContext, *mut AVDeviceCapabilitiesQuery) -> libc::c_int,
    >,
    pub data_codec: AVCodecID,
    pub init: Option<unsafe extern "C" fn(*mut AVFormatContext) -> libc::c_int>,
    pub deinit: Option<unsafe extern "C" fn(*mut AVFormatContext) -> ()>,
    pub check_bitstream:
        Option<unsafe extern "C" fn(*mut AVFormatContext, *const AVPacket) -> libc::c_int>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVInputFormat {
    pub name: *const libc::c_char,
    pub long_name: *const libc::c_char,
    pub flags: libc::c_int,
    pub extensions: *const libc::c_char,
    pub codec_tag: *const *const AVCodecTag,
    pub priv_class: *const AVClass,
    pub mime_type: *const libc::c_char,
    pub next: *mut AVInputFormat,
    pub raw_codec_id: libc::c_int,
    pub priv_data_size: libc::c_int,
    pub read_probe: Option<unsafe extern "C" fn(*const AVProbeData) -> libc::c_int>,
    pub read_header: Option<unsafe extern "C" fn(*mut AVFormatContext) -> libc::c_int>,
    pub read_packet:
        Option<unsafe extern "C" fn(*mut AVFormatContext, *mut AVPacket) -> libc::c_int>,
    pub read_close: Option<unsafe extern "C" fn(*mut AVFormatContext) -> libc::c_int>,
    pub read_seek: Option<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            libc::c_int,
            int64_t,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub read_timestamp: Option<
        unsafe extern "C" fn(*mut AVFormatContext, libc::c_int, *mut int64_t, int64_t) -> int64_t,
    >,
    pub read_play: Option<unsafe extern "C" fn(*mut AVFormatContext) -> libc::c_int>,
    pub read_pause: Option<unsafe extern "C" fn(*mut AVFormatContext) -> libc::c_int>,
    pub read_seek2: Option<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            libc::c_int,
            int64_t,
            int64_t,
            int64_t,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub get_device_list:
        Option<unsafe extern "C" fn(*mut AVFormatContext, *mut AVDeviceInfoList) -> libc::c_int>,
    pub create_device_capabilities: Option<
        unsafe extern "C" fn(*mut AVFormatContext, *mut AVDeviceCapabilitiesQuery) -> libc::c_int,
    >,
    pub free_device_capabilities: Option<
        unsafe extern "C" fn(*mut AVFormatContext, *mut AVDeviceCapabilitiesQuery) -> libc::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SwsVector {
    pub coeff: *mut libc::c_double,
    pub length: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SwsFilter {
    pub lumH: *mut SwsVector,
    pub lumV: *mut SwsVector,
    pub chrH: *mut SwsVector,
    pub chrV: *mut SwsVector,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoState {
    pub pFormatCtx: *mut AVFormatContext,
    pub dec_ctx: *mut AVCodecContext,
    pub videoStream: libc::c_int,
    pub audioStream: libc::c_int,
    pub subtitleStream: libc::c_int,
    pub av_sync_type: libc::c_int,
    pub seek_req: libc::c_int,
    pub seek_by_bytes: libc::c_int,
    pub seek_no_flush: libc::c_int,
    pub seek_pts: libc::c_double,
    pub seek_flags: libc::c_int,
    pub seek_pos: int64_t,
    pub audio_clock: libc::c_double,
    pub audio_st: *mut AVStream,
    pub subtitle_st: *mut AVStream,
    pub audio_buf_size: libc::c_uint,
    pub audio_buf_index: libc::c_uint,
    pub audio_pkt: AVPacket,
    pub audio_pkt_temp: AVPacket,
    pub audio_hw_buf_size: libc::c_int,
    pub audio_diff_cum: libc::c_double,
    pub audio_diff_avg_coef: libc::c_double,
    pub audio_diff_threshold: libc::c_double,
    pub audio_diff_avg_count: libc::c_int,
    pub frame_timer: libc::c_double,
    pub frame_last_pts: libc::c_double,
    pub frame_last_delay: libc::c_double,
    pub video_clock: libc::c_double,
    pub video_clock_submitted: libc::c_double,
    pub video_current_pts: libc::c_double,
    pub video_current_pts_time: int64_t,
    pub video_st: *mut AVStream,
    pub pFrame: *mut AVFrame,
    pub filename: [libc::c_char; 1024],
    pub quit: libc::c_int,
    pub frame: *mut AVFrame,
    pub duration: libc::c_double,
    pub fps: libc::c_double,
    pub img_convert_ctx: *mut SwsContext,
}
#[no_mangle]
pub static mut pass: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut test_pts: libc::c_double = 0.0f64;
#[inline]
unsafe extern "C" fn av_q2d(a: AVRational) -> libc::c_double {
    return a.num as libc::c_double / a.den as libc::c_double;
}
#[inline(always)]
unsafe extern "C" fn avio_tell(s: *mut AVIOContext) -> int64_t {
    return avio_seek(s, 0 as libc::c_int as int64_t, 1 as libc::c_int);
}
#[no_mangle]
pub static mut av_log_level: libc::c_int = 32 as libc::c_int;
#[no_mangle]
pub static mut is: *mut VideoState = 0 as *const VideoState as *mut VideoState;
#[no_mangle]
pub static mut myoptions: *mut AVDictionary = 0 as *const AVDictionary as *mut AVDictionary;
#[no_mangle]
pub static mut global_video_state: *mut VideoState = 0 as *const VideoState as *mut VideoState;
#[no_mangle]
pub static mut flush_pkt: AVPacket = AVPacket {
    buf: 0 as *const AVBufferRef as *mut AVBufferRef,
    pts: 0,
    dts: 0,
    data: 0 as *const uint8_t as *mut uint8_t,
    size: 0,
    stream_index: 0,
    flags: 0,
    side_data: 0 as *const AVPacketSideData as *mut AVPacketSideData,
    side_data_elems: 0,
    duration: 0,
    pos: 0,
    convergence_duration: 0,
};
#[no_mangle]
pub static mut pev_best_effort_timestamp: int64_t = 0 as libc::c_int as int64_t;
#[no_mangle]
pub static mut video_stream_index: libc::c_int = -(1 as libc::c_int);
#[no_mangle]
pub static mut audio_stream_index: libc::c_int = -(1 as libc::c_int);
#[no_mangle]
pub static mut have_frame_rate: libc::c_int = 0;
#[no_mangle]
pub static mut stream_index: libc::c_int = 0;
#[no_mangle]
pub static mut best_effort_timestamp: int64_t = 0;
static mut in_file: *mut libc::FILE = 0 as *const libc::FILE as *mut libc::FILE;
static mut sample_file: *mut libc::FILE = 0 as *const libc::FILE as *mut libc::FILE;
static mut timing_file: *mut libc::FILE = 0 as *const libc::FILE as *mut libc::FILE;
#[no_mangle]
pub static mut is_AC3: libc::c_int = 0;
#[no_mangle]
pub static mut AC3_rate: libc::c_int = 0;
#[no_mangle]
pub static mut AC3_mode: libc::c_int = 0;
#[no_mangle]
pub static mut is_h264: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut is_AAC: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut demux_pid: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut demux_asf: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut last_pid: libc::c_int = 0;
#[no_mangle]
pub static mut pids: [libc::c_int; 100] = [0; 100];
#[no_mangle]
pub static mut pid_type: [libc::c_int; 100] = [0; 100];
#[no_mangle]
pub static mut pid_pcr: [libc::c_int; 100] = [0; 100];
#[no_mangle]
pub static mut pid_pid: [libc::c_int; 100] = [0; 100];
#[no_mangle]
pub static mut top_pid_count: [libc::c_int; 8192] = [0; 8192];
#[no_mangle]
pub static mut top_pid_pid: libc::c_int = 0;
#[no_mangle]
pub static mut pid: libc::c_int = 0;
#[no_mangle]
pub static mut selected_video_pid: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut selected_audio_pid: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut selected_subtitle_pid: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut selection_restart_count: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut found_pids: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut pts: int64_t = 0;
#[no_mangle]
pub static mut initial_pts: libc::c_double = 0.0f64;
#[no_mangle]
pub static mut final_pts: int64_t = 0;
#[no_mangle]
pub static mut pts_offset: libc::c_double = 0.0f64;
#[no_mangle]
pub static mut initial_pts_set: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut initial_apts: libc::c_double = 0.;
#[no_mangle]
pub static mut apts_offset: libc::c_double = 0.0f64;
#[no_mangle]
pub static mut initial_apts_set: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut do_audio_repair: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut muxrate: libc::c_int = 0;
#[no_mangle]
pub static mut byterate: libc::c_int = 10000 as libc::c_int;
#[no_mangle]
pub static mut soft_seeking: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut pict_type: libc::c_char = 0;
#[no_mangle]
pub static mut tempstring: [libc::c_char; 512] = [0; 512];
static mut sigint: libc::c_int = 0 as libc::c_int;
static mut verbose: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut selftest_target: libc::c_double = 0.0f64;
#[no_mangle]
pub static mut framenum: libc::c_int = 0;
#[no_mangle]
pub static mut goppos: int64_t = 0;
#[no_mangle]
pub static mut infopos: int64_t = 0;
#[no_mangle]
pub static mut packpos: int64_t = 0;
#[no_mangle]
pub static mut ptspos: int64_t = 0;
#[no_mangle]
pub static mut headerpos: int64_t = 0;
#[no_mangle]
pub static mut frompos: int64_t = 0;
#[no_mangle]
pub static mut SeekPos: int64_t = 0;
#[no_mangle]
pub static mut max_internal_repair_size: libc::c_int = 40 as libc::c_int;
#[no_mangle]
pub static mut reviewing: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut count: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut currentSecond: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut cur_hour: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut cur_minute: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut cur_second: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut reorderCC: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut csRestart: libc::c_int = 0;
#[no_mangle]
pub static mut csStartJump: libc::c_int = 0;
#[no_mangle]
pub static mut csStepping: libc::c_int = 0;
#[no_mangle]
pub static mut csJumping: libc::c_int = 0;
#[no_mangle]
pub static mut csFound: libc::c_int = 0;
#[no_mangle]
pub static mut seekIter: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut seekDirection: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut retries: libc::c_int = 0;
unsafe extern "C" fn signal_handler(sig: libc::c_int) {
    sigint = 1 as libc::c_int;
    signal(sig, None);
}
static mut base_apts: libc::c_double = 0.0f64;
static mut apts: libc::c_double = 0.;
static mut top_apts: libc::c_double = 0.0f64;
static mut audio_buffer: [libc::c_short; 800000] = [0; 800000];
static mut audio_buffer_ptr: *mut libc::c_short = unsafe { audio_buffer.as_ptr() as *mut _ };
static mut audio_samples: libc::c_int = 0 as libc::c_int;
static mut sound_frame_counter: libc::c_int = 0 as libc::c_int;
static mut max_volume_found: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut ms_audio_delay: libc::c_int = 5 as libc::c_int;
#[no_mangle]
pub static mut tracks_without_sound: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut frames_without_sound: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut frames_with_loud_sound: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn retreive_frame_volume(
    from_pts: libc::c_double,
    to_pts: libc::c_double,
) -> libc::c_int {
    let mut buffer: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut volume: libc::c_int = -(1 as libc::c_int);
    let is_0: *mut VideoState = global_video_state;
    let mut i: libc::c_int = 0;
    let mut calculated_delay: libc::c_double = 0.;
    let s_per_frame: libc::c_int = ((to_pts - from_pts)
        * ((*(*(*is_0).audio_st).codec).sample_rate + 1 as libc::c_int) as libc::c_double)
        as libc::c_int;
    if s_per_frame > 1 as libc::c_int && base_apts <= from_pts && to_pts < top_apts {
        calculated_delay = 0.0f64;
        buffer = &mut *audio_buffer.as_mut_ptr().offset(
            ((from_pts - base_apts)
                * ((*(*(*is_0).audio_st).codec).sample_rate as libc::c_double + 0.5f64))
                as libc::c_int as isize,
        ) as *mut libc::c_short;
        volume = 0 as libc::c_int;
        if !sample_file.is_null() {
            libc::fprintf(
                sample_file,
                b"Frame %i\n\0" as *const u8 as *const libc::c_char,
                sound_frame_counter,
            );
        }
        i = 0 as libc::c_int;
        while i < s_per_frame {
            if !sample_file.is_null() {
                libc::fprintf(
                    sample_file,
                    b"%i\n\0" as *const u8 as *const libc::c_char,
                    *buffer as libc::c_int,
                );
            }
            volume += if *buffer as libc::c_int > 0 as libc::c_int {
                *buffer as libc::c_int
            } else {
                -(*buffer as libc::c_int)
            };
            buffer = buffer.offset(1);
            i += 1;
        }
        volume = volume / s_per_frame;
        if !timing_file.is_null() && csStepping == 0 && csJumping == 0 && csStartJump == 0 {
            libc::fprintf(
                timing_file,
                b"%7s, %12.3f, %12.3f, %12.3f, %12.3f, %12.3f, %12.3f, %d\n\0" as *const u8
                    as *const libc::c_char,
                b"a  read\0" as *const u8 as *const libc::c_char,
                (*is_0).audio_clock,
                calculated_delay,
                to_pts,
                from_pts,
                to_pts - from_pts,
                volume as libc::c_double,
                s_per_frame,
            );
        }
        audio_samples -= ((from_pts - base_apts)
            * ((*(*(*is_0).audio_st).codec).sample_rate as libc::c_double + 0.5f64))
            as libc::c_int;
        audio_samples -= s_per_frame;
        if volume == 0 as libc::c_int {
            frames_without_sound += 1;
        } else if volume > 20000 as libc::c_int {
            if volume > 256000 as libc::c_int {
                volume = 220000 as libc::c_int;
            }
            frames_with_loud_sound += 1;
            volume = -(1 as libc::c_int);
        } else {
            frames_without_sound = 0 as libc::c_int;
        }
        if max_volume_found < volume {
            max_volume_found = volume;
        }
        audio_buffer_ptr = audio_buffer.as_mut_ptr();
        if audio_samples > 0 as libc::c_int {
            i = 0 as libc::c_int;
            while i < audio_samples {
                let fresh0 = buffer;
                buffer = buffer.offset(1);
                let fresh1 = audio_buffer_ptr;
                audio_buffer_ptr = audio_buffer_ptr.offset(1);
                *fresh1 = *fresh0;
                i += 1;
            }
        }
        base_apts = to_pts;
        top_apts = base_apts
            + audio_samples as libc::c_double
                / (*(*(*is_0).audio_st).codec).sample_rate as libc::c_double;
        sound_frame_counter += 1;
    }
    return volume;
}
#[no_mangle]
pub unsafe extern "C" fn backfill_frame_volumes() {
    let mut f: libc::c_int = 0;
    let mut volume: libc::c_int = 0;
    if framenum < 3 as libc::c_int {
        return;
    }
    f = framenum - 2 as libc::c_int;
    while get_frame_pts(f) + initial_pts > base_apts && f > 1 as libc::c_int {
        f -= 1;
    }
    while f < framenum - 1 as libc::c_int
        && get_frame_pts(f + 1 as libc::c_int) + initial_pts <= top_apts
        && top_apts - base_apts > 0.2f64
    {
        volume = retreive_frame_volume(
            libm::fmax(get_frame_pts(f) + initial_pts, base_apts),
            get_frame_pts(f + 1 as libc::c_int) + initial_pts,
        );
        if volume > -(1 as libc::c_int) {
            set_frame_volume(f as uint32_t, volume);
        }
        f += 1;
    }
}
#[no_mangle]
pub static mut ALIGN_AC3_PACKETS: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn sound_to_frames(
    is_0: *mut VideoState,
    b: *mut *mut libc::c_short,
    s: libc::c_int,
    c: libc::c_int,
    format: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut volume: libc::c_int = 0;
    static mut old_c: libc::c_int = 0 as libc::c_int;
    let mut old_base_apts: libc::c_double = 0.;
    static mut old_audio_clock: libc::c_double = 0.0f64;
    let mut calculated_delay: libc::c_double = 0.0f64;
    let mut avg_volume: libc::c_double = 0.0f64;
    let planar: libc::c_int = av_sample_fmt_is_planar(format as AVSampleFormat);
    let mut fb: [*mut libc::c_float; 16] = [0 as *mut libc::c_float; 16];
    let mut sb: [*mut libc::c_short; 16] = [0 as *mut libc::c_short; 16];
    static mut old_sample_rate: libc::c_int = 0 as libc::c_int;
    audio_samples =
        audio_buffer_ptr.offset_from(audio_buffer.as_mut_ptr()) as libc::c_long as libc::c_int;
    if old_sample_rate == (*(*(*is_0).audio_st).codec).sample_rate
        && ((audio_buffer_ptr.offset_from(audio_buffer.as_mut_ptr()) as libc::c_long)
            < 0 as libc::c_int as libc::c_long
            || audio_buffer_ptr.offset_from(audio_buffer.as_mut_ptr()) as libc::c_long
                >= 800000 as libc::c_int as libc::c_long
            || (top_apts - base_apts)
                * ((*(*(*is_0).audio_st).codec).sample_rate as libc::c_double + 0.5f64)
                > 800000 as libc::c_int as libc::c_double
            || top_apts < base_apts
            || !(libm::fabs(
                audio_samples as libc::c_double
                    / ((*(*(*is_0).audio_st).codec).sample_rate as libc::c_double + 0.5f64)
                    + base_apts
                    - top_apts,
            ) < 0.001f64)
            || audio_samples < 0 as libc::c_int
            || audio_samples >= 800000 as libc::c_int)
    {
        Debug(
            1 as libc::c_int,
            b"Panic: Audio buffering corrupt\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        audio_buffer_ptr = audio_buffer.as_mut_ptr();
        base_apts = 0 as libc::c_int as libc::c_double;
        top_apts = base_apts;
        audio_samples = 0 as libc::c_int;
        return;
    }
    if old_c != 0 as libc::c_int && old_c != c {
        Debug(
            5 as libc::c_int,
            b"Audio channels switched at pts=%6.5f from %d to %d\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            base_apts,
            old_c,
            c,
        );
    }
    audio_channels = c;
    old_c = c;
    if old_sample_rate != 0 as libc::c_int
        && old_sample_rate != (*(*(*is_0).audio_st).codec).sample_rate
    {
        Debug(
            5 as libc::c_int,
            b"Audio samplerate switched from %d to %d\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            old_sample_rate,
            (*(*(*is_0).audio_st).codec).sample_rate,
        );
    }
    old_sample_rate = (*(*(*is_0).audio_st).codec).sample_rate;
    old_base_apts = base_apts;
    if libm::fabs(
        base_apts
            - ((*is_0).audio_clock
                - audio_samples as libc::c_double
                    / (*(*(*is_0).audio_st).codec).sample_rate as libc::c_double),
    ) > 0.0001f64
    {
        base_apts = (*is_0).audio_clock
            - audio_samples as libc::c_double
                / (*(*(*is_0).audio_st).codec).sample_rate as libc::c_double;
    }
    if ALIGN_AC3_PACKETS != 0
        && (*(*(*is_0).audio_st).codec).codec_id as libc::c_uint
            == AV_CODEC_ID_AC3 as libc::c_int as libc::c_uint
    {
        if libm::fabs(base_apts - old_base_apts - 0.032f64) < 0.001f64
            || libm::fabs(base_apts - old_base_apts - -0.032f64) < 0.001f64
            || libm::fabs(base_apts - old_base_apts - 0.064f64) < 0.001f64
            || libm::fabs(base_apts - old_base_apts - -0.064f64) < 0.001f64
            || libm::fabs(base_apts - old_base_apts - -0.096f64) < 0.001f64
        {
            old_base_apts = base_apts;
        }
    }
    if old_base_apts != 0.0f64 && libm::fabs(base_apts - old_base_apts) > 0.01f64 {
        Debug(
            8 as libc::c_int,
            b"Jump in base apts from %6.5f to %6.5f, delta=%6.5f\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            old_base_apts,
            base_apts,
            base_apts - old_base_apts,
        );
    }
    if s + audio_samples > 800000 as libc::c_int {
        Debug(
            1 as libc::c_int,
            b"Panic: Audio buffer overflow, resetting audio buffer\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        audio_buffer_ptr = audio_buffer.as_mut_ptr();
        base_apts = 0 as libc::c_int as libc::c_double;
        top_apts = base_apts;
        audio_samples = 0 as libc::c_int;
        return;
    }
    if s > 0 as libc::c_int {
        if format == AV_SAMPLE_FMT_FLTP as libc::c_int {
            l = 0 as libc::c_int;
            while l < (*(*(*is_0).audio_st).codec).channels {
                fb[l as usize] = *b.offset(l as isize) as *mut libc::c_float;
                l += 1;
            }
            i = 0 as libc::c_int;
            while i < s {
                volume = 0 as libc::c_int;
                if planar != 0 {
                    l = 0 as libc::c_int;
                    while l < (*(*(*is_0).audio_st).codec).channels {
                        let fresh2 = fb[l as usize];
                        fb[l as usize] = (fb[l as usize]).offset(1);
                        volume = (volume as libc::c_float
                            + *fresh2 * 64000 as libc::c_int as libc::c_float)
                            as libc::c_int;
                        l += 1;
                    }
                } else {
                    l = 0 as libc::c_int;
                    while l < (*(*(*is_0).audio_st).codec).channels {
                        let fresh3 = fb[0 as libc::c_int as usize];
                        fb[0 as libc::c_int as usize] = (fb[0 as libc::c_int as usize]).offset(1);
                        volume = (volume as libc::c_float
                            + *fresh3 * 64000 as libc::c_int as libc::c_float)
                            as libc::c_int;
                        l += 1;
                    }
                }
                let fresh4 = audio_buffer_ptr;
                audio_buffer_ptr = audio_buffer_ptr.offset(1);
                *fresh4 = (volume / (*(*(*is_0).audio_st).codec).channels) as libc::c_short;
                avg_volume += libc::abs(volume / (*(*(*is_0).audio_st).codec).channels) as libc::c_double;
                i += 1;
            }
        } else {
            l = 0 as libc::c_int;
            while l < (*(*(*is_0).audio_st).codec).channels {
                sb[l as usize] = *b.offset(l as isize);
                l += 1;
            }
            i = 0 as libc::c_int;
            while i < s {
                volume = 0 as libc::c_int;
                if planar != 0 {
                    l = 0 as libc::c_int;
                    while l < (*(*(*is_0).audio_st).codec).channels {
                        let fresh5 = sb[l as usize];
                        sb[l as usize] = (sb[l as usize]).offset(1);
                        volume += *fresh5 as libc::c_int;
                        l += 1;
                    }
                } else {
                    l = 0 as libc::c_int;
                    while l < (*(*(*is_0).audio_st).codec).channels {
                        let fresh6 = sb[0 as libc::c_int as usize];
                        sb[0 as libc::c_int as usize] = (sb[0 as libc::c_int as usize]).offset(1);
                        volume += *fresh6 as libc::c_int;
                        l += 1;
                    }
                }
                let fresh7 = audio_buffer_ptr;
                audio_buffer_ptr = audio_buffer_ptr.offset(1);
                *fresh7 = (volume / (*(*(*is_0).audio_st).codec).channels) as libc::c_short;
                avg_volume += libc::abs(volume / (*(*(*is_0).audio_st).codec).channels) as libc::c_double;
                i += 1;
            }
        }
    }
    avg_volume /= s as libc::c_double;
    audio_samples =
        audio_buffer_ptr.offset_from(audio_buffer.as_mut_ptr()) as libc::c_long as libc::c_int;
    top_apts = base_apts
        + audio_samples as libc::c_double
            / (*(*(*is_0).audio_st).codec).sample_rate as libc::c_double;
    calculated_delay = (*is_0).audio_clock - old_audio_clock;
    if !timing_file.is_null() && csStepping == 0 && csJumping == 0 && csStartJump == 0 {
        libc::fprintf(
            timing_file,
            b"%7s, %12.3f, %12.3f, %12.3f, %12.3f, %12.3f, %12.3f, %d\n\0" as *const u8
                as *const libc::c_char,
            b"a frame\0" as *const u8 as *const libc::c_char,
            (*is_0).audio_clock,
            calculated_delay,
            top_apts,
            base_apts,
            top_apts - base_apts,
            avg_volume,
            s,
        );
    }
    old_audio_clock = (*is_0).audio_clock;
    backfill_frame_volumes();
}
static mut ac3_packet: [uint8_t; 100000] = [0; 100000];
static mut ac3_packet_index: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut data_size: libc::c_int = 0;
#[no_mangle]
pub static mut ac3_package_misalignment_count: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn audio_packet_process(mut is_0: *mut VideoState, pkt: *mut AVPacket) {
    let mut prev_codec_id: libc::c_int = -(1 as libc::c_int);
    let mut len1: libc::c_int = 0;
    let mut data_size_0: libc::c_int = 0;
    let mut pp: *mut uint8_t = 0 as *mut uint8_t;
    let mut prev_audio_clock: libc::c_double = 0.;
    let mut rps: libc::c_int = 0;
    let mut ps: libc::c_int = 0;
    let mut pkt_temp: *mut AVPacket = &mut (*is_0).audio_pkt_temp;
    let mut got_frame: libc::c_int = 0;
    if reviewing == 0 {
        dump_audio_start();
        dump_audio(
            (*pkt).data as *mut libc::c_char,
            ((*pkt).data).offset((*pkt).size as isize) as *mut libc::c_char,
        );
    }
    let ref mut fresh8 = (*pkt_temp).data;
    *fresh8 = (*pkt).data;
    (*pkt_temp).size = (*pkt).size;
    if ALIGN_AC3_PACKETS == 0
        && (*(*(*is_0).audio_st).codec).codec_id as libc::c_uint
            == AV_CODEC_ID_AC3 as libc::c_int as libc::c_uint
        && (*((*pkt_temp).data).offset(0 as libc::c_int as isize) as libc::c_int
            != 0xb as libc::c_int
            || *((*pkt_temp).data).offset(1 as libc::c_int as isize) as libc::c_int
                != 0x77 as libc::c_int)
    {
        ac3_package_misalignment_count += 1;
    } else {
        ac3_package_misalignment_count = 0 as libc::c_int;
    }
    if ALIGN_AC3_PACKETS == 0 && ac3_package_misalignment_count > 4 as libc::c_int {
        Debug(
            8 as libc::c_int,
            b"AC3 packets misaligned, enabling AC3 re-alignment\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        ALIGN_AC3_PACKETS = 1 as libc::c_int;
    }
    if ALIGN_AC3_PACKETS != 0
        && (*(*(*is_0).audio_st).codec).codec_id as libc::c_uint
            == AV_CODEC_ID_AC3 as libc::c_int as libc::c_uint
    {
        if ac3_packet_index + (*pkt_temp).size >= 100000 as libc::c_int {
            Debug(
                8 as libc::c_int,
                b"AC3 sync error\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            return;
        }
        libc::memcpy(
            &mut *ac3_packet.as_mut_ptr().offset(ac3_packet_index as isize) as *mut uint8_t
                as *mut libc::c_void,
            (*pkt_temp).data as *const libc::c_void,
            (*pkt_temp).size as usize,
        );
        let ref mut fresh9 = (*pkt_temp).data;
        *fresh9 = ac3_packet.as_mut_ptr();
        (*pkt_temp).size += ac3_packet_index;
        ac3_packet_index = (*pkt_temp).size;
        ps = 0 as libc::c_int;
        while (*pkt_temp).size >= 2 as libc::c_int
            && (*((*pkt_temp).data).offset(0 as libc::c_int as isize) as libc::c_int
                != 0xb as libc::c_int
                || *((*pkt_temp).data).offset(1 as libc::c_int as isize) as libc::c_int
                    != 0x77 as libc::c_int)
        {
            let ref mut fresh10 = (*pkt_temp).data;
            *fresh10 = (*fresh10).offset(1);
            let ref mut fresh11 = (*pkt_temp).size;
            *fresh11 -= 1;
            ps += 1;
        }
        if (*pkt_temp).size < 2 as libc::c_int {
            return;
        }
        if ps > 0 as libc::c_int {
            Debug(
                8 as libc::c_int,
                b"Skipped %d of added %d bytes in audio input stream around frame %d\n\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                ps,
                (*pkt).size,
                framenum,
            );
        }
        pp = (*pkt_temp).data;
        rps = (*pkt_temp).size - 2 as libc::c_int;
        while rps > 1 as libc::c_int
            && (*pp.offset(rps as isize) as libc::c_int != 0xb as libc::c_int
                || *pp.offset((rps + 1 as libc::c_int) as isize) as libc::c_int
                    != 0x77 as libc::c_int)
        {
            rps -= 1;
        }
        if rps >= 2 as libc::c_int {
            (*pkt_temp).size = rps;
        } else {
            rps = (*pkt_temp).size;
            pp = &mut *((*pkt_temp).data).offset(0 as libc::c_int as isize) as *mut uint8_t;
            (*pkt_temp).size = 0 as libc::c_int;
            return;
        }
        if (*pkt_temp).size % 768 as libc::c_int != 0 as libc::c_int {
            Debug(
                8 as libc::c_int,
                b"Strange packet size of %d bytes in audio input stream around frame %d\n\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                rps,
                framenum,
            );
        }
    }
    if (*pkt).pts != 0x8000000000000000 as libc::c_ulonglong as int64_t {
        prev_audio_clock = (*is_0).audio_clock;
        (*is_0).audio_clock = av_q2d((*(*is_0).audio_st).time_base)
            * ((*pkt).pts
                - (if (*(*is_0).audio_st).start_time
                    != 0x8000000000000000 as libc::c_ulonglong as int64_t
                {
                    (*(*is_0).audio_st).start_time
                } else {
                    0 as libc::c_int as libc::c_longlong
                })) as libc::c_double
            - apts_offset;
        if ALIGN_AC3_PACKETS != 0
            && (*(*(*is_0).audio_st).codec).codec_id as libc::c_uint
                == AV_CODEC_ID_AC3 as libc::c_int as libc::c_uint
        {
            if libm::fabs((*is_0).audio_clock - prev_audio_clock - 0.032f64) < 0.001f64
                || libm::fabs((*is_0).audio_clock - prev_audio_clock - -0.032f64) < 0.001f64
                || libm::fabs((*is_0).audio_clock - prev_audio_clock - 0.064f64) < 0.001f64
                || libm::fabs((*is_0).audio_clock - prev_audio_clock - -0.064f64) < 0.001f64
                || libm::fabs((*is_0).audio_clock - prev_audio_clock - -0.096f64) < 0.001f64
            {
                prev_audio_clock = (*is_0).audio_clock;
            }
        }
        if initial_apts_set != 0
            && (*is_0).audio_clock != 0.0f64
            && libm::fabs((*is_0).audio_clock - prev_audio_clock) > 0.02f64
        {
            if do_audio_repair != 0
                && libm::fabs((*is_0).audio_clock - prev_audio_clock) < 1 as libc::c_int as libc::c_double
            {
                (*is_0).audio_clock = prev_audio_clock;
            } else {
                Debug(
                    8 as libc::c_int,
                    b"Strange audio pts step of %6.5f instead of %6.5f at frame %d\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    (*is_0).audio_clock - prev_audio_clock + 0.0005f64,
                    0.0f64,
                    framenum,
                );
                do_audio_repair != 0;
            }
        }
        if initial_apts_set == 0 {
            initial_apts = (*is_0).audio_clock;
            Debug(
                10 as libc::c_int,
                b"\nInitial audio pts = %10.3f\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                initial_apts,
            );
        }
    }
    initial_apts_set = 1 as libc::c_int;
    while (*pkt_temp).size > 0 as libc::c_int {
        let ref mut fresh12 = (*is_0).frame;
        *fresh12 = av_frame_alloc();
        if (*fresh12).is_null() {
            return;
        }
        len1 = avcodec_decode_audio4(
            (*(*is_0).audio_st).codec,
            (*is_0).frame,
            &mut got_frame,
            pkt_temp,
        );
        if prev_codec_id != -(1 as libc::c_int)
            && prev_codec_id as libc::c_uint
                != (*(*(*is_0).audio_st).codec).codec_id as libc::c_uint
        {
            Debug(
                2 as libc::c_int,
                b"Audio format change\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        prev_codec_id = (*(*(*is_0).audio_st).codec).codec_id as libc::c_int;
        if len1 < 0 as libc::c_int && ALIGN_AC3_PACKETS == 0 {
            (*pkt_temp).size = 0 as libc::c_int;
            if (*(*(*is_0).audio_st).codec).codec_id as libc::c_uint
                == AV_CODEC_ID_AC3 as libc::c_int as libc::c_uint
            {
                ac3_packet_index = 0 as libc::c_int;
            }
            break;
        } else if len1 < 0 as libc::c_int && ALIGN_AC3_PACKETS != 0 {
            len1 = 2 as libc::c_int;
            let ref mut fresh13 = (*pkt_temp).data;
            *fresh13 = (*fresh13).offset(len1 as isize);
            (*pkt_temp).size -= len1;
            break;
        } else {
            let ref mut fresh14 = (*pkt_temp).data;
            *fresh14 = (*fresh14).offset(len1 as isize);
            (*pkt_temp).size -= len1;
            if !(got_frame == 0) {
                data_size_0 = av_samples_get_buffer_size(
                    0 as *mut libc::c_int,
                    (*(*is_0).frame).channels,
                    (*(*is_0).frame).nb_samples,
                    (*(*is_0).frame).format as AVSampleFormat,
                    1 as libc::c_int,
                );
                if data_size_0 > 0 as libc::c_int {
                    sound_to_frames(
                        is_0,
                        ((*(*is_0).frame).data).as_mut_ptr() as *mut *mut libc::c_short,
                        (*(*is_0).frame).nb_samples,
                        (*(*is_0).frame).channels,
                        (*(*is_0).frame).format,
                    );
                }
                (*is_0).audio_clock += data_size_0 as libc::c_double
                    / ((*(*is_0).frame).channels
                        * (*(*is_0).frame).sample_rate
                        * av_get_bytes_per_sample((*(*is_0).frame).format as AVSampleFormat))
                        as libc::c_double;
                av_frame_free(&mut (*is_0).frame);
            }
        }
    }
    if ALIGN_AC3_PACKETS != 0
        && (*(*(*is_0).audio_st).codec).codec_id as libc::c_uint
            == AV_CODEC_ID_AC3 as libc::c_int as libc::c_uint
    {
        ps = 0 as libc::c_int;
        rps =
            ((*pkt_temp).data).offset_from(ac3_packet.as_mut_ptr()) as libc::c_long as libc::c_int;
        while (0 as libc::c_int) < ac3_packet_index - rps {
            ac3_packet[ps as usize] = ac3_packet[rps as usize];
            ps += 1;
            rps += 1;
        }
        ac3_packet_index = ps;
    }
}
unsafe extern "C" fn print_fps(final_0: libc::c_int) -> libc::c_double {
    static mut frame_counter: uint32_t = 0 as libc::c_int as uint32_t;
    static mut tv_beg: libc::timeval = libc::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    static mut tv_start: libc::timeval = libc::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    static mut total_elapsed: libc::c_int = 0;
    static mut last_count: libc::c_int = 0 as libc::c_int;
    let mut tv_end: libc::timeval = libc::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut fps: libc::c_double = 0.;
    let mut tfps: libc::c_double = 0.;
    let mut frames: libc::c_int = 0;
    let mut elapsed: libc::c_int = 0;
    let mut cur_pos: [libc::c_char; 100] = *::std::mem::transmute::<
        &[u8; 100],
        &mut [libc::c_char; 100],
    >(
        b"0:00:00\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    if verbose != 0 {
        return 0.0f64;
    }
    if csStepping != 0 {
        return 0.0f64;
    }
    if final_0 < 0 as libc::c_int {
        frame_counter = 0 as libc::c_int as uint32_t;
        last_count = 0 as libc::c_int;
        return 0.0f64;
    }
    libc::gettimeofday(&mut tv_end, 0 as *mut libc::c_void);
    if frame_counter == 0 {
        tv_beg = tv_end;
        tv_start = tv_beg;
        signal(
            2 as libc::c_int,
            Some(signal_handler as unsafe extern "C" fn(libc::c_int) -> ()),
        );
    }
    elapsed = ((tv_end.tv_sec - tv_beg.tv_sec) * 100 as libc::c_int as libc::c_long
        + ((tv_end.tv_usec - tv_beg.tv_usec) / 10000 as libc::c_int) as libc::c_long)
        as libc::c_int;
    total_elapsed = ((tv_end.tv_sec - tv_start.tv_sec) * 100 as libc::c_int as libc::c_long
        + ((tv_end.tv_usec - tv_start.tv_usec) / 10000 as libc::c_int) as libc::c_long)
        as libc::c_int;
    if final_0 != 0 {
        if total_elapsed != 0 {
            tfps = frame_counter as libc::c_double * 100.0f64 / total_elapsed as libc::c_double;
        } else {
            tfps = 0 as libc::c_int as libc::c_double;
        }
        libc::fprintf(
            __stderrp,
            b"\n%d frames decoded in %.2f seconds (%.2f fps)\n\0" as *const u8
                as *const libc::c_char,
            frame_counter,
            total_elapsed as libc::c_double / 100.0f64,
            tfps,
        );
        libc::fflush(__stderrp);
        return tfps;
    }
    frame_counter = frame_counter.wrapping_add(1);
    frames = frame_counter.wrapping_sub(last_count as libc::c_uint) as libc::c_int;
    if elapsed < 100 as libc::c_int {
        return 0.0f64;
    }
    tv_beg = tv_end;
    cur_second = (framenum as libc::c_double / get_fps()) as libc::c_int;
    cur_hour = cur_second / (60 as libc::c_int * 60 as libc::c_int);
    cur_second -= cur_hour * 60 as libc::c_int * 60 as libc::c_int;
    cur_minute = cur_second / 60 as libc::c_int;
    cur_second -= cur_minute * 60 as libc::c_int;
    libc::sprintf(
        cur_pos.as_mut_ptr(),
        b"%2i:%.2i:%.2i\0" as *const u8 as *const libc::c_char,
        cur_hour,
        cur_minute,
        cur_second,
    );
    fps = frames as libc::c_double * 100.0f64 / elapsed as libc::c_double;
    tfps = frame_counter as libc::c_double * 100.0f64 / total_elapsed as libc::c_double;
    libc::fprintf(
        __stderrp,
        b"%s - %d frames in %.2f sec(%.2f fps), %.2f sec(%.2f fps), %d%%\r\0" as *const u8
            as *const libc::c_char,
        cur_pos.as_mut_ptr(),
        frame_counter,
        total_elapsed as libc::c_double / 100.0f64,
        tfps,
        elapsed as libc::c_double / 100.0f64,
        fps,
        (100.0f64 * framenum as libc::c_double / get_fps() / (*global_video_state).duration)
            as libc::c_int,
    );
    libc::fflush(__stderrp);
    last_count = frame_counter as libc::c_int;
    return tfps;
}
#[no_mangle]
pub unsafe extern "C" fn SubmitFrame(
    video_st: *mut AVStream,
    pFrame: *mut AVFrame,
    pts_0: libc::c_double,
) -> libc::c_int {
    let mut res: libc::c_int = 0 as libc::c_int;
    let mut changed: libc::c_int = 0 as libc::c_int;
    let mut line: libc::c_int = 0 as libc::c_int;
    if (*pFrame).format == AV_PIX_FMT_YUV420P10LE as libc::c_int {
        line = 1 as libc::c_int;
    }
    if (*pFrame).linesize[line as usize] > 3840 as libc::c_int
        || (*pFrame).height > 1200 as libc::c_int
        || (*pFrame).linesize[line as usize] < 100 as libc::c_int
        || (*pFrame).height < 100 as libc::c_int
    {
        Debug(
            1 as libc::c_int,
            b"Panic: illegal height (%d), width (%d) or frame period (%d)\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            (*pFrame).height,
            (*pFrame).width,
            (*pFrame).linesize[line as usize],
        );
        frame_ptr = 0 as *mut libc::c_uchar;
        return 0 as libc::c_int;
    }
    if height != (*pFrame).height
        && (*pFrame).height > 100 as libc::c_int
        && (*pFrame).height < 2000 as libc::c_int
    {
        height = (*pFrame).height;
        changed = 1 as libc::c_int;
    }
    if width != (*pFrame).linesize[line as usize]
        && (*pFrame).linesize[line as usize] > 100 as libc::c_int
        && (*pFrame).linesize[line as usize] < 3841 as libc::c_int
    {
        width = (*pFrame).linesize[line as usize];
        changed = 1 as libc::c_int;
    }
    if videowidth
        != (*pFrame).width
            / (if line == 0 as libc::c_int {
                1 as libc::c_int
            } else {
                2 as libc::c_int
            })
        && (*pFrame).width > 100 as libc::c_int
        && (*pFrame).width < 2000 as libc::c_int
    {
        videowidth = (*pFrame).width
            / (if line == 0 as libc::c_int {
                1 as libc::c_int
            } else {
                2 as libc::c_int
            });
        changed = 1 as libc::c_int;
    }
    if changed != 0 {
        Debug(
            5 as libc::c_int,
            b"Format changed to [%d : %d]\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            videowidth,
            height,
        );
    }
    infopos = headerpos;
    frame_ptr = (*pFrame).data[line as usize];
    if frame_ptr.is_null() {
        return 0 as libc::c_int;
    }
    if (*pFrame).pict_type as libc::c_uint == AV_PICTURE_TYPE_B as libc::c_int as libc::c_uint {
        pict_type = 'B' as i32 as libc::c_char;
    } else if (*pFrame).pict_type as libc::c_uint
        == AV_PICTURE_TYPE_I as libc::c_int as libc::c_uint
    {
        pict_type = 'I' as i32 as libc::c_char;
    } else {
        pict_type = 'P' as i32 as libc::c_char;
    }
    if selftest == 2 as libc::c_int
        && framenum == 0 as libc::c_int
        && pass == 0 as libc::c_int
        && test_pts == 0.0f64
    {
        test_pts = pts_0;
    }
    if selftest == 2 as libc::c_int && pass > 0 as libc::c_int {
        if test_pts != pts_0 {
            sample_file = libc::fopen(
                b"seektest.log\0" as *const u8 as *const libc::c_char,
                b"a+\0" as *const u8 as *const libc::c_char,
            );
            libc::fprintf(
                sample_file,
                b"Reset file Failed, initial pts = %6.3f, seek pts = %6.3f, pass = %d, \"%s\"\n\0"
                    as *const u8 as *const libc::c_char,
                test_pts,
                pts_0,
                pass + 1 as libc::c_int,
                ((*is).filename).as_mut_ptr(),
            );
            libc::fclose(sample_file);
            Debug(
                1 as libc::c_int,
                b"\nSelftest %d FAILED: Reset\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                selftest,
            );
        } else {
            Debug(
                1 as libc::c_int,
                b"\nSelftest 2 OK: Reset\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        ::std::process::exit(1 as libc::c_int);
    }
    if reviewing == 0 {
        print_fps(0 as libc::c_int);
        res = DetectCommercials(framenum, pts_0);
        framenum += 1;
        if selftest == 2 as libc::c_int && pass == 0 as libc::c_int && framenum > 20 as libc::c_int
        {
            res = 1 as libc::c_int;
            pass += 1;
        }
        if res != 0 {
            framenum = 0 as libc::c_int;
            sound_frame_counter = 0 as libc::c_int;
            (*is).seek_req = 1 as libc::c_int;
            (*is).seek_pos = 0 as libc::c_int as int64_t;
            (*is).seek_pts = 0.0f64;
        }
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn Set_seek(mut is_0: *mut VideoState, mut pts_0: libc::c_double) {
    let ic: *mut AVFormatContext = (*is_0).pFormatCtx;
    let length: libc::c_double = (*is_0).duration;
    (*is_0).seek_flags = 4 as libc::c_int;
    (*is_0).seek_flags = 1 as libc::c_int;
    (*is_0).seek_req = 1 as libc::c_int;
    (*is_0).seek_pts = pts_0;
    pts_0 = libm::fmax(0.0f64, pts_0 - 2.0f64);
    if (*is_0).seek_by_bytes != 0 {
        let size: uint64_t = avio_size((*ic).pb) as uint64_t;
        if length < 0 as libc::c_int as libc::c_double {
            (*is_0).seek_pos = (size as libc::c_double
                * libm::fmax(0 as libc::c_int as libc::c_double, pts_0 - 4.0f64)
                / (frame_count as libc::c_double * get_fps()))
                as int64_t;
        } else {
            (*is_0).seek_pos = (size as libc::c_double
                * libm::fmax(0 as libc::c_int as libc::c_double, pts_0 - 4.0f64)
                / length) as int64_t;
        }
        (*is_0).seek_flags |= 2 as libc::c_int;
    } else {
        pts_0 = libm::fmax(0 as libc::c_int as libc::c_double, pts_0 + initial_pts);
        (*is_0).seek_pos = (pts_0 / av_q2d((*(*is_0).video_st).time_base)) as int64_t;
        if (*(*is_0).video_st).start_time != 0x8000000000000000 as libc::c_ulonglong as int64_t {
            let ref mut fresh15 = (*is_0).seek_pos;
            *fresh15 += (*(*is_0).video_st).start_time;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn DoSeekRequest(mut is_0: *mut VideoState) {
    let mut ret: libc::c_int = 0;
    loop {
        ret = av_seek_frame(
            (*is_0).pFormatCtx,
            (*is_0).videoStream,
            (*is_0).seek_pos,
            (*is_0).seek_flags,
        );
        pev_best_effort_timestamp = 0 as libc::c_int as int64_t;
        best_effort_timestamp = 0 as libc::c_int as int64_t;
        (*is_0).video_clock = 0.0f64;
        (*is_0).audio_clock = 0.0f64;
        if !(ret < 0 as libc::c_int) {
            break;
        }
        let mut error_text: *mut libc::c_char = 0 as *mut libc::c_char;
        if ((*(*(*is_0).pFormatCtx).iformat).read_seek).is_some() {
            error_text =
                b"Format specific\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        } else if ((*(*(*is_0).pFormatCtx).iformat).read_timestamp).is_some() {
            error_text = b"Frame binary\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        } else {
            error_text = b"Generic\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        libc::fprintf(
            __stderrp,
            b"%s error while seeking. target=%6.3f, \"%s\"\n\0" as *const u8 as *const libc::c_char,
            error_text,
            (*is_0).seek_pts,
            ((*(*is_0).pFormatCtx).filename).as_mut_ptr(),
        );
        if selftest != 0 {
            sample_file = libc::fopen(
                b"seektest.log\0" as *const u8 as *const libc::c_char,
                b"a+\0" as *const u8 as *const libc::c_char,
            );
            libc::fprintf(
                sample_file,
                b"%s error while seeking, target=%6.3f, \"%s\"\n\0" as *const u8
                    as *const libc::c_char,
                error_text,
                (*is_0).seek_pts,
                ((*(*is_0).pFormatCtx).filename).as_mut_ptr(),
            );
            libc::fclose(sample_file);
        }
        if !((*is_0).seek_by_bytes == 0) {
            break;
        }
        (*is_0).seek_by_bytes = 1 as libc::c_int;
        Set_seek(is_0, (*is_0).seek_pts);
    }
    if (*is_0).seek_no_flush == 0 {
        if (*is_0).audioStream >= 0 as libc::c_int {
            avcodec_flush_buffers((*(*is_0).audio_st).codec);
        }
        if (*is_0).videoStream >= 0 as libc::c_int {
            avcodec_flush_buffers((*(*is_0).video_st).codec);
        }
    }
    (*is_0).seek_no_flush = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn DecodeOnePicture(f: *mut libc::FILE, pts_0: libc::c_double) {
    let mut is_0: *mut VideoState = global_video_state;
    let mut packet: *mut AVPacket = 0 as *mut AVPacket;
    file_open();
    is_0 = global_video_state;
    reviewing = 1 as libc::c_int;
    Set_seek(is_0, pts_0);
    pev_best_effort_timestamp = 0 as libc::c_int as int64_t;
    best_effort_timestamp = 0 as libc::c_int as int64_t;
    pts_offset = 0.0f64;
    frame_ptr = 0 as *mut libc::c_uchar;
    packet = &mut (*is_0).audio_pkt;
    let mut current_block_20: u64;
    's_49: while !((*is_0).quit != 0) {
        if (*is_0).seek_req != 0 {
            current_block_20 = 6081211954207503520;
        } else {
            current_block_20 = 1891801507340478169;
        }
        loop {
            match current_block_20 {
                6081211954207503520 => {
                    DoSeekRequest(is_0);
                    current_block_20 = 1891801507340478169;
                }
                _ => {
                    if av_read_frame((*is_0).pFormatCtx, packet) < 0 as libc::c_int {
                        break 's_49;
                    }
                    if !((*is_0).seek_req != 0) {
                        break;
                    }
                    let packet_time: libc::c_double = ((*packet).pts
                        - (if (*(*is_0).video_st).start_time
                            != 0x8000000000000000 as libc::c_ulonglong as int64_t
                        {
                            (*(*is_0).video_st).start_time
                        } else {
                            0 as libc::c_int as libc::c_longlong
                        })) as libc::c_double
                        * av_q2d((*(*is_0).video_st).time_base);
                    if (*packet).pts == 0x8000000000000000 as libc::c_ulonglong as int64_t {
                        av_packet_unref(packet);
                        current_block_20 = 1891801507340478169;
                    } else if (*is_0).seek_req < 6 as libc::c_int
                        && (*is_0).seek_flags & 2 as libc::c_int != 0
                        && (*is_0).duration > 0 as libc::c_int as libc::c_double
                        && libm::fabs(packet_time - ((*is_0).seek_pts - 2.5f64))
                            < (*is_0).duration
                                / (10 as libc::c_int * (*is_0).seek_req) as libc::c_double
                    {
                        let ref mut fresh16 = (*is_0).seek_pos;
                        *fresh16 = (*fresh16 as libc::c_double
                            + ((*is_0).seek_pts - 2.5f64 - packet_time) / (*is_0).duration
                                * avio_size((*(*is_0).pFormatCtx).pb) as libc::c_double
                                * 1.1f64) as int64_t;
                        let ref mut fresh17 = (*is_0).seek_req;
                        *fresh17 += 1;
                        current_block_20 = 6081211954207503520;
                    } else {
                        (*is_0).seek_req = 0 as libc::c_int;
                        break;
                    }
                }
            }
        }
        (*is_0).seek_req = 0 as libc::c_int;
        if (*packet).stream_index == (*is_0).videoStream {
            retries = 1 as libc::c_int;
            if video_packet_process(is_0, packet) != 0 {
                if retries == 0 as libc::c_int {
                    av_packet_unref(packet);
                    break;
                }
            }
        } else {
            (*packet).stream_index == (*is_0).audioStream;
        }
        av_packet_unref(packet);
    }
    reviewing = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn raise_exception() {
    asm!("brk #0x1", options(preserves_flags));
}
#[no_mangle]
pub unsafe extern "C" fn filter() -> libc::c_int {
    libc::printf(b"Exception raised, Comskip is terminating\n\0" as *const u8 as *const libc::c_char);
    ::std::process::exit(99 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn video_packet_process(
    mut is_0: *mut VideoState,
    packet: *mut AVPacket,
) -> libc::c_int {
    let current_block: u64;
    let mut frame_delay: libc::c_double = 0.;
    let mut len1: libc::c_int = 0;
    let mut frameFinished: libc::c_int = 0;
    let mut repeat: libc::c_int = 0;
    let mut pts_0: libc::c_double = 0.;
    let mut real_pts: libc::c_double = 0.;
    static mut find_29fps: libc::c_int = 0 as libc::c_int;
    static mut force_29fps: libc::c_int = 0 as libc::c_int;
    static mut find_25fps: libc::c_int = 0 as libc::c_int;
    static mut force_25fps: libc::c_int = 0 as libc::c_int;
    static mut find_24fps: libc::c_int = 0 as libc::c_int;
    static mut force_24fps: libc::c_int = 0 as libc::c_int;
    static mut prev_pts: libc::c_double = 0.0f64;
    static mut prev_real_pts: libc::c_double = 0.0f64;
    static mut prev_strange_step: libc::c_double = 0.0f64;
    static mut prev_strange_framenum: libc::c_int = 0 as libc::c_int;
    let mut calculated_delay: libc::c_double = 0.;
    if reviewing == 0 {
        dump_video_start();
        dump_video(
            (*packet).data as *mut libc::c_char,
            ((*packet).data).offset((*packet).size as isize) as *mut libc::c_char,
        );
    }
    real_pts = 0.0f64;
    pts_0 = 0 as libc::c_int as libc::c_double;
    if hardware_decode == 0 {
        (*(*(*is_0).video_st).codec).flags |= (1 as libc::c_int) << 13 as libc::c_int;
    }
    len1 = avcodec_decode_video2(
        (*(*is_0).video_st).codec,
        (*is_0).pFrame,
        &mut frameFinished,
        packet,
    );
    len1 < 0 as libc::c_int;
    if frameFinished != 0 {
        if (*(*is_0).pFrame).format == AV_PIX_FMT_YUV420P10LE as libc::c_int {
            let ref mut fresh18 = (*is_0).img_convert_ctx;
            *fresh18 = sws_getCachedContext(
                (*is_0).img_convert_ctx,
                (*(*is_0).pFrame).width,
                (*(*is_0).pFrame).height,
                (*(*is_0).pFrame).format as AVPixelFormat,
                (*(*is_0).pFrame).width,
                (*(*is_0).pFrame).height,
                AV_PIX_FMT_YUV420P,
                0x10 as libc::c_int,
                0 as *mut SwsFilter,
                0 as *mut SwsFilter,
                0 as *const libc::c_double,
            );
            let mut newframe: *mut AVFrame = av_frame_alloc();
            av_frame_copy_props(newframe, (*is_0).pFrame);
            (*newframe).format = AV_PIX_FMT_YUV420P as libc::c_int;
            (*newframe).width = (*(*is_0).pFrame).width;
            (*newframe).height = (*(*is_0).pFrame).height;
            av_frame_get_buffer(newframe, 0 as libc::c_int);
            sws_scale(
                (*is_0).img_convert_ctx,
                ((*(*is_0).pFrame).data).as_mut_ptr() as *const *const uint8_t,
                ((*(*is_0).pFrame).linesize).as_mut_ptr() as *const libc::c_int,
                0 as libc::c_int,
                (*(*is_0).pFrame).height,
                ((*newframe).data).as_mut_ptr() as *const *mut uint8_t,
                ((*newframe).linesize).as_mut_ptr() as *const libc::c_int,
            );
            av_frame_unref((*is_0).pFrame);
            let ref mut fresh19 = (*is_0).pFrame;
            *fresh19 = newframe;
        }
        frame_delay = av_q2d((*(*(*is_0).video_st).codec).time_base)
            * (*(*(*is_0).video_st).codec).ticks_per_frame as libc::c_double;
        repeat = if !(av_stream_get_parser((*is_0).video_st)).is_null() {
            (*av_stream_get_parser((*is_0).video_st)).repeat_pict
        } else {
            4 as libc::c_int
        };
        pev_best_effort_timestamp = best_effort_timestamp;
        best_effort_timestamp = av_frame_get_best_effort_timestamp((*is_0).pFrame);
        calculated_delay = (best_effort_timestamp - pev_best_effort_timestamp) as libc::c_double
            * av_q2d((*(*is_0).video_st).time_base);
        if best_effort_timestamp == 0x8000000000000000 as libc::c_ulonglong as int64_t {
            real_pts = 0 as libc::c_int as libc::c_double;
        } else {
            headerpos = avio_tell((*(*is_0).pFormatCtx).pb);
            if initial_pts_set < 3 as libc::c_int && reviewing == 0
                || reviewing != 0 && initial_pts_set < 2 as libc::c_int
            {
                if !(libm::fabs(
                    initial_pts
                        - ((best_effort_timestamp
                            - (if (*(*is_0).video_st).start_time
                                != 0x8000000000000000 as libc::c_ulonglong as int64_t
                            {
                                (*(*is_0).video_st).start_time
                            } else {
                                0 as libc::c_int as libc::c_longlong
                            })) as libc::c_double
                            * av_q2d((*(*is_0).video_st).time_base)
                            - frame_delay * framenum as libc::c_double),
                ) < 0.001f64)
                {
                    initial_pts = (best_effort_timestamp
                        - (if (*(*is_0).video_st).start_time
                            != 0x8000000000000000 as libc::c_ulonglong as int64_t
                        {
                            (*(*is_0).video_st).start_time
                        } else {
                            0 as libc::c_int as libc::c_longlong
                        })) as libc::c_double
                        * av_q2d((*(*is_0).video_st).time_base)
                        - frame_delay * framenum as libc::c_double;
                    Debug(
                        10 as libc::c_int,
                        b"\nInitial video pts = %10.3f\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        initial_pts,
                    );
                }
                initial_pts_set += 1;
                final_pts = 0 as libc::c_int as int64_t;
                pts_offset = 0.0f64;
            }
            real_pts = av_q2d((*(*is_0).video_st).time_base)
                * (best_effort_timestamp
                    - (if (*(*is_0).video_st).start_time
                        != 0x8000000000000000 as libc::c_ulonglong as int64_t
                    {
                        (*(*is_0).video_st).start_time
                    } else {
                        0 as libc::c_int as libc::c_longlong
                    })) as libc::c_double
                - initial_pts;
            final_pts = best_effort_timestamp
                - (if (*(*is_0).video_st).start_time
                    != 0x8000000000000000 as libc::c_ulonglong as int64_t
                {
                    (*(*is_0).video_st).start_time
                } else {
                    0 as libc::c_int as libc::c_longlong
                });
        }
        calculated_delay = real_pts - prev_real_pts;
        if framenum < 500 as libc::c_int {
            if !(libm::fabs(frame_delay - 0.03336666f64) < 0.001f64)
                && (libm::fabs(calculated_delay - 0.0333333f64) < 0.0001f64
                    || libm::fabs(calculated_delay - 0.033f64) < 0.0001f64
                    || libm::fabs(calculated_delay - 0.034f64) < 0.0001f64
                    || libm::fabs(calculated_delay - 0.067f64) < 0.0001f64
                    || libm::fabs(calculated_delay - 0.066f64) < 0.0001f64
                    || libm::fabs(calculated_delay - 0.06673332f64) < 0.0001f64)
            {
                find_29fps += 1;
            } else {
                find_29fps = 0 as libc::c_int;
            }
            if force_29fps == 0 as libc::c_int && find_29fps == 5 as libc::c_int {
                force_29fps = 1 as libc::c_int;
                force_25fps = 0 as libc::c_int;
                force_24fps = 0 as libc::c_int;
                Debug(
                    1 as libc::c_int,
                    b"Framerate forced to 29.97fps at frame %d\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    frame_count,
                );
            }
            if !(libm::fabs(frame_delay - 0.040f64) < 0.001f64)
                && (libm::fabs(calculated_delay - 0.04f64) < 0.0001f64
                    || libm::fabs(calculated_delay - 0.039f64) < 0.0001f64
                    || libm::fabs(calculated_delay - 0.041f64) < 0.0001f64)
            {
                find_25fps += 1;
            } else {
                find_25fps = 0 as libc::c_int;
            }
            if force_25fps == 0 as libc::c_int && find_25fps == 5 as libc::c_int {
                force_29fps = 0 as libc::c_int;
                force_25fps = 1 as libc::c_int;
                force_24fps = 0 as libc::c_int;
                Debug(
                    1 as libc::c_int,
                    b"Framerate forced to 25.00fps at frame %d\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    frame_count,
                );
            }
            if find_24fps & 1 as libc::c_int == 0 as libc::c_int
                && libm::fabs(calculated_delay - 0.050f64) < 0.001f64
                || find_24fps & 1 as libc::c_int == 1 as libc::c_int
                    && libm::fabs(calculated_delay - 0.033f64) < 0.001f64
            {
                find_24fps += 1;
            } else {
                find_24fps = 0 as libc::c_int;
            }
            if force_24fps == 0 as libc::c_int && find_24fps == 5 as libc::c_int {
                force_29fps = 0 as libc::c_int;
                force_25fps = 0 as libc::c_int;
                force_24fps = 1 as libc::c_int;
                Debug(
                    1 as libc::c_int,
                    b"Framerate forced to 24.00fps at frame %d\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    frame_count,
                );
            }
        }
        if force_29fps != 0 {
            frame_delay = 0.033366666666666669f64;
        }
        if force_25fps != 0 {
            frame_delay = 0.04f64;
        }
        if force_24fps != 0 {
            frame_delay = 0.0416666666666667f64;
        }
        pts_offset *= 0.9f64;
        if reviewing == 0 && timeline_repair != 0 {
            if framenum > 1 as libc::c_int
                && libm::fabs(calculated_delay - pts_offset - frame_delay) < 1.0f64
            {
                if !(libm::fabs(
                    3 as libc::c_int as libc::c_double * frame_delay
                        / (*(*(*is_0).video_st).codec).ticks_per_frame as libc::c_double
                        - calculated_delay,
                ) < 0.001f64)
                {
                    if !(libm::fabs(
                        1 as libc::c_int as libc::c_double * frame_delay
                            / (*(*(*is_0).video_st).codec).ticks_per_frame as libc::c_double
                            - calculated_delay,
                    ) < 0.001f64)
                    {
                        pts_offset = pts_offset + frame_delay - calculated_delay;
                    }
                }
            }
        } else {
            do_audio_repair = 0 as libc::c_int;
        }
        pts_0 = real_pts + pts_offset;
        calculated_delay = pts_0 - prev_pts;
        if reviewing == 0
            && framenum > 1 as libc::c_int
            && libm::fabs(calculated_delay - frame_delay) > 0.01f64
            && !(libm::fabs(
                3 as libc::c_int as libc::c_double * frame_delay
                    / (*(*(*is_0).video_st).codec).ticks_per_frame as libc::c_double
                    - calculated_delay,
            ) < 0.001f64)
            && !(libm::fabs(
                1 as libc::c_int as libc::c_double * frame_delay
                    / (*(*(*is_0).video_st).codec).ticks_per_frame as libc::c_double
                    - calculated_delay,
            ) < 0.001f64)
        {
            if prev_strange_framenum + 1 as libc::c_int != framenum
                && prev_strange_step < libm::fabs(calculated_delay - frame_delay)
            {
                Debug(
                    8 as libc::c_int,
                    b"Strange video pts step of %6.5f instead of %6.5f at frame %d\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    calculated_delay + 0.0005f64,
                    frame_delay + 0.0005f64,
                    framenum,
                );
                if calculated_delay < -0.5f64 {
                    do_audio_repair = 0 as libc::c_int;
                }
            }
            prev_strange_framenum = framenum;
            prev_strange_step = libm::fabs(calculated_delay - frame_delay);
        }
        set_fps(
            frame_delay,
            (*is_0).fps,
            repeat,
            av_q2d((*(*is_0).video_st).r_frame_rate),
            av_q2d((*(*is_0).video_st).avg_frame_rate),
        );
        if pts_0 != 0 as libc::c_int as libc::c_double {
            (*is_0).video_clock = pts_0;
            if !timing_file.is_null() && csStepping == 0 && csJumping == 0 && csStartJump == 0 {
                libc::fprintf(
                    timing_file,
                    b"%7s, %12.3f, %12.3f, %12.3f, %12.3f, %12.3f, %12.3f, %d\n\0" as *const u8
                        as *const libc::c_char,
                    b"v   set\0" as *const u8 as *const libc::c_char,
                    real_pts,
                    calculated_delay,
                    pts_0,
                    (*is_0).video_clock,
                    pts_0 - (*is_0).video_clock,
                    pts_offset,
                    repeat,
                );
            }
        } else {
            if !timing_file.is_null() && csStepping == 0 && csJumping == 0 && csStartJump == 0 {
                libc::fprintf(
                    timing_file,
                    b"%7s, %12.3f, %12.3f, %12.3f, %12.3f, %12.3f, %12.3f, %d\n\0" as *const u8
                        as *const libc::c_char,
                    b"v clock\0" as *const u8 as *const libc::c_char,
                    real_pts,
                    calculated_delay,
                    pts_0,
                    (*is_0).video_clock,
                    pts_0 - (*is_0).video_clock,
                    pts_offset,
                    repeat,
                );
            }
            pts_0 = (*is_0).video_clock;
        }
        (*is_0).video_clock_submitted = (*is_0).video_clock;
        if retries == 0 as libc::c_int {
            if (*is_0).video_clock - (*is_0).seek_pts > -frame_delay / 2.0f64 {
                if selftest == 1 as libc::c_int && pass == 1 as libc::c_int {
                    if (*is_0).video_clock < selftest_target - 0.05f64
                        || (*is_0).video_clock > selftest_target + 0.05f64
                    {
                        sample_file = libc::fopen(
                            b"seektest.log\0" as *const u8 as *const libc::c_char,
                            b"a+\0" as *const u8 as *const libc::c_char,
                        );
                        libc::fprintf(
                            sample_file,
                            b"Seek error: target=%8.1f, result=%8.1f, error=%6.3f, size=%8.1f, mode=%s, \"%s\"\n\0"
                                as *const u8 as *const libc::c_char,
                            (*is_0).seek_pts,
                            (*is_0).video_clock,
                            (*is_0).video_clock - (*is_0).seek_pts,
                            (*is_0).duration,
                            if (*is_0).seek_by_bytes != 0 {
                                b"byteseek\0" as *const u8 as *const libc::c_char
                            } else {
                                b"timeseek\0" as *const u8 as *const libc::c_char
                            },
                            ((*is_0).filename).as_mut_ptr(),
                        );
                        libc::fclose(sample_file);
                        Debug(
                            1 as libc::c_int,
                            b"\nSelftest 1 FAILED: Seektest\n:Starting test 3\n\0" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char,
                        );
                    } else {
                        Debug(
                            1 as libc::c_int,
                            b"\nSelftest 1 OK: Seektest\nStarting test 3\n\0" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char,
                        );
                    }
                    selftest = 3 as libc::c_int;
                    live_tv_retries = 1 as libc::c_int;
                    pass = 0 as libc::c_int;
                }
                if SubmitFrame((*is_0).video_st, (*is_0).pFrame, (*is_0).video_clock) != 0 {
                    current_block = 14537325084018676057;
                } else {
                    current_block = 2838755337219234678;
                }
            } else {
                current_block = 2838755337219234678;
            }
        } else if (*is_0).video_clock - (*is_0).seek_pts > -frame_delay / 2.0f64 {
            if selftest == 3 as libc::c_int {
                if (*is_0).video_clock < selftest_target - 0.05f64
                    || (*is_0).video_clock > selftest_target + 0.05f64
                {
                    sample_file = libc::fopen(
                        b"seektest.log\0" as *const u8 as *const libc::c_char,
                        b"a+\0" as *const u8 as *const libc::c_char,
                    );
                    libc::fprintf(
                        sample_file,
                        b"Reopen error: target=%8.1f, result=%8.1f, error=%6.3f, size=%8.1f, mode=%s, \"%s\"\n\0"
                            as *const u8 as *const libc::c_char,
                        (*is_0).seek_pts,
                        (*is_0).video_clock,
                        (*is_0).video_clock - (*is_0).seek_pts,
                        (*is_0).duration,
                        if (*is_0).seek_by_bytes != 0 {
                            b"byteseek\0" as *const u8 as *const libc::c_char
                        } else {
                            b"timeseek\0" as *const u8 as *const libc::c_char
                        },
                        ((*is_0).filename).as_mut_ptr(),
                    );
                    libc::fclose(sample_file);
                    Debug(
                        1 as libc::c_int,
                        b"\nSelftest 3 FAILED: Reopen\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                } else {
                    Debug(
                        1 as libc::c_int,
                        b"\nSelftest 3 OK: Reopen\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                }
                ::std::process::exit(1 as libc::c_int);
            }
            retries = 0 as libc::c_int;
            if SubmitFrame((*is_0).video_st, (*is_0).pFrame, (*is_0).video_clock) != 0 {
                current_block = 14537325084018676057;
            } else {
                current_block = 2838755337219234678;
            }
        } else if libm::fabs((*is_0).seek_pts - (*is_0).video_clock) > 80 as libc::c_int as libc::c_double
        {
            Debug(
                1 as libc::c_int,
                b"Positioning file failing with pts=%6.2f\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*is_0).video_clock,
            );
            if selftest == 1 as libc::c_int || selftest == 3 as libc::c_int {
                sample_file = libc::fopen(
                    b"seektest.log\0" as *const u8 as *const libc::c_char,
                    b"a+\0" as *const u8 as *const libc::c_char,
                );
                libc::fprintf(
                    sample_file,
                    b"Seek error : target=%8.1f, result=%8.1f, error=%6.3f, size=%8.1f, mode=%s, \"%s\"\n\0"
                        as *const u8 as *const libc::c_char,
                    (*is_0).seek_pts,
                    (*is_0).video_clock,
                    (*is_0).video_clock - (*is_0).seek_pts,
                    (*is_0).duration,
                    if (*is_0).seek_by_bytes != 0 {
                        b"byteseek\0" as *const u8 as *const libc::c_char
                    } else {
                        b"timeseek\0" as *const u8 as *const libc::c_char
                    },
                    ((*is_0).filename).as_mut_ptr(),
                );
                libc::fclose(sample_file);
                Debug(
                    1 as libc::c_int,
                    b"\nSelftest %d FAILED\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    selftest,
                );
                ::std::process::exit(1 as libc::c_int);
            }
            current_block = 14537325084018676057;
        } else {
            current_block = 2838755337219234678;
        }
        match current_block {
            14537325084018676057 => {}
            _ => {
                (*is_0).video_clock += frame_delay;
                prev_pts = pts_0;
                prev_real_pts = real_pts;
                if (*(*is_0).pFrame).nb_side_data != 0 {
                    let mut i: libc::c_int = 0;
                    i = 0 as libc::c_int;
                    while i < (*(*is_0).pFrame).nb_side_data {
                        let sd: *mut AVFrameSideData =
                            *((*(*is_0).pFrame).side_data).offset(i as isize);
                        if !((*sd).type_0 as libc::c_uint
                            != AV_FRAME_DATA_A53_CC as libc::c_int as libc::c_uint)
                        {
                            ccDataLen = (*sd).size + 7 as libc::c_int;
                            ccData[0 as libc::c_int as usize] = 'G' as i32 as uint8_t;
                            ccData[1 as libc::c_int as usize] = 'A' as i32 as uint8_t;
                            ccData[2 as libc::c_int as usize] = '9' as i32 as uint8_t;
                            ccData[3 as libc::c_int as usize] = '4' as i32 as uint8_t;
                            ccData[4 as libc::c_int as usize] = 3 as libc::c_int as uint8_t;
                            ccData[5 as libc::c_int as usize] =
                                ((*sd).size / 3 as libc::c_int + 64 as libc::c_int) as uint8_t;
                            i = 0 as libc::c_int;
                            while i < (*sd).size {
                                ccData[(i + 7 as libc::c_int) as usize] =
                                    *((*sd).data).offset(i as isize);
                                i += 1;
                            }
                            dump_data(ccData.as_mut_ptr() as *mut libc::c_char, ccDataLen);
                            if processCC != 0 {
                                ProcessCCData();
                            }
                            if output_srt != 0 {
                                process_block(ccData.as_mut_ptr(), ccDataLen as libc::c_long);
                            }
                        }
                        i += 1;
                    }
                }
                return 1 as libc::c_int;
            }
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn stream_component_open(
    mut is_0: *mut VideoState,
    stream_index_0: libc::c_int,
) -> libc::c_int {
    let pFormatCtx: *mut AVFormatContext = (*is_0).pFormatCtx;
    let mut codecCtx: *mut AVCodecContext = 0 as *mut AVCodecContext;
    let mut codec: *mut AVCodec = 0 as *mut AVCodec;
    if stream_index_0 < 0 as libc::c_int
        || stream_index_0 as libc::c_uint >= (*pFormatCtx).nb_streams
    {
        return -(1 as libc::c_int);
    }
    if libc::strcmp(
        (*(*pFormatCtx).iformat).name,
        b"mpegts\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        demux_pid = 1 as libc::c_int;
    }
    codecCtx = (**((*pFormatCtx).streams).offset(stream_index_0 as isize)).codec;
    avcodec_close(codecCtx);
    if (*codecCtx).codec_type as libc::c_int == AVMEDIA_TYPE_VIDEO as libc::c_int {
        if hardware_decode == 0 {
            (*codecCtx).flags |= (1 as libc::c_int) << 13 as libc::c_int;
        }
        let ref mut fresh20 = (*is_0).dec_ctx;
        *fresh20 = codecCtx;
        if (*codecCtx).codec_id as libc::c_uint
            != AV_CODEC_ID_MPEG1VIDEO as libc::c_int as libc::c_uint
        {
            (*codecCtx).thread_count = thread_count;
        }
        if (*codecCtx).codec_id as libc::c_uint == AV_CODEC_ID_H264 as libc::c_int as libc::c_uint {
            is_h264 = 1 as libc::c_int;
        } else {
            let mut w: libc::c_int = 0;
            if lowres == 10 as libc::c_int {
                w = (*codecCtx).width;
                lowres = 0 as libc::c_int;
                while w > 600 as libc::c_int {
                    w = w >> 1 as libc::c_int;
                    lowres += 1;
                }
            }
        }
        if (*codecCtx).codec_id as libc::c_uint
            != AV_CODEC_ID_MPEG1VIDEO as libc::c_int as libc::c_uint
        {
            (*codecCtx).thread_count = thread_count;
        }
    }
    codec = avcodec_find_decoder((*codecCtx).codec_id);
    if hardware_decode != 0 {
        if (*codecCtx).codec_id as libc::c_uint
            == AV_CODEC_ID_MPEG2VIDEO as libc::c_int as libc::c_uint
            && !(avcodec_find_decoder_by_name(b"mpeg2_mmal\0" as *const u8 as *const libc::c_char))
                .is_null()
        {
            codec =
                avcodec_find_decoder_by_name(b"mpeg2_mmal\0" as *const u8 as *const libc::c_char);
        }
        if (*codecCtx).codec_id as libc::c_uint == AV_CODEC_ID_H264 as libc::c_int as libc::c_uint
            && !(avcodec_find_decoder_by_name(b"h264_mmal\0" as *const u8 as *const libc::c_char))
                .is_null()
        {
            codec =
                avcodec_find_decoder_by_name(b"h264_mmal\0" as *const u8 as *const libc::c_char);
        }
        if (*codecCtx).codec_id as libc::c_uint == AV_CODEC_ID_MPEG4 as libc::c_int as libc::c_uint
            && !(avcodec_find_decoder_by_name(b"mpeg4_mmal\0" as *const u8 as *const libc::c_char))
                .is_null()
        {
            codec =
                avcodec_find_decoder_by_name(b"mpeg4_mmal\0" as *const u8 as *const libc::c_char);
        }
        if (*codecCtx).codec_id as libc::c_uint == AV_CODEC_ID_VC1 as libc::c_int as libc::c_uint
            && !(avcodec_find_decoder_by_name(b"vc1_mmal\0" as *const u8 as *const libc::c_char))
                .is_null()
        {
            codec = avcodec_find_decoder_by_name(b"vc1_mmal\0" as *const u8 as *const libc::c_char);
        }
    }
    if hardware_decode == 0 {
        av_dict_set_int(
            &mut myoptions,
            b"gray\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as int64_t,
            0 as libc::c_int,
        );
    }
    if codec.is_null() || avcodec_open2(codecCtx, codec, &mut myoptions) < 0 as libc::c_int {
        libc::fprintf(
            __stderrp,
            b"Unsupported codec!\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    match (*codecCtx).codec_type as libc::c_int {
        3 => {
            (*is_0).subtitleStream = stream_index_0;
            let ref mut fresh21 = (*is_0).subtitle_st;
            *fresh21 = *((*pFormatCtx).streams).offset(stream_index_0 as isize);
            if demux_pid != 0 {
                selected_subtitle_pid = (*(*is_0).subtitle_st).id;
            }
        }
        1 => {
            (*is_0).audioStream = stream_index_0;
            let ref mut fresh22 = (*is_0).audio_st;
            *fresh22 = *((*pFormatCtx).streams).offset(stream_index_0 as isize);
            if demux_pid != 0 {
                selected_audio_pid = (*(*is_0).audio_st).id;
            }
        }
        0 => {
            (*is_0).videoStream = stream_index_0;
            let ref mut fresh23 = (*is_0).video_st;
            *fresh23 = *((*pFormatCtx).streams).offset(stream_index_0 as isize);
            let ref mut fresh24 = (*is_0).pFrame;
            *fresh24 = av_frame_alloc();
            if hardware_decode == 0 {
                (*codecCtx).flags |= (1 as libc::c_int) << 13 as libc::c_int;
            }
            (*codecCtx).lowres = std::cmp::min(av_codec_get_max_lowres((*codecCtx).codec), lowres);
            if (*codecCtx).codec_id as libc::c_uint
                == AV_CODEC_ID_H264 as libc::c_int as libc::c_uint
            {
                is_h264 = 1 as libc::c_int;
            }
            if (*codecCtx).codec_id as libc::c_uint
                != AV_CODEC_ID_MPEG1VIDEO as libc::c_int as libc::c_uint
            {
                (*codecCtx).thread_count = thread_count;
            }
            if (*codecCtx).codec_id as libc::c_uint
                == AV_CODEC_ID_MPEG1VIDEO as libc::c_int as libc::c_uint
            {
                (*(*(*is_0).video_st).codec).ticks_per_frame = 1 as libc::c_int;
            }
            if demux_pid != 0 {
                selected_video_pid = (*(*is_0).video_st).id;
            }
            if skip_B_frames != 0 {
                (*codecCtx).skip_frame = AVDISCARD_NONREF;
            }
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn log_callback_report(
    ptr: *mut libc::c_void,
    level: libc::c_int,
    fmt: *const libc::c_char,
    vl: ::std::ffi::VaList,
) {
    let mut vl2: ::std::ffi::VaListImpl;
    let mut line: [libc::c_char; 1024] = [0; 1024];
    static mut print_prefix: libc::c_int = 1 as libc::c_int;
    if reviewing != 0 {
        return;
    }
    av_log_get_level();
    if level > av_log_level {
        return;
    }
    vl2 = vl.clone();
    av_log_format_line(
        ptr,
        level,
        fmt,
        vl2.as_va_list(),
        line.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        &mut print_prefix,
    );
    Debug(10 as libc::c_int, line.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn file_open() {
    let mut is_0: *mut VideoState = 0 as *mut VideoState;
    let mut subtitle_index: libc::c_int = -(1 as libc::c_int);
    let mut audio_index: libc::c_int = -(1 as libc::c_int);
    let mut video_index: libc::c_int = -(1 as libc::c_int);
    let mut openretries: libc::c_int = 0 as libc::c_int;
    if global_video_state.is_null() {
        is_0 = av_mallocz(::std::mem::size_of::<VideoState>() as libc::c_ulong) as *mut VideoState;
        libc::memset(
            &mut (*is_0).audio_pkt as *mut AVPacket as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<AVPacket>() as usize,
        );
        libc::strcpy(((*is_0).filename).as_mut_ptr(), mpegfilename.as_mut_ptr());
        av_log_level = 32 as libc::c_int;
        av_log_set_callback(Some(
            log_callback_report
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    libc::c_int,
                    *const libc::c_char,
                    ::std::ffi::VaList,
                ) -> (),
        ));
        av_log_set_flags(1 as libc::c_int);
        avcodec_register_all();
        av_register_all();
        avformat_network_init();
        global_video_state = is_0;
        (*is_0).videoStream = -(1 as libc::c_int);
        (*is_0).audioStream = -(1 as libc::c_int);
        (*is_0).subtitleStream = -(1 as libc::c_int);
        let ref mut fresh25 = (*is_0).pFormatCtx;
        *fresh25 = 0 as *mut AVFormatContext;
        if hardware_decode == 0 {
            av_dict_set_int(
                &mut myoptions,
                b"gray\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int as int64_t,
                0 as libc::c_int,
            );
        }
        av_dict_set_int(
            &mut myoptions,
            b"threads\0" as *const u8 as *const libc::c_char,
            thread_count as int64_t,
            0 as libc::c_int,
        );
        av_dict_set_int(
            &mut myoptions,
            b"refcounted_frames\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as int64_t,
            0 as libc::c_int,
        );
    } else {
        is_0 = global_video_state;
    }
    if ((*is_0).pFormatCtx).is_null() {
        let ref mut fresh26 = (*is_0).pFormatCtx;
        *fresh26 = avformat_alloc_context();
        let ref mut fresh27 = (*(*is_0).pFormatCtx).max_analyze_duration;
        *fresh27 *= 4 as libc::c_int as libc::c_longlong;
        's_142: {
            loop {
                if avformat_open_input(
                    &mut (*is_0).pFormatCtx,
                    ((*is_0).filename).as_mut_ptr(),
                    0 as *mut AVInputFormat,
                    &mut myoptions,
                ) != 0 as libc::c_int
                {
                    libc::fprintf(
                        __stderrp,
                        b"%s: Can not open file\n\0" as *const u8 as *const libc::c_char,
                        ((*is_0).filename).as_mut_ptr(),
                    );
                    let fresh28 = openretries;
                    openretries = openretries + 1;
                    if fresh28 < live_tv_retries {
                        std::thread::sleep(std::time::Duration::from_millis(1000));
                    } else {
                        ::std::process::exit(-(1 as libc::c_int));
                    }
                } else {
                    break 's_142;
                }
            }
        }
        (*is_0).seek_by_bytes = ((*(*(*is_0).pFormatCtx).iformat).flags & 0x200 as libc::c_int != 0
            && libc::strcmp(
                b"ogg\0" as *const u8 as *const libc::c_char,
                (*(*(*is_0).pFormatCtx).iformat).name,
            ) != 0) as libc::c_int;
        if avformat_find_stream_info((*is_0).pFormatCtx, 0 as *mut *mut AVDictionary)
            < 0 as libc::c_int
        {
            libc::fprintf(
                __stderrp,
                b"%s: Can not find stream info\n\0" as *const u8 as *const libc::c_char,
                ((*is_0).filename).as_mut_ptr(),
            );
            ::std::process::exit(-(1 as libc::c_int));
        }
        if retries == 0 as libc::c_int {
            av_dump_format(
                (*is_0).pFormatCtx,
                0 as libc::c_int,
                ((*is_0).filename).as_mut_ptr(),
                0 as libc::c_int,
            );
        }
    }
    if (*is_0).videoStream == -(1 as libc::c_int) {
        video_index = av_find_best_stream(
            (*is_0).pFormatCtx,
            AVMEDIA_TYPE_VIDEO,
            -(1 as libc::c_int),
            -(1 as libc::c_int),
            0 as *mut *mut AVCodec,
            0 as libc::c_int,
        );
        if video_index >= 0 as libc::c_int {
            stream_component_open(is_0, video_index);
        }
        if (*is_0).videoStream < 0 as libc::c_int {
            Debug(
                0 as libc::c_int,
                b"Could not open video codec\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            libc::fprintf(
                __stderrp,
                b"%s: could not open video codec\n\0" as *const u8 as *const libc::c_char,
                ((*is_0).filename).as_mut_ptr(),
            );
            ::std::process::exit(-(1 as libc::c_int));
        }
        if (*(*is_0).video_st).duration == 0x8000000000000000 as libc::c_ulonglong as int64_t
            || (*(*is_0).video_st).duration < 0 as libc::c_int as libc::c_longlong
        {
            (*is_0).duration = ((*(*is_0).pFormatCtx).duration as libc::c_float
                / 1000000 as libc::c_int as libc::c_float)
                as libc::c_double;
        } else {
            (*is_0).duration = av_q2d((*(*is_0).video_st).time_base)
                * (*(*is_0).video_st).duration as libc::c_double;
        }
        if (*is_0).duration < 0 as libc::c_int as libc::c_double
            && live_tv_retries > 0 as libc::c_int
        {
            Debug(
                0 as libc::c_int,
                b"Could not establish duration, live TV decoding may fail\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        if (*(*is_0).video_st).r_frame_rate.den != 0 && (*(*is_0).video_st).r_frame_rate.num != 0 {
            (*is_0).fps = av_q2d((*(*is_0).video_st).r_frame_rate);
        } else {
            (*is_0).fps =
                1 as libc::c_int as libc::c_double / av_q2d((*(*(*is_0).video_st).codec).time_base);
        }
    }
    if (*is_0).audioStream == -(1 as libc::c_int) && video_index >= 0 as libc::c_int {
        audio_index = av_find_best_stream(
            (*is_0).pFormatCtx,
            AVMEDIA_TYPE_AUDIO,
            -(1 as libc::c_int),
            video_index,
            0 as *mut *mut AVCodec,
            0 as libc::c_int,
        );
        if audio_index >= 0 as libc::c_int {
            stream_component_open(is_0, audio_index);
            if (*is_0).audioStream < 0 as libc::c_int {
                Debug(
                    1 as libc::c_int,
                    b"Could not open audio decoder or no audio present\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
        }
    }
    if (*is_0).subtitleStream == -(1 as libc::c_int) && video_index >= 0 as libc::c_int {
        subtitle_index = av_find_best_stream(
            (*is_0).pFormatCtx,
            AVMEDIA_TYPE_SUBTITLE,
            -(1 as libc::c_int),
            video_index,
            0 as *mut *mut AVCodec,
            0 as libc::c_int,
        );
        if subtitle_index >= 0 as libc::c_int {
            let ref mut fresh29 = (*is_0).subtitle_st;
            *fresh29 = *((*(*is_0).pFormatCtx).streams).offset(subtitle_index as isize);
            if demux_pid != 0 {
                selected_subtitle_pid = (*(*is_0).subtitle_st).id;
            }
        }
    }
    av_log_level = 16 as libc::c_int;
    (*is_0).seek_req = 0 as libc::c_int;
    pts_offset = 0.0f64;
    (*is_0).video_clock = 0.0f64;
    (*is_0).audio_clock = 0.0f64;
    initial_apts = 0 as libc::c_int as libc::c_double;
    apts_offset = 0.0f64;
    base_apts = 0.0f64;
    top_apts = 0.0f64;
    apts = 0.0f64;
    audio_buffer_ptr = audio_buffer.as_mut_ptr();
    audio_samples = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn file_close() {
    is = global_video_state;
    if (*is).videoStream != -(1 as libc::c_int) {
        avcodec_close((**((*(*is).pFormatCtx).streams).offset((*is).videoStream as isize)).codec);
    }
    (*is).videoStream = -(1 as libc::c_int);
    if (*is).audioStream != -(1 as libc::c_int) {
        avcodec_close((**((*(*is).pFormatCtx).streams).offset((*is).audioStream as isize)).codec);
    }
    (*is).audioStream = -(1 as libc::c_int);
    if (*is).subtitleStream != -(1 as libc::c_int) {
        avcodec_close(
            (**((*(*is).pFormatCtx).streams).offset((*is).subtitleStream as isize)).codec,
        );
    }
    (*is).subtitleStream = -(1 as libc::c_int);
    avformat_close_input(&mut (*is).pFormatCtx);
    av_frame_free(&mut (*is).frame);
    avformat_network_deinit();
}
unsafe fn main_0(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut packet_time: libc::c_double = 0.;
    let mut current_block: u64;
    let mut pkt1: AVPacket = AVPacket {
        buf: 0 as *const AVBufferRef as *mut AVBufferRef,
        pts: 0,
        dts: 0,
        data: 0 as *const uint8_t as *mut uint8_t,
        size: 0,
        stream_index: 0,
        flags: 0,
        side_data: 0 as *const AVPacketSideData as *mut AVPacketSideData,
        side_data_elems: 0,
        duration: 0,
        pos: 0,
        convergence_duration: 0,
    };
    let mut packet: *mut AVPacket = &mut pkt1;
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut ret: libc::c_int = 0;
    let mut tfps: libc::c_double = 0.;
    let mut old_clock: libc::c_double = 0.0f64;
    let mut empty_packet_count: libc::c_int = 0 as libc::c_int;
    let mut last_packet_pos: int64_t = 0 as libc::c_int as int64_t;
    let mut last_packet_pts: int64_t = 0 as libc::c_int as int64_t;
    let mut retry_target: libc::c_double = 0.0f64;
    retries = 0 as libc::c_int;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    if !(libc::strstr(
        *argv.offset(0 as libc::c_int as isize),
        b"comskipGUI\0" as *const u8 as *const libc::c_char,
    ))
    .is_null()
    {
        output_debugwindow = 1 as libc::c_int;
    }
    ptr = *argv.offset(0 as libc::c_int as isize);
    if *ptr as libc::c_int == '"' as i32 {
        ptr = ptr.offset(1);
        len = (libc::strchr(ptr, '"' as i32)).offset_from(ptr) as libc::c_long as size_t;
    } else {
        len = libc::strlen(ptr) as u64;
    }
    libc::strncpy(HomeDir.as_mut_ptr(), ptr, len as usize);
    ptr = libc::strrchr(HomeDir.as_mut_ptr(), '\\' as i32);
    if ptr.is_null()
        || ptr.offset_from(HomeDir.as_mut_ptr()) as libc::c_long == 0 as libc::c_int as libc::c_long
    {
        HomeDir[0 as libc::c_int as usize] = '.' as i32 as libc::c_char;
        HomeDir[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    } else {
        *ptr = '\0' as i32 as libc::c_char;
    }
    libc::fprintf(
        __stderrp,
        b"%s, made using ffmpeg\n\0" as *const u8 as *const libc::c_char,
        b"Comskip 0.82.005\0" as *const u8 as *const libc::c_char,
    );
    libc::fprintf(
        __stderrp,
        b"Donator build\n\0" as *const u8 as *const libc::c_char,
    );
    in_file = LoadSettings(argc, argv);
    file_open();
    csRestart = 0 as libc::c_int;
    framenum = 0 as libc::c_int;
    if output_timing != 0 {
        libc::sprintf(
            tempstring.as_mut_ptr(),
            b"%s.timing.csv\0" as *const u8 as *const libc::c_char,
            inbasename.as_mut_ptr(),
        );
        timing_file = libc::fopen(
            tempstring.as_mut_ptr(),
            b"w\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !timing_file.is_null() {
            libc::fprintf(
                timing_file,
                b"sep=,\ntype   ,real_pts, step        ,pts         ,clock       ,delta       ,offset, repeat\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
    }
    av_log_set_level(32 as libc::c_int);
    is = global_video_state;
    packet = &mut (*is).audio_pkt;
    while !((*is).quit != 0) {
        if (*is).seek_req != 0 {
            if (*is).seek_pts > 0.0f64 {
                DoSeekRequest(is);
            } else {
                (*is).seek_req = 0 as libc::c_int;
                file_close();
                file_open();
                if !timing_file.is_null() {
                    libc::fclose(timing_file);
                    timing_file = 0 as *mut libc::FILE;
                }
                if output_timing != 0 {
                    libc::sprintf(
                        tempstring.as_mut_ptr(),
                        b"%s.timing.csv\0" as *const u8 as *const libc::c_char,
                        inbasename.as_mut_ptr(),
                    );
                    timing_file = libc::fopen(
                        tempstring.as_mut_ptr(),
                        b"w\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                    if !timing_file.is_null() {
                        libc::fprintf(
                            timing_file,
                            b"sep=,\ntype   ,real_pts, step        ,pts         ,clock       ,delta       ,offset, repeat\n\0"
                                as *const u8 as *const libc::c_char,
                        );
                    }
                }
                if !timing_file.is_null() {
                    libc::fprintf(
                        timing_file,
                        b"sep=,\ntype   ,real_pts, step        ,pts         ,clock       ,delta       ,offset, repeat\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
                close_data();
                if output_srt != 0 || output_smi != 0 {
                    CEW_reinit();
                }
            }
        }
        loop {
            ret = av_read_frame((*is).pFormatCtx, packet);
            if !(ret >= 0 as libc::c_int && (*is).seek_req != 0) {
                current_block = 7158658067966855297;
                break;
            }
            packet_time = ((*packet).pts
                - (if (*(*is).video_st).start_time
                    != 0x8000000000000000 as libc::c_ulonglong as int64_t
                {
                    (*(*is).video_st).start_time
                } else {
                    0 as libc::c_int as libc::c_longlong
                })) as libc::c_double
                * av_q2d((*(*is).video_st).time_base);
            if (*packet).pts == 0x8000000000000000 as libc::c_ulonglong as int64_t
                || (*packet).pts == 0 as libc::c_int as libc::c_longlong
            {
                av_packet_unref(packet);
            } else if (*is).seek_req < 6 as libc::c_int
                && (*is).seek_flags & 2 as libc::c_int != 0
                && (*is).duration > 0 as libc::c_int as libc::c_double
                && libm::fabs(packet_time - ((*is).seek_pts - 2.5f64))
                    < (*is).duration / (10 as libc::c_int * (*is).seek_req) as libc::c_double
            {
                current_block = 12758904613967585247;
                break;
            } else {
                current_block = 2606304779496145856;
                break;
            }
        }
        match current_block {
            2606304779496145856 => {
                if retries != 0 {
                    Debug(
                        9 as libc::c_int,
                        b"Retry t_pos=%lld, l_pos=%lld, t_pts=%lld, l_pts=%lld\n\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        last_packet_pos,
                        (*packet).pos,
                        last_packet_pts,
                        (*packet).pts,
                    );
                }
                (*is).seek_req = 0 as libc::c_int;
            }
            12758904613967585247 => {
                let ref mut fresh30 = (*is).seek_pos;
                *fresh30 = (*fresh30 as libc::c_double
                    + ((*is).seek_pts - 2.5f64 - packet_time) / (*is).duration
                        * avio_size((*(*is).pFormatCtx).pb) as libc::c_double
                        * 0.9f64) as int64_t;
                let ref mut fresh31 = (*is).seek_req;
                *fresh31 += 1;
                continue;
            }
            _ => {}
        }
        (*is).seek_req = 0 as libc::c_int;
        if selftest == 3 as libc::c_int
            && retries == 0 as libc::c_int
            && (*is).video_clock >= 500.0f64
        {
            ret = -((('E' as i32
                | ('O' as i32) << 8 as libc::c_int
                | ('F' as i32) << 16 as libc::c_int) as libc::c_uint
                | (' ' as i32 as libc::c_uint) << 24 as libc::c_int)
                as libc::c_int);
            live_tv = 1 as libc::c_int;
            live_tv_retries = 2 as libc::c_int;
        }
        if selftest == 4 as libc::c_int
            && retries == 0 as libc::c_int
            && framenum > 0 as libc::c_int
            && framenum % 500 as libc::c_int == 0 as libc::c_int
        {
            ret = -((('E' as i32
                | ('O' as i32) << 8 as libc::c_int
                | ('F' as i32) << 16 as libc::c_int) as libc::c_uint
                | (' ' as i32 as libc::c_uint) << 24 as libc::c_int)
                as libc::c_int);
            live_tv = 1 as libc::c_int;
            live_tv_retries = 2 as libc::c_int;
        }
        if ret < 0 as libc::c_int {
            if ret
                == -((('E' as i32
                    | ('O' as i32) << 8 as libc::c_int
                    | ('F' as i32) << 16 as libc::c_int) as libc::c_uint
                    | (' ' as i32 as libc::c_uint) << 24 as libc::c_int)
                    as libc::c_int)
                || (*(*(*is).pFormatCtx).pb).eof_reached != 0
            {
                if selftest == 3 as libc::c_int {
                    if retries > 0 as libc::c_int {
                        if (*is).video_clock < selftest_target - 0.05f64
                            || (*is).video_clock > selftest_target + 0.05f64
                        {
                            sample_file = libc::fopen(
                                b"seektest.log\0" as *const u8 as *const libc::c_char,
                                b"a+\0" as *const u8 as *const libc::c_char,
                            );
                            libc::fprintf(
                                sample_file,
                                b"\"%s\": reopen file failed, size=%8.1f, pts=%6.2f\n\0"
                                    as *const u8
                                    as *const libc::c_char,
                                ((*is).filename).as_mut_ptr(),
                                (*is).duration,
                                (*is).video_clock,
                            );
                            libc::fclose(sample_file);
                            Debug(
                                1 as libc::c_int,
                                b"\nSelftest %d FAILED\n\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                selftest,
                            );
                            ::std::process::exit(1 as libc::c_int);
                        }
                    } else {
                        if (*is).video_clock < 500.0f64 {
                            selftest_target = (*is).video_clock - 2.0f64;
                        } else {
                            selftest_target = 500.0f64;
                        }
                        Debug(
                            1 as libc::c_int,
                            b"\nSelftest %d starting: Reopen\n\0" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char,
                            selftest,
                        );
                        selftest_target = libm::fmax(selftest_target, 0.5f64);
                        live_tv = 1 as libc::c_int;
                        live_tv_retries = 2 as libc::c_int;
                    }
                }
                if !(live_tv != 0 && retries < live_tv_retries) {
                    break;
                }
                let frame_delay: libc::c_double = av_q2d((*(*(*is).video_st).codec).time_base)
                    * (*(*(*is).video_st).codec).ticks_per_frame as libc::c_double;
                if retries == 0 as libc::c_int {
                    if selftest == 3 as libc::c_int {
                        retry_target = selftest_target;
                    } else {
                        retry_target = (*is).video_clock + frame_delay;
                    }
                }
                file_close();
                Debug(
                    1 as libc::c_int,
                    b"\nRetry=%d at frame=%d, time=%8.2f seconds\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    retries,
                    framenum,
                    retry_target,
                );
                Debug(
                    9 as libc::c_int,
                    b"Retry target pos=%lld, pts=%lld\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    last_packet_pos,
                    last_packet_pts,
                );
                if selftest == 0 as libc::c_int {
                    std::thread::sleep(std::time::Duration::from_millis(4000));
                }
                file_open();
                Set_seek(is, retry_target);
                retries += 1;
                continue;
            }
        }
        if (*packet).pts != 0x8000000000000000 as libc::c_ulonglong as int64_t
            && (*packet).pts != 0 as libc::c_int as libc::c_longlong
        {
            last_packet_pts = (*packet).pts;
        }
        if (*packet).pos != 0 as libc::c_int as libc::c_longlong
            && (*packet).pos != -(1 as libc::c_int) as libc::c_longlong
        {
            last_packet_pos = (*packet).pos;
        }
        if (*packet).stream_index == (*is).videoStream {
            if (*packet).size > 0 as libc::c_int && !((*packet).data).is_null() {
                video_packet_process(is, packet);
            }
        } else if (*packet).stream_index == (*is).audioStream {
            if (*packet).size > 0 as libc::c_int && !((*packet).data).is_null() {
                audio_packet_process(is, packet);
            }
        }
        av_packet_unref(packet);
        if (*is).video_clock == old_clock {
            empty_packet_count += 1;
            if empty_packet_count > 1000 as libc::c_int {
                Debug(
                    0 as libc::c_int,
                    b"Empty input\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
            empty_packet_count = 0 as libc::c_int;
        } else {
            old_clock = (*is).video_clock;
            empty_packet_count = 0 as libc::c_int;
        }
        if selftest == 1 as libc::c_int
            && pass == 0 as libc::c_int
            && (*is).seek_req == 0 as libc::c_int
            && framenum == 50 as libc::c_int
        {
            if (*is).duration > 2 as libc::c_int as libc::c_double {
                selftest_target = libm::fmin(
                    450.0f64,
                    (*is).duration - 2 as libc::c_int as libc::c_double,
                );
            } else {
                selftest_target = 1.0f64;
            }
            Set_seek(is, selftest_target);
            pass = 1 as libc::c_int;
            framenum += 1;
        }
    }
    if selftest == 1 as libc::c_int && pass == 1 as libc::c_int {
        if (*is).video_clock < selftest_target - 0.08f64
            || (*is).video_clock > selftest_target + 0.08f64
        {
            sample_file = libc::fopen(
                b"seektest.log\0" as *const u8 as *const libc::c_char,
                b"a+\0" as *const u8 as *const libc::c_char,
            );
            libc::fprintf(
                sample_file,
                b"Seek error: target=%8.1f, result=%8.1f, error=%6.3f, size=%8.1f, mode=%s\"%s\"\n\0"
                    as *const u8 as *const libc::c_char,
                (*is).seek_pts,
                (*is).video_clock,
                (*is).video_clock - (*is).seek_pts,
                (*is).duration,
                if (*is).seek_by_bytes != 0 {
                    b"byteseek\0" as *const u8 as *const libc::c_char
                } else {
                    b"timeseek\0" as *const u8 as *const libc::c_char
                },
                ((*is).filename).as_mut_ptr(),
            );
            libc::fclose(sample_file);
        } else {
            Debug(
                1 as libc::c_int,
                b"\nSelftest 1 OK: Seektest\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        selftest = 3 as libc::c_int;
        pass = 0 as libc::c_int;
    }
    if live_tv != 0 {
        lastFrameCommCalculated = 0 as libc::c_int;
        BuildCommListAsYouGo();
    }
    tfps = print_fps(1 as libc::c_int);
    Debug(
        10 as libc::c_int,
        b"\nParsed %d video frames and %d audio frames at %8.2f fps\n\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        framenum,
        sound_frame_counter,
        tfps,
    );
    Debug(
        10 as libc::c_int,
        b"\nMaximum Volume found is %d\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        max_volume_found,
    );
    in_file = 0 as *mut libc::FILE;
    if framenum > 0 as libc::c_int {
        if BuildMasterCommList() != 0 {
            result = 1 as libc::c_int;
            libc::printf(b"Commercials were found.\n\0" as *const u8 as *const libc::c_char);
        } else {
            result = 0 as libc::c_int;
            libc::printf(b"Commercials were not found.\n\0" as *const u8 as *const libc::c_char);
        }
        if output_debugwindow != 0 {
            processCC = 0 as libc::c_int;
            libc::printf(b"Close window when done\n\0" as *const u8 as *const libc::c_char);
            if !timing_file.is_null() {
                libc::fclose(timing_file);
                timing_file = 0 as *mut libc::FILE;
            }
            if output_timing != 0 {
                output_timing = 0 as libc::c_int;
            }
        }
    }
    ::std::process::exit((result == 0) as libc::c_int);
}
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
