#![allow(non_snake_case, non_camel_case_types)]

extern crate libc;

#[cfg(windows)]
use libc::wchar_t;
use libc::{c_char, c_double, c_float, c_int, c_short, c_uint, c_void, size_t};
use std::i64;

#[link(name = "sndfile")]
extern "C" {}

#[cfg(test)]
mod test;

pub const SF_FORMAT_WAV: c_int = 0x010000;
pub const SF_FORMAT_AIFF: c_int = 0x020000;
pub const SF_FORMAT_AU: c_int = 0x030000;
pub const SF_FORMAT_RAW: c_int = 0x040000;
pub const SF_FORMAT_PAF: c_int = 0x050000;
pub const SF_FORMAT_SVX: c_int = 0x060000;
pub const SF_FORMAT_NIST: c_int = 0x070000;
pub const SF_FORMAT_VOC: c_int = 0x080000;
pub const SF_FORMAT_IRCAM: c_int = 0x0A0000;
pub const SF_FORMAT_W64: c_int = 0x0B0000;
pub const SF_FORMAT_MAT4: c_int = 0x0C0000;
pub const SF_FORMAT_MAT5: c_int = 0x0D0000;
pub const SF_FORMAT_PVF: c_int = 0x0E0000;
pub const SF_FORMAT_XI: c_int = 0x0F0000;
pub const SF_FORMAT_HTK: c_int = 0x100000;
pub const SF_FORMAT_SDS: c_int = 0x110000;
pub const SF_FORMAT_AVR: c_int = 0x120000;
pub const SF_FORMAT_WAVEX: c_int = 0x130000;
pub const SF_FORMAT_SD2: c_int = 0x160000;
pub const SF_FORMAT_FLAC: c_int = 0x170000;
pub const SF_FORMAT_CAF: c_int = 0x180000;
pub const SF_FORMAT_WVE: c_int = 0x190000;
pub const SF_FORMAT_OGG: c_int = 0x200000;
pub const SF_FORMAT_MPC2K: c_int = 0x210000;
pub const SF_FORMAT_RF64: c_int = 0x220000;
pub const SF_FORMAT_MPEG: c_int = 0x230000;
pub const SF_FORMAT_PCM_S8: c_int = 0x0001;
pub const SF_FORMAT_PCM_16: c_int = 0x0002;
pub const SF_FORMAT_PCM_24: c_int = 0x0003;
pub const SF_FORMAT_PCM_32: c_int = 0x0004;
pub const SF_FORMAT_PCM_U8: c_int = 0x0005;
pub const SF_FORMAT_FLOAT: c_int = 0x0006;
pub const SF_FORMAT_DOUBLE: c_int = 0x0007;
pub const SF_FORMAT_ULAW: c_int = 0x0010;
pub const SF_FORMAT_ALAW: c_int = 0x0011;
pub const SF_FORMAT_IMA_ADPCM: c_int = 0x0012;
pub const SF_FORMAT_MS_ADPCM: c_int = 0x0013;
pub const SF_FORMAT_GSM610: c_int = 0x0020;
pub const SF_FORMAT_VOX_ADPCM: c_int = 0x0021;
pub const SF_FORMAT_NMS_ADPCM_16: c_int = 0x0022;
pub const SF_FORMAT_NMS_ADPCM_24: c_int = 0x0023;
pub const SF_FORMAT_NMS_ADPCM_32: c_int = 0x0024;
pub const SF_FORMAT_G721_32: c_int = 0x0030;
pub const SF_FORMAT_G723_24: c_int = 0x0031;
pub const SF_FORMAT_G723_40: c_int = 0x0032;
pub const SF_FORMAT_DWVW_12: c_int = 0x0040;
pub const SF_FORMAT_DWVW_16: c_int = 0x0041;
pub const SF_FORMAT_DWVW_24: c_int = 0x0042;
pub const SF_FORMAT_DWVW_N: c_int = 0x0043;
pub const SF_FORMAT_DPCM_8: c_int = 0x0050;
pub const SF_FORMAT_DPCM_16: c_int = 0x0051;
pub const SF_FORMAT_VORBIS: c_int = 0x0060;
pub const SF_FORMAT_OPUS: c_int = 0x0064;
pub const SF_FORMAT_ALAC_16: c_int = 0x0070;
pub const SF_FORMAT_ALAC_20: c_int = 0x0071;
pub const SF_FORMAT_ALAC_24: c_int = 0x0072;
pub const SF_FORMAT_ALAC_32: c_int = 0x0073;
pub const SF_FORMAT_MPEG_LAYER_I: c_int = 0x0080;
pub const SF_FORMAT_MPEG_LAYER_II: c_int = 0x0081;
pub const SF_FORMAT_MPEG_LAYER_III: c_int = 0x0082;
pub const SF_ENDIAN_FILE: c_int = 0x00000000;
pub const SF_ENDIAN_LITTLE: c_int = 0x10000000;
pub const SF_ENDIAN_BIG: c_int = 0x20000000;
pub const SF_ENDIAN_CPU: c_int = 0x30000000;
pub const SF_FORMAT_SUBMASK: c_int = 0x0000FFFF;
pub const SF_FORMAT_TYPEMASK: c_int = 0x0FFF0000;
pub const SF_FORMAT_ENDMASK: c_int = 0x30000000;

pub const SFC_GET_LIB_VERSION: c_int = 0x1000;
pub const SFC_GET_LOG_INFO: c_int = 0x1001;
pub const SFC_GET_CURRENT_SF_INFO: c_int = 0x1002;
pub const SFC_GET_NORM_DOUBLE: c_int = 0x1010;
pub const SFC_GET_NORM_FLOAT: c_int = 0x1011;
pub const SFC_SET_NORM_DOUBLE: c_int = 0x1012;
pub const SFC_SET_NORM_FLOAT: c_int = 0x1013;
pub const SFC_SET_SCALE_FLOAT_INT_READ: c_int = 0x1014;
pub const SFC_SET_SCALE_INT_FLOAT_WRITE: c_int = 0x1015;
pub const SFC_GET_SIMPLE_FORMAT_COUNT: c_int = 0x1020;
pub const SFC_GET_SIMPLE_FORMAT: c_int = 0x1021;
pub const SFC_GET_FORMAT_INFO: c_int = 0x1028;
pub const SFC_GET_FORMAT_MAJOR_COUNT: c_int = 0x1030;
pub const SFC_GET_FORMAT_MAJOR: c_int = 0x1031;
pub const SFC_GET_FORMAT_SUBTYPE_COUNT: c_int = 0x1032;
pub const SFC_GET_FORMAT_SUBTYPE: c_int = 0x1033;
pub const SFC_CALC_SIGNAL_MAX: c_int = 0x1040;
pub const SFC_CALC_NORM_SIGNAL_MAX: c_int = 0x1041;
pub const SFC_CALC_MAX_ALL_CHANNELS: c_int = 0x1042;
pub const SFC_CALC_NORM_MAX_ALL_CHANNELS: c_int = 0x1043;
pub const SFC_GET_SIGNAL_MAX: c_int = 0x1044;
pub const SFC_GET_MAX_ALL_CHANNELS: c_int = 0x1045;
pub const SFC_SET_ADD_PEAK_CHUNK: c_int = 0x1050;
pub const SFC_UPDATE_HEADER_NOW: c_int = 0x1060;
pub const SFC_SET_UPDATE_HEADER_AUTO: c_int = 0x1061;
pub const SFC_FILE_TRUNCATE: c_int = 0x1080;
pub const SFC_SET_RAW_START_OFFSET: c_int = 0x1090;
pub const SFC_SET_DITHER_ON_WRITE: c_int = 0x10A0;
pub const SFC_SET_DITHER_ON_READ: c_int = 0x10A1;
pub const SFC_GET_DITHER_INFO_COUNT: c_int = 0x10A2;
pub const SFC_GET_DITHER_INFO: c_int = 0x10A3;
pub const SFC_GET_EMBED_FILE_INFO: c_int = 0x10B0;
pub const SFC_SET_CLIPPING: c_int = 0x10C0;
pub const SFC_GET_CLIPPING: c_int = 0x10C1;
pub const SFC_GET_CUE_COUNT: c_int = 0x10CD;
pub const SFC_GET_CUE: c_int = 0x10CE;
pub const SFC_SET_CUE: c_int = 0x10CF;
pub const SFC_GET_INSTRUMENT: c_int = 0x10D0;
pub const SFC_SET_INSTRUMENT: c_int = 0x10D1;
pub const SFC_GET_LOOP_INFO: c_int = 0x10E0;
pub const SFC_GET_BROADCAST_INFO: c_int = 0x10F0;
pub const SFC_SET_BROADCAST_INFO: c_int = 0x10F1;
pub const SFC_GET_CHANNEL_MAP_INFO: c_int = 0x1100;
pub const SFC_SET_CHANNEL_MAP_INFO: c_int = 0x1101;
pub const SFC_RAW_DATA_NEEDS_ENDSWAP: c_int = 0x1110;
pub const SFC_WAVEX_SET_AMBISONIC: c_int = 0x1200;
pub const SFC_WAVEX_GET_AMBISONIC: c_int = 0x1201;
pub const SFC_RF64_AUTO_DOWNGRADE: c_int = 0x1210;
pub const SFC_SET_VBR_ENCODING_QUALITY: c_int = 0x1300;
pub const SFC_SET_COMPRESSION_LEVEL: c_int = 0x1301;
pub const SFC_SET_OGG_PAGE_LATENCY_MS: c_int = 0x1302;
pub const SFC_SET_OGG_PAGE_LATENCY: c_int = 0x1303;
pub const SFC_GET_BITRATE_MODE: c_int = 0x1304;
pub const SFC_SET_BITRATE_MODE: c_int = 0x1305;
pub const SFC_SET_CART_INFO: c_int = 0x1400;
pub const SFC_GET_CART_INFO: c_int = 0x1401;
pub const SFC_SET_ORIGINAL_SAMPLERATE: c_int = 0x1500;
pub const SFC_GET_ORIGINAL_SAMPLERATE: c_int = 0x1501;
pub const SFC_TEST_IEEE_FLOAT_REPLACE: c_int = 0x6001;
pub const SFC_SET_ADD_HEADER_PAD_CHUNK: c_int = 0x1051;
pub const SFC_SET_ADD_DITHER_ON_WRITE: c_int = 0x1070;
pub const SFC_SET_ADD_DITHER_ON_READ: c_int = 0x1071;

pub const SF_STR_TITLE: c_int = 0x01;
pub const SF_STR_COPYRIGHT: c_int = 0x02;
pub const SF_STR_SOFTWARE: c_int = 0x03;
pub const SF_STR_ARTIST: c_int = 0x04;
pub const SF_STR_COMMENT: c_int = 0x05;
pub const SF_STR_DATE: c_int = 0x06;
pub const SF_STR_ALBUM: c_int = 0x07;
pub const SF_STR_LICENSE: c_int = 0x08;
pub const SF_STR_TRACKNUMBER: c_int = 0x09;
pub const SF_STR_GENRE: c_int = 0x10;
pub const SF_STR_FIRST: c_int = SF_STR_TITLE;
pub const SF_STR_LAST: c_int = SF_STR_GENRE;

pub const SF_FALSE: c_int = 0;
pub const SF_TRUE: c_int = 1;

pub const SFM_READ: c_int = 0x10;
pub const SFM_WRITE: c_int = 0x20;
pub const SFM_RDWR: c_int = 0x30;

pub const SF_AMBISONIC_NONE: c_int = 0x40;
pub const SF_AMBISONIC_B_FORMAT: c_int = 0x41;

pub const SF_ERR_NO_ERROR: c_int = 0;
pub const SF_ERR_UNRECOGNISED_FORMAT: c_int = 1;
pub const SF_ERR_SYSTEM: c_int = 2;
pub const SF_ERR_MALFORMED_FILE: c_int = 3;
pub const SF_ERR_UNSUPPORTED_ENCODING: c_int = 4;

pub const SF_CHANNEL_MAP_INVALID: c_int = 0;
pub const SF_CHANNEL_MAP_MONO: c_int = 1;
pub const SF_CHANNEL_MAP_LEFT: c_int = 2;
pub const SF_CHANNEL_MAP_RIGHT: c_int = 3;
pub const SF_CHANNEL_MAP_CENTER: c_int = 4;
pub const SF_CHANNEL_MAP_FRONT_LEFT: c_int = 5;
pub const SF_CHANNEL_MAP_FRONT_RIGHT: c_int = 6;
pub const SF_CHANNEL_MAP_FRONT_CENTER: c_int = 7;
pub const SF_CHANNEL_MAP_REAR_CENTER: c_int = 8;
pub const SF_CHANNEL_MAP_REAR_LEFT: c_int = 9;
pub const SF_CHANNEL_MAP_REAR_RIGHT: c_int = 10;
pub const SF_CHANNEL_MAP_LFE: c_int = 11;
pub const SF_CHANNEL_MAP_FRONT_LEFT_OF_CENTER: c_int = 12;
pub const SF_CHANNEL_MAP_FRONT_RIGHT_OF_CENTER: c_int = 13;
pub const SF_CHANNEL_MAP_SIDE_LEFT: c_int = 14;
pub const SF_CHANNEL_MAP_SIDE_RIGHT: c_int = 15;
pub const SF_CHANNEL_MAP_TOP_CENTER: c_int = 16;
pub const SF_CHANNEL_MAP_TOP_FRONT_LEFT: c_int = 17;
pub const SF_CHANNEL_MAP_TOP_FRONT_RIGHT: c_int = 18;
pub const SF_CHANNEL_MAP_TOP_FRONT_CENTER: c_int = 19;
pub const SF_CHANNEL_MAP_TOP_REAR_LEFT: c_int = 20;
pub const SF_CHANNEL_MAP_TOP_REAR_RIGHT: c_int = 21;
pub const SF_CHANNEL_MAP_TOP_REAR_CENTER: c_int = 22;
pub const SF_CHANNEL_MAP_AMBISONIC_B_W: c_int = 23;
pub const SF_CHANNEL_MAP_AMBISONIC_B_X: c_int = 24;
pub const SF_CHANNEL_MAP_AMBISONIC_B_Y: c_int = 25;
pub const SF_CHANNEL_MAP_AMBISONIC_B_Z: c_int = 26;
pub const SF_CHANNEL_MAP_MAX: c_int = 27;

pub type SNDFILE = c_void;

pub type sf_count_t = i64;

pub const SF_COUNT_MAX: i64 = i64::MAX;

#[repr(C)]
#[derive(Debug)]
pub struct SF_INFO {
    pub frames: sf_count_t,
    pub samplerate: c_int,
    pub channels: c_int,
    pub format: c_int,
    pub sections: c_int,
    pub seekable: c_int,
}

#[repr(C)]
#[derive(Debug)]
pub struct SF_FORMAT_INFO {
    pub format: c_int,
    pub name: *const c_char,
    pub extension: *const c_char,
}

pub const SFD_DEFAULT_LEVEL: c_int = 0;
pub const SFD_CUSTOM_LEVEL: c_int = 0x40000000;
pub const SFD_NO_DITHER: c_int = 500;
pub const SFD_WHITE: c_int = 501;
pub const SFD_TRIANGULAR_PDF: c_int = 502;

#[repr(C)]
#[derive(Debug)]
pub struct SF_DITHER_INFO {
    pub type_: c_int,
    pub level: c_double,
    pub name: *const c_char,
}

#[repr(C)]
#[derive(Debug)]
pub struct SF_EMBED_FILE_INFO {
    pub offset: sf_count_t,
    pub length: sf_count_t,
}

#[repr(C)]
pub struct SF_CUE_POINT {
    pub indx: i32,
    pub position: u32,
    pub fcc_chunk: i32,
    pub chunk_start: i32,
    pub block_start: i32,
    pub sample_offset: u32,
    pub name: [c_char; 256],
}

#[repr(C)]
pub struct SF_CUES {
    pub cue_count: u32,
    pub cue_points: [SF_CUE_POINT; 100],
}

pub const SF_LOOP_NONE: c_int = 800;
pub const SF_LOOP_FORWARD: c_int = 801;
pub const SF_LOOP_BACKWARD: c_int = 802;
pub const SF_LOOP_ALTERNATING: c_int = 803;

#[repr(C)]
#[derive(Debug)]
pub struct SF_INSTRUMENT_LOOP {
    pub mode: c_int,
    pub start: u32,
    pub end: u32,
    pub count: u32,
}

#[repr(C)]
pub struct SF_INSTRUMENT {
    pub gain: c_int,
    pub basenote: c_char,
    pub detune: c_char,
    pub velocity_lo: c_char,
    pub velocity_hi: c_char,
    pub key_lo: c_char,
    pub key_hi: c_char,
    pub loop_count: c_int,
    pub loops: [SF_INSTRUMENT_LOOP; 16],
}

#[repr(C)]
pub struct SF_LOOP_INFO {
    pub time_sig_num: c_short,
    pub time_sig_den: c_short,
    pub loop_mode: c_int,
    pub num_beats: c_int,
    pub bpm: c_float,
    pub root_key: c_int,
    pub future: [c_int; 6],
}

#[repr(C)]
pub struct SF_BROADCAST_INFO {
    pub description: [c_char; 256],
    pub originator: [c_char; 32],
    pub originator_reference: [c_char; 32],
    pub origination_date: [c_char; 10],
    pub origination_time: [c_char; 8],
    pub time_reference_low: u32,
    pub time_reference_high: u32,
    pub version: c_short,
    pub umid: [c_char; 64],
    pub reserved: [c_char; 190],
    pub coding_history_size: u32,
    pub coding_history: [c_char; 256],
}

#[repr(C)]
pub struct SF_CART_TIMER {
    pub usage: [c_char; 4],
    pub value: i32,
}

#[repr(C)]
pub struct SF_CART_INFO {
    pub version: [c_char; 4],
    pub title: [c_char; 64],
    pub artist: [c_char; 64],
    pub cut_id: [c_char; 64],
    pub client_id: [c_char; 64],
    pub category: [c_char; 64],
    pub classification: [c_char; 64],
    pub out_cue: [c_char; 64],
    pub start_date: [c_char; 10],
    pub start_time: [c_char; 8],
    pub end_date: [c_char; 10],
    pub end_time: [c_char; 8],
    pub producer_app_id: [c_char; 64],
    pub producer_app_version: [c_char; 64],
    pub user_def: [c_char; 64],
    pub level_reference: i32,
    pub post_timers: [SF_CART_TIMER; 8],
    pub reserved: [c_char; 276],
    pub url: [c_char; 1024],
    pub tag_text_size: i32,
    pub tag_text: [c_char; 256],
}

pub type sf_vio_get_filelen = extern "C" fn(user_data: *mut c_void) -> sf_count_t;
pub type sf_vio_seek =
    extern "C" fn(offset: sf_count_t, whence: c_int, user_data: *mut c_void) -> sf_count_t;
pub type sf_vio_read =
    extern "C" fn(ptr: *mut c_void, count: sf_count_t, user_data: *mut c_void) -> sf_count_t;
pub type sf_vio_write =
    extern "C" fn(ptr: *const c_void, count: sf_count_t, user_data: *mut c_void) -> sf_count_t;
pub type sf_vio_tell = extern "C" fn(user_data: *mut c_void) -> sf_count_t;

#[repr(C)]
#[derive(Debug)]
pub struct SF_VIRTUAL_IO {
    pub get_filelen: sf_vio_get_filelen,
    pub seek: sf_vio_seek,
    pub read: sf_vio_read,
    pub write: sf_vio_write,
    pub tell: sf_vio_tell,
}

pub const SF_SEEK_SET: c_int = 0;
pub const SF_SEEK_CUR: c_int = 1;
pub const SF_SEEK_END: c_int = 2;

#[repr(C)]
pub struct SF_CHUNK_INFO {
    id: [c_char; 64],
    id_size: c_uint,
    datalen: c_uint,
    data: *mut c_void,
}

pub type SF_CHUNK_ITERATOR = c_void;

extern "C" {
    pub fn sf_open(path: *const c_char, mode: c_int, sfinfo: *mut SF_INFO) -> *mut SNDFILE;
    pub fn sf_open_fd(
        fd: c_int,
        mode: c_int,
        sfinfo: *mut SF_INFO,
        close_desc: c_int,
    ) -> *mut SNDFILE;
    pub fn sf_open_virtual(
        sfvirtual: *mut SF_VIRTUAL_IO,
        mode: c_int,
        sfinfo: *mut SF_INFO,
        user_data: *mut c_void,
    ) -> *mut SNDFILE;
    #[cfg(windows)]
    pub fn sf_wchar_open(wpath: *const wchar_t, mode: c_int, sfinfo: *mut SF_INFO) -> *mut SNDFILE;
    pub fn sf_error(sndfile: *mut SNDFILE) -> c_int;
    pub fn sf_strerror(sndfile: *mut SNDFILE) -> *const c_char;
    pub fn sf_error_number(errnum: c_int) -> *const c_char;
    pub fn sf_perror(sndfile: *mut SNDFILE) -> c_int;
    pub fn sf_error_str(sndfile: *mut SNDFILE, str_: *mut c_char, len: size_t) -> c_int;
    pub fn sf_command(
        sndfile: *mut SNDFILE,
        command: c_int,
        data: *mut c_void,
        datasize: c_int,
    ) -> c_int;
    pub fn sf_format_check(info: *const SF_INFO) -> c_int;
    pub fn sf_seek(sndfile: *mut SNDFILE, frames: sf_count_t, whence: c_int) -> sf_count_t;
    pub fn sf_set_string(sndfile: *mut SNDFILE, str_type: c_int, str_: *const c_char) -> c_int;
    pub fn sf_get_string(sndfile: *mut SNDFILE, str_type: c_int) -> *const c_char;
    pub fn sf_version_string() -> *const c_char;
    pub fn sf_current_byterate(sndfile: *mut SNDFILE) -> c_int;
    pub fn sf_read_raw(sndfile: *mut SNDFILE, ptr: *mut c_void, bytes: sf_count_t) -> sf_count_t;
    pub fn sf_write_raw(sndfile: *mut SNDFILE, ptr: *const c_void, bytes: sf_count_t)
        -> sf_count_t;
    pub fn sf_readf_short(
        sndfile: *mut SNDFILE,
        ptr: *mut c_short,
        frames: sf_count_t,
    ) -> sf_count_t;
    pub fn sf_writef_short(
        sndfile: *mut SNDFILE,
        ptr: *const c_short,
        frames: sf_count_t,
    ) -> sf_count_t;
    pub fn sf_readf_int(sndfile: *mut SNDFILE, ptr: *mut c_int, frames: sf_count_t) -> sf_count_t;
    pub fn sf_writef_int(
        sndfile: *mut SNDFILE,
        ptr: *const c_int,
        frames: sf_count_t,
    ) -> sf_count_t;
    pub fn sf_readf_float(
        sndfile: *mut SNDFILE,
        ptr: *mut c_float,
        frames: sf_count_t,
    ) -> sf_count_t;
    pub fn sf_writef_float(
        sndfile: *mut SNDFILE,
        ptr: *const c_float,
        frames: sf_count_t,
    ) -> sf_count_t;
    pub fn sf_readf_double(
        sndfile: *mut SNDFILE,
        ptr: *mut c_double,
        frames: sf_count_t,
    ) -> sf_count_t;
    pub fn sf_writef_double(
        sndfile: *mut SNDFILE,
        ptr: *const c_double,
        frames: sf_count_t,
    ) -> sf_count_t;
    pub fn sf_read_short(sndfile: *mut SNDFILE, ptr: *mut c_short, items: sf_count_t)
        -> sf_count_t;
    pub fn sf_write_short(
        sndfile: *mut SNDFILE,
        ptr: *const c_short,
        items: sf_count_t,
    ) -> sf_count_t;
    pub fn sf_read_int(sndfile: *mut SNDFILE, ptr: *mut c_int, items: sf_count_t) -> sf_count_t;
    pub fn sf_write_int(sndfile: *mut SNDFILE, ptr: *const c_int, items: sf_count_t) -> sf_count_t;
    pub fn sf_read_float(sndfile: *mut SNDFILE, ptr: *mut c_float, items: sf_count_t)
        -> sf_count_t;
    pub fn sf_write_float(
        sndfile: *mut SNDFILE,
        ptr: *const c_float,
        items: sf_count_t,
    ) -> sf_count_t;
    pub fn sf_read_double(
        sndfile: *mut SNDFILE,
        ptr: *mut c_double,
        items: sf_count_t,
    ) -> sf_count_t;
    pub fn sf_write_double(
        sndfile: *mut SNDFILE,
        ptr: *const c_double,
        items: sf_count_t,
    ) -> sf_count_t;
    pub fn sf_close(sndfile: *mut SNDFILE) -> c_int;
    pub fn sf_write_sync(sndfile: *mut SNDFILE);
    pub fn sf_set_chunk(sndfile: *mut SNDFILE, chunk_info: *const SF_CHUNK_INFO) -> c_int;
    pub fn sf_get_chunk_iterator(
        sndfile: *mut SNDFILE,
        chunk_info: *const SF_CHUNK_INFO,
    ) -> *mut SF_CHUNK_ITERATOR;
    pub fn sf_next_chunk_iterator(iterator: *mut SF_CHUNK_ITERATOR) -> *mut SF_CHUNK_ITERATOR;
    pub fn sf_get_chunk_size(it: *const SF_CHUNK_ITERATOR, chunk_info: *mut SF_CHUNK_INFO)
        -> c_int;
    pub fn sf_get_chunk_data(it: *const SF_CHUNK_ITERATOR, chunk_info: *mut SF_CHUNK_INFO)
        -> c_int;
}
