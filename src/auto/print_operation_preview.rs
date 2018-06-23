// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use PageSetup;
use PrintContext;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct PrintOperationPreview(Object<ffi::GtkPrintOperationPreview, ffi::GtkPrintOperationPreviewIface>);

    match fn {
        get_type => || ffi::gtk_print_operation_preview_get_type(),
    }
}

pub trait PrintOperationPreviewExt {
    fn end_preview(&self);

    fn is_selected(&self, page_nr: i32) -> bool;

    fn render_page(&self, page_nr: i32);

    fn connect_got_page_size<F: Fn(&Self, &PrintContext, &PageSetup) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_ready<F: Fn(&Self, &PrintContext) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<PrintOperationPreview> + IsA<glib::object::Object>> PrintOperationPreviewExt for O {
    fn end_preview(&self) {
        unsafe {
            ffi::gtk_print_operation_preview_end_preview(self.to_glib_none().0);
        }
    }

    fn is_selected(&self, page_nr: i32) -> bool {
        unsafe {
            from_glib(ffi::gtk_print_operation_preview_is_selected(self.to_glib_none().0, page_nr))
        }
    }

    fn render_page(&self, page_nr: i32) {
        unsafe {
            ffi::gtk_print_operation_preview_render_page(self.to_glib_none().0, page_nr);
        }
    }

    fn connect_got_page_size<F: Fn(&Self, &PrintContext, &PageSetup) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &PrintContext, &PageSetup) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "got-page-size",
                transmute(got_page_size_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_ready<F: Fn(&Self, &PrintContext) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &PrintContext) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "ready",
                transmute(ready_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn got_page_size_trampoline<P>(this: *mut ffi::GtkPrintOperationPreview, context: *mut ffi::GtkPrintContext, page_setup: *mut ffi::GtkPageSetup, f: glib_ffi::gpointer)
where P: IsA<PrintOperationPreview> {
    let f: &&(Fn(&P, &PrintContext, &PageSetup) + 'static) = transmute(f);
    f(&PrintOperationPreview::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(context), &from_glib_borrow(page_setup))
}

unsafe extern "C" fn ready_trampoline<P>(this: *mut ffi::GtkPrintOperationPreview, context: *mut ffi::GtkPrintContext, f: glib_ffi::gpointer)
where P: IsA<PrintOperationPreview> {
    let f: &&(Fn(&P, &PrintContext) + 'static) = transmute(f);
    f(&PrintOperationPreview::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(context))
}
