#![allow(non_camel_case_types)]

use std::fmt;
use std::os::raw::{c_void, c_char};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ttfp_font {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct ttfp_outline_builder {
    move_to: unsafe extern "C" fn(x: f32, y: f32, data: *mut c_void),
    line_to: unsafe extern "C" fn(x: f32, y: f32, data: *mut c_void),
    quad_to: unsafe extern "C" fn(x1: f32, y1: f32, x: f32, y: f32, data: *mut c_void),
    curve_to: unsafe extern "C" fn(x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32, data: *mut c_void),
    close_path: unsafe extern "C" fn(data: *mut c_void),
}

struct Builder(ttfp_outline_builder, *mut c_void);

impl ttf_parser::OutlineBuilder for Builder {
    fn move_to(&mut self, x: f32, y: f32) {
        unsafe { (self.0.move_to)(x, y, self.1) }
    }

    fn line_to(&mut self, x: f32, y: f32) {
        unsafe { (self.0.line_to)(x, y, self.1) }
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        unsafe { (self.0.quad_to)(x1, y1, x, y, self.1) }
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        unsafe { (self.0.curve_to)(x1, y1, x2, y2, x, y, self.1) }
    }

    fn close(&mut self) {
        unsafe { (self.0.close_path)(self.1) }
    }
}

fn font_from_ptr(font: *const ttfp_font) -> &'static ttf_parser::Font<'static> {
    unsafe { &*(font as *const ttf_parser::Font) }
}

#[no_mangle]
pub extern "C" fn ttfp_font_create(data: *const c_char, len: u32, index: u32) -> *const ttfp_font {
    let data = unsafe { std::slice::from_raw_parts(data as *const _, len as usize) };
    let font = ttf_parser::Font::from_data(data, index).unwrap();
    Box::into_raw(Box::new(font)) as *const _
}

#[no_mangle]
pub extern "C" fn ttfp_font_destroy(font: *mut ttfp_font) {
    unsafe { Box::from_raw(font) };
}

#[no_mangle]
pub extern "C" fn ttfp_outline_glyph(
    font: *const ttfp_font,
    raw_builder: ttfp_outline_builder,
    user_data: *mut c_void,
    glyph_id: u16,
    raw_bbox: *mut ttf_parser::Rect,
) -> bool {
    let mut builder = Builder(raw_builder, user_data);
    match font_from_ptr(font).outline_glyph(ttf_parser::GlyphId(glyph_id), &mut builder) {
        Some(bbox) => {
            unsafe { *raw_bbox = bbox }
            true
        }
        None => false,
    }
}

#[no_mangle]
pub extern "C" fn ttfp_outline_variable_glyph(
    font: *const ttfp_font,
    raw_builder: ttfp_outline_builder,
    user_data: *mut c_void,
    glyph_id: u16,
    coordinates: *const i32,
    coordinates_size: u32,
    raw_bbox: *mut ttf_parser::Rect,
) -> bool {
    let coordinates = unsafe { std::slice::from_raw_parts(coordinates, coordinates_size as usize) };

    let mut builder = Builder(raw_builder, user_data);
    match font_from_ptr(font).outline_variable_glyph(ttf_parser::GlyphId(glyph_id), coordinates, &mut builder) {
        Some(bbox) => {
            unsafe { *raw_bbox = bbox }
            true
        }
        None => false,
    }
}

#[no_mangle]
pub extern "C" fn ttfp_ascender(font: *const ttfp_font) -> i16 {
    font_from_ptr(font).ascender()
}

#[no_mangle]
pub extern "C" fn ttfp_height(font: *const ttfp_font) -> i16 {
    font_from_ptr(font).height()
}

#[no_mangle]
pub extern "C" fn ttfp_number_of_glyphs(font: *const ttfp_font) -> u16 {
    font_from_ptr(font).number_of_glyphs()
}

#[no_mangle]
pub extern "C" fn ttfp_variation_axes_count(font: *const ttfp_font) -> u16 {
    font_from_ptr(font).variation_axes().count() as u16
}

#[no_mangle]
pub extern "C" fn ttfp_get_variation_axis(
    font: *const ttfp_font,
    index: u16,
    raw_axis: *mut ttf_parser::VariationAxis,
) -> bool {
    match font_from_ptr(font).variation_axes().nth(index as usize) {
        Some(axis) => {
            unsafe { *raw_axis = axis };
            true
        }
        None => false,
    }
}

#[no_mangle]
pub extern "C" fn ttfp_get_variation_axis_by_tag(
    font: *const ttfp_font,
    tag: ttf_parser::Tag,
    raw_axis: *mut ttf_parser::VariationAxis,
) -> bool {
    match font_from_ptr(font).variation_axes().find(|axis| axis.tag == tag) {
        Some(axis) => {
            unsafe { *raw_axis = axis };
            true
        }
        None => false,
    }
}

#[no_mangle]
pub extern "C" fn ttfp_map_variation_coordinates(
    font: *const ttfp_font,
    coordinates: *mut i32,
    coordinates_size: u32,
) -> bool {
    let coordinates = unsafe { std::slice::from_raw_parts_mut(coordinates, coordinates_size as usize) };
    match font_from_ptr(font).map_variation_coordinates(coordinates) {
        Some(_) => true,
        None => false,
    }
}

#[no_mangle]
pub extern "C" fn ttfp_init_log() {
    fern::Dispatch::new()
        .format(log_format)
        .level(log::LevelFilter::Warn)
        .chain(std::io::stderr())
        .apply()
        .unwrap();
}

fn log_format(
    out: fern::FormatCallback,
    message: &fmt::Arguments,
    record: &log::Record,
) {
    let lvl = match record.level() {
        log::Level::Error => "Error",
        log::Level::Warn => "Warning",
        log::Level::Info => "Info",
        log::Level::Debug => "Debug",
        log::Level::Trace => "Trace",
    };

    out.finish(format_args!(
        "{} (in {}:{}): {}",
        lvl,
        record.target(),
        record.line().unwrap_or(0),
        message
    ))
}
