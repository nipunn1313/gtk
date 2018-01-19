// This file was generated by gir (c6b70b0) from gir-files (469db10)
// DO NOT EDIT

use Error;
use NumberUpLayout;
use PageOrientation;
use PageSet;
use PaperSize;
use PrintDuplex;
use PrintPages;
use PrintQuality;
use Unit;
use ffi;
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct PrintSettings(Object<ffi::GtkPrintSettings>);

    match fn {
        get_type => || ffi::gtk_print_settings_get_type(),
    }
}

impl PrintSettings {
    pub fn new() -> PrintSettings {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_print_settings_new())
        }
    }

    pub fn new_from_file<P: AsRef<std::path::Path>>(file_name: P) -> Result<PrintSettings, Error> {
        assert_initialized_main_thread!();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gtk_print_settings_new_from_file(file_name.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    pub fn new_from_gvariant(variant: &glib::Variant) -> PrintSettings {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_print_settings_new_from_gvariant(variant.to_glib_none().0))
        }
    }

    pub fn new_from_key_file<'a, P: Into<Option<&'a str>>>(key_file: &glib::KeyFile, group_name: P) -> Result<PrintSettings, Error> {
        assert_initialized_main_thread!();
        let group_name = group_name.into();
        let group_name = group_name.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gtk_print_settings_new_from_key_file(key_file.to_glib_none().0, group_name.0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }
}

impl Default for PrintSettings {
    fn default() -> Self {
        Self::new()
    }
}

pub trait PrintSettingsExt {
    fn copy(&self) -> Option<PrintSettings>;

    //fn foreach<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, func: /*Unknown conversion*//*Unimplemented*/PrintSettingsFunc, user_data: P);

    fn get(&self, key: &str) -> Option<String>;

    fn get_bool(&self, key: &str) -> bool;

    fn get_collate(&self) -> bool;

    fn get_default_source(&self) -> Option<String>;

    fn get_dither(&self) -> Option<String>;

    fn get_double(&self, key: &str) -> f64;

    fn get_double_with_default(&self, key: &str, def: f64) -> f64;

    fn get_duplex(&self) -> PrintDuplex;

    fn get_finishings(&self) -> Option<String>;

    fn get_int(&self, key: &str) -> i32;

    fn get_int_with_default(&self, key: &str, def: i32) -> i32;

    fn get_length(&self, key: &str, unit: Unit) -> f64;

    fn get_media_type(&self) -> Option<String>;

    fn get_n_copies(&self) -> i32;

    fn get_number_up(&self) -> i32;

    fn get_number_up_layout(&self) -> NumberUpLayout;

    fn get_orientation(&self) -> PageOrientation;

    fn get_output_bin(&self) -> Option<String>;

    fn get_page_set(&self) -> PageSet;

    fn get_paper_height(&self, unit: Unit) -> f64;

    fn get_paper_size(&self) -> PaperSize;

    fn get_paper_width(&self, unit: Unit) -> f64;

    fn get_print_pages(&self) -> PrintPages;

    fn get_printer(&self) -> Option<String>;

    fn get_printer_lpi(&self) -> f64;

    fn get_quality(&self) -> PrintQuality;

    fn get_resolution(&self) -> i32;

    fn get_resolution_x(&self) -> i32;

    fn get_resolution_y(&self) -> i32;

    fn get_reverse(&self) -> bool;

    fn get_scale(&self) -> f64;

    fn get_use_color(&self) -> bool;

    fn has_key(&self, key: &str) -> bool;

    fn load_file<P: AsRef<std::path::Path>>(&self, file_name: P) -> Result<(), Error>;

    fn load_key_file<'a, P: Into<Option<&'a str>>>(&self, key_file: &glib::KeyFile, group_name: P) -> Result<(), Error>;

    fn set<'a, P: Into<Option<&'a str>>>(&self, key: &str, value: P);

    fn set_bool(&self, key: &str, value: bool);

    fn set_collate(&self, collate: bool);

    fn set_default_source(&self, default_source: &str);

    fn set_dither(&self, dither: &str);

    fn set_double(&self, key: &str, value: f64);

    fn set_duplex(&self, duplex: PrintDuplex);

    fn set_finishings(&self, finishings: &str);

    fn set_int(&self, key: &str, value: i32);

    fn set_length(&self, key: &str, value: f64, unit: Unit);

    fn set_media_type(&self, media_type: &str);

    fn set_n_copies(&self, num_copies: i32);

    fn set_number_up(&self, number_up: i32);

    fn set_number_up_layout(&self, number_up_layout: NumberUpLayout);

    fn set_orientation(&self, orientation: PageOrientation);

    fn set_output_bin(&self, output_bin: &str);

    fn set_page_set(&self, page_set: PageSet);

    fn set_paper_height(&self, height: f64, unit: Unit);

    fn set_paper_size(&self, paper_size: &PaperSize);

    fn set_paper_width(&self, width: f64, unit: Unit);

    fn set_print_pages(&self, pages: PrintPages);

    fn set_printer(&self, printer: &str);

    fn set_printer_lpi(&self, lpi: f64);

    fn set_quality(&self, quality: PrintQuality);

    fn set_resolution(&self, resolution: i32);

    fn set_resolution_xy(&self, resolution_x: i32, resolution_y: i32);

    fn set_reverse(&self, reverse: bool);

    fn set_scale(&self, scale: f64);

    fn set_use_color(&self, use_color: bool);

    fn to_file<P: AsRef<std::path::Path>>(&self, file_name: P) -> Result<(), Error>;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn to_gvariant(&self) -> Option<glib::Variant>;

    fn to_key_file(&self, key_file: &glib::KeyFile, group_name: &str);

    fn unset(&self, key: &str);
}

impl<O: IsA<PrintSettings>> PrintSettingsExt for O {
    fn copy(&self) -> Option<PrintSettings> {
        unsafe {
            from_glib_full(ffi::gtk_print_settings_copy(self.to_glib_none().0))
        }
    }

    //fn foreach<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, func: /*Unknown conversion*//*Unimplemented*/PrintSettingsFunc, user_data: P) {
    //    unsafe { TODO: call ffi::gtk_print_settings_foreach() }
    //}

    fn get(&self, key: &str) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_print_settings_get(self.to_glib_none().0, key.to_glib_none().0))
        }
    }

    fn get_bool(&self, key: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_print_settings_get_bool(self.to_glib_none().0, key.to_glib_none().0))
        }
    }

    fn get_collate(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_print_settings_get_collate(self.to_glib_none().0))
        }
    }

    fn get_default_source(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_print_settings_get_default_source(self.to_glib_none().0))
        }
    }

    fn get_dither(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_print_settings_get_dither(self.to_glib_none().0))
        }
    }

    fn get_double(&self, key: &str) -> f64 {
        unsafe {
            ffi::gtk_print_settings_get_double(self.to_glib_none().0, key.to_glib_none().0)
        }
    }

    fn get_double_with_default(&self, key: &str, def: f64) -> f64 {
        unsafe {
            ffi::gtk_print_settings_get_double_with_default(self.to_glib_none().0, key.to_glib_none().0, def)
        }
    }

    fn get_duplex(&self) -> PrintDuplex {
        unsafe {
            from_glib(ffi::gtk_print_settings_get_duplex(self.to_glib_none().0))
        }
    }

    fn get_finishings(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_print_settings_get_finishings(self.to_glib_none().0))
        }
    }

    fn get_int(&self, key: &str) -> i32 {
        unsafe {
            ffi::gtk_print_settings_get_int(self.to_glib_none().0, key.to_glib_none().0)
        }
    }

    fn get_int_with_default(&self, key: &str, def: i32) -> i32 {
        unsafe {
            ffi::gtk_print_settings_get_int_with_default(self.to_glib_none().0, key.to_glib_none().0, def)
        }
    }

    fn get_length(&self, key: &str, unit: Unit) -> f64 {
        unsafe {
            ffi::gtk_print_settings_get_length(self.to_glib_none().0, key.to_glib_none().0, unit.to_glib())
        }
    }

    fn get_media_type(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_print_settings_get_media_type(self.to_glib_none().0))
        }
    }

    fn get_n_copies(&self) -> i32 {
        unsafe {
            ffi::gtk_print_settings_get_n_copies(self.to_glib_none().0)
        }
    }

    fn get_number_up(&self) -> i32 {
        unsafe {
            ffi::gtk_print_settings_get_number_up(self.to_glib_none().0)
        }
    }

    fn get_number_up_layout(&self) -> NumberUpLayout {
        unsafe {
            from_glib(ffi::gtk_print_settings_get_number_up_layout(self.to_glib_none().0))
        }
    }

    fn get_orientation(&self) -> PageOrientation {
        unsafe {
            from_glib(ffi::gtk_print_settings_get_orientation(self.to_glib_none().0))
        }
    }

    fn get_output_bin(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_print_settings_get_output_bin(self.to_glib_none().0))
        }
    }

    fn get_page_set(&self) -> PageSet {
        unsafe {
            from_glib(ffi::gtk_print_settings_get_page_set(self.to_glib_none().0))
        }
    }

    fn get_paper_height(&self, unit: Unit) -> f64 {
        unsafe {
            ffi::gtk_print_settings_get_paper_height(self.to_glib_none().0, unit.to_glib())
        }
    }

    fn get_paper_size(&self) -> PaperSize {
        unsafe {
            from_glib_full(ffi::gtk_print_settings_get_paper_size(self.to_glib_none().0))
        }
    }

    fn get_paper_width(&self, unit: Unit) -> f64 {
        unsafe {
            ffi::gtk_print_settings_get_paper_width(self.to_glib_none().0, unit.to_glib())
        }
    }

    fn get_print_pages(&self) -> PrintPages {
        unsafe {
            from_glib(ffi::gtk_print_settings_get_print_pages(self.to_glib_none().0))
        }
    }

    fn get_printer(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_print_settings_get_printer(self.to_glib_none().0))
        }
    }

    fn get_printer_lpi(&self) -> f64 {
        unsafe {
            ffi::gtk_print_settings_get_printer_lpi(self.to_glib_none().0)
        }
    }

    fn get_quality(&self) -> PrintQuality {
        unsafe {
            from_glib(ffi::gtk_print_settings_get_quality(self.to_glib_none().0))
        }
    }

    fn get_resolution(&self) -> i32 {
        unsafe {
            ffi::gtk_print_settings_get_resolution(self.to_glib_none().0)
        }
    }

    fn get_resolution_x(&self) -> i32 {
        unsafe {
            ffi::gtk_print_settings_get_resolution_x(self.to_glib_none().0)
        }
    }

    fn get_resolution_y(&self) -> i32 {
        unsafe {
            ffi::gtk_print_settings_get_resolution_y(self.to_glib_none().0)
        }
    }

    fn get_reverse(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_print_settings_get_reverse(self.to_glib_none().0))
        }
    }

    fn get_scale(&self) -> f64 {
        unsafe {
            ffi::gtk_print_settings_get_scale(self.to_glib_none().0)
        }
    }

    fn get_use_color(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_print_settings_get_use_color(self.to_glib_none().0))
        }
    }

    fn has_key(&self, key: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_print_settings_has_key(self.to_glib_none().0, key.to_glib_none().0))
        }
    }

    fn load_file<P: AsRef<std::path::Path>>(&self, file_name: P) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_print_settings_load_file(self.to_glib_none().0, file_name.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn load_key_file<'a, P: Into<Option<&'a str>>>(&self, key_file: &glib::KeyFile, group_name: P) -> Result<(), Error> {
        let group_name = group_name.into();
        let group_name = group_name.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_print_settings_load_key_file(self.to_glib_none().0, key_file.to_glib_none().0, group_name.0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set<'a, P: Into<Option<&'a str>>>(&self, key: &str, value: P) {
        let value = value.into();
        let value = value.to_glib_none();
        unsafe {
            ffi::gtk_print_settings_set(self.to_glib_none().0, key.to_glib_none().0, value.0);
        }
    }

    fn set_bool(&self, key: &str, value: bool) {
        unsafe {
            ffi::gtk_print_settings_set_bool(self.to_glib_none().0, key.to_glib_none().0, value.to_glib());
        }
    }

    fn set_collate(&self, collate: bool) {
        unsafe {
            ffi::gtk_print_settings_set_collate(self.to_glib_none().0, collate.to_glib());
        }
    }

    fn set_default_source(&self, default_source: &str) {
        unsafe {
            ffi::gtk_print_settings_set_default_source(self.to_glib_none().0, default_source.to_glib_none().0);
        }
    }

    fn set_dither(&self, dither: &str) {
        unsafe {
            ffi::gtk_print_settings_set_dither(self.to_glib_none().0, dither.to_glib_none().0);
        }
    }

    fn set_double(&self, key: &str, value: f64) {
        unsafe {
            ffi::gtk_print_settings_set_double(self.to_glib_none().0, key.to_glib_none().0, value);
        }
    }

    fn set_duplex(&self, duplex: PrintDuplex) {
        unsafe {
            ffi::gtk_print_settings_set_duplex(self.to_glib_none().0, duplex.to_glib());
        }
    }

    fn set_finishings(&self, finishings: &str) {
        unsafe {
            ffi::gtk_print_settings_set_finishings(self.to_glib_none().0, finishings.to_glib_none().0);
        }
    }

    fn set_int(&self, key: &str, value: i32) {
        unsafe {
            ffi::gtk_print_settings_set_int(self.to_glib_none().0, key.to_glib_none().0, value);
        }
    }

    fn set_length(&self, key: &str, value: f64, unit: Unit) {
        unsafe {
            ffi::gtk_print_settings_set_length(self.to_glib_none().0, key.to_glib_none().0, value, unit.to_glib());
        }
    }

    fn set_media_type(&self, media_type: &str) {
        unsafe {
            ffi::gtk_print_settings_set_media_type(self.to_glib_none().0, media_type.to_glib_none().0);
        }
    }

    fn set_n_copies(&self, num_copies: i32) {
        unsafe {
            ffi::gtk_print_settings_set_n_copies(self.to_glib_none().0, num_copies);
        }
    }

    fn set_number_up(&self, number_up: i32) {
        unsafe {
            ffi::gtk_print_settings_set_number_up(self.to_glib_none().0, number_up);
        }
    }

    fn set_number_up_layout(&self, number_up_layout: NumberUpLayout) {
        unsafe {
            ffi::gtk_print_settings_set_number_up_layout(self.to_glib_none().0, number_up_layout.to_glib());
        }
    }

    fn set_orientation(&self, orientation: PageOrientation) {
        unsafe {
            ffi::gtk_print_settings_set_orientation(self.to_glib_none().0, orientation.to_glib());
        }
    }

    fn set_output_bin(&self, output_bin: &str) {
        unsafe {
            ffi::gtk_print_settings_set_output_bin(self.to_glib_none().0, output_bin.to_glib_none().0);
        }
    }

    fn set_page_set(&self, page_set: PageSet) {
        unsafe {
            ffi::gtk_print_settings_set_page_set(self.to_glib_none().0, page_set.to_glib());
        }
    }

    fn set_paper_height(&self, height: f64, unit: Unit) {
        unsafe {
            ffi::gtk_print_settings_set_paper_height(self.to_glib_none().0, height, unit.to_glib());
        }
    }

    fn set_paper_size(&self, paper_size: &PaperSize) {
        unsafe {
            ffi::gtk_print_settings_set_paper_size(self.to_glib_none().0, mut_override(paper_size.to_glib_none().0));
        }
    }

    fn set_paper_width(&self, width: f64, unit: Unit) {
        unsafe {
            ffi::gtk_print_settings_set_paper_width(self.to_glib_none().0, width, unit.to_glib());
        }
    }

    fn set_print_pages(&self, pages: PrintPages) {
        unsafe {
            ffi::gtk_print_settings_set_print_pages(self.to_glib_none().0, pages.to_glib());
        }
    }

    fn set_printer(&self, printer: &str) {
        unsafe {
            ffi::gtk_print_settings_set_printer(self.to_glib_none().0, printer.to_glib_none().0);
        }
    }

    fn set_printer_lpi(&self, lpi: f64) {
        unsafe {
            ffi::gtk_print_settings_set_printer_lpi(self.to_glib_none().0, lpi);
        }
    }

    fn set_quality(&self, quality: PrintQuality) {
        unsafe {
            ffi::gtk_print_settings_set_quality(self.to_glib_none().0, quality.to_glib());
        }
    }

    fn set_resolution(&self, resolution: i32) {
        unsafe {
            ffi::gtk_print_settings_set_resolution(self.to_glib_none().0, resolution);
        }
    }

    fn set_resolution_xy(&self, resolution_x: i32, resolution_y: i32) {
        unsafe {
            ffi::gtk_print_settings_set_resolution_xy(self.to_glib_none().0, resolution_x, resolution_y);
        }
    }

    fn set_reverse(&self, reverse: bool) {
        unsafe {
            ffi::gtk_print_settings_set_reverse(self.to_glib_none().0, reverse.to_glib());
        }
    }

    fn set_scale(&self, scale: f64) {
        unsafe {
            ffi::gtk_print_settings_set_scale(self.to_glib_none().0, scale);
        }
    }

    fn set_use_color(&self, use_color: bool) {
        unsafe {
            ffi::gtk_print_settings_set_use_color(self.to_glib_none().0, use_color.to_glib());
        }
    }

    fn to_file<P: AsRef<std::path::Path>>(&self, file_name: P) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_print_settings_to_file(self.to_glib_none().0, file_name.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn to_gvariant(&self) -> Option<glib::Variant> {
        unsafe {
            from_glib_none(ffi::gtk_print_settings_to_gvariant(self.to_glib_none().0))
        }
    }

    fn to_key_file(&self, key_file: &glib::KeyFile, group_name: &str) {
        unsafe {
            ffi::gtk_print_settings_to_key_file(self.to_glib_none().0, key_file.to_glib_none().0, group_name.to_glib_none().0);
        }
    }

    fn unset(&self, key: &str) {
        unsafe {
            ffi::gtk_print_settings_unset(self.to_glib_none().0, key.to_glib_none().0);
        }
    }
}
