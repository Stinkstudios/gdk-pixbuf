// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Pixbuf;
use PixbufAnimation;
use ffi;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct PixbufSimpleAnim(Object<ffi::GdkPixbufSimpleAnim, ffi::GdkPixbufSimpleAnimClass, PixbufSimpleAnimClass>) @extends PixbufAnimation;

    match fn {
        get_type => || ffi::gdk_pixbuf_simple_anim_get_type(),
    }
}

impl PixbufSimpleAnim {
    pub fn new(width: i32, height: i32, rate: f32) -> PixbufSimpleAnim {
        unsafe {
            from_glib_full(ffi::gdk_pixbuf_simple_anim_new(width, height, rate))
        }
    }
}

pub const NONE_PIXBUF_SIMPLE_ANIM: Option<&PixbufSimpleAnim> = None;

pub trait PixbufSimpleAnimExt: 'static {
    fn add_frame<P: IsA<Pixbuf>>(&self, pixbuf: &P);

    fn get_loop(&self) -> bool;

    fn set_loop(&self, loop_: bool);

    fn connect_property_loop_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<PixbufSimpleAnim>> PixbufSimpleAnimExt for O {
    fn add_frame<P: IsA<Pixbuf>>(&self, pixbuf: &P) {
        unsafe {
            ffi::gdk_pixbuf_simple_anim_add_frame(self.as_ref().to_glib_none().0, pixbuf.as_ref().to_glib_none().0);
        }
    }

    fn get_loop(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_pixbuf_simple_anim_get_loop(self.as_ref().to_glib_none().0))
        }
    }

    fn set_loop(&self, loop_: bool) {
        unsafe {
            ffi::gdk_pixbuf_simple_anim_set_loop(self.as_ref().to_glib_none().0, loop_.to_glib());
        }
    }

    fn connect_property_loop_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::loop\0".as_ptr() as *const _,
                transmute(notify_loop_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_loop_trampoline<P>(this: *mut ffi::GdkPixbufSimpleAnim, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<PixbufSimpleAnim> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&PixbufSimpleAnim::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for PixbufSimpleAnim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PixbufSimpleAnim")
    }
}
