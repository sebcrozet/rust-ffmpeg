#![crate_id = "avfilter3"]
#![license = "MIT"]
#![crate_type = "dylib"]
#![allow(non_camel_case_types)]
/* automatically generated by rust-bindgen */
extern crate libc;
extern crate avutil52;
extern crate avcodec54;
use libc::{c_void,c_int,c_uint,c_char,uint8_t,int64_t,uint64_t,int8_t,c_double};

pub type __int128_t = c_void;
pub type __uint128_t = c_void;
pub type __builtin_va_list = [__va_list_tag, ..1u];
pub type AVFilterContext = Struct_AVFilterContext;
pub type AVFilterLink = Struct_AVFilterLink;
pub type AVFilterPad = Struct_AVFilterPad;
pub type Struct_AVFilterFormats = c_void;
pub type AVFilterFormats = Struct_AVFilterFormats;
pub struct Struct_AVFilterBuffer {
    pub data: [*mut uint8_t, ..8u],
    pub extended_data: *mut *mut uint8_t,
    pub linesize: [c_int, ..8u],
    pub _priv: *mut c_void,
    pub free: ::std::option::Option<extern "C" fn
                                        (arg1: *mut Struct_AVFilterBuffer)>,
    pub format: c_int,
    pub w: c_int,
    pub h: c_int,
    pub refcount: c_uint,
}
pub type AVFilterBuffer = Struct_AVFilterBuffer;
pub struct Struct_AVFilterBufferRefAudioProps {
    pub channel_layout: uint64_t,
    pub nb_samples: c_int,
    pub sample_rate: c_int,
    pub channels: c_int,
}
pub type AVFilterBufferRefAudioProps = Struct_AVFilterBufferRefAudioProps;
pub struct Struct_AVFilterBufferRefVideoProps {
    pub w: c_int,
    pub h: c_int,
    pub sample_aspect_ratio: avutil52::AVRational,
    pub interlaced: c_int,
    pub top_field_first: c_int,
    pub pict_type: avutil52::Enum_AVPictureType,
    pub key_frame: c_int,
    pub qp_table_linesize: c_int,
    pub qp_table_size: c_int,
    pub qp_table: *mut int8_t,
}
pub type AVFilterBufferRefVideoProps = Struct_AVFilterBufferRefVideoProps;
pub struct Struct_AVFilterBufferRef {
    pub buf: *mut AVFilterBuffer,
    pub data: [*mut uint8_t, ..8u],
    pub extended_data: *mut *mut uint8_t,
    pub linesize: [c_int, ..8u],
    pub video: *mut AVFilterBufferRefVideoProps,
    pub audio: *mut AVFilterBufferRefAudioProps,
    pub pts: int64_t,
    pub pos: int64_t,
    pub format: c_int,
    pub perms: c_int,
    pub _type: avutil52::Enum_AVMediaType,
    pub metadata: *mut avutil52::AVDictionary,
}
pub type AVFilterBufferRef = Struct_AVFilterBufferRef;
pub struct Struct_AVFilterPad {
    pub name: *c_char,
    pub _type: avutil52::Enum_AVMediaType,
    pub min_perms: c_int,
    pub rej_perms: c_int,
    pub start_frame: ::std::option::Option<extern "C" fn
                                               (arg1: *mut AVFilterLink,
                                                arg2: *mut AVFilterBufferRef)
                                               -> c_int>,
    pub get_video_buffer: ::std::option::Option<extern "C" fn
                                                    (arg1: *mut AVFilterLink,
                                                     arg2: c_int, arg3: c_int)
                                                    -> *mut avcodec54::AVFrame>,
    pub get_audio_buffer: ::std::option::Option<extern "C" fn
                                                    (arg1: *mut AVFilterLink,
                                                     arg2: c_int)
                                                    -> *mut avcodec54::AVFrame>,
    pub end_frame: ::std::option::Option<extern "C" fn
                                             (arg1: *mut AVFilterLink)
                                             -> c_int>,
    pub draw_slice: ::std::option::Option<extern "C" fn
                                              (arg1: *mut AVFilterLink,
                                               arg2: c_int, arg3: c_int,
                                               arg4: c_int) -> c_int>,
    pub filter_frame: ::std::option::Option<extern "C" fn
                                                (arg1: *mut AVFilterLink,
                                                 arg2: *mut avcodec54::AVFrame)
                                                -> c_int>,
    pub poll_frame: ::std::option::Option<extern "C" fn
                                              (arg1: *mut AVFilterLink)
                                              -> c_int>,
    pub request_frame: ::std::option::Option<extern "C" fn
                                                 (arg1: *mut AVFilterLink)
                                                 -> c_int>,
    pub config_props: ::std::option::Option<extern "C" fn
                                                (arg1: *mut AVFilterLink)
                                                -> c_int>,
    pub needs_fifo: c_int,
    pub needs_writable: c_int,
}
pub struct Struct_AVFilter {
    pub name: *c_char,
    pub description: *c_char,
    pub inputs: *AVFilterPad,
    pub outputs: *AVFilterPad,
    pub priv_class: *avutil52::AVClass,
    pub flags: c_int,
    pub init: ::std::option::Option<extern "C" fn(arg1: *mut AVFilterContext)
                                        -> c_int>,
    pub init_dict: ::std::option::Option<extern "C" fn
                                             (arg1: *mut AVFilterContext,
                                              arg2: *mut *mut avutil52::AVDictionary)
                                             -> c_int>,
    pub uninit: ::std::option::Option<extern "C" fn
                                          (arg1: *mut AVFilterContext)>,
    pub query_formats: ::std::option::Option<extern "C" fn
                                                 (arg1: *mut AVFilterContext)
                                                 -> c_int>,
    pub priv_size: c_int,
    pub next: *mut Struct_AVFilter,
    pub process_command: ::std::option::Option<extern "C" fn
                                                   (arg1:
                                                        *mut AVFilterContext,
                                                    arg2: *c_char,
                                                    arg3: *c_char,
                                                    arg4: *mut c_char,
                                                    arg5: c_int, arg6: c_int)
                                                   -> c_int>,
    pub init_opaque: ::std::option::Option<extern "C" fn
                                               (arg1: *mut AVFilterContext,
                                                arg2: *mut c_void) -> c_int>,
}
pub type AVFilter = Struct_AVFilter;
pub type Struct_AVFilterInternal = c_void;
pub type AVFilterInternal = Struct_AVFilterInternal;
pub struct Struct_AVFilterContext {
    pub av_class: *avutil52::AVClass,
    pub filter: *AVFilter,
    pub name: *mut c_char,
    pub input_pads: *mut AVFilterPad,
    pub inputs: *mut *mut AVFilterLink,
    pub input_count: c_uint,
    pub nb_inputs: c_uint,
    pub output_pads: *mut AVFilterPad,
    pub outputs: *mut *mut AVFilterLink,
    pub output_count: c_uint,
    pub nb_outputs: c_uint,
    pub _priv: *mut c_void,
    pub graph: *mut Struct_AVFilterGraph,
    pub thread_type: c_int,
    pub internal: *mut AVFilterInternal,
    pub command_queue: *mut Struct_AVFilterCommand,
    pub enable_str: *mut c_char,
    pub enable: *mut c_void,
    pub var_values: *mut c_double,
    pub is_disabled: c_int,
}
pub type Struct_AVFilterCommand = c_void;
pub struct Struct_AVFilterLink {
    pub src: *mut AVFilterContext,
    pub srcpad: *mut AVFilterPad,
    pub dst: *mut AVFilterContext,
    pub dstpad: *mut AVFilterPad,
    pub _type: avutil52::Enum_AVMediaType,
    pub w: c_int,
    pub h: c_int,
    pub sample_aspect_ratio: avutil52::AVRational,
    pub channel_layout: uint64_t,
    pub sample_rate: c_int,
    pub format: c_int,
    pub time_base: avutil52::AVRational,
    pub in_formats: *mut AVFilterFormats,
    pub out_formats: *mut AVFilterFormats,
    pub in_samplerates: *mut AVFilterFormats,
    pub out_samplerates: *mut AVFilterFormats,
    pub in_channel_layouts: *mut Struct_AVFilterChannelLayouts,
    pub out_channel_layouts: *mut Struct_AVFilterChannelLayouts,
    pub request_samples: c_int,
    pub init_state: Enum_Unnamed1,
    pub pool: *mut Struct_AVFilterPool,
    pub graph: *mut Struct_AVFilterGraph,
    pub current_pts: int64_t,
    pub age_index: c_int,
    pub frame_rate: avutil52::AVRational,
    pub partial_buf: *mut avcodec54::AVFrame,
    pub partial_buf_size: c_int,
    pub min_samples: c_int,
    pub max_samples: c_int,
    pub cur_buf_copy: *mut AVFilterBufferRef,
    pub closed: c_int,
    pub channels: c_int,
    pub frame_requested: c_uint,
    pub flags: c_uint,
    pub frame_count: int64_t,
}
pub type Struct_AVFilterChannelLayouts = c_void;
pub type Enum_Unnamed1 = c_uint;
pub static AVLINK_UNINIT: c_uint = 0;
pub static AVLINK_STARTINIT: c_uint = 1;
pub static AVLINK_INIT: c_uint = 2;
pub type Struct_AVFilterPool = c_void;
pub type Struct_AVFilterGraphInternal = c_void;
pub type AVFilterGraphInternal = Struct_AVFilterGraphInternal;
pub type avfilter_action_func = c_void;
pub type avfilter_execute_func = c_void;
pub struct Struct_AVFilterGraph {
    pub av_class: *avutil52::AVClass,
    pub filter_count_unused: c_uint,
    pub filters: *mut *mut AVFilterContext,
    pub scale_sws_opts: *mut c_char,
    pub resample_lavr_opts: *mut c_char,
    pub nb_filters: c_uint,
    pub thread_type: c_int,
    pub nb_threads: c_int,
    pub internal: *mut AVFilterGraphInternal,
    pub opaque: *mut c_void,
    pub execute: *mut avfilter_execute_func,
    pub aresample_swr_opts: *mut c_char,
    pub sink_links: *mut *mut AVFilterLink,
    pub sink_links_count: c_int,
    pub disable_auto_convert: c_uint,
}
pub type AVFilterGraph = Struct_AVFilterGraph;
pub type Enum_Unnamed2 = c_int;
pub static AVFILTER_AUTO_CONVERT_ALL: c_int = 0;
pub static AVFILTER_AUTO_CONVERT_NONE: c_int = -1;
pub struct Struct_AVFilterInOut {
    pub name: *mut c_char,
    pub filter_ctx: *mut AVFilterContext,
    pub pad_idx: c_int,
    pub next: *mut Struct_AVFilterInOut,
}
pub type AVFilterInOut = Struct_AVFilterInOut;
pub type __va_list_tag = Struct___va_list_tag;
pub struct Struct___va_list_tag {
    pub gp_offset: c_uint,
    pub fp_offset: c_uint,
    pub overflow_arg_area: *mut c_void,
    pub reg_save_area: *mut c_void,
}
#[link(name = "avfilter")]
extern "C" {
    pub fn avfilter_version() -> c_uint;
    pub fn avfilter_configuration() -> *c_char;
    pub fn avfilter_license() -> *c_char;
    pub fn avfilter_copy_buffer_ref_props(dst: *mut AVFilterBufferRef,
                                          src: *mut AVFilterBufferRef);
    pub fn avfilter_ref_buffer(_ref: *mut AVFilterBufferRef, pmask: c_int) ->
     *mut AVFilterBufferRef;
    pub fn avfilter_unref_buffer(_ref: *mut AVFilterBufferRef);
    pub fn avfilter_unref_bufferp(_ref: *mut *mut AVFilterBufferRef);
    pub fn avfilter_ref_get_channels(_ref: *mut AVFilterBufferRef) -> c_int;
    pub fn avfilter_pad_count(pads: *AVFilterPad) -> c_int;
    pub fn avfilter_pad_get_name(pads: *AVFilterPad, pad_idx: c_int) ->
     *c_char;
    pub fn avfilter_pad_get_type(pads: *AVFilterPad, pad_idx: c_int) ->
     avutil52::Enum_AVMediaType;
    pub fn avfilter_link(src: *mut AVFilterContext, srcpad: c_uint,
                         dst: *mut AVFilterContext, dstpad: c_uint) -> c_int;
    pub fn avfilter_link_free(link: *mut *mut AVFilterLink);
    pub fn avfilter_link_get_channels(link: *mut AVFilterLink) -> c_int;
    pub fn avfilter_link_set_closed(link: *mut AVFilterLink, closed: c_int);
    pub fn avfilter_config_links(filter: *mut AVFilterContext) -> c_int;
    pub fn avfilter_get_video_buffer_ref_from_arrays(data:
                                                         [*mut uint8_t, ..4u],
                                                     linesize: [c_int, ..4u],
                                                     perms: c_int, w: c_int,
                                                     h: c_int,
                                                     format:
                                                         avutil52::Enum_AVPixelFormat)
     -> *mut AVFilterBufferRef;
    pub fn avfilter_get_audio_buffer_ref_from_arrays(data: *mut *mut uint8_t,
                                                     linesize: c_int,
                                                     perms: c_int,
                                                     nb_samples: c_int,
                                                     sample_fmt:
                                                         avutil52::Enum_AVSampleFormat,
                                                     channel_layout: uint64_t)
     -> *mut AVFilterBufferRef;
    pub fn avfilter_get_audio_buffer_ref_from_arrays_channels(data:
                                                                  *mut *mut uint8_t,
                                                              linesize: c_int,
                                                              perms: c_int,
                                                              nb_samples:
                                                                  c_int,
                                                              sample_fmt:
                                                                  avutil52::Enum_AVSampleFormat,
                                                              channels: c_int,
                                                              channel_layout:
                                                                  uint64_t) ->
     *mut AVFilterBufferRef;
    pub fn avfilter_process_command(filter: *mut AVFilterContext,
                                    cmd: *c_char, arg: *c_char,
                                    res: *mut c_char, res_len: c_int,
                                    flags: c_int) -> c_int;
    pub fn avfilter_register_all();
    pub fn avfilter_uninit();
    pub fn avfilter_register(filter: *mut AVFilter) -> c_int;
    pub fn avfilter_get_by_name(name: *c_char) -> *mut AVFilter;
    pub fn avfilter_next(prev: *AVFilter) -> *AVFilter;
    pub fn av_filter_next(filter: *mut *mut AVFilter) -> *mut *mut AVFilter;
    pub fn avfilter_open(filter_ctx: *mut *mut AVFilterContext,
                         filter: *mut AVFilter, inst_name: *c_char) -> c_int;
    pub fn avfilter_init_filter(filter: *mut AVFilterContext, args: *c_char,
                                opaque: *mut c_void) -> c_int;
    pub fn avfilter_init_str(ctx: *mut AVFilterContext, args: *c_char) ->
     c_int;
    pub fn avfilter_init_dict(ctx: *mut AVFilterContext,
                              options: *mut *mut avutil52::AVDictionary) -> c_int;
    pub fn avfilter_free(filter: *mut AVFilterContext);
    pub fn avfilter_insert_filter(link: *mut AVFilterLink,
                                  filt: *mut AVFilterContext,
                                  filt_srcpad_idx: c_uint,
                                  filt_dstpad_idx: c_uint) -> c_int;
    pub fn avfilter_copy_frame_props(dst: *mut AVFilterBufferRef,
                                     src: *avcodec54::AVFrame) -> c_int;
    pub fn avfilter_copy_buf_props(dst: *mut avcodec54::AVFrame, src: *AVFilterBufferRef)
     -> c_int;
    pub fn avfilter_get_class() -> *avutil52::AVClass;
    pub fn avfilter_graph_alloc() -> *mut AVFilterGraph;
    pub fn avfilter_graph_alloc_filter(graph: *mut AVFilterGraph,
                                       filter: *AVFilter, name: *c_char) ->
     *mut AVFilterContext;
    pub fn avfilter_graph_get_filter(graph: *mut AVFilterGraph,
                                     name: *mut c_char) ->
     *mut AVFilterContext;
    pub fn avfilter_graph_add_filter(graphctx: *mut AVFilterGraph,
                                     filter: *mut AVFilterContext) -> c_int;
    pub fn avfilter_graph_create_filter(filt_ctx: *mut *mut AVFilterContext,
                                        filt: *AVFilter, name: *c_char,
                                        args: *c_char, opaque: *mut c_void,
                                        graph_ctx: *mut AVFilterGraph) ->
     c_int;
    pub fn avfilter_graph_set_auto_convert(graph: *mut AVFilterGraph,
                                           flags: c_uint);
    pub fn avfilter_graph_config(graphctx: *mut AVFilterGraph,
                                 log_ctx: *mut c_void) -> c_int;
    pub fn avfilter_graph_free(graph: *mut *mut AVFilterGraph);
    pub fn avfilter_inout_alloc() -> *mut AVFilterInOut;
    pub fn avfilter_inout_free(inout: *mut *mut AVFilterInOut);
    pub fn avfilter_graph_parse(graph: *mut AVFilterGraph, filters: *c_char,
                                inputs: *mut *mut AVFilterInOut,
                                outputs: *mut *mut AVFilterInOut,
                                log_ctx: *mut c_void) -> c_int;
    pub fn avfilter_graph_parse_ptr(graph: *mut AVFilterGraph,
                                    filters: *c_char,
                                    inputs: *mut *mut AVFilterInOut,
                                    outputs: *mut *mut AVFilterInOut,
                                    log_ctx: *mut c_void) -> c_int;
    pub fn avfilter_graph_parse2(graph: *mut AVFilterGraph, filters: *c_char,
                                 inputs: *mut *mut AVFilterInOut,
                                 outputs: *mut *mut AVFilterInOut) -> c_int;
    pub fn avfilter_graph_send_command(graph: *mut AVFilterGraph,
                                       target: *c_char, cmd: *c_char,
                                       arg: *c_char, res: *mut c_char,
                                       res_len: c_int, flags: c_int) -> c_int;
    pub fn avfilter_graph_queue_command(graph: *mut AVFilterGraph,
                                        target: *c_char, cmd: *c_char,
                                        arg: *c_char, flags: c_int,
                                        ts: c_double) -> c_int;
    pub fn avfilter_graph_dump(graph: *mut AVFilterGraph, options: *c_char)
     -> *mut c_char;
    pub fn avfilter_graph_request_oldest(graph: *mut AVFilterGraph) -> c_int;
}

pub fn version() -> uint{
    unsafe {
        avfilter_version() as uint
    }
}
pub fn license() -> ~str {
    unsafe {
        std::str::raw::from_c_str(avfilter_license())
    }
}
