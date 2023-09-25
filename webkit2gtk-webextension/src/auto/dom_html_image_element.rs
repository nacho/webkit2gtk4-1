// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT
#![allow(deprecated)]

use crate::{DOMElement, DOMEventTarget, DOMHTMLElement, DOMNode, DOMObject};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "WebKitDOMHTMLImageElement")]
    pub struct DOMHTMLImageElement(Object<ffi::WebKitDOMHTMLImageElement, ffi::WebKitDOMHTMLImageElementClass>) @extends DOMHTMLElement, DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        type_ => || ffi::webkit_dom_html_image_element_get_type(),
    }
}

impl DOMHTMLImageElement {
    pub const NONE: Option<&'static DOMHTMLImageElement> = None;
}

pub trait DOMHTMLImageElementExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_image_element_get_align")]
    #[doc(alias = "get_align")]
    fn align(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_image_element_get_alt")]
    #[doc(alias = "get_alt")]
    fn alt(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_image_element_get_border")]
    #[doc(alias = "get_border")]
    fn border(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_image_element_get_complete")]
    #[doc(alias = "get_complete")]
    fn is_complete(&self) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_image_element_get_height")]
    #[doc(alias = "get_height")]
    fn height(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_image_element_get_hspace")]
    #[doc(alias = "get_hspace")]
    fn hspace(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_image_element_get_is_map")]
    #[doc(alias = "get_is_map")]
    fn is_map(&self) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_image_element_get_long_desc")]
    #[doc(alias = "get_long_desc")]
    fn long_desc(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_image_element_get_lowsrc")]
    #[doc(alias = "get_lowsrc")]
    fn lowsrc(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_image_element_get_name")]
    #[doc(alias = "get_name")]
    fn name(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_image_element_get_natural_height")]
    #[doc(alias = "get_natural_height")]
    fn natural_height(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_image_element_get_natural_width")]
    #[doc(alias = "get_natural_width")]
    fn natural_width(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_image_element_get_src")]
    #[doc(alias = "get_src")]
    fn src(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_image_element_get_use_map")]
    #[doc(alias = "get_use_map")]
    fn use_map(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_image_element_get_vspace")]
    #[doc(alias = "get_vspace")]
    fn vspace(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_image_element_get_width")]
    #[doc(alias = "get_width")]
    fn width(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_image_element_get_x")]
    #[doc(alias = "get_x")]
    fn x(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_image_element_get_y")]
    #[doc(alias = "get_y")]
    fn y(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_image_element_set_align")]
    fn set_align(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_image_element_set_alt")]
    fn set_alt(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_image_element_set_border")]
    fn set_border(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_image_element_set_height")]
    fn set_height(&self, value: libc::c_long);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_image_element_set_hspace")]
    fn set_hspace(&self, value: libc::c_long);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_image_element_set_is_map")]
    fn set_is_map(&self, value: bool);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_image_element_set_long_desc")]
    fn set_long_desc(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_image_element_set_lowsrc")]
    fn set_lowsrc(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_image_element_set_name")]
    fn set_name(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_image_element_set_src")]
    fn set_src(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_image_element_set_use_map")]
    fn set_use_map(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_image_element_set_vspace")]
    fn set_vspace(&self, value: libc::c_long);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_image_element_set_width")]
    fn set_width(&self, value: libc::c_long);

    #[doc(alias = "align")]
    fn connect_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "alt")]
    fn connect_alt_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "border")]
    fn connect_border_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "complete")]
    fn connect_complete_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "height")]
    fn connect_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "hspace")]
    fn connect_hspace_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "is-map")]
    fn connect_is_map_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "long-desc")]
    fn connect_long_desc_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "lowsrc")]
    fn connect_lowsrc_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "name")]
    fn connect_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "natural-height")]
    fn connect_natural_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "natural-width")]
    fn connect_natural_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "src")]
    fn connect_src_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "use-map")]
    fn connect_use_map_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "vspace")]
    fn connect_vspace_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "width")]
    fn connect_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "x")]
    fn connect_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "y")]
    fn connect_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLImageElement>> DOMHTMLImageElementExt for O {
    #[allow(deprecated)]
    fn align(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_image_element_get_align(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[allow(deprecated)]
    fn alt(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_image_element_get_alt(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[allow(deprecated)]
    fn border(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_image_element_get_border(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[allow(deprecated)]
    fn is_complete(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_image_element_get_complete(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[allow(deprecated)]
    fn height(&self) -> libc::c_long {
        unsafe { ffi::webkit_dom_html_image_element_get_height(self.as_ref().to_glib_none().0) }
    }

    #[allow(deprecated)]
    fn hspace(&self) -> libc::c_long {
        unsafe { ffi::webkit_dom_html_image_element_get_hspace(self.as_ref().to_glib_none().0) }
    }

    #[allow(deprecated)]
    fn is_map(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_image_element_get_is_map(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[allow(deprecated)]
    fn long_desc(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_image_element_get_long_desc(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[allow(deprecated)]
    fn lowsrc(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_image_element_get_lowsrc(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[allow(deprecated)]
    fn name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_image_element_get_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[allow(deprecated)]
    fn natural_height(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_image_element_get_natural_height(self.as_ref().to_glib_none().0)
        }
    }

    #[allow(deprecated)]
    fn natural_width(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_image_element_get_natural_width(self.as_ref().to_glib_none().0)
        }
    }

    #[allow(deprecated)]
    fn src(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_image_element_get_src(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[allow(deprecated)]
    fn use_map(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_image_element_get_use_map(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[allow(deprecated)]
    fn vspace(&self) -> libc::c_long {
        unsafe { ffi::webkit_dom_html_image_element_get_vspace(self.as_ref().to_glib_none().0) }
    }

    #[allow(deprecated)]
    fn width(&self) -> libc::c_long {
        unsafe { ffi::webkit_dom_html_image_element_get_width(self.as_ref().to_glib_none().0) }
    }

    #[allow(deprecated)]
    fn x(&self) -> libc::c_long {
        unsafe { ffi::webkit_dom_html_image_element_get_x(self.as_ref().to_glib_none().0) }
    }

    #[allow(deprecated)]
    fn y(&self) -> libc::c_long {
        unsafe { ffi::webkit_dom_html_image_element_get_y(self.as_ref().to_glib_none().0) }
    }

    #[allow(deprecated)]
    fn set_align(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_image_element_set_align(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[allow(deprecated)]
    fn set_alt(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_image_element_set_alt(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[allow(deprecated)]
    fn set_border(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_image_element_set_border(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[allow(deprecated)]
    fn set_height(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_image_element_set_height(self.as_ref().to_glib_none().0, value);
        }
    }

    #[allow(deprecated)]
    fn set_hspace(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_image_element_set_hspace(self.as_ref().to_glib_none().0, value);
        }
    }

    #[allow(deprecated)]
    fn set_is_map(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_image_element_set_is_map(
                self.as_ref().to_glib_none().0,
                value.into_glib(),
            );
        }
    }

    #[allow(deprecated)]
    fn set_long_desc(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_image_element_set_long_desc(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[allow(deprecated)]
    fn set_lowsrc(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_image_element_set_lowsrc(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[allow(deprecated)]
    fn set_name(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_image_element_set_name(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[allow(deprecated)]
    fn set_src(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_image_element_set_src(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[allow(deprecated)]
    fn set_use_map(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_image_element_set_use_map(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[allow(deprecated)]
    fn set_vspace(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_image_element_set_vspace(self.as_ref().to_glib_none().0, value);
        }
    }

    #[allow(deprecated)]
    fn set_width(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_image_element_set_width(self.as_ref().to_glib_none().0, value);
        }
    }

    fn connect_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_align_trampoline<
            P: IsA<DOMHTMLImageElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLImageElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLImageElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::align\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_align_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_alt_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_alt_trampoline<
            P: IsA<DOMHTMLImageElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLImageElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLImageElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::alt\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_alt_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_border_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_border_trampoline<
            P: IsA<DOMHTMLImageElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLImageElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLImageElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::border\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_border_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_complete_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_complete_trampoline<
            P: IsA<DOMHTMLImageElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLImageElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLImageElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::complete\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_complete_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_height_trampoline<
            P: IsA<DOMHTMLImageElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLImageElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLImageElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::height\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_height_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_hspace_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_hspace_trampoline<
            P: IsA<DOMHTMLImageElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLImageElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLImageElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::hspace\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_hspace_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_is_map_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_map_trampoline<
            P: IsA<DOMHTMLImageElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLImageElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLImageElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-map\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_map_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_long_desc_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_long_desc_trampoline<
            P: IsA<DOMHTMLImageElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLImageElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLImageElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::long-desc\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_long_desc_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_lowsrc_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_lowsrc_trampoline<
            P: IsA<DOMHTMLImageElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLImageElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLImageElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::lowsrc\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_lowsrc_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<
            P: IsA<DOMHTMLImageElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLImageElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLImageElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_natural_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_natural_height_trampoline<
            P: IsA<DOMHTMLImageElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLImageElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLImageElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::natural-height\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_natural_height_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_natural_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_natural_width_trampoline<
            P: IsA<DOMHTMLImageElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLImageElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLImageElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::natural-width\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_natural_width_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_src_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_src_trampoline<
            P: IsA<DOMHTMLImageElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLImageElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLImageElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::src\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_src_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_use_map_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_map_trampoline<
            P: IsA<DOMHTMLImageElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLImageElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLImageElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::use-map\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_use_map_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_vspace_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_vspace_trampoline<
            P: IsA<DOMHTMLImageElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLImageElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLImageElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::vspace\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_vspace_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_width_trampoline<
            P: IsA<DOMHTMLImageElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLImageElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLImageElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::width\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_width_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_x_trampoline<
            P: IsA<DOMHTMLImageElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLImageElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLImageElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::x\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_x_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_y_trampoline<
            P: IsA<DOMHTMLImageElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLImageElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLImageElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::y\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_y_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DOMHTMLImageElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMHTMLImageElement")
    }
}
