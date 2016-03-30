// This file was generated by gir (becf3b4) from gir-files (11e0e6d)
// DO NOT EDIT

use EventController;
use Gesture;
#[cfg(feature = "v3_14")]
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct GestureZoom(Object<ffi::GtkGestureZoom>): Gesture, EventController;

    match fn {
        get_type => || ffi::gtk_gesture_zoom_get_type(),
    }
}

impl GestureZoom {
    #[cfg(feature = "v3_14")]
    pub fn new<T: IsA<Widget>>(widget: &T) -> GestureZoom {
        skip_assert_initialized!();
        unsafe {
            Gesture::from_glib_full(ffi::gtk_gesture_zoom_new(widget.to_glib_none().0)).downcast_unchecked()
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn get_scale_delta(&self) -> f64 {
        unsafe {
            ffi::gtk_gesture_zoom_get_scale_delta(self.to_glib_none().0)
        }
    }
}
