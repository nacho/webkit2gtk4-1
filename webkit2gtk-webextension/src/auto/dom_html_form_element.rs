// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT
#![allow(deprecated)]

use crate::{DOMElement, DOMEventTarget, DOMHTMLCollection, DOMHTMLElement, DOMNode, DOMObject};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "WebKitDOMHTMLFormElement")]
    pub struct DOMHTMLFormElement(Object<ffi::WebKitDOMHTMLFormElement, ffi::WebKitDOMHTMLFormElementClass>) @extends DOMHTMLElement, DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        type_ => || ffi::webkit_dom_html_form_element_get_type(),
    }
}

impl DOMHTMLFormElement {
    pub const NONE: Option<&'static DOMHTMLFormElement> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::DOMHTMLFormElement>> Sealed for T {}
}

pub trait DOMHTMLFormElementExt: IsA<DOMHTMLFormElement> + sealed::Sealed + 'static {
    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_form_element_get_accept_charset")]
    #[doc(alias = "get_accept_charset")]
    fn accept_charset(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_form_element_get_accept_charset(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_form_element_get_action")]
    #[doc(alias = "get_action")]
    fn action(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_form_element_get_action(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_form_element_get_elements")]
    #[doc(alias = "get_elements")]
    fn elements(&self) -> Option<DOMHTMLCollection> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_form_element_get_elements(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_form_element_get_encoding")]
    #[doc(alias = "get_encoding")]
    fn encoding(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_form_element_get_encoding(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_form_element_get_enctype")]
    #[doc(alias = "get_enctype")]
    fn enctype(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_form_element_get_enctype(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_form_element_get_length")]
    #[doc(alias = "get_length")]
    fn length(&self) -> libc::c_long {
        unsafe { ffi::webkit_dom_html_form_element_get_length(self.as_ref().to_glib_none().0) }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_form_element_get_method")]
    #[doc(alias = "get_method")]
    fn method(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_form_element_get_method(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_form_element_get_name")]
    #[doc(alias = "get_name")]
    fn name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_form_element_get_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_form_element_get_target")]
    #[doc(alias = "get_target")]
    fn target(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_form_element_get_target(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_form_element_reset")]
    fn reset(&self) {
        unsafe {
            ffi::webkit_dom_html_form_element_reset(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_form_element_set_accept_charset")]
    fn set_accept_charset(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_form_element_set_accept_charset(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_form_element_set_action")]
    fn set_action(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_form_element_set_action(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_form_element_set_encoding")]
    fn set_encoding(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_form_element_set_encoding(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_form_element_set_enctype")]
    fn set_enctype(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_form_element_set_enctype(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_form_element_set_method")]
    fn set_method(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_form_element_set_method(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_form_element_set_name")]
    fn set_name(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_form_element_set_name(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_form_element_set_target")]
    fn set_target(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_form_element_set_target(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_form_element_submit")]
    fn submit(&self) {
        unsafe {
            ffi::webkit_dom_html_form_element_submit(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "accept-charset")]
    fn connect_accept_charset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_accept_charset_trampoline<
            P: IsA<DOMHTMLFormElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLFormElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLFormElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::accept-charset\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_accept_charset_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "action")]
    fn connect_action_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_action_trampoline<
            P: IsA<DOMHTMLFormElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLFormElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLFormElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::action\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_action_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "elements")]
    fn connect_elements_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_elements_trampoline<
            P: IsA<DOMHTMLFormElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLFormElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLFormElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::elements\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_elements_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "encoding")]
    fn connect_encoding_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_encoding_trampoline<
            P: IsA<DOMHTMLFormElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLFormElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLFormElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::encoding\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_encoding_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "enctype")]
    fn connect_enctype_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_enctype_trampoline<
            P: IsA<DOMHTMLFormElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLFormElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLFormElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::enctype\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_enctype_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "length")]
    fn connect_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_length_trampoline<
            P: IsA<DOMHTMLFormElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLFormElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLFormElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::length\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_length_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "method")]
    fn connect_method_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_method_trampoline<
            P: IsA<DOMHTMLFormElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLFormElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLFormElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::method\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_method_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "name")]
    fn connect_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<
            P: IsA<DOMHTMLFormElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLFormElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLFormElement::from_glib_borrow(this).unsafe_cast_ref())
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

    #[doc(alias = "target")]
    fn connect_target_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_target_trampoline<
            P: IsA<DOMHTMLFormElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLFormElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLFormElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::target\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_target_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<DOMHTMLFormElement>> DOMHTMLFormElementExt for O {}

impl fmt::Display for DOMHTMLFormElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMHTMLFormElement")
    }
}
