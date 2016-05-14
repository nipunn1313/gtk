// This file was generated by gir (4d68d19) from gir-files (11e0e6d)
// DO NOT EDIT

use Bin;
use Container;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;

glib_wrapper! {
    pub struct EventBox(Object<ffi::GtkEventBox>): Bin, Container, Widget;

    match fn {
        get_type => || ffi::gtk_event_box_get_type(),
    }
}

impl EventBox {
    pub fn new() -> EventBox {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_event_box_new()).downcast_unchecked()
        }
    }

    pub fn get_above_child(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_event_box_get_above_child(self.to_glib_none().0))
        }
    }

    pub fn get_visible_window(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_event_box_get_visible_window(self.to_glib_none().0))
        }
    }

    pub fn set_above_child(&self, above_child: bool) {
        unsafe {
            ffi::gtk_event_box_set_above_child(self.to_glib_none().0, above_child.to_glib());
        }
    }

    pub fn set_visible_window(&self, visible_window: bool) {
        unsafe {
            ffi::gtk_event_box_set_visible_window(self.to_glib_none().0, visible_window.to_glib());
        }
    }
}
