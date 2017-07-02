// This file was generated by gir (33e9567) from gir-files (71d73f0)
// DO NOT EDIT

use Actionable;
use Bin;
use Button;
use Container;
use Orientable;
use ScaleButton;
use Widget;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use gobject_ffi;

glib_wrapper! {
    pub struct VolumeButton(Object<ffi::GtkVolumeButton>): ScaleButton, Button, Bin, Container, Widget, Actionable, Orientable;

    match fn {
        get_type => || ffi::gtk_volume_button_get_type(),
    }
}

impl VolumeButton {
    pub fn new() -> VolumeButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_volume_button_new()).downcast_unchecked()
        }
    }
}

pub trait VolumeButtonExt {
    fn get_property_use_symbolic(&self) -> bool;

    fn set_property_use_symbolic(&self, use_symbolic: bool);
}

impl<O: IsA<VolumeButton> + IsA<glib::object::Object>> VolumeButtonExt for O {
    fn get_property_use_symbolic(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "use-symbolic".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_use_symbolic(&self, use_symbolic: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "use-symbolic".to_glib_none().0, Value::from(&use_symbolic).to_glib_none().0);
        }
    }
}
