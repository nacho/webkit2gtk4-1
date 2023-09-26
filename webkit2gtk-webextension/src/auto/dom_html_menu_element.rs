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
    #[doc(alias = "WebKitDOMHTMLMenuElement")]
    pub struct DOMHTMLMenuElement(Object<ffi::WebKitDOMHTMLMenuElement, ffi::WebKitDOMHTMLMenuElementClass>) @extends DOMHTMLElement, DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        type_ => || ffi::webkit_dom_html_menu_element_get_type(),
    }
}

impl DOMHTMLMenuElement {
    pub const NONE: Option<&'static DOMHTMLMenuElement> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::DOMHTMLMenuElement>> Sealed for T {}
}

pub trait DOMHTMLMenuElementExt: IsA<DOMHTMLMenuElement> + sealed::Sealed + 'static {
    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_menu_element_get_compact")]
    #[doc(alias = "get_compact")]
    fn is_compact(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_menu_element_get_compact(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_menu_element_set_compact")]
    fn set_compact(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_menu_element_set_compact(
                self.as_ref().to_glib_none().0,
                value.into_glib(),
            );
        }
    }

    #[doc(alias = "compact")]
    fn connect_compact_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_compact_trampoline<
            P: IsA<DOMHTMLMenuElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLMenuElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLMenuElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::compact\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_compact_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<DOMHTMLMenuElement>> DOMHTMLMenuElementExt for O {}

impl fmt::Display for DOMHTMLMenuElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMHTMLMenuElement")
    }
}
