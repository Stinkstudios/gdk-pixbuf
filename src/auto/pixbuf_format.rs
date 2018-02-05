// This file was generated by gir (0f1d1c1) from gir-files (77d1f70)
// DO NOT EDIT

use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct PixbufFormat(Boxed<ffi::GdkPixbufFormat>);

    match fn {
        copy => |ptr| ffi::gdk_pixbuf_format_copy(mut_override(ptr)),
        free => |ptr| ffi::gdk_pixbuf_format_free(ptr),
        get_type => || ffi::gdk_pixbuf_format_get_type(),
    }
}

impl PixbufFormat {
    pub fn get_description(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gdk_pixbuf_format_get_description(mut_override(self.to_glib_none().0)))
        }
    }

    pub fn get_extensions(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gdk_pixbuf_format_get_extensions(mut_override(self.to_glib_none().0)))
        }
    }

    pub fn get_license(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gdk_pixbuf_format_get_license(mut_override(self.to_glib_none().0)))
        }
    }

    pub fn get_mime_types(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gdk_pixbuf_format_get_mime_types(mut_override(self.to_glib_none().0)))
        }
    }

    pub fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gdk_pixbuf_format_get_name(mut_override(self.to_glib_none().0)))
        }
    }

    pub fn is_disabled(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_pixbuf_format_is_disabled(mut_override(self.to_glib_none().0)))
        }
    }

    #[cfg(any(feature = "v2_36", feature = "dox"))]
    pub fn is_save_option_supported(&self, option_key: &str) -> bool {
        unsafe {
            from_glib(ffi::gdk_pixbuf_format_is_save_option_supported(mut_override(self.to_glib_none().0), option_key.to_glib_none().0))
        }
    }

    pub fn is_scalable(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_pixbuf_format_is_scalable(mut_override(self.to_glib_none().0)))
        }
    }

    pub fn is_writable(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_pixbuf_format_is_writable(mut_override(self.to_glib_none().0)))
        }
    }

    pub fn set_disabled(&mut self, disabled: bool) {
        unsafe {
            ffi::gdk_pixbuf_format_set_disabled(self.to_glib_none_mut().0, disabled.to_glib());
        }
    }
}
