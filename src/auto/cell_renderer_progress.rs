// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use CellRenderer;
use Orientable;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct CellRendererProgress(Object<gtk_sys::GtkCellRendererProgress, gtk_sys::GtkCellRendererProgressClass, CellRendererProgressClass>) @extends CellRenderer, @implements Orientable;

    match fn {
        get_type => || gtk_sys::gtk_cell_renderer_progress_get_type(),
    }
}

impl CellRendererProgress {
    pub fn new() -> CellRendererProgress {
        assert_initialized_main_thread!();
        unsafe {
            CellRenderer::from_glib_none(gtk_sys::gtk_cell_renderer_progress_new()).unsafe_cast()
        }
    }
}

impl Default for CellRendererProgress {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_CELL_RENDERER_PROGRESS: Option<&CellRendererProgress> = None;

pub trait CellRendererProgressExt: 'static {
    fn get_property_inverted(&self) -> bool;

    fn set_property_inverted(&self, inverted: bool);

    fn get_property_pulse(&self) -> i32;

    fn set_property_pulse(&self, pulse: i32);

    fn get_property_text(&self) -> Option<GString>;

    fn set_property_text(&self, text: Option<&str>);

    fn get_property_text_xalign(&self) -> f32;

    fn set_property_text_xalign(&self, text_xalign: f32);

    fn get_property_text_yalign(&self) -> f32;

    fn set_property_text_yalign(&self, text_yalign: f32);

    fn get_property_value(&self) -> i32;

    fn set_property_value(&self, value: i32);

    fn connect_property_inverted_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_pulse_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_text_xalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_text_yalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<CellRendererProgress>> CellRendererProgressExt for O {
    fn get_property_inverted(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"inverted\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_inverted(&self, inverted: bool) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"inverted\0".as_ptr() as *const _, Value::from(&inverted).to_glib_none().0);
        }
    }

    fn get_property_pulse(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"pulse\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_pulse(&self, pulse: i32) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"pulse\0".as_ptr() as *const _, Value::from(&pulse).to_glib_none().0);
        }
    }

    fn get_property_text(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"text\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_text(&self, text: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"text\0".as_ptr() as *const _, Value::from(text).to_glib_none().0);
        }
    }

    fn get_property_text_xalign(&self) -> f32 {
        unsafe {
            let mut value = Value::from_type(<f32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"text-xalign\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_text_xalign(&self, text_xalign: f32) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"text-xalign\0".as_ptr() as *const _, Value::from(&text_xalign).to_glib_none().0);
        }
    }

    fn get_property_text_yalign(&self) -> f32 {
        unsafe {
            let mut value = Value::from_type(<f32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"text-yalign\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_text_yalign(&self, text_yalign: f32) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"text-yalign\0".as_ptr() as *const _, Value::from(&text_yalign).to_glib_none().0);
        }
    }

    fn get_property_value(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"value\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_value(&self, value: i32) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"value\0".as_ptr() as *const _, Value::from(&value).to_glib_none().0);
        }
    }

    fn connect_property_inverted_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::inverted\0".as_ptr() as *const _,
                Some(transmute(notify_inverted_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_pulse_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::pulse\0".as_ptr() as *const _,
                Some(transmute(notify_pulse_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::text\0".as_ptr() as *const _,
                Some(transmute(notify_text_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_text_xalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::text-xalign\0".as_ptr() as *const _,
                Some(transmute(notify_text_xalign_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_text_yalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::text-yalign\0".as_ptr() as *const _,
                Some(transmute(notify_text_yalign_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::value\0".as_ptr() as *const _,
                Some(transmute(notify_value_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_inverted_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkCellRendererProgress, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<CellRendererProgress> {
    let f: &F = &*(f as *const F);
    f(&CellRendererProgress::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_pulse_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkCellRendererProgress, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<CellRendererProgress> {
    let f: &F = &*(f as *const F);
    f(&CellRendererProgress::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_text_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkCellRendererProgress, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<CellRendererProgress> {
    let f: &F = &*(f as *const F);
    f(&CellRendererProgress::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_text_xalign_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkCellRendererProgress, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<CellRendererProgress> {
    let f: &F = &*(f as *const F);
    f(&CellRendererProgress::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_text_yalign_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkCellRendererProgress, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<CellRendererProgress> {
    let f: &F = &*(f as *const F);
    f(&CellRendererProgress::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_value_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkCellRendererProgress, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<CellRendererProgress> {
    let f: &F = &*(f as *const F);
    f(&CellRendererProgress::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for CellRendererProgress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CellRendererProgress")
    }
}
